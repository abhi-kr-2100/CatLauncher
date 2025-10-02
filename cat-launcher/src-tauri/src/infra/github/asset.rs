use std::path::PathBuf;

use downloader::{Download, Downloader};
use serde::{Deserialize, Serialize};

use crate::infra::github::error::GitHubError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubAsset {
    pub id: u64,
    pub browser_download_url: String,
    pub name: String,
}

impl GitHubAsset {
    pub async fn download(&self, downloader: &mut Downloader) -> Result<PathBuf, GitHubError> {
        let dl = Download::new(&self.browser_download_url);
        let results = downloader.async_download(&[dl]).await?;

        if let Some(res) = results.into_iter().next() {
            match res {
                Ok(summary) => Ok(summary.file_name),
                Err(e) => Err(e.into()),
            }
        } else {
            Err(GitHubError::Unknown("No download result found".into()))
        }
    }
}
