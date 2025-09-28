use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub prerelease: bool,
}


#[derive(Debug, Error)]
pub enum GithubFetchError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Deserialization failed: {0}")]
    Deserialize(String),
}
pub async fn fetch_github_releases(client: &Client, repo: &str) -> Result<Vec<GithubRelease>, GithubFetchError> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(GithubFetchError::Request)?;
    let releases = response
        .json::<Vec<GithubRelease>>()
        .await
        .map_err(|e| {
            if e.is_decode() {
                GithubFetchError::Deserialize(e.to_string())
            } else {
                GithubFetchError::Request(e)
            }
        })?;
    Ok(releases)
}
