use reqwest::Error as ReqwestError;
use serde::ser::Serialize as SerializeTrait;
use serde::ser::Serializer;
use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Serialize)]
pub enum FetchReleasesError {
    #[error("Failed to fetch GitHub releases: {0}")]
    Github(#[from] GithubFetchError),
}

#[derive(Debug, ThisError)]
pub enum GithubFetchError {
    #[error("Request failed: {0}")]
    Request(ReqwestError),

    #[error("Deserialization failed: {0}")]
    Deserialize(ReqwestError),
}

impl SerializeTrait for GithubFetchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
