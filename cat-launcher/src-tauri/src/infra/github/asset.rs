use std::path::Path;
use std::sync::Arc;

use downloader::progress::Reporter;
use serde::{Deserialize, Serialize};

use crate::infra::download::{DownloadFileError, Downloader};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitHubAsset {
  pub id: u64,
  pub browser_download_url: String,
  pub name: String,
  pub digest: Option<String>,
}

impl GitHubAsset {
  pub async fn download(
    &self,
    downloader: &Downloader,
    download_dir: &Path,
    progress: Arc<dyn Reporter + Send + Sync>,
  ) -> Result<(), AssetDownloadError> {
    downloader
      .download_file(
        &self.browser_download_url,
        &download_dir.join(&self.name),
        progress,
      )
      .await?;
    Ok(())
  }
}

#[derive(thiserror::Error, Debug)]
pub enum AssetDownloadError {
  #[error("failed to download asset: {0}")]
  Download(#[from] DownloadFileError),
}
