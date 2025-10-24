use std::path::Path;

use reqwest::Client;

use crate::filesystem::paths::{
    get_or_create_asset_download_dir, get_or_create_asset_installation_dir, AssetDownloadDirError,
    AssetExtractionDirError,
};
use crate::game_release::game_release::{GameRelease, GameReleaseStatus};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::http_client::create_downloader;
use crate::infra::utils::{Arch, OS};
use crate::install_release::installation_progress_payload::{
    InstallationProgressPayload, InstallationProgressStatus,
};
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::repository::releases_repository::ReleasesRepository;

#[derive(thiserror::Error, Debug)]
pub enum ReleaseInstallationError<E: std::error::Error + Send + Sync + 'static> {
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

    #[error("status update callback failed: {0}")]
    Callback(E),
}

impl GameRelease {
    pub async fn install_release<E: std::error::Error + Send + Sync + 'static, F, Fut>(
        &mut self,
        client: &Client,
        os: &OS,
        arch: &Arch,
        data_dir: &Path,
        resources_dir: &Path,
        releases_repository: &dyn ReleasesRepository,
        on_status_update: F,
    ) -> Result<(), ReleaseInstallationError<E>>
    where
        F: Fn(InstallationProgressPayload) -> Fut,
        Fut: std::future::Future<Output = Result<(), E>> + Send,
    {
        if self.status == GameReleaseStatus::Unknown {
            self.status = self
                .get_installation_status(os, arch, data_dir, resources_dir, releases_repository)
                .await?;
        }

        if self.status == GameReleaseStatus::ReadyToPlay {
            return Ok(());
        }

        let download_dir = get_or_create_asset_download_dir(&self.variant, data_dir).await?;
        let asset = self
            .get_asset(os, arch, resources_dir, releases_repository)
            .await
            .ok_or(ReleaseInstallationError::NoCompatibleAsset)?;

        if self.status == GameReleaseStatus::NotDownloaded
            || self.status == GameReleaseStatus::Corrupted
            || self.status == GameReleaseStatus::Unknown
        {
            on_status_update(InstallationProgressPayload {
                status: InstallationProgressStatus::Downloading,
                release_id: self.version.clone(),
            })
            .await
            .map_err(ReleaseInstallationError::Callback)?;

            let mut downloader = create_downloader(client.clone(), &download_dir)?;
            asset.download(&mut downloader).await?;
            self.status = GameReleaseStatus::NotInstalled;
        }

        let download_filepath = download_dir.join(&asset.name);
        let installation_dir =
            get_or_create_asset_installation_dir(&self.variant, &self.version, data_dir).await?;

        on_status_update(InstallationProgressPayload {
            status: InstallationProgressStatus::Installing,
            release_id: self.version.clone(),
        })
        .await
        .map_err(ReleaseInstallationError::Callback)?;

        extract_archive(&download_filepath, &installation_dir, os).await?;

        self.status = GameReleaseStatus::ReadyToPlay;

        on_status_update(InstallationProgressPayload {
            status: InstallationProgressStatus::Success,
            release_id: self.version.clone(),
        })
        .await
        .map_err(ReleaseInstallationError::Callback)?;

        Ok(())
    }
}
