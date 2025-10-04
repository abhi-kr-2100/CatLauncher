use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::game_release::GameRelease;
use crate::launch_game::launch_game::LaunchGameError;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum LaunchGameCommandError {
    #[error("failed to launch game: {0}")]
    LaunchGame(#[from] LaunchGameError),

    #[error("system directory not found: {0}")]
    SystemDirectoryNotFound(#[from] tauri::Error),
}

#[command]
pub fn launch_game(
    app_handle: AppHandle,
    release: GameRelease,
) -> Result<(), LaunchGameCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;

    release.launch_game(&data_dir)?;

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
