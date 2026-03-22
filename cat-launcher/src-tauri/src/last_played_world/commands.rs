use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::last_played_world::last_played_world::{
  get_last_played_world as get_last_played_world_impl,
  GetLastPlayedWorldError,
};
use crate::variants::GameVariant;
use cat_macros::CommandErrorSerialize;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetLastPlayedWorldCommandError {
  #[error("failed to get last played world: {0}")]
  GetLastPlayedWorld(#[from] GetLastPlayedWorldError),

  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),
}

#[command]
pub async fn get_last_played_world(
  app_handle: AppHandle,
  variant: GameVariant,
) -> Result<Option<String>, GetLastPlayedWorldCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let last_played_world =
    get_last_played_world_impl(&data_dir, &variant).await?;

  Ok(last_played_world)
}
