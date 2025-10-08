use std::env::consts::OS;

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::infra::http_client::HTTP_CLIENT;
use crate::install_release::install_release::ReleaseInstallationError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum InstallReleaseCommandError {
    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("installation failed: {0}")]
    Install(#[from] ReleaseInstallationError),

    #[error("failed to obtain release: {0}")]
    Release(#[from] GetReleaseError),
}

#[command]
pub async fn install_release(
    app_handle: AppHandle,
    variant: GameVariant,
    release_id: &str,
) -> Result<GameRelease, InstallReleaseCommandError> {
    let cache_dir = app_handle.path().app_cache_dir()?;
    let data_dir = app_handle.path().app_local_data_dir()?;

    let mut release = get_release_by_id(&variant, OS, &data_dir, &cache_dir, release_id).await?;
    release
        .install_release(&HTTP_CLIENT, OS, &cache_dir, &data_dir)
        .await?;

    Ok(release)
}

impl serde::Serialize for InstallReleaseCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("InstallReleaseCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
