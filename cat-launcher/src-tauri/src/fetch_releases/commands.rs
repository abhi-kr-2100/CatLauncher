use reqwest::Client;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::fetch_releases::FetchReleasesError;
use crate::fetch_releases::repository::releases_repository::ReleasesRepository;
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

#[command]
pub async fn fetch_releases_for_variant(
  app_handle: AppHandle,
  variant: GameVariant,
  releases_repository: State<'_, Box<dyn ReleasesRepository>>,
  client: State<'_, Client>,
) -> Result<(), FetchReleasesCommandError> {
  let resources_dir = app_handle.path().resource_dir()?;
  let releases_repository = releases_repository.inner();
  let client = client.inner();

  let initial_releases = variant
    .get_initial_releases_payload(
      &resources_dir,
      &**releases_repository,
    )
    .await?;
  app_handle.emit("releases-update", initial_releases)?;

  let releases = variant
    .fetch_releases_from_github(client, &**releases_repository)
    .await?;
  app_handle.emit("releases-update", releases)?;

  Ok(())
}
