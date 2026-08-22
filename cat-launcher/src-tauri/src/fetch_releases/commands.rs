use std::env::consts::{ARCH, OS};

use tauri::{AppHandle, Emitter, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::fetch_releases::{
  FetchReleaseNotesError, FetchReleasesError, ReleasesUpdatePayload,
};
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::infra::http_client::ReqwestHttpClient;
use crate::infra::utils::{HostSystem, HostSystemError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum FetchReleasesCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to fetch releases: {0}")]
  Fetch(#[from] FetchReleasesError<tauri::Error>),

  #[error("failed to determine host system: {0}")]
  HostSystem(#[from] HostSystemError),
}

#[command]
pub async fn fetch_releases_for_variant(
  app_handle: AppHandle,
  variant: GameVariant,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, ReqwestHttpClient>,
) -> Result<(), FetchReleasesCommandError> {
  let resources_dir = app_handle.path().resource_dir()?;
  let host_system = HostSystem::current(OS, ARCH)?;

  let on_releases = move |payload: ReleasesUpdatePayload| {
    app_handle.emit("releases-update", payload)?;
    Ok(())
  };

  variant
    .fetch_releases(
      client.inner(),
      &resources_dir,
      &*releases_repository,
      on_releases,
      &host_system,
    )
    .await?;

  Ok(())
}

#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum FetchReleaseNotesCommandError {
  #[error("failed to fetch release notes: {0}")]
  Fetch(#[from] FetchReleaseNotesError),
}

#[command]
pub async fn fetch_release_notes(
  variant: GameVariant,
  release_id: String,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, ReqwestHttpClient>,
) -> Result<Option<String>, FetchReleaseNotesCommandError> {
  let notes = variant
    .fetch_release_notes(
      &release_id,
      client.inner(),
      &*releases_repository,
    )
    .await?;

  Ok(notes)
}
