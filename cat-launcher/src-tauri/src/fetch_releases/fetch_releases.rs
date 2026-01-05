use std::error::Error;
use std::path::Path;

use reqwest::Client;
use serde::Serialize;
use ts_rs::TS;

use crate::fetch_releases::repository::{
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::fetch_releases::utils::{
  get_default_releases, get_releases_payload, merge_releases,
};
use crate::game_release::game_release::GameRelease;
use crate::infra::github::utils::{
  fetch_github_release_by_tag, fetch_github_releases,
  FetchGitHubReleaseByTagError, GitHubReleaseFetchError,
};
use crate::infra::utils::get_github_repo_for_variant;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchReleasesError<E: Error> {
  #[error("failed to get releases from github: {0}")]
  Fetch(#[from] GitHubReleaseFetchError),

  #[error("failed to access releases cache: {0}")]
  Repository(#[from] ReleasesRepositoryError),

  #[error("failed to send release update: {0}")]
  Send(E),
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
  Error,
}

#[derive(thiserror::Error, Debug)]
pub enum FetchReleaseNotesError {
  #[error("failed to get release from github: {0}")]
  Fetch(#[from] FetchGitHubReleaseByTagError),

  #[error("failed to access releases cache: {0}")]
  Repository(#[from] ReleasesRepositoryError),
}

impl GameVariant {
  pub async fn fetch_releases<E, F>(
    &self,
    client: &Client,
    resources_dir: &Path,
    releases_repository: &dyn ReleasesRepository,
    on_releases: F,
  ) -> Result<(), FetchReleasesError<E>>
  where
    E: Error,
    F: Fn(ReleasesUpdatePayload) -> Result<(), E>,
  {
    // Both default and cached releases are stored locally, and are quick to fetch.
    // We fetch them together so that if the last played release was cached, the frontend
    // can preselect it.
    let default_releases =
      get_default_releases(self, resources_dir).await;
    let cached_releases =
      releases_repository.get_cached_releases(self).await?;
    let merged = merge_releases(&default_releases, &cached_releases);
    let payload = get_releases_payload(
      self,
      &merged,
      ReleasesUpdateStatus::Fetching,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    let repo = get_github_repo_for_variant(self);
    // Fetching 100 releases makes it likely that we have the last played release.
    // TODO: Fetch the last played release separately.
    let fetched_releases =
      fetch_github_releases(client, repo, Some(100)).await?;
    let payload = get_releases_payload(
      self,
      &fetched_releases,
      ReleasesUpdateStatus::Success,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    releases_repository
      .update_cached_releases(self, &fetched_releases)
      .await?;

    Ok(())
  }

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
