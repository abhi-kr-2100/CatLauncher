use std::error::Error;

use async_trait::async_trait;

use crate::settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum GetSettingsError {
  #[error("failed to get settings: {0}")]
  Get(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(thiserror::Error, Debug)]
pub enum SaveSettingsError {
  #[error("failed to save settings: {0}")]
  Save(#[source] Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
  async fn get_settings(&self) -> Result<Settings, GetSettingsError>;

  async fn save_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SaveSettingsError>;
}
