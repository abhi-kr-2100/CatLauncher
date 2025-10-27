use serde::ser::SerializeMap;
use serde::Serialize;
use strum::IntoStaticStr;
use tauri::State;

use crate::play_time::play_time::{get_play_time_for_variant as get_play_time_for_variant_feature, get_play_time_for_version as get_play_time_for_version_feature};
use crate::play_time::repository::PlayTimeRepositoryError;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GetPlayTimeCommandError {
    #[error("Failed to get play time: {0}")]
    Repository(#[from] PlayTimeRepositoryError),
}

impl Serialize for GetPlayTimeCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("type", &self.to_string())?;
        state.serialize_entry("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn get_play_time_for_variant(
    game_variant_id: GameVariant,
    repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
    let result = get_play_time_for_variant_feature(&game_variant_id, &*repository).await?;
    Ok(result)
}

#[tauri::command]
pub async fn get_play_time_for_version(
    game_variant_id: GameVariant,
    version: String,
    repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
    let result = get_play_time_for_version_feature(&game_variant_id, &version, &*repository).await?;
    Ok(result)
}
