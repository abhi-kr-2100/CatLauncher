use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::mods::get_mod_installation_status::{self, GetModInstallationStatusError, ModInstallationStatus};
use crate::mods::install_mod::{self, InstallModError};
use crate::mods::list_all_mods::{self, ListAllModsError};
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::uninstall_mod::{self, UninstallModError};
use crate::mods::Mod;
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize)]
pub enum ListAllModsCommandError {
    #[error("failed to list all mods: {0}")]
    List(#[from] ListAllModsError),

    #[error("failed to get data directory: {0}")]
    DataDirectoryError(#[from] tauri::Error),

    #[error("failed to get OS: {0}")]
    OsError(#[from] OSNotSupportedError),
}

#[command]
pub async fn list_all_mods(
    app: AppHandle,
    variant: GameVariant,
    active_release_repository: State<'_, SqliteActiveReleaseRepository>,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<Vec<Mod>, ListAllModsCommandError> {
    let data_dir = app.path().app_local_data_dir()?;
    let resource_dir = app.path().resource_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;

    let mods = list_all_mods::list_all_mods(
        &data_dir,
        &resource_dir,
        &variant,
        &os,
        &*active_release_repository,
        &*installed_mods_repository,
    )
    .await?;

    Ok(mods)
}

#[derive(thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize)]
pub enum InstallModCommandError {
    #[error("failed to install mod: {0}")]
    Install(#[from] InstallModError),

    #[error("failed to get data directory: {0}")]
    DataDirectoryError(#[from] tauri::Error),

    #[error("failed to get OS: {0}")]
    OsError(#[from] OSNotSupportedError),
}

#[command]
pub async fn install_mod(
    app: AppHandle,
    variant: GameVariant,
    mod_id: String,
    settings: State<'_, Settings>,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), InstallModCommandError> {
    let data_dir = app.path().app_local_data_dir()?;
    let resource_dir = app.path().resource_dir()?;
    let temp_dir = app.path().temp_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;

    install_mod::install_third_party_mod(
        &variant,
        &mod_id,
        &data_dir,
        &resource_dir,
        &temp_dir,
        &os,
        &settings,
        &*installed_mods_repository,
    )
    .await?;

    Ok(())
}

#[derive(thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize)]
pub enum UninstallModCommandError {
    #[error("failed to uninstall mod: {0}")]
    Uninstall(#[from] UninstallModError),

    #[error("failed to get data directory: {0}")]
    DataDirectoryError(#[from] tauri::Error),
}

#[command]
pub async fn uninstall_mod_for_variant(
    app: AppHandle,
    variant: GameVariant,
    mod_id: String,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), UninstallModCommandError> {
    let data_dir = app.path().app_local_data_dir()?;

    uninstall_mod::uninstall_mod_for_variant(
        &variant,
        &mod_id,
        &data_dir,
        &*installed_mods_repository,
    )
    .await?;

    Ok(())
}

#[derive(thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize)]
pub enum GetModInstallationStatusCommandError {
    #[error("failed to get mod installation status: {0}")]
    Status(#[from] GetModInstallationStatusError),

    #[error("failed to get data directory: {0}")]
    DataDirectoryError(#[from] tauri::Error),
}

#[command]
pub async fn get_mod_installation_status(
    app: AppHandle,
    variant: GameVariant,
    mod_id: String,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<ModInstallationStatus, GetModInstallationStatusCommandError> {
    let data_dir = app.path().app_local_data_dir()?;

    let status = get_mod_installation_status::get_mod_installation_status(
        &variant,
        &mod_id,
        &data_dir,
        &*installed_mods_repository,
    )
    .await?;

    Ok(status)
}
