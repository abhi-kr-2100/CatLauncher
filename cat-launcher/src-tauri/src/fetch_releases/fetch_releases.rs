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
  fetch_github_releases, GitHubReleaseFetchError,
};
use crate::infra::utils::{get_github_repo_for_variant, Arch, OS};
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

impl GameVariant {
  pub async fn fetch_releases<E, F>(
    &self,
    client: &Client,
    resources_dir: &Path,
    releases_repository: &dyn ReleasesRepository,
    on_releases: F,
    os: &OS,
    arch: &Arch,
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
      os,
      arch,
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
      os,
      arch,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    releases_repository
      .update_cached_releases(self, &fetched_releases)
      .await?;

    Ok(())
  }
}
