use reqwest::Client;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::fetch_releases::{
  FetchReleasesError, ReleasesUpdatePayload,
};
use crate::fetch_releases::release_notes::ReleaseNotesUpdatePayload;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum FetchReleasesCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to fetch releases: {0}")]
  Fetch(#[from] FetchReleasesError<tauri::Error>),
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum FetchReleaseNotesCommandError {
  #[error("failed to start fetching release notes")]
  Start,
}

#[command]
pub async fn fetch_releases_for_variant(
  app_handle: AppHandle,
  variant: GameVariant,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, Client>,
) -> Result<(), FetchReleasesCommandError> {
  let resources_dir = app_handle.path().resource_dir()?;

  let on_releases = move |payload: ReleasesUpdatePayload| {
    app_handle.emit("releases-update", payload)?;
    Ok(())
  };

  variant
    .fetch_releases(
      &client,
      &resources_dir,
      &*releases_repository,
      on_releases,
    )
    .await?;

  Ok(())
}

#[command]
pub async fn fetch_release_notes_for_variant(
  app_handle: AppHandle,
  request_id: String,
  variant: GameVariant,
  versions: Vec<String>,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, Client>,
) -> Result<(), FetchReleaseNotesCommandError> {
  let releases_repository = releases_repository.inner().clone();
  let client = client.inner().clone();

  let emit_handle = app_handle.clone();

  tauri::async_runtime::spawn(async move {
    let on_update =
      move |payload: ReleaseNotesUpdatePayload| -> Result<(), tauri::Error> {
        emit_handle.emit("release-notes-update", payload)?;
        Ok(())
      };

    if let Err(e) = variant
      .fetch_release_notes(
        &request_id,
        &client,
        &releases_repository,
        &releases_repository,
        &versions,
        on_update,
      )
      .await
    {
      eprintln!("Failed to fetch release notes: {e}");
    }
  });

  Ok(())
}
