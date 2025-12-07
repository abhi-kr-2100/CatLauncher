use strum::IntoStaticStr;
use tauri::{command, State};

use crate::infra::command_error::SerializableError;
use crate::settings::Settings;
use crate::variants::get_game_variants_info::{self, GameVariantInfo, GetGameVariantsInfoError};
use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;
use crate::variants::update_game_variant_order::{self, UpdateGameVariantOrderError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum UpdateGameVariantOrderCommandError {
    #[error("failed to update game variant order: {0}")]
    Update(#[from] UpdateGameVariantOrderError),
}

#[command]
pub async fn update_game_variant_order(
    variants: Vec<GameVariant>,
    game_variant_order_repository: State<'_, SqliteGameVariantOrderRepository>,
) -> Result<(), SerializableError> {
    let result =
        update_game_variant_order_inner(variants, game_variant_order_repository).await;
    result.map_err(SerializableError::from)
}

pub async fn update_game_variant_order_inner(
    variants: Vec<GameVariant>,
    game_variant_order_repository: State<'_, SqliteGameVariantOrderRepository>,
) -> Result<(), UpdateGameVariantOrderCommandError> {
    update_game_variant_order::update_game_variant_order(
        &variants,
        &*game_variant_order_repository,
    )
    .await?;
    Ok(())
}

impl From<UpdateGameVariantOrderCommandError> for SerializableError {
    fn from(error: UpdateGameVariantOrderCommandError) -> Self {
        SerializableError::new(error)
    }
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GetGameVariantsInfoCommandError {
    #[error("failed to get game variant order: {0}")]
    Get(#[from] GetGameVariantsInfoError),
}

#[command]
pub async fn get_game_variants_info(
    settings: State<'_, Settings>,
    game_variant_order_repository: State<'_, SqliteGameVariantOrderRepository>,
) -> Result<Vec<GameVariantInfo>, SerializableError> {
    let result = get_game_variants_info_inner(settings, game_variant_order_repository).await;
    result.map_err(SerializableError::from)
}

pub async fn get_game_variants_info_inner(
    settings: State<'_, Settings>,
    game_variant_order_repository: State<'_, SqliteGameVariantOrderRepository>,
) -> Result<Vec<GameVariantInfo>, GetGameVariantsInfoCommandError> {
    let res =
        get_game_variants_info::get_game_variants_info(&settings, &*game_variant_order_repository)
            .await?;
    Ok(res)
}

impl From<GetGameVariantsInfoCommandError> for SerializableError {
    fn from(error: GetGameVariantsInfoCommandError) -> Self {
        SerializableError::new(error)
    }
}
