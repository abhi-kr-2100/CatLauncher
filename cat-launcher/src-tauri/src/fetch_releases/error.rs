use serde::Serialize;
use thiserror::Error as ThisError;

use crate::infra::github::error::GitHubError;

#[derive(Debug, ThisError, Serialize)]
pub enum FetchReleasesError {
    #[error("Failed to fetch releases from GitHub: {0}")]
    Github(#[from] GitHubError),
}
