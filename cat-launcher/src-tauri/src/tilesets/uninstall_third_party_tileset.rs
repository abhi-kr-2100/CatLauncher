use std::io;
use std::path::Path;

use crate::filesystem::paths::{
  GetUserGameDataDirError, get_or_create_user_game_data_dir,
};
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

/// Errors that can occur when uninstalling a third-party tileset.
#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyTilesetError {
  /// An error occurred while removing the tileset record from the repository.
  #[error("failed to remove installed tileset from repository: {0}")]
  Repository(#[from] InstalledTilesetsRepositoryError),
  /// An error occurred while determining the user game data directory.
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
  /// An error occurred while deleting the tileset directory from the filesystem.
  #[error("failed to delete tileset directory: {0}")]
  DeleteTilesetDirectory(#[from] io::Error),
}

/// Uninstalls a third-party tileset by removing its files and repository record.
pub async fn uninstall_third_party_tileset(
  tileset_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  repository: &impl InstalledTilesetsRepository,
) -> Result<(), UninstallThirdPartyTilesetError> {
  // Remove from repository
  repository
    .delete_installed_tileset(tileset_id, game_variant)
    .await?;

  // Delete tileset directory
  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let tileset_dir = user_game_data_dir.join("gfx").join(tileset_id);
  tokio::fs::remove_dir_all(&tileset_dir).await?;

  Ok(())
}
