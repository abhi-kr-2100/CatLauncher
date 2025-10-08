use std::env::consts::OS;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::launch_game::launch_game::LaunchGameError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum LaunchGameCommandError {
    #[error("failed to launch game: {0}")]
    LaunchGame(#[from] LaunchGameError),

    #[error("system directory not found: {0}")]
    SystemDirectoryNotFound(#[from] tauri::Error),

    #[error("failed to get system time: {0}")]
    SystemTime(#[from] SystemTimeError),

    #[error("failed to obtain release: {0}")]
    Release(#[from] GetReleaseError),
}

#[command]
pub async fn launch_game(
    app_handle: AppHandle,
    variant: GameVariant,
    release_id: &str,
) -> Result<(), LaunchGameCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let cache_dir = app_handle.path().app_cache_dir()?;

    let release = get_release_by_id(&variant, release_id, OS, &cache_dir, &data_dir).await?;

    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    release.launch_game(OS, time, &data_dir).await?;

    Ok(())
}

impl serde::Serialize for LaunchGameCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("LaunchGameCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
