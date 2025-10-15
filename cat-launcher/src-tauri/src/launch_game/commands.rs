use std::env::consts::OS;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager};

use crate::launch_game::launch_game::{launch_and_monitor_game, GameEvent, LaunchGameError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum LaunchGameCommandError {
    #[error("failed to launch game: {0}")]
    LaunchGame(#[from] LaunchGameError),

    #[error("system directory not found: {0}")]
    SystemDirectoryNotFound(#[from] tauri::Error),

    #[error("failed to get system time: {0}")]
    SystemTime(#[from] SystemTimeError),
}

#[command]
pub async fn launch_game(
    app_handle: AppHandle,
    variant: GameVariant,
    release_id: &str,
) -> Result<(), LaunchGameCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let cache_dir = app_handle.path().app_cache_dir()?;
    let resource_dir = app_handle.path().resource_dir()?;

    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let emitter = app_handle.clone();
    let on_game_event = move |event: GameEvent| {
        let emitter = emitter.clone();
        async move {
            // We cannot handle emit errors
            let _ = emitter.emit("game-event", event);
        }
    };

    launch_and_monitor_game(
        &variant,
        release_id,
        OS,
        time,
        &cache_dir,
        &data_dir,
        &resource_dir,
        on_game_event,
    )
    .await?;

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
