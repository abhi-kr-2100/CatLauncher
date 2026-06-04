use tauri::State;

use cat_macros::CommandErrorSerialize;

use crate::play_time::play_time::{
  get_play_time_for_variant as get_play_time_for_variant_feature,
  get_play_time_for_version as get_play_time_for_version_feature,
  log_play_time as log_play_time_feature,
};
use crate::play_time::repository::PlayTimeRepositoryError;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::variants::game_variant::GameVariant;

/// Errors that can occur when retrieving play time.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetPlayTimeCommandError {
  /// An error occurred in the play time repository.
  #[error("Failed to get play time: {0}")]
  Repository(#[from] PlayTimeRepositoryError),
}

/// Errors that can occur when logging play time.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum LogPlayTimeCommandError {
  /// An error occurred in the play time repository.
  #[error("Failed to log play time: {0}")]
  Repository(#[from] PlayTimeRepositoryError),
}

/// Retrieves the total play time for a specific game variant.
#[tauri::command]
pub async fn get_play_time_for_variant(
  variant: GameVariant,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
  let result =
    get_play_time_for_variant_feature(&variant, &*repository).await?;
  Ok(result)
}

/// Retrieves the play time for a specific version of a game variant.
#[tauri::command]
pub async fn get_play_time_for_version(
  variant: GameVariant,
  version: String,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
  let result = get_play_time_for_version_feature(
    &variant,
    &version,
    &*repository,
  )
  .await?;
  Ok(result)
}

/// Logs play time for a specific version of a game variant.
#[tauri::command]
pub async fn log_play_time(
  variant: GameVariant,
  version: String,
  duration_in_seconds: i64,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<(), LogPlayTimeCommandError> {
  log_play_time_feature(
    &variant,
    &version,
    duration_in_seconds,
    &*repository,
  )
  .await?;
  Ok(())
}
