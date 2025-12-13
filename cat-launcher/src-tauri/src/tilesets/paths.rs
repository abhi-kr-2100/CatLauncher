use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  get_game_resources_dir, GetGameExecutableDirError,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetStockTilesetsDirError {
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

pub async fn get_stock_tilesets_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetStockTilesetsDirError> {
  let game_resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  Ok(game_resources_dir.join("gfx"))
}

pub fn get_tilesets_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("tilesets.json")
}
