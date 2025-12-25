use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::settings::apply_settings::{
  apply_settings, ApplySettingsError,
};
use crate::settings::list_fonts::{list_fonts, ListFontsError};
use crate::settings::list_themes::{ListThemesError, list_themes };
use crate::settings::settings::SettingsError;
use crate::settings::{
  ColorTheme, Font, Settings, SettingsData, ThemeColors,
};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetSettingsCommandError {
  #[error("failed to get settings: {0}")]
  Settings(#[from] SettingsError),
}

#[command]
pub async fn get_settings_command(
  settings: State<'_, Settings>,
) -> Result<SettingsData, GetSettingsCommandError> {
  Ok(settings.get_data().await?)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListFontsCommandError {
  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] tauri::Error),

  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),

  #[error("failed to list fonts: {0}")]
  ListFonts(#[from] ListFontsError),
}

#[command]
pub async fn list_fonts_command(
  handle: AppHandle,
) -> Result<Vec<Font>, ListFontsCommandError> {
  let data_dir = handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;
  Ok(list_fonts(&data_dir, &os).await?)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListThemesCommandError {
  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] tauri::Error),

  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),

  #[error("failed to list themes: {0}")]
  ListThemes(#[from] ListThemesError),
}

#[command]
pub async fn list_themes_command(
  handle: AppHandle,
  active_release_repo: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Vec<ColorTheme>, ListThemesCommandError> {
  let data_dir = handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;
  Ok(list_themes(data_dir, active_release_repo.inner(), &os).await?)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ApplySettingsCommandError {
  #[error("failed to apply settings: {0}")]
  ApplySettings(#[from] ApplySettingsError),
}

#[command]
pub async fn apply_settings_command(
  max_backups: usize,
  parallel_requests: u16,
  font_location: Option<String>,
  theme_colors: Option<ThemeColors>,
  settings: State<'_, Settings>,
) -> Result<(), ApplySettingsCommandError> {
  Ok(
    apply_settings(
      max_backups,
      parallel_requests,
      font_location,
      theme_colors,
      settings.inner(),
    )
    .await?,
  )
}
