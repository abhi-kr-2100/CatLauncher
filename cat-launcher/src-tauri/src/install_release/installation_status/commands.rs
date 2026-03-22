use std::env::consts::OS;

use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_release::game_release::GameReleaseStatus;
use crate::game_release::utils::{
  get_release_by_id, GetReleaseError,
};
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GetInstallationStatusCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to obtain release: {0}")]
  Release(#[from] GetReleaseError),

  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),
}

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
