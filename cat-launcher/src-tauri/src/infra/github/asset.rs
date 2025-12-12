use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use serde::{Deserialize, Serialize};

use crate::infra::download::{Downloader, DownloadFileError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubAsset {
    pub id: u64,
    pub browser_download_url: String,
    pub name: String,
    pub digest: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AssetDownloadError {
    #[error("failed to download asset: {0}")]
    Download(#[from] DownloadFileError),
}

impl GitHubAsset {
    pub async fn download(
        &self,
        downloader: &Downloader,
        download_dir: &Path,
        progress: Arc<dyn Reporter + Send + Sync>,
    ) -> Result<PathBuf, AssetDownloadError> {
        downloader
            .download_file(&self.browser_download_url, download_dir, progress)
            .await
            .map_err(AssetDownloadError::from)
    }
}