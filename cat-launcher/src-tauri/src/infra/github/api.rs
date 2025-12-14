use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubCommit {
    commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    committer: Signature,
}

#[derive(Debug, Deserialize)]
struct Signature {
    date: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetLatestCommitTimestampError {
    #[error("failed to make request to github api: {0}")]
    GitHubApiRequest(#[from] reqwest::Error),

    #[error("failed to parse github url `{0}`")]
    ParseGithubUrl(String),
}

pub async fn get_latest_commit_timestamp(
    github_url: &str,
    http_client: &Client,
) -> Result<String, GetLatestCommitTimestampError> {
    let url_parts: Vec<&str> = github_url.split('/').collect();
    let owner = url_parts.get(3).ok_or_else(|| GetLatestCommitTimestampError::ParseGithubUrl(github_url.to_string()))?;
    let repo = url_parts.get(4).ok_or_else(|| GetLatestCommitTimestampError::ParseGithubUrl(github_url.to_string()))?;

    let url = format!("https://api.github.com/repos/{}/{}/commits", owner, repo);

    let response = http_client.get(&url).send().await?;
    let commits: Vec<GitHubCommit> = response.json().await?;

    if let Some(latest_commit) = commits.first() {
        return Ok(latest_commit.commit.committer.date.clone());
    }

    // It's unlikely that a repo has no commits, but we should handle this case.
    Ok("".to_string())
}
