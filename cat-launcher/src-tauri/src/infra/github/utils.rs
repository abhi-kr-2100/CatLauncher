use reqwest::header::LINK;

use crate::infra::github::release::GitHubRelease;
use crate::infra::http_client::{HttpClient, HttpClientError};

#[derive(thiserror::Error, Debug)]
pub enum GitHubReleaseFetchError {
  #[error("failed to fetch from GitHub: {0}")]
  Fetch(#[from] HttpClientError),

  #[error("failed to parse GitHub response: {0}")]
  Parse(#[from] serde_json::Error),
}

fn next_page_url(link_header: &str) -> Option<String> {
  link_header.split(',').find_map(|link| {
    let (url_part, rel_part) = link.trim().split_once(';')?;
    if rel_part.trim() != r#"rel="next""# {
      return None;
    }

    let url = url_part.trim().strip_prefix('<')?.strip_suffix('>')?;
    Some(url.to_string())
  })
}

pub async fn fetch_github_releases(
  client: &dyn HttpClient,
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

    let response = client.get(&url).await?;
    response
      .error_for_status_ref()
      .map_err(HttpClientError::from)?;

    let link_header = response
      .headers()
      .get(LINK)
      .and_then(|value| value.to_str().ok());

    next_url = link_header.and_then(next_page_url);

    let response_text =
      response.text().await.map_err(HttpClientError::from)?;
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

#[derive(thiserror::Error, Debug)]
pub enum FetchGitHubReleaseByTagError {
  #[error("failed to fetch from GitHub: {0}")]
  Fetch(#[from] HttpClientError),

  #[error("failed to parse GitHub response: {0}")]
  Parse(#[from] serde_json::Error),
}

pub async fn fetch_github_release_by_tag(
  client: &dyn HttpClient,
  repo: &str,
  tag: &str,
) -> Result<GitHubRelease, FetchGitHubReleaseByTagError> {
  let url = format!(
    "https://api.github.com/repos/{}/releases/tags/{}",
    repo,
    urlencoding::encode(tag)
  );

  let response = client.get(&url).await?;
  response
    .error_for_status_ref()
    .map_err(HttpClientError::from)?;

  let response_text =
    response.text().await.map_err(HttpClientError::from)?;
  Ok(serde_json::from_str::<GitHubRelease>(&response_text)?)
}
