use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum GitHubReleaseFetchError {
    #[error("failed to fetch releases from github: {0}")]
    Fetch(#[from] reqwest::Error),
}

pub async fn fetch_github_releases<T: DeserializeOwned>(
    client: &Client,
    repo: &str,
    per_page: Option<u8>,
) -> Result<Vec<T>, GitHubReleaseFetchError> {
    let per_page = per_page.unwrap_or(30);
    let url = format!(
        "https://api.github.com/repos/{}/releases?per_page={}",
        repo, per_page
    );
    let releases = client.get(&url).send().await?.json::<Vec<T>>().await?;
    Ok(releases)
}
