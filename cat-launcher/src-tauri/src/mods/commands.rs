use std::env::consts::OS;

use strum::IntoStaticStr;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::mods::get_mod_activity::{get_mod_activity, GetModActivityError};
use crate::mods::get_third_party_mod_installation_status::{
    get_third_party_mod_installation_status, GetThirdPartyModInstallationStatusError,
};
use crate::mods::install_third_party_mod::{install_third_party_mod, InstallThirdPartyModError};
use crate::mods::list_all_mods::{list_all_mods, ListAllModsError};
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::types::{Mod, ModInstallationStatus};
use crate::mods::uninstall_third_party_mod::{
    uninstall_third_party_mod, UninstallThirdPartyModError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListAllModsCommandError {
  #[error("failed to get app data directory")]
  AppDataDir(#[from] tauri::Error),

  #[error("failed to get OS information")]
  OSInfo(#[from] OSNotSupportedError),

  #[error("failed to list mods: {0}")]
  ListMods(#[from] ListAllModsError),
}

#[tauri::command]
pub async fn list_all_mods_command(
  variant: GameVariant,
  app: tauri::AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<Mod>, ListAllModsCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let mods = list_all_mods(
    &variant,
    &data_dir,
    &resource_dir,
    &os,
    active_release_repository.inner(),
  )
  .await?;

  Ok(mods)
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
  app: tauri::AppHandle,
  downloader: State<'_, Downloader>,
  repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), InstallThirdPartyModCommandError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resource_dir = app.path().resource_dir()?;
  let temp_dir = app.path().app_cache_dir()?;

  let os = get_os_enum(OS)?;

  install_third_party_mod(
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
pub enum GetModActivityCommandError {
    #[error("failed to get app resource directory")]
    ResourceDir(#[from] tauri::Error),

    #[error("failed to get mod activity: {0}")]
    GetModActivity(#[from] GetModActivityError),
}

#[tauri::command]
pub async fn get_mod_activity_command(
    id: String,
    variant: GameVariant,
    app: tauri::AppHandle,
    http_client: State<'_, reqwest::Client>,
) -> Result<Option<String>, GetModActivityCommandError> {
    let resource_dir = app.path().resource_dir()?;

    let activity =
        get_mod_activity(&id, &variant, &resource_dir, http_client.inner()).await?;

    Ok(activity)
}
