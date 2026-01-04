use tauri::{command, State};

use cat_macros::CommandErrorSerialize;

use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::settings::fonts::get_all_fonts;
use crate::settings::repository::settings_repository::{
  SettingsRepository, SettingsRepositoryError,
};
use crate::settings::repository::sqlite_settings_repository::SqliteSettingsRepository;
use crate::settings::types::Font;
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
pub enum SettingsCommandError {
  #[error("failed to access settings: {0}")]
  Repository(#[from] SettingsRepositoryError),
}

#[command]
pub async fn get_settings(
  repository: State<'_, SqliteSettingsRepository>,
) -> Result<Settings, SettingsCommandError> {
  let settings = repository.get_settings().await?;
  Ok(settings)
}

#[command]
pub async fn update_settings(
  settings: Settings,
  repository: State<'_, SqliteSettingsRepository>,
) -> Result<(), SettingsCommandError> {
  repository.save_settings(&settings).await?;
  Ok(())
}

#[command]
pub fn get_default_settings() -> Settings {
  Settings::default()
}
