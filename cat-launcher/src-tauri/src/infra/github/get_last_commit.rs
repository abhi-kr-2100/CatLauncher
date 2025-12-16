use reqwest::Client;

use crate::infra::github::types::GitHubCommit;

#[derive(thiserror::Error, Debug)]
pub enum GetLastCommitError {
  #[error("failed to make API call: {0}")]
  FetchGithub(#[from] reqwest::Error),

  #[error("invalid GitHub response: {0}")]
  InvalidResponse(String),

  #[error("failed to parse GitHub response: {0}")]
  ParseGithub(#[from] serde_json::Error),

  #[error("no commits found in repository")]
  NoCommitsFound,
}

pub async fn get_last_commit(
  repo: &str,
  client: &Client,
) -> Result<GitHubCommit, GetLastCommitError> {
  let api_url = format!(
    "https://api.github.com/repos/{}/commits?per_page=1",
    repo
  );

  let response = client.get(&api_url).send().await?;

  if !response.status().is_success() {
    return Err(GetLastCommitError::InvalidResponse(format!(
      "GitHub API returned status: {}",
      response.status()
    )));
  }

  let commits: Vec<GitHubCommit> = response.json().await?;

  let commit = commits
    .into_iter()
    .next()
    .ok_or(GetLastCommitError::NoCommitsFound)?;

  Ok(commit)
}
