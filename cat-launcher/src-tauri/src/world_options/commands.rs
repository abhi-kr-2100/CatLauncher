use std::collections::HashMap;

use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use crate::variants::GameVariant;
use crate::world_options::metadata::{
  get_metadata_for_variant, GetMetadataError,
};
use crate::world_options::repository::fs_world_options_repository::FsWorldOptionsRepository;
use crate::world_options::repository::{
  WorldOptionsError, WorldOptionsRepository,
};
use crate::world_options::types::{
  World, WorldOption, WorldOptionMetadata,
};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetWorldsError {
  #[error("failed to get worlds: {0}")]
  WorldOptions(#[from] WorldOptionsError),
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetWorldOptionsError {
  #[error("failed to get world options: {0}")]
  WorldOptions(#[from] WorldOptionsError),
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UpdateWorldOptionsError {
  #[error("failed to update world options: {0}")]
  WorldOptions(#[from] WorldOptionsError),
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetWorldOptionsMetadataError {
  #[error("failed to get metadata: {0}")]
  GetMetadata(#[from] GetMetadataError),

  #[error("failed to get resource directory: {0}")]
  ResourceDir(#[from] tauri::Error),
}

#[command]
pub async fn get_worlds(
  variant: GameVariant,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<Vec<World>, GetWorldsError> {
  Ok(repo.get_worlds(&variant).await?)
}

#[command]
pub async fn get_world_options(
  variant: GameVariant,
  world: String,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<Vec<WorldOption>, GetWorldOptionsError> {
  Ok(repo.get_world_options(&variant, &world).await?)
}

#[command]
pub async fn update_world_options(
  variant: GameVariant,
  world: String,
  options: Vec<WorldOption>,
  repo: State<'_, FsWorldOptionsRepository>,
) -> Result<(), UpdateWorldOptionsError> {
  Ok(
    repo
      .update_world_options(&variant, &world, &options)
      .await?,
  )
}

#[command]
pub async fn get_world_options_metadata(
  variant: GameVariant,
  app_handle: AppHandle,
) -> Result<
  HashMap<String, WorldOptionMetadata>,
  GetWorldOptionsMetadataError,
> {
  let resources_dir = app_handle.path().resource_dir()?;
  let metadata =
    get_metadata_for_variant(&variant, &resources_dir).await?;
  Ok(metadata)
}
