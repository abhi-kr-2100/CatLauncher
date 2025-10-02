use reqwest::Client;

use crate::infra::github::error::GitHubError;
use crate::infra::github::release::GitHubRelease;

pub async fn fetch_github_releases(
    client: &Client,
    repo: &str,
) -> Result<Vec<GitHubRelease>, GitHubError> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let response = client.get(&url).send().await?;
    response.error_for_status_ref()?;

    let releases = response.json::<Vec<GitHubRelease>>().await?;
    Ok(releases)
}
