use tauri::command;

use super::game_release::GameRelease;
use crate::variants::GameVariant;

#[command]
pub async fn fetch_releases_for_variant(variant: GameVariant) -> Result<Vec<GameRelease>, String> {
    variant.fetch_releases().await.map_err(|e| e.to_string())
}
