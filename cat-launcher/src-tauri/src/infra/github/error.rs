use downloader::Error as DownloaderError;
use reqwest::Error as ReqwestError;
use serde::ser::{SerializeStruct, Serializer};
use strum_macros::IntoStaticStr;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, IntoStaticStr)]
pub enum GitHubError {
    #[error("HTTP request failed: {0}")]
    Http(ReqwestError),

    #[error("Failed to parse GitHub response: {0}")]
    Parse(ReqwestError),

    #[error("Download error: {0}")]
    Download(DownloaderError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<ReqwestError> for GitHubError {
    fn from(err: ReqwestError) -> Self {
        if err.is_decode() {
            GitHubError::Parse(err)
        } else {
            GitHubError::Http(err)
        }
    }
}

impl From<DownloaderError> for GitHubError {
    fn from(err: DownloaderError) -> Self {
        GitHubError::Download(err)
    }
}

impl serde::Serialize for GitHubError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("GitHubError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
