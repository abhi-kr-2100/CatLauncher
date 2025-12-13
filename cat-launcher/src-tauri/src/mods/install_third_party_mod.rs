use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::fs::{create_dir_all, read_to_string};

use crate::filesystem::paths::{
    get_or_create_directory, get_or_create_user_game_data_dir, GetOrCreateDirectoryError,
    GetUserGameDataDirError,
};
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::download::{DownloadFileError, Downloader, NoOpReporter};
use crate::infra::utils::OS;
use crate::mods::paths::get_mods_resource_path;
use crate::mods::repository::installed_mods_repository::{
    InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyModError {
    #[error("failed to get mod from mods.json: {0}")]
    GetModFromJson(#[from] GetModFromJsonError),

    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),

    #[error("failed to download mod: {0}")]
    Download(#[from] DownloadFileError),

    #[error("failed to extract mod: {0}")]
    Extract(#[from] ExtractionError),

    #[error("failed to get mod parent dir: {0}")]
    GetModParentDir(#[from] GetModParentDirError),

    #[error("failed to get user game data dir: {0}")]
    GetUserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to get user mod data dir: {0}")]
    GetUserModDataDir(#[from] GetOrCreateDirectoryError),

    #[error("failed to copy mod: {0}")]
    Copy(#[from] CopyDirError),

    #[error("failed to update repository: {0}")]
    UpdateRepository(#[from] InstalledModsRepositoryError),
}

pub async fn install_third_party_mod(
    mod_id: &str,
    game_variant: &GameVariant,
    data_dir: &Path,
    resource_dir: &Path,
    temp_dir: &Path,
    os: &OS,
    downloader: &Downloader,
    repository: &impl InstalledModsRepository,
) -> Result<(), InstallThirdPartyModError> {
    // Get mod details from mods.json
    let mod_details = get_mod_from_json(game_variant, mod_id, resource_dir).await?;

    // Create a temp directory for this mod download
    let mod_temp_dir = temp_dir.join("cat-launcher-mod-install-dir").join(mod_id);
    create_dir_all(&mod_temp_dir).await?;

    // Download the mod
    let reporter = Arc::new(NoOpReporter);
    let downloaded_file = downloader
        .download_file(
            &mod_details.installation.download_url,
            &mod_temp_dir,
            reporter,
        )
        .await?;

    // Extract the mod to the temp directory
    let extraction_dir = mod_temp_dir.join("extracted");
    create_dir_all(&extraction_dir).await?;
    extract_archive(&downloaded_file, &extraction_dir, os).await?;

    // Get the mod parent directory from the modinfo path
    let mod_parent_dir = get_mod_parent_dir(&extraction_dir, &mod_details.installation.modinfo)?;

    // Get the mods directory in user game data
    let user_game_data_dir = get_or_create_user_game_data_dir(game_variant, data_dir).await?;
    let mods_dir = get_or_create_directory(&user_game_data_dir, "mods").await?;

    // Copy the mod parent directory to the mods directory
    let mod_install_dir = mods_dir.join(mod_id);
    copy_dir_all(&mod_parent_dir, &mod_install_dir, os).await?;

    // Mark the mod as installed in the repository
    repository.add_installed_mod(mod_id, game_variant).await?;

    // Clean up temp files, ignore any errors
    let _ = tokio::fs::remove_dir_all(&mod_temp_dir).await;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetModFromJsonError {
    #[error("failed to read modinfo.json: {0}")]
    ReadModInfoJson(#[from] std::io::Error),

    #[error("failed to parse modinfo.json: {0}")]
    ParseModInfoJson(#[from] serde_json::Error),

    #[error("no mods found for variant {0}")]
    NoModsForVariant(GameVariant),

    #[error("mod with id {0} not found")]
    ModNotFound(String),
}

async fn get_mod_from_json(
    game_variant: &GameVariant,
    mod_id: &str,
    resource_dir: &Path,
) -> Result<ThirdPartyMod, GetModFromJsonError> {
    let mods_json_path = get_mods_resource_path(resource_dir);
    let content = read_to_string(&mods_json_path).await?;

    let mods_data: HashMap<GameVariant, HashMap<String, serde_json::Value>> =
        serde_json::from_str(&content)?;

    let variant_mods = mods_data
        .get(game_variant)
        .ok_or(GetModFromJsonError::NoModsForVariant(*game_variant))?;

    let mod_data = variant_mods
        .get(mod_id)
        .ok_or(GetModFromJsonError::ModNotFound(mod_id.to_string()))?;

    let third_party_mod = serde_json::from_value::<ThirdPartyMod>(mod_data.clone())?;

    Ok(third_party_mod)
}

#[derive(Debug, thiserror::Error)]
pub enum GetModParentDirError {
    #[error("failed to get parent directory for modinfo path")]
    ParentDirNotFound,
}

fn get_mod_parent_dir(
    extracted_dir: &Path,
    modinfo_relative_path: &str,
) -> Result<PathBuf, GetModParentDirError> {
    let modinfo_path = extracted_dir.join(modinfo_relative_path);

    modinfo_path
        .parent()
        .ok_or(GetModParentDirError::ParentDirNotFound)
        .map(|p| p.to_path_buf())
}
