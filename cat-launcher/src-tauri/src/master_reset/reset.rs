use std::path::Path;

use tokio::fs;

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::settings::repository::settings_repository::{
  SaveSettingsError, SettingsRepository,
};
use crate::settings::Settings;
use crate::soundpacks::repository::installed_soundpacks_repository::{
  InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum MasterResetError {
  #[error("failed to get user data directory: {0}")]
  UserDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to read directory: {0}")]
  ReadDir(#[from] std::io::Error),

  #[error("failed to remove entry: {0}")]
  RemoveEntry(std::io::Error),

  #[error("failed to delete installed mods: {0}")]
  DeleteMods(#[from] InstalledModsRepositoryError),

  #[error("failed to delete installed soundpacks: {0}")]
  DeleteSoundpacks(#[from] InstalledSoundpacksRepositoryError),

  #[error("failed to delete installed tilesets: {0}")]
  DeleteTilesets(#[from] InstalledTilesetsRepositoryError),

  #[error("failed to reset settings: {0}")]
  ResetSettings(#[from] SaveSettingsError),
}

async fn should_skip(
  entry: &fs::DirEntry,
) -> Result<bool, std::io::Error> {
  let file_name = entry.file_name();
  let file_name_str = file_name.to_string_lossy();

  if file_name_str.eq_ignore_ascii_case("save")
    && entry.file_type().await?.is_dir()
  {
    return Ok(true);
  }

  Ok(false)
}

pub async fn master_reset(
  variant: &GameVariant,
  data_dir: &Path,
  installed_mods_repository: &dyn InstalledModsRepository,
  installed_soundpacks_repository: &dyn InstalledSoundpacksRepository,
  installed_tilesets_repository: &dyn InstalledTilesetsRepository,
  settings_repository: &dyn SettingsRepository,
) -> Result<(), MasterResetError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;

  let mut entries = fs::read_dir(&user_data_dir).await?;

  while let Some(entry) = entries.next_entry().await? {
    if should_skip(&entry).await? {
      continue;
    }

    let path = entry.path();
    if entry.file_type().await?.is_dir() {
      fs::remove_dir_all(&path)
        .await
        .map_err(MasterResetError::RemoveEntry)?;
    } else {
      fs::remove_file(&path)
        .await
        .map_err(MasterResetError::RemoveEntry)?;
    }
  }

  // Update DB sequentially to avoid deadlocks
  installed_mods_repository
    .delete_all_installed_mods(variant)
    .await?;

  installed_soundpacks_repository
    .delete_all_installed_soundpacks(variant)
    .await?;

  installed_tilesets_repository
    .delete_all_installed_tilesets(variant)
    .await?;

  settings_repository
    .save_settings(&Settings::default())
    .await?;

  Ok(())
}
