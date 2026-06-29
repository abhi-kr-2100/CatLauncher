use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::infra::http_client::{
  HttpClient, HttpClientError, ReqwestHttpClient,
};
use downloader::progress::Reporter;
use thiserror::Error;

/// Represents errors that can occur during file download.
#[derive(Error, Debug)]
pub enum DownloadFileError {
  #[error("downloader creation failed: {0}")]
  DownloaderCreation(#[from] downloader::Error),

  #[error("no download result found")]
  NoDownloadResult,
}

/// A struct for managing file downloads using a client C that implements `HttpClient`.
pub struct Downloader<C = ReqwestHttpClient> {
  client: C,
  parallel_requests: NonZeroU16,
}

impl<C: HttpClient + Clone + 'static> Downloader<C> {
  /// Creates a new `Downloader` with the given client and max parallel requests.
  pub fn new(client: C, parallel_requests: NonZeroU16) -> Self {
    Self {
      client,
      parallel_requests,
    }
  }

  /// Downloads a file from the given `url` to the `download_dir` and reports progress.
  pub async fn download_file(
    &self,
    url: &str,
    download_dir: &Path,
    reporter: Arc<dyn Reporter + Send + Sync>,
  ) -> Result<PathBuf, DownloadFileError> {
    let mut builder = downloader::downloader::Builder::default();
    builder
      .download_folder(download_dir)
      .parallel_requests(self.parallel_requests.get());

    let mut downloader = builder.build_with_client(
      DownloaderClient::new(self.client.clone()),
    )?;

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

/// Adapts the launcher's `HttpClient` to the `downloader` crate's `HttpClient` trait.
#[derive(Clone)]
struct DownloaderClient<C>(C);

impl<C> DownloaderClient<C> {
  fn new(client: C) -> Self {
    Self(client)
  }
}

impl<C: HttpClient + Clone + 'static> downloader::HttpClient
  for DownloaderClient<C>
{
  type Error = HttpClientError;
  type Response = DownloaderResponse;

  async fn get(
    &self,
    url: &str,
  ) -> Result<Self::Response, Self::Error> {
    Ok(DownloaderResponse(self.0.get(url).await?))
  }
}

/// Adapts a `reqwest::Response` to the `downloader` crate's `Response` trait.
struct DownloaderResponse(reqwest::Response);

impl downloader::Response for DownloaderResponse {
  type Error = HttpClientError;
  type Bytes = bytes::Bytes;

  fn status(&self) -> u16 {
    self.0.status().as_u16()
  }

  fn content_length(&self) -> Option<u64> {
    self.0.content_length()
  }

  async fn chunk(
    &mut self,
  ) -> Result<Option<Self::Bytes>, Self::Error> {
    self.0.chunk().await.map_err(HttpClientError::Http)
  }
}
