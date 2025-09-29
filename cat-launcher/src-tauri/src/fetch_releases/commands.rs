use tauri::command;
use crate::variants::GameVariant;
use crate::fetch_releases::FetchReleasesAsync;
use super::GameRelease;

#[command]
pub async fn fetch_releases_for_variant(variant: GameVariant) -> Result<Vec<GameRelease>, String> {
	variant.fetch().await.map_err(|e| e.to_string())
}
