use std::env::consts::OS;

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use crate::game_release::game_release::GameReleaseStatus;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::infra::utils::{get_arch_enum, get_os_enum, ArchNotSupportedError, OSNotSupportedError};
use crate::repository::file_releases_repository::FileReleasesRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GetInstallationStatusCommandError {
    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to obtain release: {0}")]
    Release(#[from] GetReleaseError),

    #[error("failed to get OS enum: {0}")]
    Os(#[from] OSNotSupportedError),

    #[error("failed to get arch enum: {0}")]
    Arch(#[from] ArchNotSupportedError),
}

impl serde::Serialize for GetInstallationStatusCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("GetInstallationStatusCommandError", 2)?;

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
    releases_repository: State<'_, FileReleasesRepository>,
) -> Result<GameReleaseStatus, GetInstallationStatusCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let resource_dir = app_handle.path().resource_dir()?;

    let os = get_os_enum(OS)?;
    let arch = get_arch_enum(std::env::consts::ARCH)?;

    let release = get_release_by_id(
        &variant,
        release_id,
        &os,
        &arch,
        &data_dir,
        &resource_dir,
        &*releases_repository,
    )
    .await?;

    Ok(release.status)
}
