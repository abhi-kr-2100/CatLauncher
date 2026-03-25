use strum::IntoStaticStr;
use tauri::{command, AppHandle, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_tips::lib::get_all_tips_for_variant;
use crate::game_tips::lib::GetAllTipsForVariantError;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetTipsError {
  #[error("failed to get tips for variant: {0}")]
  GetForVariant(#[from] GetAllTipsForVariantError),
}

#[command]
pub async fn get_tips(
  app_handle: AppHandle,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<Vec<String>, GetTipsError> {
  let tips = get_all_tips_for_variant(
    &app_handle,
    &variant,
    &*active_release_repository,
    &*releases_repository,
  )
  .await?;
  Ok(tips)
}
