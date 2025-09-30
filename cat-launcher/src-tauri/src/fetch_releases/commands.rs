use tauri::command;

use super::game_release::GameRelease;
use crate::{fetch_releases::error::FetchReleasesError, variants::GameVariant};

#[command]
pub async fn fetch_releases_for_variant(
    variant: GameVariant,
) -> Result<Vec<GameRelease>, FetchReleasesError> {
    variant.fetch_releases().await
}
