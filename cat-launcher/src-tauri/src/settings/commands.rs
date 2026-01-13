use std::env::consts::OS;

use tauri::{command, AppHandle, Manager, State};

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

#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetFontsError {
  #[error("failed to get fonts: {0}")]
  OS(#[from] OSNotSupportedError),
}

#[command]
pub async fn get_fonts() -> Result<Vec<Font>, GetFontsError> {
  let os_str = std::env::consts::OS;
  let os = get_os_enum(os_str)?;
  Ok(get_all_fonts(os).await)
}

#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetColorThemesCommandError {
  #[error("failed to get color themes: {0}")]
  Get(#[from] GetColorThemesError),

  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),

  #[error("failed to get os: {0}")]
  OS(#[from] OSNotSupportedError),
}

#[command]
pub async fn get_color_themes(
  app_handle: AppHandle,
  active_release_repo: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<ColorTheme>, GetColorThemesCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(OS)?;

  let themes =
    get_available_color_themes(&data_dir, &active_release_repo, &os)
      .await?;
  Ok(themes)
}

#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetSettingsCommandError {
  #[error("failed to get settings: {0}")]
  Get(#[from] GetSettingsError),
}

#[command]
pub async fn get_settings(
  repository: State<'_, SqliteSettingsRepository>,
) -> Result<Settings, GetSettingsCommandError> {
  let settings = repository.get_settings().await?;
  Ok(settings)
}

#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum UpdateSettingsCommandError {
  #[error("failed to update settings: {0}")]
  Update(#[from] UpdateSettingsError),

  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),
}

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

#[command]
pub fn get_default_settings() -> Settings {
  Settings::default()
}
