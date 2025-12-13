use std::env::consts::OS;

use strum::IntoStaticStr;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::soundpacks::get_third_party_soundpack_installation_status::{
    get_third_party_soundpack_installation_status, GetThirdPartySoundpackInstallationStatusError,
};
use crate::soundpacks::install_third_party_soundpack::{
    install_third_party_soundpack, InstallThirdPartySoundpackError,
};
use crate::soundpacks::list_all_soundpacks::{list_all_soundpacks, ListAllSoundpacksError};
use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
use crate::soundpacks::types::{Soundpack, SoundpackInstallationStatus};
use crate::soundpacks::uninstall_third_party_soundpack::{
    uninstall_third_party_soundpack, UninstallThirdPartySoundpackError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListAllSoundpacksCommandError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to list soundpacks: {0}")]
  ListSoundpacks(#[from] ListAllSoundpacksError),
}

#[tauri::command]
pub async fn list_all_soundpacks_command(
  variant: GameVariant,
  app: tauri::AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<Soundpack>, ListAllSoundpacksCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let soundpacks = list_all_soundpacks(
    &variant,
    &data_dir,
    &resource_dir,
    &os,
    active_release_repository.inner(),
  )
  .await?;

  Ok(soundpacks)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartySoundpackCommandError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to install soundpack: {0}")]
  Install(#[from] InstallThirdPartySoundpackError),
}

#[tauri::command]
pub async fn install_third_party_soundpack_command(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  repository: State<'_, SqliteInstalledSoundpacksRepository>,
) -> Result<(), InstallThirdPartySoundpackCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  install_third_party_soundpack(
    &id,
    &variant,
    &data_dir,
    &resource_dir,
    &temp_dir,
    &os,
    downloader.inner(),
    repository.inner(),
  )
  .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartySoundpackCommandError {
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to uninstall soundpack: {0}")]
  Uninstall(#[from] UninstallThirdPartySoundpackError),
}

#[tauri::command]
pub async fn uninstall_third_party_soundpack_command(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  repository: State<'_, SqliteInstalledSoundpacksRepository>,
) -> Result<(), UninstallThirdPartySoundpackCommandError> {
  let data_dir = app.path().app_local_data_dir()?;

  uninstall_third_party_soundpack(
    &id,
    &variant,
    &data_dir,
    repository.inner(),
  )
  .await?;
  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetThirdPartySoundpackInstallationStatusCommandError {
  #[error("failed to get soundpack installation status: {0}")]
  GetStatus(#[from] GetThirdPartySoundpackInstallationStatusError),
}

#[tauri::command]
pub async fn get_third_party_soundpack_installation_status_command(
  id: String,
  variant: GameVariant,
  repository: State<'_, SqliteInstalledSoundpacksRepository>,
) -> Result<
  SoundpackInstallationStatus,
  GetThirdPartySoundpackInstallationStatusCommandError,
> {
  let status = get_third_party_soundpack_installation_status(
    &id,
    &variant,
    repository.inner(),
  )
  .await?;
  Ok(status)
}
