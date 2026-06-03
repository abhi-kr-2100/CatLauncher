use std::error::Error;
use std::path::Path;

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
  FetchGitHubReleaseByTagError, GitHubReleaseFetchError,
  fetch_github_release_by_tag, fetch_github_releases,
};
use crate::infra::http_client::HttpClient;
use crate::infra::utils::{Arch, OS, get_github_repo_for_variant};
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
    client: &dyn HttpClient,
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
    // 1. Fetch and emit cached releases.
    let cached_releases =
      releases_repository.get_cached_releases(self).await?;
    let payload = get_releases_payload(
      self,
      &cached_releases,
      ReleasesUpdateStatus::Fetching,
      os,
      arch,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    // 2. Fetch and emit releases from GitHub.
    // Fetching 100 releases makes it likely that we have the last played release.
    // TODO: Fetch the last played release separately.
    let repo = get_github_repo_for_variant(self);
    let fetched_releases =
      fetch_github_releases(client, repo, Some(100)).await?;

    releases_repository
      .update_cached_releases(self, &fetched_releases)
      .await?;

    let payload = get_releases_payload(
      self,
      &fetched_releases,
      ReleasesUpdateStatus::Fetching,
      os,
      arch,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    // 3. Fetch and emit default releases.
    // These are only fetched and emitted at the end so that GitHub releases
    // are displayed first on first launch.
    let default_releases =
      get_default_releases(self, resources_dir).await;
    let payload = get_releases_payload(
      self,
      &default_releases,
      ReleasesUpdateStatus::Success,
      os,
      arch,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    Ok(())
  }

  pub async fn fetch_release_notes(
    &self,
    release_id: &str,
    client: &dyn HttpClient,
    releases_repository: &dyn ReleasesRepository,
  ) -> Result<Option<String>, FetchReleaseNotesError> {
    let cached_release = releases_repository
      .get_cached_release_by_tag(self, release_id)
      .await?;

    if let Some(release) = cached_release
      && let Some(body) = &release.body
    {
      return Ok(Some(body.clone()));
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

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use github_mock_api::MockServer;
  use super::*;

  async fn setup() -> (MockServer, TestHttpClient, TestDatabase, SqliteReleasesRepository) {
    let server = MockServer::start().await.unwrap();
    let mut host_mappings = HashMap::new();
    let uri = server.uri();
    let host_port = uri.strip_prefix("http://").unwrap().to_string();
    host_mappings.insert("api.github.com".to_string(), host_port);
    let client = TestHttpClient::new(host_mappings);
    let db = TestDatabase::builder().build().unwrap();
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    (server, client, db, repo)
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_hit() {
    let (_server, client, _db, repo) = setup().await;
    let variant = GameVariant::DarkDaysAhead;
    let tag = "v1";
    let body = "cached body";

    let release = GitHubRelease {
      id: 1,
      tag_name: tag.to_string(),
      prerelease: false,
      body: Some(body.to_string()),
      assets: vec![],
      created_at: chrono::Utc::now(),
    };

    repo.update_cached_releases(&variant, &[release]).await.unwrap();

    let result = variant.fetch_release_notes(tag, &client, &repo).await.unwrap();
    assert_eq!(result, Some(body.to_string()));
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_hit_missing_body() {
    // Fails because github-mock-api omits the 'assets' field when empty,
    // but the production code's GitHubRelease struct requires it.
    // If a test scenario cannot be implemented using github-mock-api, fail it with a comment.
    panic!("Scenario cannot be implemented using github-mock-api: missing 'assets' field in mock response causes parse error in production code");
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_miss_github_hit() {
    // Fails because github-mock-api omits the 'assets' field when empty,
    // but the production code's GitHubRelease struct requires it.
    // If a test scenario cannot be implemented using github-mock-api, fail it with a comment.
    panic!("Scenario cannot be implemented using github-mock-api: missing 'assets' field in mock response causes parse error in production code");
  }

  #[tokio::test]
  async fn test_fetch_release_notes_github_404() {
    let (_server, client, _db, repo) = setup().await;
    let variant = GameVariant::DarkDaysAhead;
    let tag = "non-existent";

    let result = variant.fetch_release_notes(tag, &client, &repo).await;

    assert!(result.is_err());
    match result.unwrap_err() {
      FetchReleaseNotesError::Fetch(_) => (),
      _ => panic!("Expected Fetch error"),
    }
  }

  #[tokio::test]
  async fn test_fetch_release_notes_github_500() {
    // Fails because github-mock-api does not support forcing 500 errors easily.
    // If a test scenario cannot be implemented using github-mock-api, fail it with a comment.
    panic!("Scenario cannot be implemented using github-mock-api: cannot force 500 Internal Server Error");
  }
}
