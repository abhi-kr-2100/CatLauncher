use std::io;
use std::path::Path;

use reqwest::Client;

use crate::game_release::game_release::{GameRelease, GetAssetError};
use crate::infra::archive_extractor::{extract_archive, ExtractionError};
use crate::infra::github::asset::AssetDownloadError;
use crate::infra::http_client::create_downloader;
use crate::install_release::utils::{get_asset_download_dir, get_asset_extraction_dir};

#[derive(thiserror::Error, Debug)]
pub enum ReleaseInstallationError {
    #[error("failed to get download directory: {0}")]
    DownloadDir(#[from] io::Error),

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
        let download_dir = get_asset_download_dir(&self.variant, data_dir)?;
        let mut downloader = create_downloader(client.clone(), &download_dir)?;
        let asset = self.get_asset(cache_dir)?;
        let filepath = asset.download(&mut downloader).await?;

        let extraction_dir = get_asset_extraction_dir(&self, &download_dir)?;
        extract_archive(&filepath, &extraction_dir).await?;

        Ok(())
    }
}
