use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::error::GithubFetchError;
use crate::infra::rfc3339;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRelease {
    pub id: u64,
    pub tag_name: String,
    pub prerelease: bool,
    pub assets: Vec<GithubAsset>,
    #[serde(with = "rfc3339")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubAsset {
    pub id: u64,
    pub browser_download_url: String,
}

pub async fn fetch_github_releases(
    client: &Client,
    repo: &str,
) -> Result<Vec<GithubRelease>, GithubFetchError> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(GithubFetchError::Request)?;
    let releases = response.json::<Vec<GithubRelease>>().await.map_err(|e| {
        if e.is_decode() {
            GithubFetchError::Deserialize(e.to_string())
        } else {
            GithubFetchError::Request(e)
        }
    })?;
    Ok(releases)
}
