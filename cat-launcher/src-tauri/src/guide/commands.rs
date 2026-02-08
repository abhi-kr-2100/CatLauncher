use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::guide::lib::{get_guide_entity, search_guide, GuideError};
use crate::guide::types::{GuideEntityDetail, GuideEntry};
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum GuideCommandError {
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),

  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),

  #[error("guide error: {0}")]
  Guide(#[from] GuideError),
}

#[command]
pub async fn search_guide_command(
  app_handle: AppHandle,
  query: String,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<Vec<GuideEntry>, GuideCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;

  let entries = search_guide(
    query,
    variant,
    &data_dir,
    &os,
    &*active_release_repository,
    &*releases_repository,
  )
  .await?;
  Ok(entries)
}

#[command]
pub async fn get_guide_entity_command(
  app_handle: AppHandle,
  id: String,
  variant: GameVariant,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  releases_repository: State<'_, SqliteReleasesRepository>,
) -> Result<GuideEntityDetail, GuideCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;

  let detail = get_guide_entity(
    id,
    variant,
    &data_dir,
    &os,
    &*active_release_repository,
    &*releases_repository,
  )
  .await?;
  Ok(detail)
}
