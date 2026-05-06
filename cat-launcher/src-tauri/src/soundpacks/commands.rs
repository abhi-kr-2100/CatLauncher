use std::env::consts::OS;
use std::sync::Arc;

use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
use crate::infra::installation_progress_monitor::channel_reporter::ChannelReporter;
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

/// Errors that can occur when listing all soundpacks via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListAllSoundpacksCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while retrieving OS information.
  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  /// An error occurred while listing the soundpacks.
  #[error("failed to list soundpacks: {0}")]
  ListSoundpacks(#[from] ListAllSoundpacksError),
}

/// Lists all available soundpacks (both stock and third-party) for a game variant.
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

/// Errors that can occur when installing a third-party soundpack via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartySoundpackCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while retrieving OS information.
  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  /// An error occurred while installing the soundpack.
  #[error("failed to install soundpack: {0}")]
  Install(#[from] InstallThirdPartySoundpackError),
}

/// Installs a third-party soundpack for a game variant.
#[tauri::command]
pub async fn install_third_party_soundpack_command(
  id: String,
  variant: GameVariant,
  channel: Channel,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  repository: State<'_, SqliteInstalledSoundpacksRepository>,
) -> Result<(), InstallThirdPartySoundpackCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  let reporter = Arc::new(ChannelReporter::new(channel));

  install_third_party_soundpack(
    &id,
    &variant,
    &data_dir,
    &resource_dir,
    &temp_dir,
    &os,
    downloader.inner(),
    repository.inner(),
    reporter,
  )
  .await?;

  Ok(())
}

/// Errors that can occur when uninstalling a third-party soundpack via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartySoundpackCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while uninstalling the soundpack.
  #[error("failed to uninstall soundpack: {0}")]
  Uninstall(#[from] UninstallThirdPartySoundpackError),
}

/// Uninstalls a previously installed third-party soundpack.
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

/// Errors that can occur when retrieving the installation status of a third-party soundpack via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetThirdPartySoundpackInstallationStatusCommandError {
  /// An error occurred while getting the installation status.
  #[error("failed to get soundpack installation status: {0}")]
  GetStatus(#[from] GetThirdPartySoundpackInstallationStatusError),
}

/// Retrieves the current installation status of a third-party soundpack.
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
