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
  use std::error::Error;

  use strum::IntoEnumIterator;

  use crate::infra::testing::test_database::TestDatabase;
  use crate::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
  use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;
  use crate::variants::GameVariant;

  use super::get_game_variants_info;

  #[tokio::test]
  async fn returns_variants_in_default_enum_order_when_order_table_is_empty()
  -> Result<(), Box<dyn Error + Send + Sync>> {
    let repository = repository(&TestDatabase::builder().build()?);

    let infos = get_game_variants_info(&repository).await?;

    assert_infos_eq(infos, GameVariant::iter().collect::<Vec<_>>());

    Ok(())
  }

  #[tokio::test]
  async fn returns_variants_in_persisted_order_when_table_is_full()
  -> Result<(), Box<dyn Error + Send + Sync>> {
    let db = TestDatabase::builder().build()?;
    let repository = repository(&db);

    repository
      .update_order(&[
        GameVariant::BrightNights,
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
      ])
      .await?;

    let infos = get_game_variants_info(&repository).await?;

    assert_infos_eq(
      infos,
      vec![
        GameVariant::BrightNights,
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
      ],
    );

    Ok(())
  }

  #[tokio::test]
  async fn returns_all_variants_when_order_table_is_partially_populated()
  -> Result<(), Box<dyn Error + Send + Sync>> {
    let db = TestDatabase::builder().build()?;
    let repository = repository(&db);

    repository
      .update_order(&[
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
      ])
      .await?;

    let infos = get_game_variants_info(&repository).await?;

    assert_infos_eq(
      infos,
      vec![
        GameVariant::TheLastGeneration,
        GameVariant::DarkDaysAhead,
        GameVariant::BrightNights,
      ],
    );

    Ok(())
  }

  fn assert_infos_eq(
    actual: Vec<super::GameVariantInfo>,
    expected_variants: Vec<GameVariant>,
  ) {
    let actual_variants =
      actual.iter().map(|info| info.id).collect::<Vec<_>>();
    assert_eq!(actual_variants, expected_variants);

    let actual_names = actual
      .iter()
      .map(|info| info.name.as_str())
      .collect::<Vec<_>>();
    let expected_names = expected_variants
      .iter()
      .map(GameVariant::name)
      .collect::<Vec<_>>();
    assert_eq!(actual_names, expected_names);
  }

  fn repository(
    db: &TestDatabase,
  ) -> SqliteGameVariantOrderRepository {
    SqliteGameVariantOrderRepository::new(db.pool().clone())
  }
}
