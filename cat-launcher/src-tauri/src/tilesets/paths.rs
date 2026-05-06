use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetGameExecutableDirError, get_game_resources_dir,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

/// Errors that can occur when determining the stock tilesets directory.
#[derive(thiserror::Error, Debug)]
pub enum GetStockTilesetsDirError {
  /// An error occurred while determining the game resources directory.
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

/// Returns the absolute path to the stock tilesets directory for a given game variant and version.
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

/// Returns the path to the tilesets metadata resource file.
pub fn get_tilesets_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("tilesets.json")
}
