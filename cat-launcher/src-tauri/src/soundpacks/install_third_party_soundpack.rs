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
use crate::soundpacks::paths::get_soundpacks_resource_path;
use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::soundpacks::types::ThirdPartySoundpack;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartySoundpackError {
  #[error("failed to get soundpack from soundpacks.json: {0}")]
  GetSoundpackFromJson(#[from] GetSoundpackFromJsonError),

  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),

  #[error("failed to download soundpack: {0}")]
  Download(#[from] DownloadFileError),

  #[error("failed to extract soundpack: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to get soundpack parent dir: {0}")]
  GetSoundpackParentDir(#[from] GetSoundpackParentDirError),

  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get user soundpack data dir: {0}")]
  GetUserSoundpackDataDir(#[from] GetOrCreateDirectoryError),

  #[error("failed to copy soundpack: {0}")]
  Copy(#[from] CopyDirError),

  #[error("failed to update repository: {0}")]
  UpdateRepository(#[from] InstalledSoundpacksRepositoryError),
}

#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_soundpack(
  soundpack_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  repository: &impl InstalledSoundpacksRepository,
) -> Result<(), InstallThirdPartySoundpackError> {
  // Get soundpack details from soundpacks.json
  let soundpack_details =
    get_soundpack_from_json(game_variant, soundpack_id, resource_dir)
      .await?;

  // Create a temp directory for this soundpack download
  let soundpack_temp_dir = temp_dir
    .join("cat-launcher-soundpack-install-dir")
    .join(soundpack_id);
  create_dir_all(&soundpack_temp_dir).await?;

  // Download the soundpack
  let reporter = Arc::new(NoOpReporter);
  let downloaded_file = downloader
    .download_file(
      &soundpack_details.installation.download_url,
      &soundpack_temp_dir,
      reporter,
    )
    .await?;

  // Extract the soundpack to the temp directory
  let extraction_dir = soundpack_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  // Get the soundpack parent directory from the soundpack path
  let soundpack_parent_dir = get_soundpack_parent_dir(
    &extraction_dir,
    &soundpack_details.installation.soundpack,
  )?;

  // Get the sounds directory in user game data
  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let sounds_dir =
    get_or_create_directory(&user_game_data_dir, "sound").await?;

  // Copy the soundpack parent directory to the sounds directory
  let soundpack_install_dir = sounds_dir.join(soundpack_id);
  copy_dir_all(&soundpack_parent_dir, &soundpack_install_dir, os)
    .await?;

  // Mark the soundpack as installed in the repository
  repository
    .add_installed_soundpack(soundpack_id, game_variant)
    .await?;

  // Clean up temp files, ignore any errors
  let _ = tokio::fs::remove_dir_all(&soundpack_temp_dir).await;

  Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetSoundpackFromJsonError {
  #[error("failed to read soundpacks.json: {0}")]
  ReadSoundpacksJson(#[from] std::io::Error),

  #[error("failed to parse soundpacks.json: {0}")]
  ParseSoundpacksJson(#[from] serde_json::Error),

  #[error("no soundpacks found for variant {0}")]
  NoSoundpacksForVariant(GameVariant),

  #[error("soundpack with id {0} not found")]
  SoundpackNotFound(String),
}

async fn get_soundpack_from_json(
  game_variant: &GameVariant,
  soundpack_id: &str,
  resource_dir: &Path,
) -> Result<ThirdPartySoundpack, GetSoundpackFromJsonError> {
  let soundpacks_json_path =
    get_soundpacks_resource_path(resource_dir);
  let content = read_to_string(&soundpacks_json_path).await?;

  let soundpacks_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_soundpacks = soundpacks_data.get(game_variant).ok_or(
    GetSoundpackFromJsonError::NoSoundpacksForVariant(*game_variant),
  )?;

  let soundpack_data = variant_soundpacks.get(soundpack_id).ok_or(
    GetSoundpackFromJsonError::SoundpackNotFound(
      soundpack_id.to_string(),
    ),
  )?;

  let third_party_soundpack = serde_json::from_value::<
    ThirdPartySoundpack,
  >(soundpack_data.clone())?;

  Ok(third_party_soundpack)
}

#[derive(Debug, thiserror::Error)]
pub enum GetSoundpackParentDirError {
  #[error("failed to get parent directory for soundpack path")]
  ParentDirNotFound,
}

fn get_soundpack_parent_dir(
  extracted_dir: &Path,
  soundpack_relative_path: &str,
) -> Result<PathBuf, GetSoundpackParentDirError> {
  let soundpack_path = extracted_dir.join(soundpack_relative_path);

  soundpack_path
    .parent()
    .ok_or(GetSoundpackParentDirError::ParentDirNotFound)
    .map(|p| p.to_path_buf())
}
