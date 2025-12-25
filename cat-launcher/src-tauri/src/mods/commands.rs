use std::env::consts::OS;
use std::sync::Arc;

use reqwest::Client;
use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
use crate::infra::installation_progress_monitor::channel_reporter::ChannelReporter;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::mods::fetch_mods::{FetchModsError, ModsUpdatePayload};
use crate::mods::get_last_activity_for_third_party_mod::{
  get_last_activity_for_third_party_mod, GetLastActivityError,
  LastModActivity,
};
use crate::mods::get_third_party_mod_installation_status::{
  get_third_party_mod_installation_status,
  GetThirdPartyModInstallationStatusError,
};
use crate::mods::install_third_party_mod::{
  install_third_party_mod, InstallThirdPartyModError,
};
use crate::mods::repository::sqlite_cached_mods_repository::SqliteCachedModsRepository;
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::types::ModInstallationStatus;
use crate::mods::uninstall_third_party_mod::{
  uninstall_third_party_mod, UninstallThirdPartyModError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum FetchModsCommandError {
  #[error("failed to get system directory: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to fetch mods: {0}")]
  Fetch(#[from] FetchModsError<tauri::Error>),
}

#[command]
pub async fn fetch_mods_for_variant(
  app_handle: AppHandle,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  cached_mods_repository: State<'_, SqliteCachedModsRepository>,
  client: State<'_, Client>,
) -> Result<(), FetchModsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resources_dir = app_handle.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let on_mods = move |payload: ModsUpdatePayload| {
    app_handle.emit("mods-update", payload)?;
    Ok(())
  };

  variant
    .fetch_mods(
      &client,
      &data_dir,
      &resources_dir,
      &os,
      active_release_repository.inner(),
      cached_mods_repository.inner(),
      on_mods,
    )
    .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartyModCommandError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to install mod: {0}")]
  Install(#[from] InstallThirdPartyModError),
}

#[tauri::command]
pub async fn install_third_party_mod_command(
  id: String,
  variant: GameVariant,
  channel: Channel,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
  cached_mods_repository: State<'_, SqliteCachedModsRepository>,
) -> Result<(), InstallThirdPartyModCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  let reporter = Arc::new(ChannelReporter::new(channel));

  install_third_party_mod(
    &id,
    &variant,
    &data_dir,
    &temp_dir,
    &os,
    downloader.inner(),
    installed_mods_repository.inner(),
    cached_mods_repository.inner(),
    reporter,
  )
  .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartyModCommandError {
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to uninstall mod: {0}")]
  Uninstall(#[from] UninstallThirdPartyModError),
}

#[tauri::command]
pub async fn uninstall_third_party_mod_command(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), UninstallThirdPartyModCommandError> {
  let data_dir = app.path().app_local_data_dir()?;

  uninstall_third_party_mod(
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
pub enum GetThirdPartyModInstallationStatusCommandError {
  #[error("failed to get mod installation status: {0}")]
  GetStatus(#[from] GetThirdPartyModInstallationStatusError),
}

#[tauri::command]
pub async fn get_third_party_mod_installation_status_command(
  id: String,
  variant: GameVariant,
  repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<
  ModInstallationStatus,
  GetThirdPartyModInstallationStatusCommandError,
> {
  let status = get_third_party_mod_installation_status(
    &id,
    &variant,
    repository.inner(),
  )
  .await?;
  Ok(status)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetLastActivityCommandError {
  #[error("failed to get last activity: {0}")]
  GetActivity(#[from] GetLastActivityError),
}

#[tauri::command]
pub async fn get_last_activity_on_third_party_mod_command(
  id: String,
  variant: GameVariant,
  client: State<'_, Client>,
  cached_mods_repository: State<'_, SqliteCachedModsRepository>,
) -> Result<LastModActivity, GetLastActivityCommandError> {
  let last_activity = get_last_activity_for_third_party_mod(
    &id,
    &variant,
    client.inner(),
    cached_mods_repository.inner(),
  )
  .await?;

  Ok(last_activity)
}
