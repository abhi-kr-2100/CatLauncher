use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::infra::rfc3339;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubCommit {
  pub commit: GitHubCommitData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubCommitData {
  pub author: GitHubAuthor,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubAuthor {
  #[serde(with = "rfc3339")]
  pub date: DateTime<Utc>,
}
