use std::error::Error;
use std::sync::Arc;

use reqwest::Client;
use tokio::fs;

use crate::active_release::repository::ActiveReleaseRepository;
use crate::fetch_releases::fetch_releases::ReleasesUpdatePayload;
use crate::fetch_releases::repository::ReleasesRepository;
use crate::filesystem::paths::{
  get_or_create_asset_download_dir,
  get_or_create_asset_installation_dir, AssetDownloadDirError,
  AssetExtractionDirError,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus, ReleaseType,
};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::download::Downloader;
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::utils::{Arch, OS};
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UpgradeToLatestError {
  #[error("failed to fetch releases: {0}")]
  FetchReleases(Box<dyn Error + Send + Sync>),

  #[error("no releases available")]
  NoReleasesAvailable,

  #[error("no experimental release available")]
  NoExperimentalRelease,

  #[error("failed to get download directory: {0}")]
  DownloadDir(#[from] AssetDownloadDirError),

  #[error("failed to get extraction directory: {0}")]
  ExtractionDir(#[from] AssetExtractionDirError),

  #[error("failed to create downloader: {0}")]
  Downloader(#[from] downloader::Error),

  #[error("no compatible asset found")]
  NoCompatibleAsset,

  #[error("failed to download asset: {0}")]
  Download(#[from] AssetDownloadError),

  #[error("failed to extract asset: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to get release status: {0}")]
  ReleaseStatus(#[from] GetInstallationStatusError),

  #[error("failed to set active release: {0}")]
  ActiveRelease(
    #[from] crate::active_release::active_release::ActiveReleaseError,
  ),

  #[error("failed to access releases cache: {0}")]
  Repository(
    #[from]
    crate::fetch_releases::repository::ReleasesRepositoryError,
  ),

  #[error("failed to fetch from github: {0}")]
  GitHubFetch(
    #[from] crate::infra::github::utils::GitHubReleaseFetchError,
  ),
}

#[allow(clippy::too_many_arguments)]
impl GameVariant {
  pub async fn upgrade_to_latest<E, F>(
    &self,
    client: &Client,
    downloader: &Downloader,
    os: &OS,
    arch: &Arch,
    data_dir: &std::path::Path,
    resources_dir: &std::path::Path,
    releases_repository: &dyn ReleasesRepository,
    active_release_repository: &dyn ActiveReleaseRepository,
    progress: Arc<dyn downloader::progress::Reporter + Send + Sync>,
    on_releases: F,
  ) -> Result<GameRelease, UpgradeToLatestError>
  where
    E: Error,
    F: Fn(ReleasesUpdatePayload) -> Result<(), E>,
  {
    // Try to fetch the latest releases from GitHub
    let fetched_releases =
      crate::infra::github::utils::fetch_github_releases(
        client,
        crate::infra::utils::get_github_repo_for_variant(self),
        Some(100),
      )
      .await
      .ok();

    // Get cached releases as fallback
    let cached_releases = releases_repository
      .get_cached_releases(self)
      .await
      .unwrap_or_default();

    // Use fetched releases if available, otherwise use cached
    let releases_to_use = if let Some(fetched) = fetched_releases {
      releases_repository
        .update_cached_releases(self, &fetched)
        .await
        .ok(); // Update cache but don't fail if it doesn't work
      fetched
    } else {
      cached_releases
    };

    // Convert GitHub releases to GameReleases
    let game_releases: Vec<GameRelease> = releases_to_use
      .iter()
      .map(|r| {
        crate::game_release::utils::gh_release_to_game_release(
          r, self,
        )
      })
      .collect();

    if game_releases.is_empty() {
      return Err(UpgradeToLatestError::NoReleasesAvailable);
    }

    // Find the latest experimental release
    let latest_experimental = game_releases
      .iter()
      .filter(|r| r.release_type == ReleaseType::Experimental)
      .max_by_key(|r| r.created_at)
      .ok_or(UpgradeToLatestError::NoExperimentalRelease)?
      .clone();

    // Emit update event
    let payload = crate::fetch_releases::utils::get_releases_payload(
      self,
      &releases_to_use,
      crate::fetch_releases::fetch_releases::ReleasesUpdateStatus::Success,
    );
    let _ = on_releases(payload);

    // Install the release
    let mut release = latest_experimental;
    if release.status == GameReleaseStatus::Unknown {
      release.status =
        release.get_installation_status(os, data_dir).await?;
    }

    if release.status == GameReleaseStatus::ReadyToPlay {
      self
        .set_active_release(
          &release.version,
          active_release_repository,
        )
        .await?;
      return Ok(release);
    }

    let download_dir =
      get_or_create_asset_download_dir(self, data_dir).await?;
    let asset = release
      .get_asset(os, arch, resources_dir, releases_repository)
      .await
      .ok_or(UpgradeToLatestError::NoCompatibleAsset)?;

    if release.status == GameReleaseStatus::NotDownloaded
      || release.status == GameReleaseStatus::Corrupted
      || release.status == GameReleaseStatus::Unknown
    {
      asset.download(downloader, &download_dir, progress).await?;
      release.status = GameReleaseStatus::NotInstalled;
    }

    let download_filepath = download_dir.join(&asset.name);
    let installation_dir = get_or_create_asset_installation_dir(
      self,
      &release.version,
      data_dir,
    )
    .await?;

    extract_archive(&download_filepath, &installation_dir, os)
      .await?;

    release.status = GameReleaseStatus::ReadyToPlay;

    self
      .set_active_release(&release.version, active_release_repository)
      .await?;

    // Failure to remove file does not mean failure to install
    let _ = fs::remove_file(&download_filepath).await;

    delete_other_installations(&installation_dir).await;

    Ok(release)
  }
}

async fn delete_other_installations(
  installation_dir: &std::path::Path,
) {
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
