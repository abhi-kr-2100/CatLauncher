use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetGameExecutableDirError, get_game_resources_dir,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

/// Errors that can occur when determining the stock soundpacks directory.
#[derive(thiserror::Error, Debug)]
pub enum GetStockSoundpacksDirError {
  /// An error occurred while determining the game resources directory.
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

/// Returns the absolute path to the stock soundpacks directory for a given game variant and version.
pub async fn get_stock_soundpacks_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetStockSoundpacksDirError> {
  let game_resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  Ok(game_resources_dir.join("data").join("sound"))
}

/// Returns the path to the soundpacks metadata resource file.
pub fn get_soundpacks_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("soundpacks.json")
}
