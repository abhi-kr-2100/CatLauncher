use tauri::{AppHandle, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::game_tips::lib::get_all_tips_for_variant;
use crate::game_tips::lib::GetAllTipsForVariantError;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::variants::GameVariant;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;

/// Errors that can occur when executing the get tips command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetTipsCommandError {
  /// Failed to access the app local data directory.
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),

  /// The current operating system is not supported.
  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),

  /// Failed to retrieve tips for the given game variant.
  #[error("failed to get tips for variant: {0}")]
  GetForVariant(#[from] GetAllTipsForVariantError),
}

/// Tauri command to retrieve game tips for a specified game variant.
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
