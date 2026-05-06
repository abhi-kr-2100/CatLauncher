use std::path::Path;

use crate::settings::Settings;
use crate::settings::repository::settings_repository::{
  SaveSettingsError, SettingsRepository,
};
use crate::settings::update_color_files::{
  UpdateColorFilesError, update_color_files,
};
use crate::settings::update_font_files::{
  UpdateFontFilesError, update_font_files,
};

/// Errors that can occur when updating settings.
#[derive(thiserror::Error, Debug)]
pub enum UpdateSettingsError {
  /// An error occurred while updating font files.
  #[error("failed to update font files: {0}")]
  UpdateFontFiles(#[from] UpdateFontFilesError),

  /// An error occurred while updating color files.
  #[error("failed to update color files: {0}")]
  UpdateColorFiles(#[from] UpdateColorFilesError),

  /// An error occurred while saving settings to the repository.
  #[error("failed to update settings in repository: {0}")]
  Repository(#[from] SaveSettingsError),
}

/// Updates the application settings and synchronizes them with the game's configuration files.
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
