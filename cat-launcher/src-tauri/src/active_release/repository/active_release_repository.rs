use std::error::Error;

use async_trait::async_trait;

use crate::variants::game_variant::GameVariant;

/// Errors that can occur when interacting with the active release repository.
#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseRepositoryError {
  /// An error occurred while retrieving the active release version.
  #[error("failed to get active release: {0}")]
  Get(Box<dyn Error + Send + Sync>),

  /// An error occurred while setting the active release version.
  #[error("failed to set active release: {0}")]
  Set(Box<dyn Error + Send + Sync>),
}

/// A repository for managing the version of the active release for each game variant.
#[async_trait]
pub trait ActiveReleaseRepository: Send + Sync {
  /// Retrieves the version of the currently active release for the given game variant.
  async fn get_active_release(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Option<String>, ActiveReleaseRepositoryError>;

  /// Sets the version of the active release for the given game variant.
  async fn set_active_release(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<(), ActiveReleaseRepositoryError>;
}
