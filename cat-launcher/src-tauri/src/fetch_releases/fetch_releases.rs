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

  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::fetch_releases::repository::ReleasesRepository;
  use crate::infra::endpoint::Endpoint;
  use crate::infra::github::release::GitHubRelease;
  use crate::variants::GameVariant;
  use chrono::Utc;
  use github_mock_api::{MockServer, Release};
  use crate::testing::support::db::TestDatabase;
  use crate::testing::support::rewire_client::RewireClient;

  fn create_rewire_client(mock_server_uri: &str) -> RewireClient {
    let mut redirects = HashMap::new();
    let mock_url = url::Url::parse(mock_server_uri).unwrap();
    let mock_host = mock_url.host_str().unwrap().to_string();
    let mock_port = mock_url.port();
    redirects.insert(
      Endpoint::new("api.github.com", None, "https"),
      Endpoint::new(mock_host, mock_port, "http"),
    );
    RewireClient::new(redirects)
  }

  #[tokio::test]
  async fn returns_cached_body_when_available() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let cached_release = GitHubRelease {
      id: 12345,
      tag_name: "v1.0.0".to_string(),
      prerelease: false,
      body: Some("Cached release notes".to_string()),
      assets: vec![],
      created_at: Utc::now(),
    };
    repo
      .update_cached_releases(
        &GameVariant::DarkDaysAhead,
        &[cached_release],
      )
      .await
      .expect("cache release");

    let mock_api =
      MockServer::start().await.expect("start mock server");
    let client = create_rewire_client(&mock_api.uri());

    let result = GameVariant::DarkDaysAhead
      .fetch_release_notes("v1.0.0", &client, &repo)
      .await
      .expect("fetch release notes");

    assert_eq!(result, Some("Cached release notes".to_string()));
  }

  #[tokio::test]
  async fn fetches_from_github_when_not_in_cache() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let mock_api =
      MockServer::start().await.expect("start mock server");
    let release =
      Release::new("CleverRaven", "Cataclysm-DDA", "v2.0.0")
        .body("Fresh release notes from GitHub");
    mock_api.add_release(release).await;

    let client = create_rewire_client(&mock_api.uri());

    let result = GameVariant::DarkDaysAhead
      .fetch_release_notes("v2.0.0", &client, &repo)
      .await
      .expect("fetch release notes");

    assert_eq!(
      result,
      Some("Fresh release notes from GitHub".to_string())
    );
  }

  #[tokio::test]
  async fn fetches_from_github_when_cached_but_body_is_none() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let cached_release = GitHubRelease {
      id: 12345,
      tag_name: "v3.0.0".to_string(),
      prerelease: false,
      body: None,
      assets: vec![],
      created_at: Utc::now(),
    };
    repo
      .update_cached_releases(
        &GameVariant::DarkDaysAhead,
        &[cached_release],
      )
      .await
      .expect("cache release");

    let mock_api =
      MockServer::start().await.expect("start mock server");
    let release =
      Release::new("CleverRaven", "Cataclysm-DDA", "v3.0.0")
        .body("Body was missing, fetched from GitHub");
    mock_api.add_release(release).await;

    let client = create_rewire_client(&mock_api.uri());

    let result = GameVariant::DarkDaysAhead
      .fetch_release_notes("v3.0.0", &client, &repo)
      .await
      .expect("fetch release notes");

    assert_eq!(
      result,
      Some("Body was missing, fetched from GitHub".to_string())
    );
  }

  #[tokio::test]
  async fn updates_cache_after_fetching_from_github() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let mock_api =
      MockServer::start().await.expect("start mock server");
    let release =
      Release::new("CleverRaven", "Cataclysm-DDA", "v4.0.0")
        .body("Release notes to be cached");
    mock_api.add_release(release).await;

    let client = create_rewire_client(&mock_api.uri());

    let _ = GameVariant::DarkDaysAhead
      .fetch_release_notes("v4.0.0", &client, &repo)
      .await
      .expect("fetch release notes");

    let cached: Option<GitHubRelease> = repo
      .get_cached_release_by_tag(&GameVariant::DarkDaysAhead, "v4.0.0")
      .await
      .expect("get cached release");

    assert!(cached.is_some());
    let cached = cached.unwrap();
    assert_eq!(cached.tag_name, "v4.0.0");
    assert_eq!(
      cached.body,
      Some("Release notes to be cached".to_string())
    );
  }

  #[tokio::test]
  async fn returns_error_when_github_returns_404() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let mock_api =
      MockServer::start().await.expect("start mock server");
    // Don't add any release for the nonexistent tag - the server will return 404

    let client = create_rewire_client(&mock_api.uri());

    let result = GameVariant::DarkDaysAhead
      .fetch_release_notes("nonexistent-tag", &client, &repo)
      .await;

    assert!(result.is_err());
  }

  #[tokio::test]
  async fn works_with_different_game_variants() {
    let db = TestDatabase::new();
    let repo = SqliteReleasesRepository::new(db.pool.clone());

    let mock_api =
      MockServer::start().await.expect("start mock server");

    let bn_release =
      Release::new("cataclysmbnteam", "Cataclysm-BN", "bn-v1.0")
        .body("Bright Nights notes");
    mock_api.add_release(bn_release).await;

    let tlg_release =
      Release::new("Cataclysm-TLG", "Cataclysm-TLG", "tlg-v1.0")
        .body("The Last Generation notes");
    mock_api.add_release(tlg_release).await;

    let client = create_rewire_client(&mock_api.uri());

    let bn_result = GameVariant::BrightNights
      .fetch_release_notes("bn-v1.0", &client, &repo)
      .await
      .expect("fetch BN release notes");

    let tlg_result = GameVariant::TheLastGeneration
      .fetch_release_notes("tlg-v1.0", &client, &repo)
      .await
      .expect("fetch TLG release notes");

    assert_eq!(bn_result, Some("Bright Nights notes".to_string()));
    assert_eq!(
      tlg_result,
      Some("The Last Generation notes".to_string())
    );
  }
}
