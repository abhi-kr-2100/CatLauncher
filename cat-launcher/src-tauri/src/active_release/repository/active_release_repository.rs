use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseRepositoryError {
  #[error("failed to get active release: {0}")]
  Get(Box<dyn Error + Send + Sync>),

  #[error("failed to set active release: {0}")]
  Set(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait ActiveReleaseRepository: Send + Sync {
  async fn get_active_release(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Option<String>, ActiveReleaseRepositoryError>;

  async fn set_active_release(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<(), ActiveReleaseRepositoryError>;
}
