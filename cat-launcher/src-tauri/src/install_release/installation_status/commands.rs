use tauri::{AppHandle, Manager};

use serde::ser::SerializeStruct;
use tauri::{AppHandle, Manager};

use crate::game_release::game_release::{GameRelease, GameReleaseStatus};
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, strum::IntoStaticStr)]
pub enum GetInstallationStatusCommandError {
    #[error("failed to get installation status: {0}")]
    GetStatus(#[from] GetInstallationStatusError),
    #[error("failed to get app data directory: {0}")]
    AppDataDir(#[from] tauri::Error),
    #[error("failed to get OS: {0}")]
    GetOS(#[from] OSNotSupportedError),
}

impl serde::Serialize for GetInstallationStatusCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GetInstallationStatusCommandError", 2)?;
        state.serialize_field("type", Into::<&'static str>::into(self))?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn get_installation_status(
    app_handle: AppHandle,
    variant: GameVariant,
    version: String,
) -> Result<GameReleaseStatus, GetInstallationStatusCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;
    let release = GameRelease {
        version,
        variant,
        // These fields are not used by get_installation_status, so we can use dummy values.
        release_type: crate::game_release::game_release::ReleaseType::Experimental,
        status: GameReleaseStatus::Unknown,
        created_at: chrono::Utc::now(),
    };
    release
        .get_installation_status(&os, &data_dir)
        .await
        .map_err(GetInstallationStatusCommandError::GetStatus)
}
