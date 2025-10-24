use serde::{ser::SerializeStruct, Serializer};
use strum_macros::IntoStaticStr;
use tauri::{command, State};

use crate::last_played::last_played::LastPlayedError;
use crate::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum LastPlayedCommandError {
    #[error("failed to get last played version: {0}")]
    GetLastPlayedVersion(#[from] LastPlayedError),

    #[error("failed to get system directory: {0}")]
    SystemDirectory(#[from] tauri::Error),
}

#[command]
pub async fn get_last_played_version(
    variant: GameVariant,
    repository: State<'_, SqliteLastPlayedVersionRepository>,
) -> Result<Option<String>, LastPlayedCommandError> {
    let last_played_version = variant.get_last_played_version(&*repository).await?;

    Ok(last_played_version)
}

impl serde::Serialize for LastPlayedCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("LastPlayedCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
