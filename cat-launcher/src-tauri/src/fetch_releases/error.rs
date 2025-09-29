use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum FetchReleasesError {
    #[error("Failed to fetch GitHub releases: {0}")]
    Github(#[from] GithubFetchError),
}

#[derive(Debug, ThisError)]
pub enum GithubFetchError {
    #[error("Request failed: {0}")]
    Request(#[from] ReqwestError),

    #[error("Deserialization failed: {0}")]
    Deserialize(String),
}
