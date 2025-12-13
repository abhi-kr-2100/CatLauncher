use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_tips::game_tips::get_all_tips_for_variant;
use crate::game_tips::game_tips::GetAllTipsForVariantError;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetTipsCommandError {
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),

  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),

  #[error("failed to get tips for variant: {0}")]
  GetForVariant(#[from] GetAllTipsForVariantError),
}

#[command]
pub async fn get_tips(
  app_handle: AppHandle,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<Vec<String>, GetTipsCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;

  let tips = get_all_tips_for_variant(
    &variant,
    &data_dir,
    &os,
    &*active_release_repository,
    &*releases_repository,
  )
  .await?;
  Ok(tips)
}
