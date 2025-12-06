use strum::IntoEnumIterator;

use crate::settings::Settings;
use crate::variants::links::Link;
use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
use crate::variants::GameVariant;
use ts_rs::TS;

#[derive(serde::Serialize, TS)]
#[ts(export)]
pub struct GameVariantInfo {
    pub id: GameVariant,
    pub name: String,
    pub links: Vec<Link>,
}

impl GameVariantInfo {
    pub fn from_variant(variant: GameVariant, settings: &Settings) -> Self {
        GameVariantInfo {
            id: variant,
            name: variant.name(settings).to_string(),
            links: variant.links(settings).to_vec(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetGameVariantsInfoError {
    #[error("failed to get game variant order")]
    Get(#[from] crate::variants::repository::game_variant_order_repository::GameVariantOrderRepositoryError),
}

pub async fn get_game_variants_info(
    settings: &Settings,
    game_variant_order_repository: &impl GameVariantOrderRepository,
) -> Result<Vec<GameVariantInfo>, GetGameVariantsInfoError> {
    let ordered_variants = game_variant_order_repository.get_ordered_variants().await?;

    let variants_to_display = if ordered_variants.is_empty() {
        GameVariant::iter().collect::<Vec<_>>()
    } else {
        ordered_variants
    };

    let result = variants_to_display
        .into_iter()
        .map(|variant| GameVariantInfo::from_variant(variant, settings))
        .collect();
    Ok(result)
}
