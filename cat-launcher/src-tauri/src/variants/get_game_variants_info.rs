use strum::IntoEnumIterator;

use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;

use crate::variants::GameVariant;
use ts_rs::TS;

/// Information about a game variant, suitable for display in the UI.
#[derive(serde::Serialize, TS)]
#[ts(export)]
pub struct GameVariantInfo {
  /// The unique identifier for the game variant.
  pub id: GameVariant,
  /// The human-readable name of the game variant.
  pub name: String,
}

impl GameVariantInfo {
  /// Creates a new `GameVariantInfo` from a `GameVariant`.
  pub fn from_variant(variant: GameVariant) -> Self {
    GameVariantInfo {
      id: variant,
      name: variant.name().to_string(),
    }
  }
}

/// Errors that can occur when retrieving game variant information.
#[derive(thiserror::Error, Debug)]
pub enum GetGameVariantsInfoError {
  /// Failed to retrieve the variant order from the repository.
  #[error("failed to get game variant order")]
    Get(#[from] crate::variants::repository::game_variant_order_repository::GameVariantOrderRepositoryError),
}

/// Retrieves information for all game variants in their preferred display order.
pub async fn get_game_variants_info(
  game_variant_order_repository: &impl GameVariantOrderRepository,
) -> Result<Vec<GameVariantInfo>, GetGameVariantsInfoError> {
  let ordered_variants =
    game_variant_order_repository.get_ordered_variants().await?;

  let variants_to_display = if ordered_variants.is_empty() {
    GameVariant::iter().collect::<Vec<_>>()
  } else {
    ordered_variants
  };

  let result = variants_to_display
    .into_iter()
    .map(GameVariantInfo::from_variant)
    .collect();
  Ok(result)
}

#[cfg(test)]
mod tests {
  use crate::variants::GameVariant;
  use crate::variants::get_game_variants_info::get_game_variants_info;
  use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
  use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;

  use crate::testing::support::db::TestDatabase;

  #[tokio::test]
  async fn returns_all_variants_in_enum_order_when_no_order_is_saved() {
    let db = TestDatabase::new();
    let repository =
      SqliteGameVariantOrderRepository::new(db.pool.clone());

    let variants = get_game_variants_info(&repository)
      .await
      .expect("get variants");

    let ids = variants
      .iter()
      .map(|variant| variant.id)
      .collect::<Vec<_>>();
    let names = variants
      .iter()
      .map(|variant| variant.name.as_str())
      .collect::<Vec<_>>();

    assert_eq!(
      ids,
      vec![
        GameVariant::DarkDaysAhead,
        GameVariant::BrightNights,
        GameVariant::TheLastGeneration,
      ]
    );
    assert_eq!(
      names,
      vec!["Dark Days Ahead", "Bright Nights", "The Last Generation"]
    );
  }

  #[tokio::test]
  async fn returns_variants_in_saved_order() {
    let db = TestDatabase::new();
    let repository =
      SqliteGameVariantOrderRepository::new(db.pool.clone());
    repository
      .update_order(&[
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
        GameVariant::BrightNights,
      ])
      .await
      .expect("save variant order");

    let variants = get_game_variants_info(&repository)
      .await
      .expect("get variants");

    let ids = variants
      .iter()
      .map(|variant| variant.id)
      .collect::<Vec<_>>();
    let names = variants
      .iter()
      .map(|variant| variant.name.as_str())
      .collect::<Vec<_>>();

    assert_eq!(
      ids,
      vec![
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
        GameVariant::BrightNights,
      ]
    );
    assert_eq!(
      names,
      vec!["The Last Generation", "Dark Days Ahead", "Bright Nights"]
    );
  }
}
