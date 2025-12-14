use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubCommit {
  pub commit: CommitDetails,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommitDetails {
  pub committer: Signature,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Signature {
  pub date: String,
}
