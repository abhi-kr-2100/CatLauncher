use std::io;
use std::path::Path;

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UninstallThirdPartyModError {
  #[error("failed to remove installed mod from repository: {0}")]
  Repository(#[from] InstalledModsRepositoryError),
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
  #[error("failed to delete mod directory: {0}")]
  DeleteModDirectory(#[from] io::Error),
}

pub async fn uninstall_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  data_dir: &Path,
  repository: &impl InstalledModsRepository,
) -> Result<(), UninstallThirdPartyModError> {
  // Remove from repository
  repository
    .delete_installed_mod(mod_id, game_variant)
    .await?;

  // Delete mod directory
  let user_game_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;
  let mod_dir = user_game_data_dir.join("mods").join(mod_id);
  tokio::fs::remove_dir_all(&mod_dir).await?;

  Ok(())
}
