use tauri::{State, command};

use cat_macros::CommandErrorSerialize;

use crate::variants::get_game_variants_info::{self, GameVariantInfo, GetGameVariantsInfoError};
use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;
use crate::variants::update_game_variant_order::{self, UpdateGameVariantOrderError};
use crate::variants::GameVariant;

/// Errors that can occur when updating the game variant display order.
#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum UpdateGameVariantOrderCommandError {
  /// The update operation failed in the repository.
  #[error("failed to update game variant order: {0}")]
  Update(#[from] UpdateGameVariantOrderError),
}

/// A Tauri command that updates the display order of game variants.
#[command]
pub async fn update_game_variant_order(
  variants: Vec<GameVariant>,
  game_variant_order_repository: State<
    '_,
    SqliteGameVariantOrderRepository,
  >,
) -> Result<(), UpdateGameVariantOrderCommandError> {
  update_game_variant_order::update_game_variant_order(
    &variants,
    &*game_variant_order_repository,
  )
  .await?;

  Ok(())
}

/// Errors that can occur when retrieving game variant information.
#[derive(
  thiserror::Error, Debug, strum::IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetGameVariantsInfoCommandError {
  /// The retrieval operation failed in the repository.
  #[error("failed to get game variant order: {0}")]
  Get(#[from] GetGameVariantsInfoError),
}

/// A Tauri command that retrieves information for all available game variants.
#[command]
pub async fn get_game_variants_info(
  game_variant_order_repository: State<
    '_,
    SqliteGameVariantOrderRepository,
  >,
) -> Result<Vec<GameVariantInfo>, GetGameVariantsInfoCommandError> {
  let res = get_game_variants_info::get_game_variants_info(
    &*game_variant_order_repository,
  )
  .await?;

  Ok(res)
}
