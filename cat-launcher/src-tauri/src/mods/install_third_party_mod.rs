use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use downloader::progress::Reporter;
use tokio::fs::create_dir_all;

use crate::filesystem::paths::{
  get_or_create_directory, get_or_create_user_game_data_dir,
  GetOrCreateDirectoryError, GetUserGameDataDirError,
};
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::download::{DownloadFileError, Downloader};
use crate::infra::utils::OS;
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError, ModsRepository,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstallThirdPartyModError {
  #[error("failed to get mod from repository: {0}")]
  GetModFromRepository(#[from] GetThirdPartyModByIdError),

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

#[allow(clippy::too_many_arguments)]
pub async fn install_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  temp_dir: &Path,
  os: &OS,
  downloader: &Downloader,
  installed_mods_repository: &impl InstalledModsRepository,
  mods_repository: &impl ModsRepository,
  reporter: Arc<dyn Reporter + Send + Sync>,
) -> Result<(), InstallThirdPartyModError> {
  let mod_details = mods_repository
    .get_third_party_mod_by_id(mod_id, game_variant)
    .await?;

  let mod_temp_dir =
    temp_dir.join("cat-launcher-mod-install-dir").join(mod_id);
  create_dir_all(&mod_temp_dir).await?;

  let downloaded_file = downloader
    .download_file(
      &mod_details.installation.download_url,
      &mod_temp_dir,
      reporter,
    )
    .await?;

  let extraction_dir = mod_temp_dir.join("extracted");
  create_dir_all(&extraction_dir).await?;
  extract_archive(&downloaded_file, &extraction_dir, os).await?;

  let mod_parent_dir = get_mod_parent_dir(
    &extraction_dir,
    &mod_details.installation.modinfo,
  )?;

  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let mods_dir =
    get_or_create_directory(&user_game_data_dir, "mods").await?;

  let mod_install_dir = mods_dir.join(mod_id);
  copy_dir_all(&mod_parent_dir, &mod_install_dir, os).await?;

  let _ = tokio::fs::remove_dir_all(&mod_temp_dir).await;

  installed_mods_repository
    .add_installed_mod(mod_id, game_variant)
    .await?;

  Ok(())
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
