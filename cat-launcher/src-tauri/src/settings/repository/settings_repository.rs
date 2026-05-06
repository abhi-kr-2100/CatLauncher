use std::error::Error;

use async_trait::async_trait;

use crate::settings::Settings;

/// Errors that can occur when retrieving settings from the repository.
#[derive(thiserror::Error, Debug)]
pub enum GetSettingsError {
  /// An error occurred while retrieving settings.
  #[error("failed to get settings: {0}")]
  Get(#[source] Box<dyn Error + Send + Sync>),
}

/// Errors that can occur when saving settings to the repository.
#[derive(thiserror::Error, Debug)]
pub enum SaveSettingsError {
  /// An error occurred while saving settings.
  #[error("failed to save settings: {0}")]
  Save(#[source] Box<dyn Error + Send + Sync>),
}

/// A repository for managing application settings.
#[async_trait]
pub trait SettingsRepository: Send + Sync {
  /// Retrieves the current application settings.
  async fn get_settings(&self) -> Result<Settings, GetSettingsError>;

  /// Saves the provided application settings.
  async fn save_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SaveSettingsError>;
}
