use std::env::consts::OS;
use std::sync::Arc;

use reqwest::Client;
use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
use crate::infra::installation_progress_monitor::channel_reporter::ChannelReporter;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::mods::get_last_activity_for_third_party_mod::{
  get_last_activity_for_third_party_mod as get_last_activity_for_third_party_mod_impl,
  GetLastActivityForThirdPartyModError as GetLastActivityForThirdPartyModBusinessError, LastModActivity,
};
use crate::mods::get_third_party_mod_installation_status::{
  get_third_party_mod_installation_status as get_third_party_mod_installation_status_impl,
  GetThirdPartyModInstallationStatusError as GetThirdPartyModInstallationStatusBusinessError,
};
use crate::mods::install_third_party_mod::{
  install_third_party_mod as install_third_party_mod_impl, InstallThirdPartyModError as InstallThirdPartyModBusinessError,
};
use crate::mods::lib::OnlineModRepositoryRegistry;
use crate::mods::list_all_mods::{list_all_mods as list_all_mods_impl, ListAllModsError as ListAllModsBusinessError};
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
use crate::mods::types::{
  ModInstallationStatus, ModsUpdatePayload,
};
use crate::mods::uninstall_third_party_mod::{
  uninstall_third_party_mod as uninstall_third_party_mod_impl, UninstallThirdPartyModError as UninstallThirdPartyModBusinessError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum TriggerFetchModsForVariantError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to list mods: {0}")]
  ListMods(#[from] ListAllModsBusinessError<tauri::Error>),
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn trigger_fetch_mods_for_variant(
  variant: GameVariant,
  app: tauri::AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  mods_repository: State<'_, SqliteModsRepository>,
  online_mod_repository_registry: State<
    '_,
    OnlineModRepositoryRegistry,
  >,
  client: State<'_, Client>,
) -> Result<(), TriggerFetchModsForVariantError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let on_update = move |payload: ModsUpdatePayload| {
    app.emit("mods-update", payload)?;
    Ok(())
  };

  list_all_mods_impl(
    &variant,
    &data_dir,
    &resource_dir,
    &os,
    active_release_repository.inner(),
    mods_repository.inner(),
    online_mod_repository_registry.repositories(),
    client.inner(),
    on_update,
  )
  .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallThirdPartyModError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to install mod: {0}")]
  Install(#[from] InstallThirdPartyModBusinessError),
}

#[tauri::command]
pub async fn install_third_party_mod(
  id: String,
  variant: GameVariant,
  channel: Channel,
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
  mods_repository: State<'_, SqliteModsRepository>,
) -> Result<(), InstallThirdPartyModError> {
  let data_dir = app.path().app_local_data_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  let reporter = Arc::new(ChannelReporter::new(channel));

  install_third_party_mod_impl(
    &id,
    &variant,
    &data_dir,
    &temp_dir,
    &os,
    downloader.inner(),
    installed_mods_repository.inner(),
    mods_repository.inner(),
    reporter,
  )
  .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UninstallThirdPartyModError {
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to uninstall mod: {0}")]
  Uninstall(#[from] UninstallThirdPartyModBusinessError),
}

#[tauri::command]
pub async fn uninstall_third_party_mod(
  id: String,
  variant: GameVariant,
  app: tauri::AppHandle,
  repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), UninstallThirdPartyModError> {
  let data_dir = app.path().app_local_data_dir()?;

  uninstall_third_party_mod_impl(
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
pub enum GetThirdPartyModInstallationStatusError {
  #[error("failed to get mod installation status: {0}")]
  GetStatus(#[from] GetThirdPartyModInstallationStatusBusinessError),
}

#[tauri::command]
pub async fn get_third_party_mod_installation_status(
  id: String,
  variant: GameVariant,
  repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<
  ModInstallationStatus,
  GetThirdPartyModInstallationStatusError,
> {
  let status = get_third_party_mod_installation_status_impl(
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
pub enum GetLastActivityOnThirdPartyModError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to get last activity: {0}")]
  GetActivity(#[from] GetLastActivityForThirdPartyModBusinessError),
}

#[tauri::command]
pub async fn get_last_activity_on_third_party_mod(
  id: String,
  variant: GameVariant,
  client: State<'_, Client>,
  mods_repository: State<'_, SqliteModsRepository>,
) -> Result<LastModActivity, GetLastActivityOnThirdPartyModError> {
  let last_activity = get_last_activity_for_third_party_mod_impl(
    &id,
    &variant,
    client.inner(),
    mods_repository.inner(),
  )
  .await?;

  Ok(last_activity)
}
