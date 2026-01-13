use std::path::Path;

use crate::settings::repository::settings_repository::{
  SaveSettingsError, SettingsRepository,
};
use crate::settings::update_color_files::{
  update_color_files, UpdateColorFilesError,
};
use crate::settings::update_font_files::{
  update_font_files, UpdateFontFilesError,
};
use crate::settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum UpdateSettingsError {
  #[error("failed to update font files: {0}")]
  UpdateFontFiles(#[from] UpdateFontFilesError),

  #[error("failed to update color files: {0}")]
  UpdateColorFiles(#[from] UpdateColorFilesError),

  #[error("failed to update settings in repository: {0}")]
  Repository(#[from] SaveSettingsError),
}

pub async fn update_settings(
  data_dir: &Path,
  settings: &Settings,
  repository: &impl SettingsRepository,
) -> Result<(), UpdateSettingsError> {
  update_font_files(data_dir, settings).await?;
  update_color_files(data_dir, settings).await?;
  repository.save_settings(settings).await?;
  Ok(())
}
