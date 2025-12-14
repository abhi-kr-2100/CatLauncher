use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::infra::github::asset::GitHubAsset;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitHubRelease {
  pub id: u64,
  pub tag_name: String,
  pub prerelease: bool,
  pub assets: Vec<GitHubAsset>,
  pub created_at: DateTime<Utc>,
}
