use std::path::Path;
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs;

use crate::active_release::active_release::ActiveReleaseError;
use crate::active_release::repository::ActiveReleaseRepository;
use crate::fetch_releases::repository::ReleasesRepository;
use crate::filesystem::paths::{
  AssetDownloadDirError, AssetExtractionDirError,
  get_or_create_asset_download_dir,
  get_or_create_asset_installation_dir,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::download::Downloader;
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::utils::{Arch, OS};
use crate::install_release::installation_status::status::GetInstallationStatusError;

/// Errors that can occur during the release installation process.
#[derive(thiserror::Error, Debug)]
pub enum ReleaseInstallationError {
  /// Failed to determine or create the download directory.
  #[error("failed to get download directory: {0}")]
  DownloadDir(#[from] AssetDownloadDirError),

  /// Failed to determine or create the extraction directory.
  #[error("failed to get extraction directory: {0}")]
  ExtractionDir(#[from] AssetExtractionDirError),

  /// An error occurred with the underlying downloader.
  #[error("failed to create downloader: {0}")]
  Downloader(#[from] downloader::Error),

  /// No compatible asset (OS/Arch) was found for this release.
  #[error("no compatible asset found")]
  NoCompatibleAsset,

  /// Failed to download the release asset.
  #[error("failed to download asset: {0}")]
  Download(#[from] AssetDownloadError),

  /// Failed to extract the downloaded archive.
  #[error("failed to extract asset: {0}")]
  Extract(#[from] ExtractionError),

  /// Failed to determine the release status during or after installation.
  #[error("failed to get release status: {0}")]
  ReleaseStatus(#[from] GetInstallationStatusError),

  /// Failed to update the active release in the repository.
  #[error("failed to set active release: {0}")]
  ActiveRelease(#[from] ActiveReleaseError),
}

impl GameRelease {
  /// Installs the game release.
  ///
  /// This process involves:
  /// 1. Checking the current status.
  /// 2. Downloading the appropriate asset if not already downloaded.
  /// 3. Extracting the asset to the installation directory.
  /// 4. Setting this release as the active one.
  /// 5. Cleaning up the downloaded archive and other old installations.
  #[allow(clippy::too_many_arguments)]
  pub async fn install_release(
    &mut self,
    downloader: &Downloader,
    os: &OS,
    arch: &Arch,
    data_dir: &Path,
    resources_dir: &Path,
    releases_repository: &dyn ReleasesRepository,
    active_release_repository: &dyn ActiveReleaseRepository,
    progress: Arc<dyn Reporter + Send + Sync>,
  ) -> Result<(), ReleaseInstallationError> {
    if self.status == GameReleaseStatus::Unknown {
      self.status =
        self.get_installation_status(os, data_dir).await?;
    }

    if self.status == GameReleaseStatus::ReadyToPlay {
      self
        .variant
        .set_active_release(&self.version, active_release_repository)
        .await?;
      return Ok(());
    }

    let download_dir =
      get_or_create_asset_download_dir(&self.variant, data_dir)
        .await?;
    let asset = self
      .get_asset(os, arch, resources_dir, releases_repository)
      .await
      .ok_or(ReleaseInstallationError::NoCompatibleAsset)?;

    if self.status == GameReleaseStatus::NotDownloaded
      || self.status == GameReleaseStatus::Corrupted
      || self.status == GameReleaseStatus::Unknown
    {
      asset.download(downloader, &download_dir, progress).await?;
      self.status = GameReleaseStatus::NotInstalled;
    }

    let download_filepath = download_dir.join(&asset.name);
    let installation_dir = get_or_create_asset_installation_dir(
      &self.variant,
      &self.version,
      data_dir,
    )
    .await?;

    extract_archive(&download_filepath, &installation_dir, os)
      .await?;

    self.status = GameReleaseStatus::ReadyToPlay;

    self
      .variant
      .set_active_release(&self.version, active_release_repository)
      .await?;

    // Failure to remove file does not mean failure to install
    let _ = fs::remove_file(&download_filepath).await;

    delete_other_installations(&installation_dir).await;

    Ok(())
  }
}

async fn delete_other_installations(installation_dir: &Path) {
  let Some(parent) = installation_dir.parent() else {
    return;
  };

  let Ok(mut entries) = fs::read_dir(parent).await else {
    return;
  };

  let Ok(kept_path) = fs::canonicalize(installation_dir).await else {
    return;
  };

  while let Ok(Some(entry)) = entries.next_entry().await {
    let path = entry.path();

    let Ok(metadata) = fs::metadata(&path).await else {
      continue;
    };

    if !metadata.is_dir() {
      continue;
    }

    let Ok(canonical_path) = fs::canonicalize(&path).await else {
      continue;
    };

    if canonical_path != kept_path {
      let _ = fs::remove_dir_all(&path).await;
    }
  }
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::filesystem::paths::{
    get_game_executable_filepath,
    get_or_create_asset_download_dir, get_or_create_asset_installation_dir,
  };
  use crate::game_release::game_release::{
    GameRelease, GameReleaseStatus, ReleaseType,
  };
  use crate::game_release::utils::get_platform_asset_substrs;
  use crate::infra::github::asset::GitHubAsset;
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::http_client::ReqwestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::{Arch, OS};
  use crate::variants::GameVariant;
  use chrono::Utc;
  use downloader::progress::Reporter;
  use github_mock_api::{Asset as MockAsset, MockServer};
  use std::num::NonZeroU16;
  use std::path::PathBuf;
  use std::sync::Arc;
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  struct DummyReporter;
  impl Reporter for DummyReporter {
    fn setup(&self, _max_progress: Option<u64>, _message: &str) {}
    fn progress(&self, _current: u64) {}
    fn set_message(&self, _message: &str) {}
    fn done(&self) {}
  }

  fn create_downloader() -> Downloader {
    let client = ReqwestHttpClient::new(reqwest::Client::new());
    Downloader::new(
      client,
      NonZeroU16::new(1).expect("non zero parallel requests"),
    )
  }

  fn get_test_assets_dir(variant: GameVariant) -> PathBuf {
    let variant_dir = match variant {
      GameVariant::DarkDaysAhead => "dda",
      GameVariant::BrightNights => "bn",
      GameVariant::TheLastGeneration => "tlg",
    };
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("src")
      .join("infra")
      .join("testing")
      .join("data")
      .join("assets")
      .join(variant_dir)
  }

  fn get_test_archive_path(
    variant: GameVariant,
    os: &OS,
    arch: &Arch,
  ) -> PathBuf {
    let extension = match os {
      OS::Linux => "tar.gz",
      OS::Windows => "zip",
      OS::Mac => "dmg",
    };
    let substrings = get_platform_asset_substrs(&variant, os, arch);
    let assets_dir = get_test_assets_dir(variant);
    let mut matches: Vec<PathBuf> = std::fs::read_dir(&assets_dir)
      .expect("failed to read test assets dir")
      .flatten()
      .map(|entry| entry.path())
      .filter(|path| {
        let Some(name) =
          path.file_name().and_then(|name| name.to_str())
        else {
          return false;
        };
        name.ends_with(extension)
          && substrings.iter().any(|substr| name.contains(substr))
      })
      .collect();
    matches.sort_by_key(|path| {
      let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_string();
      let size = std::fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(u64::MAX);
      (size, name)
    });
    matches
      .into_iter()
      .next()
      .expect("no test archive found for the given variant/platform")
  }

  fn get_test_asset_name(archive_path: &Path) -> String {
    archive_path
      .file_name()
      .and_then(|name| name.to_str())
      .expect("test archive name is not valid utf-8")
      .to_string()
  }

  fn create_test_release(
    variant: GameVariant,
    version: &str,
    status: GameReleaseStatus,
  ) -> GameRelease {
    GameRelease {
      variant,
      version: version.to_string(),
      body: Some("Test release notes".to_string()),
      release_type: ReleaseType::Experimental,
      status,
      created_at: Utc::now(),
    }
  }

  const PLATFORMS: [(OS, Arch); 4] = [
    (OS::Linux, Arch::X64),
    (OS::Windows, Arch::X64),
    (OS::Mac, Arch::ARM64),
    (OS::Mac, Arch::X64),
  ];

  /// Expected first asset substring for each supported platform combination.
  fn expected_asset_substr(
    variant: GameVariant,
    os: &OS,
    arch: &Arch,
  ) -> &'static str {
    match (variant, os, arch) {
      (GameVariant::DarkDaysAhead, OS::Windows, _) => {
        "windows-with-graphics-and-sounds"
      }
      (GameVariant::DarkDaysAhead, OS::Mac, _) => "osx-with-graphics",
      (GameVariant::DarkDaysAhead, OS::Linux, _) => {
        "linux-with-graphics-and-sounds"
      }
      (GameVariant::BrightNights, OS::Windows, _) => "windows-tiles",
      (GameVariant::BrightNights, OS::Mac, Arch::ARM64) => {
        "osx-tiles-arm"
      }
      (GameVariant::BrightNights, OS::Mac, Arch::X64) => {
        "osx-tiles-x64"
      }
      (GameVariant::BrightNights, OS::Linux, _) => "linux-tiles",
      (GameVariant::TheLastGeneration, OS::Windows, _) => {
        "windows-tiles-sounds-x64-msvc"
      }
      (GameVariant::TheLastGeneration, OS::Mac, _) => {
        "osx-tiles-universal"
      }
      (GameVariant::TheLastGeneration, OS::Linux, _) => {
        "linux-tiles-sounds"
      }
    }
  }

  async fn setup_test_repos() -> TestResult<(
    TestDatabase,
    SqliteReleasesRepository,
    SqliteActiveReleaseRepository,
  )> {
    let db = TestDatabase::builder().build()?;
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    Ok((db, releases_repo, active_repo))
  }

  #[tokio::test]
  async fn test_install_release_already_ready_to_play() -> TestResult
  {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let mut release = create_test_release(
        variant,
        "v1.0.0",
        GameReleaseStatus::ReadyToPlay,
      );

      release
        .install_release(
          &downloader,
          &OS::Linux,
          &Arch::X64,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await?;

      assert_eq!(release.status, GameReleaseStatus::ReadyToPlay);
      let active = active_repo.get_active_release(&variant).await?;
      assert_eq!(active, Some("v1.0.0".to_string()));
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_unknown_status_already_installed()
  -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";

      let install_dir = get_or_create_asset_installation_dir(
        &variant,
        version,
        temp_data.path(),
      )
      .await?;

      let game_sub_dir = install_dir.join("cataclysm-dda");
      tokio::fs::create_dir_all(&game_sub_dir).await?;
      tokio::fs::write(
        game_sub_dir.join("cataclysm-launcher"),
        b"dummy",
      )
      .await?;

      let mut release = create_test_release(
        variant,
        version,
        GameReleaseStatus::Unknown,
      );

      release
        .install_release(
          &downloader,
          &OS::Linux,
          &Arch::X64,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await?;

      assert_eq!(release.status, GameReleaseStatus::ReadyToPlay);
      let active = active_repo.get_active_release(&variant).await?;
      assert_eq!(active, Some(version.to_string()));
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_no_compatible_asset() -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 101 + variant_index as u64,
            tag_name: "v1.0.0".to_string(),
            prerelease: false,
            body: Some("body".to_string()),
            assets: vec![crate::infra::github::asset::GitHubAsset {
              id: 1 + variant_index as u64,
              name: "cdda-windows-x64-2026-01-01.zip".to_string(),
              browser_download_url: "http://127.0.0.1/windows.zip"
                .to_string(),
              digest: None,
            }],
            created_at: Utc::now(),
          }],
        )
        .await?;

      let mut release = create_test_release(
        variant,
        "v1.0.0",
        GameReleaseStatus::NotDownloaded,
      );

      let result = release
        .install_release(
          &downloader,
          &OS::Linux,
          &Arch::X64,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await;

      assert!(matches!(
        result,
        Err(ReleaseInstallationError::NoCompatibleAsset)
      ));
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_successful_download_and_extract()
  -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let server = MockServer::start().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      let repo_full =
        crate::infra::utils::get_github_repo_for_variant(&variant);
      let parts: Vec<&str> = repo_full.split('/').collect();
      let (owner, repo_name) = (parts[0], parts[1]);
      let version = "cdda-exp-1";
      let os = OS::Linux;
      let arch = Arch::X64;

      let archive_path = get_test_archive_path(variant, &os, &arch);
      let asset_name = get_test_asset_name(&archive_path);
      let content_type = if asset_name.ends_with(".zip") {
        "application/zip"
      } else {
        "application/gzip"
      };

      let mock_asset = MockAsset::from_path(
        &asset_name,
        &archive_path,
        content_type,
      );
      server
        .add_asset(owner, repo_name, version, mock_asset)
        .await;

      let download_url = format!(
        "{}/{}/{}/releases/download/{}/{}",
        server.uri(),
        owner,
        repo_name,
        version,
        asset_name
      );

      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 12345 + variant_index as u64,
            tag_name: version.to_string(),
            prerelease: false,
            body: Some("body".to_string()),
            assets: vec![GitHubAsset {
              id: 6789 + variant_index as u64,
              browser_download_url: download_url.clone(),
              name: asset_name.clone(),
              digest: None,
            }],
            created_at: Utc::now(),
          }],
        )
        .await?;

      let mut release = create_test_release(
        variant,
        version,
        GameReleaseStatus::NotDownloaded,
      );

      release
        .install_release(
          &downloader,
          &os,
          &arch,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await?;

      assert_eq!(release.status, GameReleaseStatus::ReadyToPlay);

      let active = active_repo.get_active_release(&variant).await?;
      assert_eq!(active, Some(version.to_string()));

      let download_dir =
        get_or_create_asset_download_dir(&variant, temp_data.path())
          .await?;
      let downloaded_file = download_dir.join(&asset_name);
      assert!(
        !downloaded_file.exists(),
        "Downloaded archive file should be removed after extraction"
      );

      let executable = get_game_executable_filepath(
        &variant,
        version,
        temp_data.path(),
        &os,
      )
      .await?;
      assert!(
        executable.is_file(),
        "Game executable should exist after extraction: {}",
        executable.display()
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_already_downloaded_not_installed()
  -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      let version = "cdda-local-1";
      let os = OS::Linux;
      let arch = Arch::X64;

      let archive_path = get_test_archive_path(variant, &os, &arch);
      let asset_name = get_test_asset_name(&archive_path);

      let download_dir =
        get_or_create_asset_download_dir(&variant, temp_data.path())
          .await?;
      let downloaded_archive = download_dir.join(&asset_name);
      tokio::fs::copy(&archive_path, &downloaded_archive).await?;

      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 111 + variant_index as u64,
            tag_name: version.to_string(),
            prerelease: false,
            body: Some("body".to_string()),
            assets: vec![GitHubAsset {
              id: 222 + variant_index as u64,
              browser_download_url: "http://invalid.local/file.zip"
                .to_string(),
              name: asset_name.clone(),
              digest: None,
            }],
            created_at: Utc::now(),
          }],
        )
        .await?;

      let mut release = create_test_release(
        variant,
        version,
        GameReleaseStatus::NotInstalled,
      );

      release
        .install_release(
          &downloader,
          &os,
          &arch,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await?;

      assert_eq!(release.status, GameReleaseStatus::ReadyToPlay);
      assert!(!downloaded_archive.exists());

      let executable = get_game_executable_filepath(
        &variant,
        version,
        temp_data.path(),
        &os,
      )
      .await?;
      assert!(
        executable.is_file(),
        "Extracted executable file should exist in installation directory: {}",
        executable.display()
      );

      let active = active_repo.get_active_release(&variant).await?;
      assert_eq!(active, Some(version.to_string()));
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_deletes_other_installations()
  -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      let os = OS::Linux;
      let arch = Arch::X64;

      let old_version = "v0.1.0";
      let old_install_dir = get_or_create_asset_installation_dir(
        &variant,
        old_version,
        temp_data.path(),
      )
      .await?;
      let old_sub_dir = old_install_dir.join("cataclysm-dda");
      tokio::fs::create_dir_all(&old_sub_dir).await?;
      tokio::fs::write(
        old_sub_dir.join("cataclysm-launcher"),
        b"old",
      )
      .await?;

      let new_version = "v0.2.0";
      let archive_path = get_test_archive_path(variant, &os, &arch);
      let asset_name = get_test_asset_name(&archive_path);

      let download_dir =
        get_or_create_asset_download_dir(&variant, temp_data.path())
          .await?;
      let downloaded_archive = download_dir.join(&asset_name);
      tokio::fs::copy(&archive_path, &downloaded_archive).await?;

      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 333 + variant_index as u64,
            tag_name: new_version.to_string(),
            prerelease: false,
            body: Some("body".to_string()),
            assets: vec![GitHubAsset {
              id: 444 + variant_index as u64,
              browser_download_url: "http://invalid.local/file.zip"
                .to_string(),
              name: asset_name.clone(),
              digest: None,
            }],
            created_at: Utc::now(),
          }],
        )
        .await?;

      let mut release = create_test_release(
        variant,
        new_version,
        GameReleaseStatus::NotInstalled,
      );

      release
        .install_release(
          &downloader,
          &os,
          &arch,
          temp_data.path(),
          temp_res.path(),
          &releases_repo,
          &active_repo,
          Arc::new(DummyReporter),
        )
        .await?;

      assert_eq!(release.status, GameReleaseStatus::ReadyToPlay);
      assert!(
        !old_install_dir.exists(),
        "Old installation directory should be deleted"
      );

      let new_install_dir = get_or_create_asset_installation_dir(
        &variant,
        new_version,
        temp_data.path(),
      )
      .await?;
      assert!(
        new_install_dir.exists(),
        "New installation directory should exist"
      );

      let executable = get_game_executable_filepath(
        &variant,
        new_version,
        temp_data.path(),
        &os,
      )
      .await?;
      assert!(
        executable.is_file(),
        "New installation should contain the game executable: {}",
        executable.display()
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_download_error() -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      let version = "v1.0.0";

      for (os, arch) in PLATFORMS {
        let asset_substrs =
          get_platform_asset_substrs(&variant, &os, &arch);
        assert_eq!(
          asset_substrs[0],
          expected_asset_substr(variant, &os, &arch),
          "unexpected asset mapping for {variant:?} on {os:?}/{arch:?}"
        );
        let asset_name = format!("{}-test.zip", asset_substrs[0]);

        let mock_server = MockServer::start().await?;
        let invalid_url =
          format!("{}/nonexistent.zip", mock_server.uri());

        releases_repo
          .update_cached_releases(
            &variant,
            &[GitHubRelease {
              id: 555 + variant_index as u64,
              tag_name: version.to_string(),
              prerelease: false,
              body: Some("body".to_string()),
              assets: vec![GitHubAsset {
                id: 666 + variant_index as u64,
                browser_download_url: invalid_url,
                name: asset_name,
                digest: None,
              }],
              created_at: Utc::now(),
            }],
          )
          .await?;

        let mut release = create_test_release(
          variant,
          version,
          GameReleaseStatus::NotDownloaded,
        );

        let result = release
          .install_release(
            &downloader,
            &os,
            &arch,
            temp_data.path(),
            temp_res.path(),
            &releases_repo,
            &active_repo,
            Arc::new(DummyReporter),
          )
          .await;

        assert!(matches!(
          result,
          Err(ReleaseInstallationError::Download(_))
        ));
      }
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_install_release_extraction_error() -> TestResult {
    let (_db, releases_repo, active_repo) =
      setup_test_repos().await?;
    let downloader = create_downloader();
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for (variant_index, variant) in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
    .into_iter()
    .enumerate()
    {
      let version = "v1.0.0";

      for (os, arch) in PLATFORMS {
        let asset_substrs =
          get_platform_asset_substrs(&variant, &os, &arch);
        assert_eq!(
          asset_substrs[0],
          expected_asset_substr(variant, &os, &arch),
          "unexpected asset mapping for {variant:?} on {os:?}/{arch:?}"
        );
        let asset_name = format!("{}-test.zip", asset_substrs[0]);

        let download_dir = get_or_create_asset_download_dir(
          &variant,
          temp_data.path(),
        )
        .await?;
        let archive_path = download_dir.join(&asset_name);
        tokio::fs::write(&archive_path, b"invalid zip content")
          .await?;

        releases_repo
          .update_cached_releases(
            &variant,
            &[GitHubRelease {
              id: 777 + variant_index as u64,
              tag_name: version.to_string(),
              prerelease: false,
              body: Some("body".to_string()),
              assets: vec![GitHubAsset {
                id: 888 + variant_index as u64,
                browser_download_url: "http://invalid.local/file.zip"
                  .to_string(),
                name: asset_name,
                digest: None,
              }],
              created_at: Utc::now(),
            }],
          )
          .await?;

        let mut release = create_test_release(
          variant,
          version,
          GameReleaseStatus::NotInstalled,
        );

        let result = release
          .install_release(
            &downloader,
            &os,
            &arch,
            temp_data.path(),
            temp_res.path(),
            &releases_repo,
            &active_repo,
            Arc::new(DummyReporter),
          )
          .await;

        assert!(matches!(
          result,
          Err(ReleaseInstallationError::Extract(_))
        ));
      }
    }
    Ok(())
  }
}
