use async_trait::async_trait;

use crate::settings::fonts::GetFontFromPathError;
use crate::settings::Settings;

#[derive(Debug, thiserror::Error)]
pub enum SettingsRepositoryError {
  #[error("failed to get settings: {0}")]
  Get(#[source] Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to update settings: {0}")]
  Update(#[source] Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to load font: {0}")]
  FontLoad(#[from] GetFontFromPathError),
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
  async fn get_settings(
    &self,
  ) -> Result<Settings, SettingsRepositoryError>;
  async fn update_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SettingsRepositoryError>;
}
