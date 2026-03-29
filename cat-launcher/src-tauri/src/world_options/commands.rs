use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::variants::GameVariant;
use crate::world_options::types::{World, WorldOption};
use crate::world_options::world_options::{
  get_world_options as get_world_options_impl,
  list_worlds as list_worlds_impl,
  update_world_options as update_world_options_impl,
  GetWorldOptionsError, ListWorldsError, UpdateWorldOptionsError,
};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum WorldOptionsCommandError {
  #[error("failed to list worlds: {0}")]
  ListWorlds(#[from] ListWorldsError),

  #[error("failed to get world options: {0}")]
  GetWorldOptions(#[from] GetWorldOptionsError),

  #[error("failed to update world options: {0}")]
  UpdateWorldOptions(#[from] UpdateWorldOptionsError),

  #[error("failed to get app directory: {0}")]
  Tauri(#[from] tauri::Error),
}

#[command]
pub async fn list_worlds(
  variant: GameVariant,
  app_handle: AppHandle,
) -> Result<Vec<World>, WorldOptionsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let worlds = list_worlds_impl(&variant, &data_dir).await?;
  Ok(worlds)
}

#[command]
pub async fn get_world_options(
  variant: GameVariant,
  world_name: String,
  app_handle: AppHandle,
) -> Result<Vec<WorldOption>, WorldOptionsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let options =
    get_world_options_impl(&variant, &world_name, &data_dir).await?;
  Ok(options)
}

#[command]
pub async fn update_world_options(
  variant: GameVariant,
  world_name: String,
  options: Vec<WorldOption>,
  app_handle: AppHandle,
) -> Result<(), WorldOptionsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  update_world_options_impl(
    &variant,
    &world_name,
    options,
    &data_dir,
  )
  .await?;
  Ok(())
}
