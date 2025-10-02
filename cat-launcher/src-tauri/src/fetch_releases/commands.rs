use tauri::command;

use crate::fetch_releases::error::FetchReleasesError;
use crate::game_release::GameRelease;
use crate::infra::http_client::HTTP_CLIENT;
use crate::variants::GameVariant;

#[command]
pub async fn fetch_releases_for_variant(
    variant: GameVariant,
) -> Result<Vec<GameRelease>, FetchReleasesError> {
    let client = match &*HTTP_CLIENT {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };

    variant.fetch_releases(&client).await
}
