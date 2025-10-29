use std::path::PathBuf;
use std::sync::Arc;

use downloader::progress::Reporter;
use downloader::{Download, Downloader};
use serde::{Deserialize, Serialize};

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
    Download(#[from] downloader::Error),

    #[error("no download result found")]
    NoDownloadResult,
}

impl GitHubAsset {
    pub async fn download(
        &self,
        downloader: &mut Downloader,
        progress: Arc<dyn Reporter + Send + Sync>,
    ) -> Result<PathBuf, AssetDownloadError> {
        let dl = Download::new(&self.browser_download_url).progress(progress);
        let results = downloader.async_download(&[dl]).await?;

        if let Some(res) = results.into_iter().next() {
            match res {
                Ok(summary) => Ok(summary.file_name),
                Err(e) => Err(e.into()),
            }
        } else {
            Err(AssetDownloadError::NoDownloadResult)
        }
    }
}
