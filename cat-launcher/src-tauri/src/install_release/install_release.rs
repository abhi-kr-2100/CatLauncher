use std::path::Path;
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs;

use crate::active_release::active_release::ActiveReleaseError;
use crate::active_release::repository::ActiveReleaseRepository;
use crate::fetch_releases::repository::ReleasesRepository;
use crate::filesystem::paths::{
  get_or_create_asset_download_dir,
  get_or_create_asset_installation_dir, AssetDownloadDirError,
  AssetExtractionDirError,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::download::Downloader;
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::utils::{Arch, OS};
use crate::install_release::installation_status::status::GetInstallationStatusError;

#[derive(thiserror::Error, Debug)]
pub enum ReleaseInstallationError {
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
  ActiveRelease(#[from] ActiveReleaseError),
}

impl GameRelease {
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
