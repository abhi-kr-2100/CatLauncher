use std::env::consts::OS;

use strum::IntoStaticStr;
use tauri::{AppHandle, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_release::game_release::GameReleaseStatus;
use crate::game_release::utils::{
  GetReleaseError, get_release_by_id,
};
use crate::infra::utils::{OSNotSupportedError, get_os_enum};
use crate::variants::GameVariant;

/// Errors that can occur when getting the installation status via a Tauri command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetInstallationStatusCommandError {
  /// The system's local data or resource directory could not be found.
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  /// Failed to retrieve the release information from the repository.
  #[error("failed to obtain release: {0}")]
  Release(#[from] GetReleaseError),

  /// The current operating system is not supported.
  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),
}

/// A Tauri command that returns the installation status of a specific release.
#[command]
pub async fn get_installation_status(
  app_handle: AppHandle,
  variant: GameVariant,
  release_id: &str,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<GameReleaseStatus, GetInstallationStatusCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resource_dir = app_handle.path().resource_dir()?;

  let os = get_os_enum(OS)?;

  let release = get_release_by_id(
    &variant,
    release_id,
    &os,
    &data_dir,
    &resource_dir,
    &*releases_repository,
  )
  .await?;

  Ok(release.status)
}
