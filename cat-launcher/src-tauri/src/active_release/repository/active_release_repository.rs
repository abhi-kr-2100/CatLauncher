use async_trait::async_trait;

use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseRepositoryError {
  #[error("failed to get active release: {0}")]
  Get(#[source] rusqlite::Error),

  #[error("failed to get active release from pool: {0}")]
  GetFromPool(#[source] r2d2::Error),

  #[error("failed to set active release: {0}")]
  Set(#[source] rusqlite::Error),

  #[error("failed to set active release from pool: {0}")]
  SetFromPool(#[source] r2d2::Error),

  #[error("failed to join task: {0}")]
  Join(#[from] tokio::task::JoinError),
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
