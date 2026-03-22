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
    get_third_party_tileset_installation_status as get_third_party_tileset_installation_status_impl,
    GetThirdPartyTilesetInstallationStatusError as GetThirdPartyTilesetInstallationStatusBusinessError,
};
use crate::tilesets::install_third_party_tileset::{
    install_third_party_tileset as install_third_party_tileset_impl,
    InstallThirdPartyTilesetError as InstallThirdPartyTilesetBusinessError,
};
use crate::tilesets::list_all_tilesets::{
    list_all_tilesets as list_all_tilesets_impl,
    ListAllTilesetsError as ListAllTilesetsBusinessError,
};
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
use crate::tilesets::types::{Tileset, TilesetInstallationStatus};
use crate::tilesets::uninstall_third_party_tileset::{
    uninstall_third_party_tileset as uninstall_third_party_tileset_impl,
    UninstallThirdPartyTilesetError as UninstallThirdPartyTilesetBusinessError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListAllTilesetsError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to list tilesets: {0}")]
  ListTilesets(#[from] ListAllTilesetsBusinessError),
}

#[tauri::command]
pub async fn list_all_tilesets(
  variant: GameVariant,
  app: tauri::AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<Tileset>, ListAllTilesetsError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let tilesets = list_all_tilesets_impl(
    &variant,
    &data_dir,
    &resource_dir,
    &os,
    active_release_repository.inner(),
  )
  .await?;

  Ok(tilesets)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartyTilesetError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to install tileset: {0}")]
  Install(#[from] InstallThirdPartyTilesetBusinessError),
}

#[tauri::command]
pub async fn install_third_party_tileset(
  id: String,
  variant: GameVariant,
  channel: Channel,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), InstallThirdPartyTilesetError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  let reporter = Arc::new(ChannelReporter::new(channel));

  install_third_party_tileset_impl(
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

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartyTilesetError {
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to uninstall tileset: {0}")]
  Uninstall(#[from] UninstallThirdPartyTilesetBusinessError),
}

#[tauri::command]
pub async fn uninstall_third_party_tileset(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), UninstallThirdPartyTilesetError> {
  let data_dir = app.path().app_local_data_dir()?;

  uninstall_third_party_tileset_impl(
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
pub enum GetThirdPartyTilesetInstallationStatusError {
  #[error("failed to get tileset installation status: {0}")]
  GetStatus(
    #[from] GetThirdPartyTilesetInstallationStatusBusinessError,
  ),
}

#[tauri::command]
pub async fn get_third_party_tileset_installation_status(
  id: String,
  variant: GameVariant,
  repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<
  TilesetInstallationStatus,
  GetThirdPartyTilesetInstallationStatusError,
> {
  let status = get_third_party_tileset_installation_status_impl(
    &id,
    &variant,
    repository.inner(),
  )
  .await?;
  Ok(status)
}
