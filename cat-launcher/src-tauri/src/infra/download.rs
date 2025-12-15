use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadFileError {
  #[error("downloader creation failed: {0}")]
  DownloaderCreation(#[from] downloader::Error),

  #[error("no download result found")]
  NoDownloadResult,
}

pub struct Downloader {
  client: Client,
  parallel_requests: NonZeroU16,
}

impl Downloader {
  pub fn new(client: Client, parallel_requests: NonZeroU16) -> Self {
    Self {
      client,
      parallel_requests,
    }
  }

  pub async fn download_file(
    &self,
    url: &str,
    download_dir: &Path,
    reporter: Arc<dyn Reporter + Send + Sync>,
  ) -> Result<PathBuf, DownloadFileError> {
    let mut builder = downloader::Downloader::builder();
    builder
      .download_folder(download_dir)
      .parallel_requests(self.parallel_requests.get());

    let mut downloader =
      builder.build_with_client(self.client.clone())?;

    let dl = downloader::Download::new(url).progress(reporter);

    let results = downloader.async_download(&[dl]).await?;

    if let Some(res) = results.into_iter().next() {
      match res {
        Ok(summary) => Ok(summary.file_name),
        Err(e) => Err(DownloadFileError::DownloaderCreation(e)),
      }
    } else {
      Err(DownloadFileError::NoDownloadResult)
    }
  }
}
