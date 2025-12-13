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
use crate::tilesets::paths::get_tilesets_resource_path;
use crate::tilesets::repository::installed_tilesets_repository::{
    InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::tilesets::types::ThirdPartyTileset;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyTilesetError {
    #[error("failed to get tileset from tilesets.json: {0}")]
    GetTilesetFromJson(#[from] GetTilesetFromJsonError),

    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),

    #[error("failed to download tileset: {0}")]
    Download(#[from] DownloadFileError),

    #[error("failed to extract tileset: {0}")]
    Extract(#[from] ExtractionError),

    #[error("failed to get tileset parent dir: {0}")]
    GetTilesetParentDir(#[from] GetTilesetParentDirError),

    #[error("failed to get user game data dir: {0}")]
    GetUserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to get user tileset data dir: {0}")]
    GetUserTilesetDataDir(#[from] GetOrCreateDirectoryError),

    #[error("failed to copy tileset: {0}")]
    Copy(#[from] CopyDirError),

    #[error("failed to update repository: {0}")]
    UpdateRepository(#[from] InstalledTilesetsRepositoryError),
}

pub async fn install_third_party_tileset(
    tileset_id: &str,
    game_variant: &GameVariant,
    data_dir: &Path,
    resource_dir: &Path,
    temp_dir: &Path,
    os: &OS,
    downloader: &Downloader,
    repository: &impl InstalledTilesetsRepository,
) -> Result<(), InstallThirdPartyTilesetError> {
    // Get tileset details from tilesets.json
    let tileset_details = get_tileset_from_json(game_variant, tileset_id, resource_dir).await?;

    // Create a temp directory for this tileset download
    let tileset_temp_dir = temp_dir.join("cat-launcher-tileset-install-dir").join(tileset_id);
    create_dir_all(&tileset_temp_dir).await?;

    // Download the tileset
    let reporter = Arc::new(NoOpReporter);
    let downloaded_file = downloader
        .download_file(
            &tileset_details.installation.download_url,
            &tileset_temp_dir,
            reporter,
        )
        .await?;

    // Extract the tileset to the temp directory
    let extraction_dir = tileset_temp_dir.join("extracted");
    create_dir_all(&extraction_dir).await?;
    extract_archive(&downloaded_file, &extraction_dir, os).await?;

    // Get the tileset parent directory from the tileset path
    let tileset_parent_dir = get_tileset_parent_dir(&extraction_dir, &tileset_details.installation.tileset)?;

    // Get the gfx directory in user game data
    let user_game_data_dir = get_or_create_user_game_data_dir(game_variant, data_dir).await?;
    let gfx_dir = get_or_create_directory(&user_game_data_dir, "gfx").await?;

    // Copy the tileset parent directory to the gfx directory
    let tileset_install_dir = gfx_dir.join(tileset_id);
    copy_dir_all(&tileset_parent_dir, &tileset_install_dir, os).await?;

    // Mark the tileset as installed in the repository
    repository.add_installed_tileset(tileset_id, game_variant).await?;

    // Clean up temp files, ignore any errors
    let _ = tokio::fs::remove_dir_all(&tileset_temp_dir).await;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetTilesetFromJsonError {
    #[error("failed to read tilesets.json: {0}")]
    ReadTilesetsJson(#[from] std::io::Error),

    #[error("failed to parse tilesets.json: {0}")]
    ParseTilesetsJson(#[from] serde_json::Error),

    #[error("no tilesets found for variant {0}")]
    NoTilesetsForVariant(GameVariant),

    #[error("tileset with id {0} not found")]
    TilesetNotFound(String),
}

async fn get_tileset_from_json(
    game_variant: &GameVariant,
    tileset_id: &str,
    resource_dir: &Path,
) -> Result<ThirdPartyTileset, GetTilesetFromJsonError> {
    let tilesets_json_path = get_tilesets_resource_path(resource_dir);
    let content = read_to_string(&tilesets_json_path).await?;

    let tilesets_data: HashMap<GameVariant, HashMap<String, serde_json::Value>> =
        serde_json::from_str(&content)?;

    let variant_tilesets = tilesets_data
        .get(game_variant)
        .ok_or(GetTilesetFromJsonError::NoTilesetsForVariant(*game_variant))?;

    let tileset_data = variant_tilesets
        .get(tileset_id)
        .ok_or(GetTilesetFromJsonError::TilesetNotFound(tileset_id.to_string()))?;

    let third_party_tileset = serde_json::from_value::<ThirdPartyTileset>(tileset_data.clone())?;

    Ok(third_party_tileset)
}

#[derive(Debug, thiserror::Error)]
pub enum GetTilesetParentDirError {
    #[error("failed to get parent directory for tileset path")]
    ParentDirNotFound,
}

fn get_tileset_parent_dir(
    extracted_dir: &Path,
    tileset_relative_path: &str,
) -> Result<PathBuf, GetTilesetParentDirError> {
    let tileset_path = extracted_dir.join(tileset_relative_path);

    tileset_path
        .parent()
        .ok_or(GetTilesetParentDirError::ParentDirNotFound)
        .map(|p| p.to_path_buf())
}
