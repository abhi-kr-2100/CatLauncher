use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::fs;
use ts_rs::TS;

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::mods::repository::{InstalledModsRepository, InstalledModsRepositoryError};
use crate::mods::validation::{validate_mod_id, InvalidModIdError};
use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ModInstallationStatus {
    Installed,
    NotInstalled,
    Corrupted, // In repository but files missing
}

#[derive(Error, Debug)]
pub enum GetModInstallationStatusError {
    #[error("invalid mod ID: {0}")]
    InvalidModId(#[from] InvalidModIdError),

    #[error("failed to check repository: {0}")]
    Repository(#[from] InstalledModsRepositoryError),

    #[error("failed to get user data directory: {0}")]
    UserDataDir(#[from] GetUserGameDataDirError),
}



pub async fn get_mod_installation_status(
    variant: &GameVariant,
    mod_id: &str,
    data_dir: &Path,
    installed_mods_repository: &dyn InstalledModsRepository,
) -> Result<ModInstallationStatus, GetModInstallationStatusError> {
    // Validate mod_id to prevent path traversal and ensure safety
    validate_mod_id(mod_id)?;

    let is_in_repository = installed_mods_repository.is_mod_installed(mod_id, variant).await?;

    if !is_in_repository {
        return Ok(ModInstallationStatus::NotInstalled);
    }

    let user_data_dir = get_or_create_user_game_data_dir(variant, data_dir).await?;
    let mod_dir = user_data_dir.join("mods").join(mod_id);

    let is_valid_dir = fs::metadata(&mod_dir)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false);

    if is_valid_dir {
        Ok(ModInstallationStatus::Installed)
    } else {
        Ok(ModInstallationStatus::Corrupted)
    }
}
