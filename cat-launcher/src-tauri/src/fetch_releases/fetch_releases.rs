use std::path::Path;

use reqwest::Client;
use serde::Serialize;
use ts_rs::TS;

use crate::fetch_releases::repository::{
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::fetch_releases::utils::{
  get_default_releases, get_releases_payload,
};
use crate::game_release::game_release::GameRelease;
use crate::infra::github::utils::{
  fetch_github_release_by_tag, fetch_github_releases,
  FetchGitHubReleaseByTagError, GitHubReleaseFetchError,
};
use crate::infra::utils::{get_github_repo_for_variant, Arch, OS};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchReleasesError {
  #[error("failed to get releases from github: {0}")]
  Fetch(#[from] GitHubReleaseFetchError),

  #[error("failed to access releases cache: {0}")]
  Repository(#[from] ReleasesRepositoryError),

  #[error("failed to send release update: {0}")]
  Send(#[from] tauri::Error),
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReleasesUpdatePayload {
  pub variant: GameVariant,
  pub releases: Vec<GameRelease>,
  pub status: ReleasesUpdateStatus,
}

#[derive(Debug, Clone, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
pub enum ReleasesUpdateStatus {
  Fetching,
  Success,
}

#[derive(thiserror::Error, Debug)]
pub enum FetchReleaseNotesError {
  #[error("failed to get release from github: {0}")]
  Fetch(#[from] FetchGitHubReleaseByTagError),

  #[error("failed to access releases cache: {0}")]
  Repository(#[from] ReleasesRepositoryError),
}

impl GameVariant {
  pub async fn fetch_release_notes(
    &self,
    release_id: &str,
    client: &Client,
    releases_repository: &dyn ReleasesRepository,
  ) -> Result<Option<String>, FetchReleaseNotesError> {
    let cached_release = releases_repository
      .get_cached_release_by_tag(self, release_id)
      .await?;

    if let Some(release) = cached_release {
      if let Some(body) = &release.body {
        return Ok(Some(body.clone()));
      }
    }

    // If not found or body is missing, fetch from GitHub
    let repo = get_github_repo_for_variant(self);
    let github_release =
      fetch_github_release_by_tag(client, repo, release_id).await?;

    // Update cache
    releases_repository
      .update_cached_releases(
        self,
        std::slice::from_ref(&github_release),
      )
      .await?;

    Ok(github_release.body)
  }
}

pub async fn fetch_releases_for_variant_and_emit_updates<F>(
  variant: &GameVariant,
  client: &Client,
  resources_dir: &Path,
  releases_repository: &dyn ReleasesRepository,
  on_releases: F,
  os: &OS,
  arch: &Arch,
) -> Result<(), FetchReleasesError>
where
  F: Fn(ReleasesUpdatePayload) -> Result<(), tauri::Error>,
{
  // 1. Fetch and emit cached releases.
  let cached_releases =
    releases_repository.get_cached_releases(variant).await?;
  let payload = get_releases_payload(
    variant,
    &cached_releases,
    ReleasesUpdateStatus::Fetching,
    os,
    arch,
  );
  on_releases(payload)?;

  // 2. Fetch and emit releases from GitHub.
  // Fetching 100 releases makes it likely that we have the last played release.
  // TODO: Fetch the last played release separately.
  let repo = get_github_repo_for_variant(variant);
  let fetched_releases =
    fetch_github_releases(client, repo, Some(100)).await?;

  releases_repository
    .update_cached_releases(variant, &fetched_releases)
    .await?;

  let payload = get_releases_payload(
    variant,
    &fetched_releases,
    ReleasesUpdateStatus::Fetching,
    os,
    arch,
  );
  on_releases(payload)?;

  // 3. Fetch and emit default releases.
  // These are only fetched and emitted at the end so that GitHub releases
  // are displayed first on first launch.
  let default_releases =
    get_default_releases(variant, resources_dir).await;
  let payload = get_releases_payload(
    variant,
    &default_releases,
    ReleasesUpdateStatus::Success,
    os,
    arch,
  );
  on_releases(payload)?;

  Ok(())
}
