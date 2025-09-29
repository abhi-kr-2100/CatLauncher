use tauri::command;
use ts_rs::TS;
use strum::IntoEnumIterator;
use super::GameVariant;
use super::GameVariantBasicInfo;

#[derive(serde::Serialize, TS)]
#[ts(export)]
pub struct GameVariantInfo {
	pub id: &'static str,
	pub name: &'static str,
	pub description: &'static str,
}

#[command]
pub fn get_game_variants_info() -> Vec<GameVariantInfo> {
	GameVariant::iter().map(|variant| {
		GameVariantInfo {
			id: variant.id(),
			name: variant.name(),
			description: variant.description(),
		}
	}).collect()
}
