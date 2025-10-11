use std::sync::LazyLock;

use regex::Regex;
use reqwest::header::LINK;
use reqwest::Client;

use crate::infra::github::release::GitHubRelease;

#[derive(thiserror::Error, Debug)]
pub enum GitHubReleaseFetchError {
    #[error("failed to fetch from GitHub: {0}")]
    Fetch(#[from] reqwest::Error),

    #[error("failed to parse GitHub response: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("regex compilation failed: {0}")]
    Regex(#[from] regex::Error),
}

static NEXT_PAGE_URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<([^>]+)>; rel="next""#).unwrap());

pub async fn fetch_github_releases(
    client: &Client,
    repo: &str,
    num_releases: Option<usize>,
) -> Result<Vec<GitHubRelease>, GitHubReleaseFetchError> {
    if let Some(0) = num_releases {
        return Ok(Vec::new());
    }

    // GitHub API returns at most 1000 releases.
    let limit = num_releases.unwrap_or(1000).min(1000);

    let mut all_releases = Vec::new();

    let per_page = limit.min(100);

    let mut next_url = Some(format!(
        "https://api.github.com/repos/{}/releases?per_page={}",
        repo, per_page
    ));

    while let Some(url) = next_url {
        if all_releases.len() >= limit {
            break;
        }

        let response = client.get(&url).send().await?;
        response.error_for_status_ref()?;

        let link_header = response
            .headers()
            .get(LINK)
            .and_then(|value| value.to_str().ok());

        next_url = link_header.and_then(|link| {
            NEXT_PAGE_URL_RE
                .captures(link)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        });

        let response_text = response.text().await?;
        match serde_json::from_str::<Vec<GitHubRelease>>(&response_text) {
            Ok(releases) => {
                all_releases.extend(releases);
            }
            Err(e) => {
                return Err(GitHubReleaseFetchError::Parse(e));
            }
        }
    }

    Ok(all_releases)
}
