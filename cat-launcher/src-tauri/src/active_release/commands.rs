use strum::IntoStaticStr;
use tauri::{State, command};

use cat_macros::CommandErrorSerialize;

use crate::active_release::active_release::ActiveReleaseError;
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::variants::GameVariant;

/// Errors that can occur when retrieving the active release.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ActiveReleaseCommandError {
  /// An error occurred while retrieving the active release from the repository.
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseError),

  /// An error occurred while determining a system directory.
  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] tauri::Error),
}

/// Retrieves the version of the active release for a specific game variant.
#[command]
pub async fn get_active_release(
  variant: GameVariant,
  repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Option<String>, ActiveReleaseCommandError> {
  let active_release =
    variant.get_active_release(&*repository).await?;

  Ok(active_release)
}
