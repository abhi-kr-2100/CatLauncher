use async_trait::async_trait;

use crate::settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum SettingsRepositoryError {
  #[error("failed to get settings: {0}")]
  Get(#[source] Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to save settings: {0}")]
  Save(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
  async fn get_settings(
    &self,
  ) -> Result<Settings, SettingsRepositoryError>;

  async fn save_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SettingsRepositoryError>;
}
