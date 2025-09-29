use tauri::command;
use strum::IntoEnumIterator;
use super::GameVariant;
use super::GameVariantBasicInfo;

#[derive(serde::Serialize)]
pub struct GameVariantInfo {
	pub name: &'static str,
	pub description: &'static str,
}

#[command]
pub fn get_game_variants_info() -> Vec<GameVariantInfo> {
	GameVariant::iter().map(|variant| {
		GameVariantInfo {
			name: variant.name(),
			description: variant.description(),
		}
	}).collect()
}
