use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::{command, State};

use crate::variants::GameVariant;
use crate::world_options::repository::fs_world_options_repository::FsWorldOptionsRepository;
use crate::world_options::repository::{
  WorldOptionsError, WorldOptionsRepository,
};
use crate::world_options::types::{World, WorldOption};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum WorldOptionsCommandError {
  #[error("failed to get worlds: {0}")]
  GetWorlds(#[source] WorldOptionsError),

  #[error("failed to get world options: {0}")]
  GetWorldOptions(#[source] WorldOptionsError),

  #[error("failed to update world options: {0}")]
  UpdateWorldOptions(#[source] WorldOptionsError),
}

#[command]
pub async fn get_worlds(
  variant: GameVariant,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<Vec<World>, WorldOptionsCommandError> {
  repo
    .get_worlds(&variant)
    .await
    .map_err(WorldOptionsCommandError::GetWorlds)
}

#[command]
pub async fn get_world_options(
  variant: GameVariant,
  world: String,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<Vec<WorldOption>, WorldOptionsCommandError> {
  repo
    .get_world_options(&variant, &world)
    .await
    .map_err(WorldOptionsCommandError::GetWorldOptions)
}

#[command]
pub async fn update_world_options(
  variant: GameVariant,
  world: String,
  options: Vec<WorldOption>,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<(), WorldOptionsCommandError> {
  repo
    .update_world_options(&variant, &world, &options)
    .await
    .map_err(WorldOptionsCommandError::UpdateWorldOptions)
}
