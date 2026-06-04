use std::env::consts::OS;

use tauri::{AppHandle, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::settings::colors::{
  get_available_color_themes, GetColorThemesError,
};
use crate::settings::fonts::get_all_fonts;
use crate::settings::repository::settings_repository::{
  GetSettingsError, SettingsRepository,
};
use crate::settings::repository::sqlite_settings_repository::SqliteSettingsRepository;
use crate::settings::types::{ColorTheme, Font};
use crate::settings::update_settings::{self, UpdateSettingsError};
use crate::settings::Settings;

/// Errors that can occur when retrieving available fonts.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetFontsError {
  /// The current operating system is not supported.
  #[error("failed to get fonts: {0}")]
  OS(#[from] OSNotSupportedError),
}

/// Retrieves a list of all monospaced fonts available on the system.
#[command]
pub async fn get_fonts() -> Result<Vec<Font>, GetFontsError> {
  let os_str = std::env::consts::OS;
  let os = get_os_enum(os_str)?;
  Ok(get_all_fonts(os).await)
}

/// Errors that can occur when retrieving color themes.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetColorThemesCommandError {
  /// An error occurred while retrieving color themes from the filesystem.
  #[error("failed to get color themes: {0}")]
  Get(#[from] GetColorThemesError),

  /// An error occurred while determining the app local data directory.
  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),

  /// The current operating system is not supported.
  #[error("failed to get os: {0}")]
  OS(#[from] OSNotSupportedError),
}

/// Retrieves all available color themes, including bundled and game-specific ones.
#[command]
pub async fn get_color_themes(
  app_handle: AppHandle,
  active_release_repo: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<ColorTheme>, GetColorThemesCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resource_dir = app_handle.path().resource_dir()?;
  let os = get_os_enum(OS)?;

  let themes = get_available_color_themes(
    &data_dir,
    &resource_dir,
    &active_release_repo,
    &os,
  )
  .await?;
  Ok(themes)
}

/// Errors that can occur when retrieving application settings.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetSettingsCommandError {
  /// An error occurred while retrieving settings from the repository.
  #[error("failed to get settings: {0}")]
  Get(#[from] GetSettingsError),
}

/// Retrieves the current application settings.
#[command]
pub async fn get_settings(
  repository: State<'_, SqliteSettingsRepository>,
) -> Result<Settings, GetSettingsCommandError> {
  let settings = repository.get_settings().await?;
  Ok(settings)
}

/// Errors that can occur when updating application settings.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum UpdateSettingsCommandError {
  /// An error occurred while updating settings.
  #[error("failed to update settings: {0}")]
  Update(#[from] UpdateSettingsError),

  /// An error occurred while determining the app local data directory.
  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),
}

/// Updates the application settings and synchronizes them with the game configurations.
#[command]
pub async fn update_settings(
  app_handle: AppHandle,
  settings: Settings,
  repository: State<'_, SqliteSettingsRepository>,
) -> Result<(), UpdateSettingsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;

  update_settings::update_settings(
    &data_dir,
    &settings,
    &*repository,
  )
  .await?;
  Ok(())
}

/// Returns the default application settings.
#[command]
pub fn get_default_settings() -> Settings {
  Settings::default()
}
