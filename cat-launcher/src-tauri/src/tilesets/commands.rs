use std::env::consts::OS;

use strum::IntoStaticStr;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::download::Downloader;
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

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum ListAllTilesetsCommandError {
    #[error("failed to get app data directory")]
    AppDataDir(#[from] tauri::Error),

    #[error("failed to get OS information")]
    OSInfo(#[from] OSNotSupportedError),

    #[error("failed to list tilesets: {0}")]
    ListTilesets(#[from] ListAllTilesetsError),
}

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

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum InstallThirdPartyTilesetCommandError {
    #[error("failed to get app data directory")]
    AppDataDir(#[from] tauri::Error),

    #[error("failed to get OS information")]
    OSInfo(#[from] OSNotSupportedError),

    #[error("failed to install tileset: {0}")]
    Install(#[from] InstallThirdPartyTilesetError),
}

#[tauri::command]
pub async fn install_third_party_tileset_command(
    id: String,
    variant: GameVariant,
    app: tauri::AppHandle,
    downloader: State<'_, Downloader>,
    repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), InstallThirdPartyTilesetCommandError> {
    let data_dir = app.path().app_local_data_dir()?;
    let resource_dir = app.path().resource_dir()?;
    let temp_dir = app.path().app_cache_dir()?;

    let os = get_os_enum(OS)?;

    install_third_party_tileset(
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

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum UninstallThirdPartyTilesetCommandError {
    #[error("failed to get app data directory: {0}")]
    AppDataDir(#[from] tauri::Error),

    #[error("failed to uninstall tileset: {0}")]
    Uninstall(#[from] UninstallThirdPartyTilesetError),
}

#[tauri::command]
pub async fn uninstall_third_party_tileset_command(
    id: String,
    variant: GameVariant,
    app: tauri::AppHandle,
    repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<(), UninstallThirdPartyTilesetCommandError> {
    let data_dir = app.path().app_local_data_dir()?;

    uninstall_third_party_tileset(&id, &variant, &data_dir, repository.inner()).await?;
    Ok(())
}

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum GetThirdPartyTilesetInstallationStatusCommandError {
    #[error("failed to get tileset installation status: {0}")]
    GetStatus(#[from] GetThirdPartyTilesetInstallationStatusError),
}

#[tauri::command]
pub async fn get_third_party_tileset_installation_status_command(
    id: String,
    variant: GameVariant,
    repository: State<'_, SqliteInstalledTilesetsRepository>,
) -> Result<TilesetInstallationStatus, GetThirdPartyTilesetInstallationStatusCommandError> {
    let status =
        get_third_party_tileset_installation_status(&id, &variant, repository.inner()).await?;
    Ok(status)
}
