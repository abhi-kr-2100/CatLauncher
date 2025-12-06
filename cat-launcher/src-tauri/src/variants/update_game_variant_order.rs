use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepositoryError;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UpdateGameVariantOrderError {
    #[error("failed to update game variant order")]
    Update(#[from] GameVariantOrderRepositoryError),
}

pub async fn update_game_variant_order(
    variants: &[GameVariant],
    game_variant_order_repository: &impl GameVariantOrderRepository,
) -> Result<(), UpdateGameVariantOrderError> {
    game_variant_order_repository.update_order(variants).await?;
    Ok(())
}
