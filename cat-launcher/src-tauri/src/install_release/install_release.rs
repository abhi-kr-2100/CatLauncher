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
mod tests {
  use super::*;
  use std::sync::Arc;
  use std::sync::atomic::{AtomicUsize, Ordering};
  use tokio::fs;
  use tempfile::TempDir;
  use github_mock_api::{MockServer, Asset as MockAsset, MockBehavior, MockError};
  use crate::infra::testing::test_database::TestDatabase;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::game_release::game_release::{GameRelease, GameReleaseStatus, ReleaseType};
  use crate::infra::utils::{OS, Arch};
  use crate::variants::GameVariant;
  use crate::infra::download::Downloader;
  use std::num::NonZeroU16;
  use downloader::progress::Reporter;
  use crate::infra::github::release::GitHubRelease;
  use chrono::Utc;

  struct ProgressReporter {
    setup_count: AtomicUsize,
    progress_count: AtomicUsize,
    done_count: AtomicUsize,
  }

  impl ProgressReporter {
    fn new() -> Self {
      Self {
        setup_count: AtomicUsize::new(0),
        progress_count: AtomicUsize::new(0),
        done_count: AtomicUsize::new(0),
      }
    }
  }

  impl Reporter for ProgressReporter {
    fn setup(&self, _: Option<u64>, _: &str) {
      self.setup_count.fetch_add(1, Ordering::Relaxed);
    }
    fn progress(&self, _: u64) {
      self.progress_count.fetch_add(1, Ordering::Relaxed);
    }
    fn set_message(&self, _: &str) {}
    fn done(&self) {
      self.done_count.fetch_add(1, Ordering::Relaxed);
    }
  }

  async fn setup() -> (
    TestDatabase,
    MockServer,
    Downloader,
    TempDir,
    SqliteReleasesRepository,
    SqliteActiveReleaseRepository,
  ) {
    let db = TestDatabase::builder().build().unwrap();
    let server = MockServer::start().await.unwrap();

    let downloader = Downloader::new(reqwest::Client::new(), NonZeroU16::new(1).unwrap());
    let temp_dir = TempDir::new().unwrap();
    let releases_repo = SqliteReleasesRepository::new(db.pool().clone());
    let active_release_repo = SqliteActiveReleaseRepository::new(db.pool().clone());

    (db, server, downloader, temp_dir, releases_repo, active_release_repo)
  }

  fn create_dummy_zip(path: &std::path::Path) {
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    zip.start_file("test.txt", options).unwrap();
    use std::io::Write;
    zip.write_all(b"hello world").unwrap();
    zip.finish().unwrap();
  }

  #[tokio::test]
  async fn test_install_release_fresh_success() {
    let (_db, server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;

    let variant = GameVariant::BrightNights;
    let version = "2026-06-05";
    let safe_version = "2026_06_05";
    let asset_name = "cbn-windows-tiles-x64-msvc-2026-06-05.zip";

    // 1. Create dummy zip
    let zip_path = temp_dir.path().join("dummy.zip");
    create_dummy_zip(&zip_path);

    // 2. Register asset in mock server
    let mock_asset = MockAsset::from_path(asset_name, &zip_path, "application/zip");
    server.add_asset("cataclysmbnteam", "Cataclysm-BN", version, mock_asset).await;

    // 3. Seed repository with the asset pointing to mock server
    let gh_asset = crate::infra::github::asset::GitHubAsset {
        id: 1,
        browser_download_url: format!("{}/cataclysmbnteam/Cataclysm-BN/releases/download/{}/{}", server.uri(), version, asset_name),
        name: asset_name.to_string(),
        digest: None,
    };

    let gh_release = GitHubRelease {
        id: 123,
        tag_name: version.to_string(),
        prerelease: true,
        body: Some("body".to_string()),
        assets: vec![gh_asset],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: Some("body".to_string()),
        release_type: ReleaseType::Experimental,
        status: GameReleaseStatus::Unknown,
        created_at: Utc::now(),
    };

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    let reporter = Arc::new(ProgressReporter::new());

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        reporter.clone(),
    ).await;

    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok(), "Installation failed: {:?}", result.err());
    assert_eq!(game_release.status, GameReleaseStatus::ReadyToPlay);

    // Verify it's set as active
    let active = active_release_repo.get_active_release(&variant).await.unwrap();
    assert_eq!(active, Some(version.to_string()));

    // Verify it was extracted
    let install_dir = data_dir.join("Assets").join(variant.id()).join(safe_version);
    assert!(install_dir.exists(), "Installation directory does not exist: {:?}", install_dir);
    assert!(install_dir.join("test.txt").exists(), "Extracted file does not exist");

    // Verify progress was reported
    assert!(reporter.setup_count.load(Ordering::Relaxed) > 0);
    assert!(reporter.done_count.load(Ordering::Relaxed) > 0);
  }

  #[tokio::test]
  async fn test_install_release_already_installed() {
    let (_db, _server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::BrightNights;
    let version = "2026-06-05";
    let safe_version = "2026_06_05";

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    // Create installation directory and dummy executable to simulate it's already installed
    let install_dir = data_dir.join("Assets").join(variant.id()).join(safe_version);
    fs::create_dir_all(&install_dir).await.unwrap();
    fs::write(install_dir.join("cataclysm-bn-tiles.exe"), "dummy exe").await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: Some("body".to_string()),
        release_type: ReleaseType::Experimental,
        status: GameReleaseStatus::Unknown,
        created_at: Utc::now(),
    };

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await;

    if let Err(e) = &result {
        eprintln!("Error in already_installed: {}", e);
    }
    assert!(result.is_ok());
    assert_eq!(game_release.status, GameReleaseStatus::ReadyToPlay);

    // Verify it's set as active
    let active = active_release_repo.get_active_release(&variant).await.unwrap();
    assert_eq!(active, Some(version.to_string()));
  }

  #[tokio::test]
  async fn test_install_release_no_compatible_asset() {
    let (_db, _server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::DarkDaysAhead;
    let version = "v1";

    // Seed repo with a release that has NO assets
    let gh_release = GitHubRelease {
        id: 123,
        tag_name: version.to_string(),
        prerelease: false,
        body: None,
        assets: vec![],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: None,
        release_type: ReleaseType::Stable,
        status: GameReleaseStatus::Unknown,
        created_at: Utc::now(),
    };

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await;

    match result {
        Err(ReleaseInstallationError::NoCompatibleAsset) => (),
        _ => panic!("Expected NoCompatibleAsset error, got {:?}", result),
    }
  }

  #[tokio::test]
  async fn test_install_release_cleanup_others() {
    let (_db, server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::BrightNights;
    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    // 1. Create an old installation
    let old_safe_version = "old_version";
    let old_install_dir = data_dir.join("Assets").join(variant.id()).join(old_safe_version);
    fs::create_dir_all(&old_install_dir).await.unwrap();
    fs::write(old_install_dir.join("old.txt"), "old content").await.unwrap();

    // 2. Install new version
    let version = "new-version";
    let safe_version = "new_version";
    let zip_path = temp_dir.path().join("dummy.zip");
    create_dummy_zip(&zip_path);
    let asset_name_zip = "cbn-windows-tiles-x64-new.zip";

    let mock_asset = MockAsset::from_path(asset_name_zip, &zip_path, "application/zip");
    server.add_asset("cataclysmbnteam", "Cataclysm-BN", version, mock_asset).await;

    let gh_asset = crate::infra::github::asset::GitHubAsset {
        id: 2,
        browser_download_url: format!("{}/cataclysmbnteam/Cataclysm-BN/releases/download/{}/{}", server.uri(), version, asset_name_zip),
        name: asset_name_zip.to_string(),
        digest: None,
    };
    let gh_release = GitHubRelease {
        id: 124,
        tag_name: version.to_string(),
        prerelease: false,
        body: None,
        assets: vec![gh_asset],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: None,
        release_type: ReleaseType::Stable,
        status: GameReleaseStatus::Unknown,
        created_at: Utc::now(),
    };

    game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await.unwrap();

    // 3. Verify old installation is gone
    assert!(!old_install_dir.exists(), "Old installation directory should have been deleted");
    // 4. Verify new installation exists
    let new_install_dir = data_dir.join("Assets").join(variant.id()).join(safe_version);
    assert!(new_install_dir.exists());
  }

  #[tokio::test]
  async fn test_install_release_corrupted_redownloads() {
    let (_db, server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::BrightNights;
    let version = "2026-06-05";
    let safe_version = "2026_06_05";
    let asset_name = "cbn-windows-tiles-x64-msvc-2026-06-05.zip";

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    // 1. Create dummy zip
    let zip_path = temp_dir.path().join("dummy.zip");
    create_dummy_zip(&zip_path);

    // 2. Register asset in mock server
    let mock_asset = MockAsset::from_path(asset_name, &zip_path, "application/zip");
    server.add_asset("cataclysmbnteam", "Cataclysm-BN", version, mock_asset).await;

    // 3. Seed repository
    let gh_asset = crate::infra::github::asset::GitHubAsset {
        id: 1,
        browser_download_url: format!("{}/cataclysmbnteam/Cataclysm-BN/releases/download/{}/{}", server.uri(), version, asset_name),
        name: asset_name.to_string(),
        digest: None,
    };
    let gh_release = GitHubRelease {
        id: 123,
        tag_name: version.to_string(),
        prerelease: true,
        body: None,
        assets: vec![gh_asset],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: None,
        release_type: ReleaseType::Experimental,
        status: GameReleaseStatus::Corrupted, // Simulate corruption
        created_at: Utc::now(),
    };

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await;

    assert!(result.is_ok());
    assert_eq!(game_release.status, GameReleaseStatus::ReadyToPlay);

    let install_dir = data_dir.join("Assets").join(variant.id()).join(safe_version);
    assert!(install_dir.exists());
    assert!(install_dir.join("test.txt").exists());
  }

  #[tokio::test]
  async fn test_install_release_download_failure() {
    let (_db, server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::BrightNights;
    let version = "2026-06-05";
    let asset_name = "cbn-windows-tiles-x64-msvc-2026-06-05.zip";

    // Register a mock behavior that returns 500 error
    server.add_mock_behavior(MockBehavior::builder().error(MockError::InternalServerError).build()).await.unwrap();

    let gh_asset = crate::infra::github::asset::GitHubAsset {
        id: 1,
        browser_download_url: format!("{}/cataclysmbnteam/Cataclysm-BN/releases/download/{}/{}", server.uri(), version, asset_name),
        name: asset_name.to_string(),
        digest: None,
    };
    let gh_release = GitHubRelease {
        id: 123,
        tag_name: version.to_string(),
        prerelease: true,
        body: None,
        assets: vec![gh_asset],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: None,
        release_type: ReleaseType::Experimental,
        status: GameReleaseStatus::NotDownloaded,
        created_at: Utc::now(),
    };

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await;

    // github-mock-api 500 should result in a download error
    assert!(result.is_err());
    assert!(matches!(result, Err(ReleaseInstallationError::Download(_))));
  }

  #[tokio::test]
  async fn test_install_release_extraction_failure() {
    let (_db, server, downloader, temp_dir, releases_repo, active_release_repo) = setup().await;
    let variant = GameVariant::BrightNights;
    let version = "2026-06-05";
    let asset_name = "cbn-windows-tiles-x64-msvc-2026-06-05.zip";

    // 1. Create an invalid zip file
    let zip_path = temp_dir.path().join("invalid.zip");
    fs::write(&zip_path, b"not a zip file").await.unwrap();

    // 2. Register asset in mock server
    let mock_asset = MockAsset::from_path(asset_name, &zip_path, "application/zip");
    server.add_asset("cataclysmbnteam", "Cataclysm-BN", version, mock_asset).await;

    // 3. Seed repository
    let gh_asset = crate::infra::github::asset::GitHubAsset {
        id: 1,
        browser_download_url: format!("{}/cataclysmbnteam/Cataclysm-BN/releases/download/{}/{}", server.uri(), version, asset_name),
        name: asset_name.to_string(),
        digest: None,
    };
    let gh_release = GitHubRelease {
        id: 123,
        tag_name: version.to_string(),
        prerelease: true,
        body: None,
        assets: vec![gh_asset],
        created_at: Utc::now(),
    };
    releases_repo.update_cached_releases(&variant, &[gh_release]).await.unwrap();

    let mut game_release = GameRelease {
        variant,
        version: version.to_string(),
        body: None,
        release_type: ReleaseType::Experimental,
        status: GameReleaseStatus::NotDownloaded,
        created_at: Utc::now(),
    };

    let data_dir = temp_dir.path().join("data");
    let resources_dir = temp_dir.path().join("resources");
    fs::create_dir_all(&data_dir).await.unwrap();
    fs::create_dir_all(&resources_dir).await.unwrap();

    let result = game_release.install_release(
        &downloader,
        &OS::Windows,
        &Arch::X64,
        &data_dir,
        &resources_dir,
        &releases_repo,
        &active_release_repo,
        Arc::new(ProgressReporter::new()),
    ).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ReleaseInstallationError::Extract(_))));
  }
}
