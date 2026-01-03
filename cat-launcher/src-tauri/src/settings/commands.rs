use strum::IntoStaticStr;
use tauri::State;

use cat_macros::CommandErrorSerialize;

use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::settings::fonts::{get_available_fonts, Font, FontError};
use crate::settings::repository::sqlite_settings_repository::SqliteSettingsRepository;
use crate::settings::settings::{
  get_settings as get_settings_logic,
  update_settings as update_settings_logic, GetSettingsError,
  UpdateSettingsError,
};
use crate::settings::Settings;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetSettingsCommandError {
  #[error("failed to get settings: {0}")]
  GetSettings(#[from] GetSettingsError),
}

#[tauri::command]
pub async fn get_settings(
  repo: State<'_, SqliteSettingsRepository>,
) -> Result<Settings, GetSettingsCommandError> {
  let settings = get_settings_logic(repo.inner()).await?;
  Ok(settings)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UpdateSettingsCommandError {
  #[error("failed to update settings: {0}")]
  UpdateSettings(#[from] UpdateSettingsError),
}

#[tauri::command]
pub async fn update_settings(
  settings: Settings,
  repo: State<'_, SqliteSettingsRepository>,
) -> Result<(), UpdateSettingsCommandError> {
  update_settings_logic(&settings, repo.inner()).await?;
  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetFontsCommandError {
  #[error("failed to get fonts: {0}")]
  GetFonts(#[from] FontError),

  #[error("failed to get OS: {0}")]
  OS(#[from] OSNotSupportedError),
}

#[tauri::command]
pub async fn get_fonts() -> Result<Vec<Font>, GetFontsCommandError> {
  let os = get_os_enum(std::env::consts::OS)?;
  let fonts = get_available_fonts(&os)?;
  Ok(fonts)
}
