use crate::infra::github::types::GitHubCommit;
use reqwest::Client;

#[derive(thiserror::Error, Debug)]
pub enum GetLatestCommitError {
  #[error("failed to make request to github api: {0}")]
  GitHubApiRequest(#[from] reqwest::Error),

  #[error("failed to parse github url `{0}`")]
  ParseGithubUrl(String),
}

pub async fn get_latest_commit(
  github_url: &str,
  http_client: &Client,
) -> Result<Option<GitHubCommit>, GetLatestCommitError> {
  let url_parts: Vec<&str> = github_url.split('/').collect();
  let owner = url_parts.get(3).ok_or_else(|| {
    GetLatestCommitError::ParseGithubUrl(github_url.to_string())
  })?;
  let repo = url_parts.get(4).ok_or_else(|| {
    GetLatestCommitError::ParseGithubUrl(github_url.to_string())
  })?;

  let url = format!(
    "https://api.github.com/repos/{}/{}/commits",
    owner, repo
  );

  let response = http_client.get(&url).send().await?;
  let commits: Vec<GitHubCommit> = response.json().await?;

  Ok(commits.into_iter().next())
}
