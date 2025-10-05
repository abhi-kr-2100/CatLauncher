use std::path::Path;

use reqwest::Client;

use crate::filesystem::paths::{
    get_or_create_asset_download_dir, get_or_create_asset_installation_dir, AssetDownloadDirError,
    AssetExtractionDirError,
};
use crate::game_release::game_release::{GameRelease, GameReleaseStatus, GetAssetError};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::http_client::create_downloader;

#[derive(thiserror::Error, Debug)]
pub enum ReleaseInstallationError {
    #[error("failed to get download directory: {0}")]
    DownloadDir(#[from] AssetDownloadDirError),

    #[error("failed to get extraction directory: {0}")]
    ExtractionDir(#[from] AssetExtractionDirError),

    #[error("failed to create downloader: {0}")]
    Downloader(#[from] downloader::Error),

    #[error("failed to choose asset: {0}")]
    Asset(#[from] GetAssetError),

    #[error("failed to download asset: {0}")]
    Download(#[from] AssetDownloadError),

    #[error("failed to extract asset: {0}")]
    Extract(#[from] ExtractionError),
}

impl GameRelease {
    pub async fn install_release(
        &self,
        client: &Client,
        cache_dir: &Path,
        data_dir: &Path,
    ) -> Result<(), ReleaseInstallationError> {
        if self.status == GameReleaseStatus::ReadyToPlay {
            return Ok(());
        }

        let download_dir = get_or_create_asset_download_dir(&self.variant, data_dir)?;
        let asset = self.get_asset(cache_dir)?;
        let download_filepath = download_dir.join(&asset.name);

        if self.status == GameReleaseStatus::NotDownloaded {
            let mut downloader = create_downloader(client.clone(), &download_dir)?;
            let _ = asset.download(&mut downloader).await?;
        }

        let installation_dir =
            get_or_create_asset_installation_dir(&self.variant, &self.version, data_dir)?;
        extract_archive(&download_filepath, &installation_dir).await?;

        Ok(())
    }
}
