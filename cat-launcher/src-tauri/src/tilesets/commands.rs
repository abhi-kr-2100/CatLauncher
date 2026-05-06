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
use crate::tilesets::get_third_party_tileset_installation_status::{
    get_third_party_tileset_installation_status, GetThirdPartyTilesetInstallationStatusError,
};
use crate::tilesets::install_third_party_tileset::{
    install_third_party_tileset, InstallThirdPartyTilesetError,
};
use crate::tilesets::list_all_tilesets::{list_all_tilesets, ListAllTilesetsError};
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
use crate::tilesets::types::{Tileset, TilesetInstallationStatus};
use crate::tilesets::uninstall_third_party_tileset::{
    uninstall_third_party_tileset, UninstallThirdPartyTilesetError,
};
use crate::variants::GameVariant;

/// Errors that can occur when listing all tilesets via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListAllTilesetsCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while retrieving OS information.
  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  /// An error occurred while listing the tilesets.
  #[error("failed to list tilesets: {0}")]
  ListTilesets(#[from] ListAllTilesetsError),
}

/// Lists all available tilesets (both stock and third-party) for a game variant.
#[tauri::command]
pub async fn list_all_tilesets_command(
  variant: GameVariant,
  app: tauri::AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<Tileset>, ListAllTilesetsCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let tilesets = list_all_tilesets(
    &variant,
    &data_dir,
    &resource_dir,
    &os,
    active_release_repository.inner(),
  )
  .await?;

  Ok(tilesets)
}

/// Errors that can occur when installing a third-party tileset via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartyTilesetCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while retrieving OS information.
  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  /// An error occurred while installing the tileset.
  #[error("failed to install tileset: {0}")]
  Install(#[from] InstallThirdPartyTilesetError),
}

/// Installs a third-party tileset for a game variant.
#[tauri::command]
pub async fn install_third_party_tileset_command(
  id: String,
  variant: GameVariant,
  channel: Channel,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), InstallThirdPartyTilesetCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  let reporter = Arc::new(ChannelReporter::new(channel));

  install_third_party_tileset(
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

/// Errors that can occur when uninstalling a third-party tileset via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartyTilesetCommandError {
  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  /// An error occurred while uninstalling the tileset.
  #[error("failed to uninstall tileset: {0}")]
  Uninstall(#[from] UninstallThirdPartyTilesetError),
}

/// Uninstalls a previously installed third-party tileset.
#[tauri::command]
pub async fn uninstall_third_party_tileset_command(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), UninstallThirdPartyTilesetCommandError> {
  let data_dir = app.path().app_local_data_dir()?;

  uninstall_third_party_tileset(
    &id,
    &variant,
    &data_dir,
    repository.inner(),
  )
  .await?;
  Ok(())
}

/// Errors that can occur when retrieving the installation status of a third-party tileset via a command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetThirdPartyTilesetInstallationStatusCommandError {
  /// An error occurred while getting the installation status.
  #[error("failed to get tileset installation status: {0}")]
  GetStatus(#[from] GetThirdPartyTilesetInstallationStatusError),
}

/// Retrieves the current installation status of a third-party tileset.
#[tauri::command]
pub async fn get_third_party_tileset_installation_status_command(
  id: String,
  variant: GameVariant,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<
  TilesetInstallationStatus,
  GetThirdPartyTilesetInstallationStatusCommandError,
> {
  let status = get_third_party_tileset_installation_status(
    &id,
    &variant,
    repository.inner(),
  )
  .await?;
  Ok(status)
}
