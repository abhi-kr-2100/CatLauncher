use tauri::{command, State};
use crate::variants::GameVariant;
use crate::variants::repository::sqlite_game_variant_order_repository::GameVariantOrderRepository;

#[command]
pub fn update_game_variant_order(
    variants: Vec<GameVariant>,
    game_variant_order_repository: State<'_, GameVariantOrderRepository>,
) -> Result<(), String> {
    game_variant_order_repository
        .update_order(&variants)
        .map_err(|e| e.to_string())
}
