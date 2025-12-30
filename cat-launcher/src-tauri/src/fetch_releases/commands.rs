use reqwest::Client;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::fetch_releases::fetch_releases::{
  FetchReleasesError, ReleasesUpdatePayload,
};
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::infra::github::utils::fetch_github_release;
use crate::infra::utils::get_github_repo_for_variant;
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
pub async fn fetch_release_notes(
  variant: GameVariant,
  release_tag_name: String,
  releases_repository: State<'_, SqliteReleasesRepository>,
  client: State<'_, Client>,
) -> Result<Option<String>, FetchReleasesCommandError> {
  let release_notes = releases_repository
    .get_release_notes(&variant, &release_tag_name)?;

  if release_notes.is_some() {
    return Ok(release_notes);
  }

  let repo = get_github_repo_for_variant(&variant);
  let release =
    fetch_github_release(&client, repo, &release_tag_name).await?;

  if let Some(body) = &release.body {
    releases_repository
      .update_cached_releases(&variant, &[release.clone()])
      .await?;
  }

  Ok(release.body)
}
