use async_trait::async_trait;

use crate::variants::GameVariant;

/// Errors that can occur when retrieving the game variant order.
#[derive(thiserror::Error, Debug)]
pub enum GetGameVariantOrderError {
  /// An underlying error occurred during retrieval.
  #[error("failed to get game variant order: {0}")]
  Get(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Errors that can occur when updating the game variant order.
#[derive(thiserror::Error, Debug)]
pub enum UpdateGameVariantOrderError {
  /// An underlying error occurred during the update.
  #[error("failed to update game variant order: {0}")]
  Update(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// A unified error type for game variant order repository operations.
#[derive(thiserror::Error, Debug)]
pub enum GameVariantOrderRepositoryError {
  /// Failed to get the order.
  #[error("failed to get game variant order: {0}")]
  Get(#[from] GetGameVariantOrderError),

  /// Failed to update the order.
  #[error("failed to update game variant order: {0}")]
  Update(#[from] UpdateGameVariantOrderError),
}

/// A repository for managing the display order of game variants.
#[async_trait]
pub trait GameVariantOrderRepository: Send + Sync {
  /// Retrieves the current ordered list of game variants.
  async fn get_ordered_variants(
    &self,
  ) -> Result<Vec<GameVariant>, GameVariantOrderRepositoryError>;

  /// Updates the display order of game variants.
  async fn update_order(
    &self,
    variants: &[GameVariant],
  ) -> Result<(), GameVariantOrderRepositoryError>;
}
