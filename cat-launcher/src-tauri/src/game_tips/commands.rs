use strum::IntoStaticStr;
use tauri::{command, AppHandle, State};

use cat_macros::CommandErrorSerialize;

use crate::{
  active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository,
  fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository,
  game_tips::repository::{
    game_tips_repository::{
      GameTipsRepository, GameTipsRepositoryError,
    },
    sqlite_game_tips_repository::SqliteGameTipsRepository,
  },
  variants::GameVariant,
};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetTipsCommandError {
  #[error("failed to get tips: {0}")]
  GetTips(#[from] GameTipsRepositoryError),
}

#[command]
pub async fn get_tips(
  app_handle: AppHandle,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<Vec<String>, GetTipsCommandError> {
  let game_tips_repository = SqliteGameTipsRepository::new(
    app_handle,
    active_release_repository.inner(),
    releases_repository.inner(),
  )?;
  let tips = game_tips_repository
    .get_all_tips_for_variant(&variant)
    .await?;
  Ok(tips)
}
