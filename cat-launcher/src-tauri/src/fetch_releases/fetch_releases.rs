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
    use super::*;
    use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
    use crate::infra::testing::http_client::TestHttpClient;
    use crate::infra::testing::test_database::TestDatabase;
    use crate::infra::utils::{Arch, OS};
    use crate::variants::GameVariant;
    use github_mock_api::{MockServer, Release, Repository};
    use std::collections::{HashMap, VecDeque};
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_fetch_releases_happy_path() {
        // This test is expected to fail because the production code has a hardcoded URL for GitHub API.
        // I am documenting this scenario as requested.
        let mock_server = MockServer::start().await.unwrap();
        let mut host_mappings = HashMap::new();
        host_mappings.insert("api.github.com".to_string(), mock_server.uri().strip_prefix("http://").unwrap().to_string());
        let client = TestHttpClient::new(host_mappings);

        let test_db = TestDatabase::builder().build().unwrap();
        let repository = SqliteReleasesRepository::new(test_db.pool().clone());

        let resources_dir = tempdir().unwrap();
        let variant = GameVariant::DarkDaysAhead;

        let repo = Repository::new("CleverRaven", "Cataclysm-DDA");
        mock_server.add_repository(repo).await;

        let mut mock_release = Release::new("CleverRaven", "Cataclysm-DDA", "v1");
        mock_release.id = 123;
        mock_release.created_at = "2024-01-01T00:00:00Z".to_string();

        let asset_json = serde_json::json!({
            "id": 1,
            "name": "cataclysm-dda-tiles-x64.zip",
            "browser_download_url": "http://example.com/asset",
            "url": "http://example.com",
            "node_id": "node",
            "state": "uploaded",
            "content_type": "application/zip",
            "size": 100,
            "download_count": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        });
        mock_release.assets.push(serde_json::from_value(asset_json).unwrap());

        mock_server.add_release(mock_release).await;

        let payloads = Arc::new(Mutex::new(VecDeque::new()));
        let payloads_clone = payloads.clone();

        let result = variant
            .fetch_releases(
                &client,
                resources_dir.path(),
                &repository,
                |payload| {
                    payloads_clone.lock().unwrap().push_back(payload);
                    Ok::<(), std::io::Error>(())
                },
                &OS::Windows,
                &Arch::X64,
            )
            .await;

        assert!(result.is_ok(), "Result should be ok, but got: {:?}", result.err());

        let mut payloads = payloads.lock().unwrap();

        // 1. Cached releases (empty)
        let p1 = payloads.pop_front().unwrap();
        assert_eq!(p1.status, ReleasesUpdateStatus::Fetching);
        assert!(p1.releases.is_empty());

        // 2. GitHub releases
        let p2 = payloads.pop_front().unwrap();
        assert_eq!(p2.status, ReleasesUpdateStatus::Fetching);
        assert_eq!(p2.releases.len(), 1);
        assert_eq!(p2.releases[0].version, "v1");

        // 3. Default releases (empty because no file exists)
        let p3 = payloads.pop_front().unwrap();
        assert_eq!(p3.status, ReleasesUpdateStatus::Success);
        assert!(p3.releases.is_empty());

        // Verify cache
        let cached = repository.get_cached_releases(&variant).await.unwrap();
        assert_eq!(cached.len(), 1);
        assert_eq!(cached[0].tag_name, "v1");
    }

    #[tokio::test]
    async fn test_fetch_releases_platform_filtering() {
        // This test is expected to fail because the production code has a hardcoded URL for GitHub API.
        let mock_server = MockServer::start().await.unwrap();
        let mut host_mappings = HashMap::new();
        host_mappings.insert("api.github.com".to_string(), mock_server.uri().strip_prefix("http://").unwrap().to_string());
        let client = TestHttpClient::new(host_mappings);

        let test_db = TestDatabase::builder().build().unwrap();
        let repository = SqliteReleasesRepository::new(test_db.pool().clone());

        let resources_dir = tempdir().unwrap();
        let variant = GameVariant::DarkDaysAhead;

        let repo = Repository::new("CleverRaven", "Cataclysm-DDA");
        mock_server.add_repository(repo).await;

        let mut linux_release = Release::new("CleverRaven", "Cataclysm-DDA", "v-linux");
        linux_release.id = 1;
        linux_release.created_at = "2024-01-01T00:00:00Z".to_string();
        let linux_asset_json = serde_json::json!({
            "id": 11,
            "name": "cataclysm-dda-tiles-x64.tar.gz",
            "browser_download_url": "http://example.com/linux",
            "url": "http://example.com",
            "node_id": "node",
            "state": "uploaded",
            "content_type": "application/zip",
            "size": 100,
            "download_count": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        });
        linux_release.assets.push(serde_json::from_value(linux_asset_json).unwrap());

        let mut windows_release = Release::new("CleverRaven", "Cataclysm-DDA", "v-windows");
        windows_release.id = 2;
        windows_release.created_at = "2024-01-02T00:00:00Z".to_string();
        let windows_asset_json = serde_json::json!({
            "id": 22,
            "name": "cataclysm-dda-tiles-x64.zip",
            "browser_download_url": "http://example.com/windows",
            "url": "http://example.com",
            "node_id": "node",
            "state": "uploaded",
            "content_type": "application/zip",
            "size": 100,
            "download_count": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        });
        windows_release.assets.push(serde_json::from_value(windows_asset_json).unwrap());

        mock_server.add_release(linux_release).await;
        mock_server.add_release(windows_release).await;

        let payloads = Arc::new(Mutex::new(VecDeque::new()));
        let payloads_clone = payloads.clone();

        variant
            .fetch_releases(
                &client,
                resources_dir.path(),
                &repository,
                |payload| {
                    payloads_clone.lock().unwrap().push_back(payload);
                    Ok::<(), std::io::Error>(())
                },
                &OS::Linux,
                &Arch::X64,
            )
            .await
            .unwrap();

        let payloads = payloads.lock().unwrap();
        let p2 = payloads.get(1).unwrap();
        assert_eq!(p2.releases.len(), 1);
        assert_eq!(p2.releases[0].version, "v-linux");
    }

    #[tokio::test]
    async fn test_fetch_releases_github_error() {
        // This test is expected to fail because the production code has a hardcoded URL for GitHub API.
        let mock_server = MockServer::start().await.unwrap();
        let mut host_mappings = HashMap::new();
        host_mappings.insert("api.github.com".to_string(), mock_server.uri().strip_prefix("http://").unwrap().to_string());
        let client = TestHttpClient::new(host_mappings);

        let test_db = TestDatabase::builder().build().unwrap();
        let repository = SqliteReleasesRepository::new(test_db.pool().clone());

        let resources_dir = tempdir().unwrap();
        let variant = GameVariant::DarkDaysAhead;

        let result = variant
            .fetch_releases(
                &client,
                resources_dir.path(),
                &repository,
                |_| Ok::<(), std::io::Error>(()),
                &OS::Linux,
                &Arch::X64,
            )
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            FetchReleasesError::Fetch(_) => (),
            _ => panic!("Expected Fetch error"),
        }
    }

    #[tokio::test]
    async fn test_fetch_releases_repo_error() {
        struct MockReleasesRepository;

        #[async_trait::async_trait]
        impl ReleasesRepository for MockReleasesRepository {
            async fn get_cached_releases(&self, _variant: &GameVariant) -> Result<Vec<crate::infra::github::release::GitHubRelease>, ReleasesRepositoryError> {
                Err(ReleasesRepositoryError::Get(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "db error"))))
            }
            async fn get_cached_release_by_tag(&self, _variant: &GameVariant, _tag: &str) -> Result<Option<crate::infra::github::release::GitHubRelease>, ReleasesRepositoryError> {
                Ok(None)
            }
            async fn update_cached_releases(&self, _variant: &GameVariant, _releases: &[crate::infra::github::release::GitHubRelease]) -> Result<(), ReleasesRepositoryError> {
                Ok(())
            }
        }

        let mock_server = MockServer::start().await.unwrap();
        let mut host_mappings = HashMap::new();
        host_mappings.insert("api.github.com".to_string(), mock_server.uri().strip_prefix("http://").unwrap().to_string());
        let client = TestHttpClient::new(host_mappings);
        let repository = MockReleasesRepository;
        let resources_dir = tempdir().unwrap();
        let variant = GameVariant::DarkDaysAhead;

        let result = variant
            .fetch_releases(
                &client,
                resources_dir.path(),
                &repository,
                |_| Ok::<(), std::io::Error>(()),
                &OS::Linux,
                &Arch::X64,
            )
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            FetchReleasesError::Repository(_) => (),
            _ => panic!("Expected Repository error"),
        }
    }
}
