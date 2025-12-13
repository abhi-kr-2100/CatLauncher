use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetGameVariantOrderError {
  #[error("failed to get game variant order: {0}")]
  Get(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateGameVariantOrderError {
  #[error("failed to update game variant order: {0}")]
  Update(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(thiserror::Error, Debug)]
pub enum GameVariantOrderRepositoryError {
  #[error("failed to get game variant order: {0}")]
  Get(#[from] GetGameVariantOrderError),

  #[error("failed to update game variant order: {0}")]
  Update(#[from] UpdateGameVariantOrderError),
}

#[async_trait]
pub trait GameVariantOrderRepository: Send + Sync {
  async fn get_ordered_variants(
    &self,
  ) -> Result<Vec<GameVariant>, GameVariantOrderRepositoryError>;

  async fn update_order(
    &self,
    variants: &[GameVariant],
  ) -> Result<(), GameVariantOrderRepositoryError>;
}
