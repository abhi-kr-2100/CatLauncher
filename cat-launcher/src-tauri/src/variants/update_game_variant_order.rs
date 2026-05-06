use crate::variants::GameVariant;
use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepositoryError;

/// Errors that can occur when updating the game variant order.
#[derive(thiserror::Error, Debug)]
pub enum UpdateGameVariantOrderError {
  /// The update operation failed in the underlying repository.
  #[error("failed to update game variant order")]
  Update(#[from] GameVariantOrderRepositoryError),
}

/// Updates the display order of game variants using the provided repository.
pub async fn update_game_variant_order(
  variants: &[GameVariant],
  game_variant_order_repository: &impl GameVariantOrderRepository,
) -> Result<(), UpdateGameVariantOrderError> {
  game_variant_order_repository.update_order(variants).await?;
  Ok(())
}
