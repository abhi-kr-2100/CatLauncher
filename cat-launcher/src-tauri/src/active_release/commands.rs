use strum::IntoStaticStr;
use tauri::{command, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::active_release::ActiveReleaseError;
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ActiveReleaseCommandError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseError),

  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] tauri::Error),
}

#[command]
pub async fn get_active_release(
  variant: GameVariant,
  repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Option<String>, ActiveReleaseCommandError> {
  let repo = repository.inner();

  let active_release = variant.get_active_release(repo).await?;

  Ok(active_release)
}
