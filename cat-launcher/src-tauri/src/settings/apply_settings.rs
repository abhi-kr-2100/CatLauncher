use std::io;

use strum::IntoStaticStr;

use crate::settings::settings::{
  Settings, SettingsError, SettingsUpdateError,
};
use crate::settings::ThemeColors;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ApplySettingsError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),

  #[error("settings update failed: {0}")]
  SettingsUpdate(#[from] SettingsUpdateError),

  #[error("settings error: {0}")]
  Settings(#[from] SettingsError),
}

pub async fn apply_settings(
  max_backups: usize,
  parallel_requests: u16,
  font_location: Option<String>,
  theme_colors: Option<ThemeColors>,
  settings: &Settings,
) -> Result<(), ApplySettingsError> {
  settings.update_max_backups(max_backups).await?;

  settings.update_parallel_requests(parallel_requests).await?;

  if let Some(loc) = font_location {
    settings.update_font(Some(loc)).await?;
  }

  if let Some(colors) = theme_colors {
    settings.update_theme(Some(colors)).await?;
  }

  Ok(())
}
