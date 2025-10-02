use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::Error as TauriError;
use thiserror::Error as ThisError;

use crate::infra::github::error::GitHubError;

#[derive(Debug, ThisError, IntoStaticStr)]
pub enum FetchReleasesError {
    #[error("Failed to fetch releases from GitHub: {0}")]
    Github(#[from] GitHubError),

    #[error("No cache directory found: {0}")]
    NoCacheDir(#[from] TauriError),
}

impl serde::Serialize for FetchReleasesError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("FetchReleasesError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
