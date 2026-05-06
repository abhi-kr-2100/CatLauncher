use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs::{create_dir_all, read_to_string};

use crate::filesystem::paths::{
  GetOrCreateDirectoryError, GetUserGameDataDirError,
  get_or_create_directory, get_or_create_user_game_data_dir,
};
use crate::filesystem::utils::{CopyDirError, copy_dir_all};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::download::{DownloadFileError, Downloader};

use crate::infra::utils::OS;
use crate::tilesets::paths::get_tilesets_resource_path;
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::tilesets::types::ThirdPartyTileset;
use crate::variants::GameVariant;

/// Errors that can occur when installing a third-party tileset.
#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyTilesetError {
  /// An error occurred while retrieving tileset details from the metadata file.
  #[error("failed to get tileset from tilesets.json: {0}")]
  GetTilesetFromJson(#[from] GetTilesetFromJsonError),

  /// An error occurred while creating a directory.
  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),

  /// An error occurred while downloading the tileset.
  #[error("failed to download tileset: {0}")]
  Download(#[from] DownloadFileError),

  /// An error occurred while extracting the tileset archive.
  #[error("failed to extract tileset: {0}")]
  Extract(#[from] ExtractionError),

  /// An error occurred while determining the parent directory of the tileset within the archive.
  #[error("failed to get tileset parent dir: {0}")]
  GetTilesetParentDir(#[from] GetTilesetParentDirError),

  /// An error occurred while determining the user game data directory.
  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  /// An error occurred while creating the user tileset data directory.
  #[error("failed to get user tileset data dir: {0}")]
  GetUserTilesetDataDir(#[from] GetOrCreateDirectoryError),

  /// An error occurred while copying tileset files.
  #[error("failed to copy tileset: {0}")]
  Copy(#[from] CopyDirError),

  /// An error occurred while updating the installed tilesets repository.
  #[error("failed to update repository: {0}")]
  UpdateRepository(#[from] InstalledTilesetsRepositoryError),
}

/// Downloads, extracts, and installs a third-party tileset for a given game variant.
#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_tileset(
  tileset_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  repository: &impl InstalledTilesetsRepository,
  reporter: Arc<dyn Reporter + Send + Sync>,
) -> Result<(), InstallThirdPartyTilesetError> {
  let tileset_details =
    get_tileset_from_json(game_variant, tileset_id, resource_dir)
      .await?;

  let tileset_temp_dir = temp_dir
    .join("cat-launcher-tileset-install-dir")
    .join(tileset_id);
  create_dir_all(&tileset_temp_dir).await?;

  let downloaded_file = downloader
    .download_file(
      &tileset_details.installation.download_url,
      &tileset_temp_dir,
      reporter,
    )
    .await?;

  let extraction_dir = tileset_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  let tileset_parent_dir = get_tileset_parent_dir(
    &extraction_dir,
    &tileset_details.installation.tileset,
  )?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let gfx_dir =
    get_or_create_directory(&user_game_data_dir, "gfx").await?;

  let tileset_install_dir = gfx_dir.join(tileset_id);
  copy_dir_all(&tileset_parent_dir, &tileset_install_dir, os).await?;

  let _ = tokio::fs::remove_dir_all(&tileset_temp_dir).await;

  repository
    .add_installed_tileset(tileset_id, game_variant)
    .await?;

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

  let tilesets_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_tilesets = tilesets_data.get(game_variant).ok_or(
    GetTilesetFromJsonError::NoTilesetsForVariant(*game_variant),
  )?;

  let tileset_data = variant_tilesets.get(tileset_id).ok_or(
    GetTilesetFromJsonError::TilesetNotFound(tileset_id.to_string()),
  )?;

  let third_party_tileset = serde_json::from_value::<
    ThirdPartyTileset,
  >(tileset_data.clone())?;

  Ok(third_party_tileset)
}

/// Errors that can occur when determining the parent directory of a tileset.
#[derive(Debug, thiserror::Error)]
pub enum GetTilesetParentDirError {
  /// The parent directory for the specified tileset path could not be found.
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
