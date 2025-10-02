use std::io::Error as IoError;

use downloader::Error as DownloaderError;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use strum_macros::IntoStaticStr;
use tauri::Error as TauriError;
use thiserror::Error as ThisError;

use crate::game_release::error::GameReleaseError;
use crate::infra::github::error::GitHubError;

#[derive(Debug, ThisError, IntoStaticStr)]
pub enum InstallReleaseError {
    #[error("Failed to set up asset download directory: {0}")]
    AssetDownloadDir(#[from] IoError),

    #[error("Failed to set up downloader: {0}")]
    Downloader(#[from] DownloaderError),

    #[error("No compatible asset found for the game release")]
    NoCompatibleAssetFound(#[from] GameReleaseError),

    #[error("Failed to download asset: {0}")]
    Download(#[from] GitHubError),

    #[error("System directory not found: {0}")]
    SystemDirectoryNotFound(#[from] TauriError),

    #[error("Failed to set up HTTP client: {0}")]
    HttpClientSetup(String),

    #[error("Unknown error")]
    Unknown,
}

impl Serialize for InstallReleaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("InstallReleaseError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
