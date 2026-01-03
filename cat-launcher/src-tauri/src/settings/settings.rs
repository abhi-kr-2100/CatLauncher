use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::constants::{
  DEFAULT_MAX_BACKUPS, DEFAULT_PARALLEL_REQUESTS,
};
use crate::settings::fonts::Font;
use crate::settings::repository::settings_repository::SettingsRepository;
use crate::settings::repository::settings_repository::SettingsRepositoryError;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Settings {
  pub max_backups: u16,
  pub parallel_requests: u16,
  pub font: Option<Font>,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      max_backups: DEFAULT_MAX_BACKUPS,
      parallel_requests: DEFAULT_PARALLEL_REQUESTS,
      font: None,
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSettingsError {
  #[error("failed to get settings: {0}")]
  Repository(#[from] SettingsRepositoryError),
}

pub async fn get_settings(
  repo: &dyn SettingsRepository,
) -> Result<Settings, GetSettingsError> {
  let settings = repo.get_settings().await?;
  Ok(settings)
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateSettingsError {
  #[error("failed to update settings: {0}")]
  Repository(#[from] SettingsRepositoryError),
}

pub async fn update_settings(
  settings: &Settings,
  repo: &dyn SettingsRepository,
) -> Result<(), UpdateSettingsError> {
  repo.update_settings(settings).await?;
  Ok(())
}
