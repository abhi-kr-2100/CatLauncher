use strum::IntoEnumIterator;
use tauri::command;
use ts_rs::TS;

use crate::variants::GameVariant;
use super::basic_info::Link;

#[derive(serde::Serialize, TS)]
#[ts(export)]
pub struct GameVariantInfo {
    pub id: GameVariant,
    pub name: &'static str,
    pub links: Vec<Link>,
}

impl From<GameVariant> for GameVariantInfo {
    fn from(variant: GameVariant) -> Self {
        GameVariantInfo {
            id: variant,
            name: variant.name(),
            links: variant.links(),
        }
    }
}

#[command]
pub fn get_game_variants_info() -> Vec<GameVariantInfo> {
    GameVariant::iter().map(GameVariantInfo::from).collect()
}
