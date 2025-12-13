use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::infra::github::asset::GitHubAsset;
use crate::infra::rfc3339;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubRelease {
  pub id: u64,
  pub tag_name: String,
  pub prerelease: bool,
  pub assets: Vec<GitHubAsset>,
  #[serde(with = "rfc3339")]
  pub created_at: DateTime<Utc>,
}
