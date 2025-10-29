use strum::IntoEnumIterator;
use tauri::{command, State};
use ts_rs::TS;

use crate::basic_info::basic_info::Link;
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(serde::Serialize, TS)]
#[ts(export)]
pub struct GameVariantInfo {
    pub id: GameVariant,
    pub name: String,
    pub links: Vec<Link>,
}

impl GameVariantInfo {
    fn from_variant(variant: GameVariant, settings: &Settings) -> Self {
        GameVariantInfo {
            id: variant,
            name: variant.name(settings).to_string(),
            links: variant.links(settings).to_vec(),
        }
    }
}

#[command]
pub fn get_game_variants_info(settings: State<'_, Settings>) -> Vec<GameVariantInfo> {
    GameVariant::iter()
        .map(|variant| GameVariantInfo::from_variant(variant, &*settings))
        .collect()
}
