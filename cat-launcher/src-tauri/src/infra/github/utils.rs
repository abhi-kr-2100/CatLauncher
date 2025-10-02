use reqwest::Client;

use crate::infra::github::release::GitHubRelease;

#[derive(thiserror::Error, Debug)]
pub enum GitHubReleaseFetchError {
    #[error("failed to fetch from GitHub: {0}")]
    Fetch(#[from] reqwest::Error),

    #[error("failed to parse GitHub response: {0}")]
    Parse(#[from] serde_json::Error),
}

pub async fn fetch_github_releases(
    client: &Client,
    repo: &str,
) -> Result<Vec<GitHubRelease>, GitHubReleaseFetchError> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let response = client.get(&url).send().await?;
    response.error_for_status_ref()?;

    let releases = response.json::<Vec<GitHubRelease>>().await?;
    Ok(releases)
}
