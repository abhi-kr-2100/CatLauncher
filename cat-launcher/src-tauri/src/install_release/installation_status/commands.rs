use std::env::consts::OS;

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::game_release::game_release::{GameRelease, GameReleaseStatus};
use crate::game_release::game_release::ReleaseType;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum InstallationStatusCommandError {
    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to get installation status: {0}")]
    InstallationStatus(#[from] GetInstallationStatusError),
}

impl serde::Serialize for InstallationStatusCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("InstallationStatusCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}

#[command]
pub async fn get_installation_status(
    app_handle: AppHandle,
    variant: GameVariant,
    version: String,
    release_type: ReleaseType,
) -> Result<GameReleaseStatus, InstallationStatusCommandError> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()?;

    let cache_dir = app_handle
        .path()
        .app_cache_dir()?;

    let release = GameRelease {
        variant,
        version,
        release_type,
        status: GameReleaseStatus::Unknown, // Placeholder status
    };

    release
        .get_installation_status(OS, &data_dir, &cache_dir)
        .await
        .map_err(|e| e.into())
}
