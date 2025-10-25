use super::repository::PlayTimeRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, State};

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum PlayTimeCommandError {
    #[error("failed to get play time for version: {0}")]
    GetPlayTimeForVersion(String),
    #[error("failed to get play time for variant: {0}")]
    GetPlayTimeForVariant(String),
    #[error("failed to get total play time: {0}")]
    GetTotalPlayTime(String),
}

impl serde::Serialize for PlayTimeCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("PlayTimeCommandError", 2)?;
        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;
        let msg = self.to_string();
        st.serialize_field("message", &msg)?;
        st.end()
    }
}

#[command]
pub async fn get_play_time_for_version(
    game_variant: String,
    version: String,
    play_time_repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, PlayTimeCommandError> {
    play_time_repository
        .get_play_time_for_version(game_variant, version)
        .await
        .map_err(|e| PlayTimeCommandError::GetPlayTimeForVersion(e.to_string()))
}

#[command]
pub async fn get_play_time_for_variant(
    game_variant: String,
    play_time_repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, PlayTimeCommandError> {
    play_time_repository
        .get_play_time_for_variant(game_variant)
        .await
        .map_err(|e| PlayTimeCommandError::GetPlayTimeForVariant(e.to_string()))
}

#[command]
pub async fn get_total_play_time(
    play_time_repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, PlayTimeCommandError> {
    play_time_repository
        .get_total_play_time()
        .await
        .map_err(|e| PlayTimeCommandError::GetTotalPlayTime(e.to_string()))
}
