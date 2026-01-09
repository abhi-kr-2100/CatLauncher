use std::env::consts::{ARCH, OS};

use reqwest::Client;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::fetch_releases::{
  fetch_releases_for_variant_and_emit_updates,
  FetchReleaseNotesError, FetchReleasesError, ReleasesUpdatePayload,
};
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::infra::utils::{
  get_arch_enum, get_os_enum, ArchNotSupportedError,
  OSNotSupportedError,
};
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum FetchReleasesCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to fetch releases: {0}")]
  Fetch(#[from] FetchReleasesError),

  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),

  #[error("failed to get arch enum: {0}")]
  Arch(#[from] ArchNotSupportedError),
}

#[command]
pub async fn fetch_releases_for_variant(
  app_handle: AppHandle,
  variant: GameVariant,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, Client>,
) -> Result<(), FetchReleasesCommandError> {
  let resources_dir = app_handle.path().resource_dir()?;
  let os = get_os_enum(OS)?;
  let arch = get_arch_enum(ARCH)?;

  let on_releases = move |payload: ReleasesUpdatePayload| {
    app_handle.emit("releases-update", payload)?;
    Ok(())
  };

  fetch_releases_for_variant_and_emit_updates(
    &variant,
    &client,
    &resources_dir,
    &*releases_repository,
    on_releases,
    &os,
    &arch,
  )
  .await?;

  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum FetchReleaseNotesCommandError {
  #[error("failed to fetch release notes: {0}")]
  Fetch(#[from] FetchReleaseNotesError),
}

#[command]
pub async fn fetch_release_notes(
  variant: GameVariant,
  release_id: String,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, Client>,
) -> Result<Option<String>, FetchReleaseNotesCommandError> {
  let notes = variant
    .fetch_release_notes(&release_id, &client, &*releases_repository)
    .await?;

  Ok(notes)
}
