use tauri::command;

use crate::fetch_releases::error::FetchReleasesError;
use crate::game_release::GameRelease;
use crate::variants::GameVariant;

#[command]
pub async fn fetch_releases_for_variant(
    variant: GameVariant,
) -> Result<Vec<GameRelease>, FetchReleasesError> {
    variant.fetch_releases().await
}
