use std::io;
use std::path::Path;

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::tilesets::repository::installed_tilesets_repository::{
    InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyTilesetError {
    #[error("failed to remove installed tileset from repository: {0}")]
    Repository(#[from] InstalledTilesetsRepositoryError),
    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
    #[error("failed to delete tileset directory: {0}")]
    DeleteTilesetDirectory(#[from] io::Error),
}

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
    let user_game_data_dir = get_or_create_user_game_data_dir(game_variant, data_dir).await?;
    let tileset_dir = user_game_data_dir.join("gfx").join(tileset_id);
    tokio::fs::remove_dir_all(&tileset_dir).await?;

    Ok(())
}
