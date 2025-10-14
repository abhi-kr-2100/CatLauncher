use std::env::consts::OS;

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::game_release::game_release::GameReleaseStatus;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum InstallationStatusCommandError {
    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to get installation status: {0}")]
    InstallationStatus(#[from] GetInstallationStatusError),

    #[error("failed to obtain release: {0}")]
    Release(#[from] GetReleaseError),
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
    release_id: &str,
) -> Result<GameReleaseStatus, InstallationStatusCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let cache_dir = app_handle.path().app_cache_dir()?;
    let resource_dir = app_handle.path().resource_dir()?;

    let release = get_release_by_id(
        &variant,
        release_id,
        OS,
        &cache_dir,
        &data_dir,
        &resource_dir,
    )
    .await?;

    Ok(release.status)
}
