use std::io;
use std::path::Path;

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartySoundpackError {
    #[error("failed to remove installed soundpack from repository: {0}")]
    Repository(#[from] InstalledSoundpacksRepositoryError),
    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
    #[error("failed to delete soundpack directory: {0}")]
    DeleteSoundpackDirectory(#[from] io::Error),
}

pub async fn uninstall_third_party_soundpack(
    soundpack_id: &str,
    game_variant: &GameVariant,
    data_dir: &Path,
    repository: &impl InstalledSoundpacksRepository,
) -> Result<(), UninstallThirdPartySoundpackError> {
    // Remove from repository
    repository
        .delete_installed_soundpack(soundpack_id, game_variant)
        .await?;

    // Delete soundpack directory
    let user_game_data_dir = get_or_create_user_game_data_dir(game_variant, data_dir).await?;
    let soundpack_dir = user_game_data_dir.join("sound").join(soundpack_id);
    tokio::fs::remove_dir_all(&soundpack_dir).await?;

    Ok(())
}
