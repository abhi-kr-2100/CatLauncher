use strum::IntoEnumIterator;
use tauri::{command, State};
use ts_rs::TS;

use crate::basic_info::basic_info::Link;
use crate::settings::Settings;
use crate::variants::GameVariant;
use crate::variants::repository::sqlite_game_variant_order_repository::GameVariantOrderRepository;

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
pub fn get_game_variants_info(
    settings: State<'_, Settings>,
    game_variant_order_repository: State<'_, GameVariantOrderRepository>,
) -> Vec<GameVariantInfo> {
    let ordered_variants = game_variant_order_repository
        .get_ordered_variants()
        .unwrap_or_default();

    let variants_to_display = if ordered_variants.is_empty() {
        GameVariant::iter().collect::<Vec<_>>()
    } else {
        ordered_variants
    };

    variants_to_display
        .into_iter()
        .map(|variant| GameVariantInfo::from_variant(variant, &settings))
        .collect()
}
