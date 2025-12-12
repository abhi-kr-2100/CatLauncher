use std::path::Path;

use thiserror::Error;
use tokio::fs::{remove_dir_all, try_exists};

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::mods::repository::{InstalledModsRepository, InstalledModsRepositoryError};
use crate::mods::validation::{validate_mod_id, InvalidModIdError};
use crate::variants::GameVariant;

#[derive(Error, Debug)]
pub enum UninstallModError {
    #[error("invalid mod ID: {0}")]
    InvalidModId(#[from] InvalidModIdError),

    #[error("failed to get user game data directory: {0}")]
    UserDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to remove mod directory: {0}")]
    RemoveDir(std::io::Error),

    #[error("failed to update repository: {0}")]
    Repository(#[from] InstalledModsRepositoryError),

    #[error("mod with ID {0} is not installed")]
    ModNotInstalled(String),
}

pub async fn uninstall_mod_for_variant(
    variant: &GameVariant,
    mod_id: &str,
    data_dir: &Path,
    installed_mods_repository: &dyn InstalledModsRepository,
) -> Result<(), UninstallModError> {
    // Validate mod_id to prevent path traversal and ensure safety
    validate_mod_id(mod_id)?;

    let is_installed = installed_mods_repository.is_mod_installed(mod_id, variant).await?;
    if !is_installed {
        return Err(UninstallModError::ModNotInstalled(mod_id.to_string()));
    }

    let user_data_dir = get_or_create_user_game_data_dir(variant, data_dir).await?;
    let mod_dir = user_data_dir.join("mods").join(mod_id);

    if try_exists(&mod_dir).await.unwrap_or(false) {
        remove_dir_all(&mod_dir)
            .await
            .map_err(UninstallModError::RemoveDir)?;
    }

    installed_mods_repository
        .delete_installed_mod(mod_id, variant)
        .await?;

    Ok(())
}
