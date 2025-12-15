use crate::infra::github::types::GitHubCommit;
use reqwest::Client;

#[derive(thiserror::Error, Debug)]
pub enum GetLatestCommitError {
  #[error("failed to make request to github api: {0}")]
  GitHubApiRequest(#[from] reqwest::Error),
}

pub async fn get_latest_commit(
  owner: &str,
  repo: &str,
  http_client: &Client,
) -> Result<Option<GitHubCommit>, GetLatestCommitError> {
  let url = format!(
    "https://api.github.com/repos/{}/{}/commits",
    owner, repo
  );

  let response = http_client.get(&url).send().await?;
  let commits: Vec<GitHubCommit> = response.json().await?;

  Ok(commits.into_iter().next())
}
