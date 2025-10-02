use tauri::{command, AppHandle, Manager};

use crate::fetch_releases::error::FetchReleasesError;
use crate::game_release::GameRelease;
use crate::variants::GameVariant;

#[command]
pub async fn fetch_releases_for_variant(
    app_handle: AppHandle,
    variant: GameVariant,
) -> Result<Vec<GameRelease>, FetchReleasesError> {
    let cache_dir = app_handle.path().app_cache_dir()?;

    variant.fetch_releases(&cache_dir).await
}
