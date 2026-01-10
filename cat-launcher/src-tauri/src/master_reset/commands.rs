use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::master_reset::reset::{
  master_reset as master_reset_fn, MasterResetError,
};
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::settings::repository::sqlite_settings_repository::SqliteSettingsRepository;
use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum MasterResetCommandError {
  #[error("failed to reset: {0}")]
  Reset(#[from] MasterResetError),

  #[error("failed to get system directory: {0}")]
  SystemDir(#[from] tauri::Error),
}

#[command]
pub async fn master_reset(
  variant: GameVariant,
  app_handle: AppHandle,
  installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
  installed_soundpacks_repository: State<
    '_,
    SqliteInstalledSoundpacksRepository,
  >,
  installed_tilesets_repository: State<
    '_,
    SqliteInstalledTilesetsRepository,
  >,
  settings_repository: State<'_, SqliteSettingsRepository>,
) -> Result<(), MasterResetCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;

  master_reset_fn(
    &variant,
    &data_dir,
    installed_mods_repository.inner(),
    installed_soundpacks_repository.inner(),
    installed_tilesets_repository.inner(),
    settings_repository.inner(),
  )
  .await?;

  Ok(())
}
