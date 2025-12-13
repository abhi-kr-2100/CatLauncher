use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  get_game_resources_dir, GetGameExecutableDirError,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetStockModsDirError {
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

pub async fn get_stock_mods_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetStockModsDirError> {
  let game_resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  Ok(game_resources_dir.join("data").join("mods"))
}

pub fn get_mods_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("mods.json")
}
