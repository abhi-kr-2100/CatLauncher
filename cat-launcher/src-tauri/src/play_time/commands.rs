use strum::IntoStaticStr;
use tauri::State;

use cat_macros::CommandErrorSerialize;

use crate::play_time::play_time::{
  get_play_time_for_variant as get_play_time_for_variant_feature,
  get_play_time_for_version as get_play_time_for_version_feature,
  log_play_time as log_play_time_feature,
};
use crate::play_time::repository::PlayTimeRepositoryError;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetPlayTimeCommandError {
  #[error("Failed to get play time: {0}")]
  Repository(#[from] PlayTimeRepositoryError),
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum LogPlayTimeCommandError {
  #[error("Failed to log play time: {0}")]
  Repository(#[from] PlayTimeRepositoryError),
}

#[tauri::command]
pub async fn get_play_time_for_variant(
  variant: GameVariant,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
  let result =
    get_play_time_for_variant_feature(&variant, &*repository).await?;
  Ok(result)
}

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
