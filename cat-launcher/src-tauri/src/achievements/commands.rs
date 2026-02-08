use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::{AppHandle, Manager, State};

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::variants::GameVariant;

use super::achievements::{get_achievements, GetAchievementsError};
use super::types::CharacterAchievements;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetAchievementsForVariantCommandError {
  #[error("failed to get achievements: {0}")]
  GetAchievements(#[from] GetAchievementsError),
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),
}

#[tauri::command]
pub async fn get_achievements_for_variant(
  variant: GameVariant,
  app_handle: AppHandle,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<
  Vec<CharacterAchievements>,
  GetAchievementsForVariantCommandError,
> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let achievements = get_achievements(
    &variant,
    &data_dir,
    active_release_repository.inner(),
  )
  .await?;

  Ok(achievements)
}
