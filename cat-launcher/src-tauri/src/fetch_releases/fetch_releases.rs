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
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use tempfile::tempdir;

  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::infra::http_client::HttpClientError;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::{Arch, OS};
  use crate::variants::GameVariant;
  use github_mock_api::{MockServer, Release};
  use reqwest::Response;

  pub struct RemappedTestHttpClient {
    pub client: TestHttpClient,
  }

  impl RemappedTestHttpClient {
    pub fn new(
      mock_server_uri: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
      let mock_host = mock_server_uri
        .strip_prefix("http://")
        .ok_or_else(|| {
          std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "mock server URI missing http:// prefix",
          )
        })?
        .to_string();
      let mut host_mappings = HashMap::new();
      host_mappings.insert("api.github.com".to_string(), mock_host);
      let client = TestHttpClient::new(host_mappings);
      Ok(Self { client })
    }
  }

  #[async_trait::async_trait]
  impl HttpClient for RemappedTestHttpClient {
    async fn get(
      &self,
      url: &str,
    ) -> Result<Response, HttpClientError> {
      self.client.get(url).await
    }
  }

  pub struct GitHubTestClient {
    pub server: MockServer,
  }

  impl GitHubTestClient {
    pub async fn start()
    -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
      let server = MockServer::start().await?;
      Ok(Self { server })
    }

    pub async fn add_release(&self, release: Release) {
      self.server.add_release(release).await;
    }

    pub fn client(
      &self,
    ) -> Result<
      RemappedTestHttpClient,
      Box<dyn std::error::Error + Send + Sync>,
    > {
      RemappedTestHttpClient::new(&self.server.uri())
    }
  }

  #[tokio::test]
  async fn test_fetch_releases_success()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Start the GitHub mock server
    let github_test = GitHubTestClient::start().await?;

    // 2. Set up the release to return from mock server
    let mut release = Release::new(
      "cataclysmbnteam",
      "Cataclysm-BN",
      "cdda-experimental-2026-06-01-0100",
    );
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/cdda-experimental-2026-06-01-0100/cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz",
        "id": 1,
        "node_id": "mock_node_id_asset_1",
        "name": "cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz",
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    // 3. Set up the RemappedTestHttpClient with mappings
    let client = github_test.client()?;

    // 4. Set up the TestDatabase
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    // 5. Set up resources_dir with a default release
    let resources_dir = tempdir()?;
    let releases_dir_path = resources_dir.path().join("releases");
    std::fs::create_dir_all(&releases_dir_path)?;
    let default_releases_file_path = releases_dir_path
      .join(format!("{}.json", GameVariant::BrightNights.id()));

    let default_releases_json = r#"[
      {
        "id": 999,
        "tag_name": "default-v1.0.0",
        "prerelease": false,
        "body": "Default release notes",
        "created_at": "2026-06-01T00:00:00Z",
        "assets": [
          {
            "id": 999,
            "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/default-v1.0.0/cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz",
            "name": "cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz"
          }
        ]
      }
    ]"#;
    std::fs::write(
      &default_releases_file_path,
      default_releases_json,
    )?;

    // 6. Call fetch_releases and capture the updates
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = payloads.clone();

    let on_releases = move |payload: ReleasesUpdatePayload| {
      let mut lock = payloads_clone.lock().map_err(|e| {
        std::io::Error::new(
          std::io::ErrorKind::Other,
          format!("Mutex poisoned: {e}"),
        )
      })?;
      lock.push(payload);
      Ok::<(), std::io::Error>(())
    };

    let result = GameVariant::BrightNights
      .fetch_releases(
        &client,
        resources_dir.path(),
        &repo,
        on_releases,
        &OS::Linux,
        &Arch::X64,
      )
      .await;

    assert!(result.is_ok());

    // 7. Verify the emitted payloads
    let payloads_lock = payloads.lock().map_err(|e| {
      std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Mutex poisoned: {e}"),
      )
    })?;
    assert_eq!(payloads_lock.len(), 3);

    // Call 1: Fetching (cached - should be empty)
    assert_eq!(
      payloads_lock[0].status,
      ReleasesUpdateStatus::Fetching
    );
    assert_eq!(payloads_lock[0].releases.len(), 0);

    // Call 2: Fetching (fetched from GitHub)
    assert_eq!(
      payloads_lock[1].status,
      ReleasesUpdateStatus::Fetching
    );
    assert_eq!(payloads_lock[1].releases.len(), 1);
    assert_eq!(
      payloads_lock[1].releases[0].version,
      "cdda-experimental-2026-06-01-0100"
    );

    // Call 3: Success (default releases)
    assert_eq!(
      payloads_lock[2].status,
      ReleasesUpdateStatus::Success
    );
    assert_eq!(payloads_lock[2].releases.len(), 1);
    assert_eq!(
      payloads_lock[2].releases[0].version,
      "default-v1.0.0"
    );

    // 8. Verify the cache is updated in the DB
    let cached_in_db =
      repo.get_cached_releases(&GameVariant::BrightNights).await?;
    assert_eq!(cached_in_db.len(), 1);
    assert_eq!(
      cached_in_db[0].tag_name,
      "cdda-experimental-2026-06-01-0100"
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_network_failure()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Set up the RemappedTestHttpClient without active mock server to simulate connection refusal / host resolution error
    let client = RemappedTestHttpClient::new("http://127.0.0.1:1")?;

    // 2. Set up the TestDatabase and pre-populate the cache
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    use chrono::TimeZone;
    let pre_cached_release = crate::infra::github::release::GitHubRelease {
      id: 123,
      tag_name: "cached-tag".to_string(),
      prerelease: false,
      body: Some("cached body".to_string()),
      assets: vec![crate::infra::github::asset::GitHubAsset {
        id: 123,
        browser_download_url: "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/cached-tag/cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz".to_string(),
        name: "cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz".to_string(),
        digest: None,
      }],
      created_at: chrono::Utc
        .with_ymd_and_hms(2026, 6, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| {
          std::io::Error::new(std::io::ErrorKind::Other, "invalid date")
        })?,
    };
    repo
      .update_cached_releases(
        &GameVariant::BrightNights,
        &[pre_cached_release],
      )
      .await?;

    // 3. Set up resources_dir
    let resources_dir = tempdir()?;

    // 4. Call fetch_releases and capture the updates
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = payloads.clone();

    let on_releases = move |payload: ReleasesUpdatePayload| {
      let mut lock = payloads_clone.lock().map_err(|e| {
        std::io::Error::new(
          std::io::ErrorKind::Other,
          format!("Mutex poisoned: {e}"),
        )
      })?;
      lock.push(payload);
      Ok::<(), std::io::Error>(())
    };

    let result = GameVariant::BrightNights
      .fetch_releases(
        &client,
        resources_dir.path(),
        &repo,
        on_releases,
        &OS::Linux,
        &Arch::X64,
      )
      .await;

    // Expect an error!
    assert!(result.is_err());

    // Verify the callback was called exactly once with the cached releases
    let payloads_lock = payloads.lock().map_err(|e| {
      std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Mutex poisoned: {e}"),
      )
    })?;
    assert_eq!(payloads_lock.len(), 1);
    assert_eq!(
      payloads_lock[0].status,
      ReleasesUpdateStatus::Fetching
    );
    assert_eq!(payloads_lock[0].releases.len(), 1);
    assert_eq!(payloads_lock[0].releases[0].version, "cached-tag");

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cached_with_body()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    // Pre-populate cache with release containing body
    use chrono::TimeZone;
    let pre_cached_release =
      crate::infra::github::release::GitHubRelease {
        id: 123,
        tag_name: "tag-123".to_string(),
        prerelease: false,
        body: Some("Pre-cached notes".to_string()),
        assets: vec![],
        created_at: chrono::Utc
          .with_ymd_and_hms(2026, 6, 1, 0, 0, 0)
          .single()
          .ok_or_else(|| {
            std::io::Error::new(
              std::io::ErrorKind::Other,
              "invalid date",
            )
          })?,
      };
    repo
      .update_cached_releases(
        &GameVariant::BrightNights,
        &[pre_cached_release],
      )
      .await?;

    let client = RemappedTestHttpClient::new("http://127.0.0.1:1")?;

    // Call fetch_release_notes
    let notes = GameVariant::BrightNights
      .fetch_release_notes("tag-123", &client, &repo)
      .await?;

    assert_eq!(notes, Some("Pre-cached notes".to_string()));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_not_cached_fetch_from_github()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_test = GitHubTestClient::start().await?;

    let mut release =
      Release::new("cataclysmbnteam", "Cataclysm-BN", "tag-456")
        .body("GitHub notes");
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/tag-456/dummy.tar.gz",
        "id": 1,
        "node_id": "mock_node_id_asset_1",
        "name": "dummy.tar.gz",
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    let client = github_test.client()?;

    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    // Call fetch_release_notes
    let notes = GameVariant::BrightNights
      .fetch_release_notes("tag-456", &client, &repo)
      .await?;

    assert_eq!(notes, Some("GitHub notes".to_string()));

    // Verify cache is updated in the DB
    let cached_release = repo
      .get_cached_release_by_tag(
        &GameVariant::BrightNights,
        "tag-456",
      )
      .await?;
    let release_entry = cached_release.ok_or_else(|| {
      std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "expected cached release",
      )
    })?;
    assert_eq!(release_entry.body, Some("GitHub notes".to_string()));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cached_without_body_fetches_from_github()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_test = GitHubTestClient::start().await?;

    let mut release =
      Release::new("cataclysmbnteam", "Cataclysm-BN", "tag-789")
        .body("Fetched notes");
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/tag-789/dummy.tar.gz",
        "id": 1,
        "node_id": "mock_node_id_asset_1",
        "name": "dummy.tar.gz",
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    let client = github_test.client()?;

    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    // Pre-populate cache with release but WITHOUT body (body is None/Null)
    use chrono::TimeZone;
    let pre_cached_release =
      crate::infra::github::release::GitHubRelease {
        id: 789,
        tag_name: "tag-789".to_string(),
        prerelease: false,
        body: None,
        assets: vec![],
        created_at: chrono::Utc
          .with_ymd_and_hms(2026, 6, 1, 0, 0, 0)
          .single()
          .ok_or_else(|| {
            std::io::Error::new(
              std::io::ErrorKind::Other,
              "invalid date",
            )
          })?,
      };
    repo
      .update_cached_releases(
        &GameVariant::BrightNights,
        &[pre_cached_release],
      )
      .await?;

    // Call fetch_release_notes
    let notes = GameVariant::BrightNights
      .fetch_release_notes("tag-789", &client, &repo)
      .await?;

    assert_eq!(notes, Some("Fetched notes".to_string()));

    // Verify cache has been updated in the DB with the fetched body
    let cached_release = repo
      .get_cached_release_by_tag(
        &GameVariant::BrightNights,
        "tag-789",
      )
      .await?;
    let release_entry = cached_release.ok_or_else(|| {
      std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "expected cached release",
      )
    })?;
    assert_eq!(release_entry.body, Some("Fetched notes".to_string()));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_empty_body_cache_bypass()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_test = GitHubTestClient::start().await?;

    // 1. Create a release on GitHub mock server with NO body (body: None)
    let mut release =
      Release::new("cataclysmbnteam", "Cataclysm-BN", "tag-empty");
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/tag-empty/dummy.tar.gz",
        "id": 1,
        "node_id": "mock_node_id_asset_1",
        "name": "dummy.tar.gz",
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    let client = github_test.client()?;

    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    // 2. Fetch the release notes the first time. This should fetch from GitHub and return None.
    let notes = GameVariant::BrightNights
      .fetch_release_notes("tag-empty", &client, &repo)
      .await?;
    assert_eq!(notes, None);

    // 3. Now, we simulate a GitHub API failure for subsequent calls
    // by using a non-existent port (simulating that if it goes to GitHub, it will fail).
    let failing_client =
      RemappedTestHttpClient::new("http://127.0.0.1:1")?;

    // 4. Fetch the release notes again.
    // If the cache worked, it should find that the release was cached (even with None body)
    // and return Ok(None) immediately without requesting GitHub.
    // However, due to the cache bypass bug, it will try to hit the API, fail, and return an Error!
    let notes_second = GameVariant::BrightNights
      .fetch_release_notes("tag-empty", &failing_client, &repo)
      .await;

    // These assertions will FAIL due to the cache bypass bug in production:
    // The second call always goes to GitHub (ignoring the cached None-body entry),
    // hits the dead port, and returns an Err instead of Ok(None).
    assert!(
      notes_second.is_ok(),
      "Expected cache to hit and return Ok(None) without querying network, got: {:?}",
      notes_second.as_ref().err().map(|e| e.to_string()),
    );
    if let Ok(body) = notes_second {
      assert_eq!(body, None, "Expected None body from cache");
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_overwrites_cached_release_notes()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Set up a database and pre-populate the release notes cache with a populated body
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    use chrono::TimeZone;
    let pre_cached_release = crate::infra::github::release::GitHubRelease {
      id: 12345,
      tag_name: "cdda-experimental-2026-06-01-0100".to_string(),
      prerelease: false,
      body: Some("Important Release Notes".to_string()),
      assets: vec![crate::infra::github::asset::GitHubAsset {
        id: 12345,
        browser_download_url: "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/cdda-experimental-2026-06-01-0100/cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz".to_string(),
        name: "cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz".to_string(),
        digest: None,
      }],
      created_at: chrono::Utc
        .with_ymd_and_hms(2026, 6, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| {
          std::io::Error::new(std::io::ErrorKind::Other, "invalid date")
        })?,
    };
    repo
      .update_cached_releases(
        &GameVariant::BrightNights,
        &[pre_cached_release],
      )
      .await?;

    // Verify it is cached with the notes
    let cached = repo
      .get_cached_release_by_tag(
        &GameVariant::BrightNights,
        "cdda-experimental-2026-06-01-0100",
      )
      .await?
      .ok_or_else(|| {
        std::io::Error::new(
          std::io::ErrorKind::NotFound,
          "expected cached release",
        )
      })?;
    assert_eq!(
      cached.body,
      Some("Important Release Notes".to_string())
    );

    // 2. Start GitHub mock server returning that same release but with NO body (body: None)
    let github_test = GitHubTestClient::start().await?;
    let mut release = Release::new(
      "cataclysmbnteam",
      "Cataclysm-BN",
      "cdda-experimental-2026-06-01-0100",
    );
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/cdda-experimental-2026-06-01-0100/cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz",
        "id": 12345,
        "node_id": "mock_node_id_asset_1",
        "name": "cdda-experimental-2026-06-01-0100-linux-tiles.tar.gz",
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    let client = github_test.client()?;
    let resources_dir = tempdir()?;

    // 3. Call fetch_releases. This will fetch from GitHub (with body: None) and update the cache.
    let result = GameVariant::BrightNights
      .fetch_releases(
        &client,
        resources_dir.path(),
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &OS::Linux,
        &Arch::X64,
      )
      .await;
    assert!(result.is_ok());

    // 4. Get the cached release again from the DB
    let cached_after = repo
      .get_cached_release_by_tag(
        &GameVariant::BrightNights,
        "cdda-experimental-2026-06-01-0100",
      )
      .await?
      .ok_or_else(|| {
        std::io::Error::new(
          std::io::ErrorKind::NotFound,
          "expected cached release",
        )
      })?;

    // This assertion will FAIL due to the overwriting bug in production:
    assert_eq!(
      cached_after.body,
      Some("Important Release Notes".to_string()),
      "Expected cached release notes to be preserved"
    );

    Ok(())
  }

  /// When the `on_releases` callback returns an `Err`, `fetch_releases` must
  /// propagate that error and stop without calling the callback further.
  #[tokio::test]
  async fn test_fetch_releases_callback_error_propagates()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_test = GitHubTestClient::start().await?;
    // Pre-populate the DB cache so the very first on_releases call has data,
    // and we can make it fail deterministically on call 1.
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    use chrono::TimeZone;
    let pre_cached = crate::infra::github::release::GitHubRelease {
      id: 1,
      tag_name: "v1.0.0".to_string(),
      prerelease: false,
      body: None,
      assets: vec![crate::infra::github::asset::GitHubAsset {
        id: 1,
        browser_download_url:
          "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/v1.0.0/linux-tiles.tar.gz"
            .to_string(),
        name: "linux-tiles.tar.gz".to_string(),
        digest: None,
      }],
      created_at: chrono::Utc
        .with_ymd_and_hms(2026, 6, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| {
          std::io::Error::new(std::io::ErrorKind::Other, "invalid date")
        })?,
    };
    repo
      .update_cached_releases(
        &GameVariant::BrightNights,
        &[pre_cached],
      )
      .await?;

    // Callback always fails immediately.
    let call_count = Arc::new(Mutex::new(0u32));
    let call_count_clone = call_count.clone();
    let on_releases = move |_payload: ReleasesUpdatePayload| {
      let mut count = call_count_clone.lock().map_err(|e| {
        std::io::Error::new(
          std::io::ErrorKind::Other,
          format!("Mutex poisoned: {e}"),
        )
      })?;
      *count += 1;
      Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "intentional callback failure",
      ))
    };

    let client = github_test.client()?;
    let resources_dir = tempdir()?;

    let result = GameVariant::BrightNights
      .fetch_releases(
        &client,
        resources_dir.path(),
        &repo,
        on_releases,
        &OS::Linux,
        &Arch::X64,
      )
      .await;

    // The function must surface the callback error.
    assert!(
      result.is_err(),
      "Expected fetch_releases to propagate the callback error"
    );

    // The callback must have been invoked exactly once (on the cached-releases
    // emit) and then fetch_releases must have stopped immediately.
    let count = call_count.lock().map_err(|e| {
      std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Mutex poisoned: {e}"),
      )
    })?;
    assert_eq!(
      *count, 1,
      "Expected callback to be called exactly once before stopping"
    );

    Ok(())
  }

  /// `fetch_release_notes` for a tag that is not in the DB and not on GitHub
  /// should return an error (GitHub 404). It must not silently return `None`.
  #[tokio::test]
  async fn test_fetch_release_notes_unknown_tag_returns_error()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Empty mock server – "nonexistent-tag" will get a 404.
    let github_test = GitHubTestClient::start().await?;
    let client = github_test.client()?;

    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());

    let result = GameVariant::BrightNights
      .fetch_release_notes("nonexistent-tag", &client, &repo)
      .await;

    assert!(
      result.is_err(),
      "Expected an error for an unknown tag, got: {:?}",
      result.as_ref().ok()
    );

    Ok(())
  }

  /// Releases fetched from GitHub that have no installable asset for the
  /// current platform are filtered out of the payload by `get_releases_payload`.
  /// This test verifies the filter works and that no release leaks through.
  #[tokio::test]
  async fn test_fetch_releases_filters_non_installable()
  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_test = GitHubTestClient::start().await?;

    // Release whose only asset has a name that does not match any Linux/X64 pattern.
    let mut release = Release::new(
      "cataclysmbnteam",
      "Cataclysm-BN",
      "cdda-experimental-2026-06-01-0100",
    );
    let assets_json = r#"[
      {
        "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
        "browser_download_url": "https://github.com/cataclysmbnteam/Cataclysm-BN/releases/download/cdda-experimental-2026-06-01-0100/cdda-experimental-windows-only.zip",
        "id": 1,
        "node_id": "node1",
        "name": "cdda-experimental-windows-only.zip",
        "state": "uploaded",
        "content_type": "application/zip",
        "size": 100,
        "download_count": 0,
        "created_at": "2026-06-01T00:00:00Z",
        "updated_at": "2026-06-01T00:00:00Z"
      }
    ]"#;
    release.assets = serde_json::from_str(assets_json)?;
    github_test.add_release(release).await;

    let client = github_test.client()?;
    let db = TestDatabase::builder().build()?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let resources_dir = tempdir()?;

    let payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = payloads.clone();

    let on_releases = move |payload: ReleasesUpdatePayload| {
      let mut lock = payloads_clone.lock().map_err(|e| {
        std::io::Error::new(
          std::io::ErrorKind::Other,
          format!("Mutex poisoned: {e}"),
        )
      })?;
      lock.push(payload);
      Ok::<(), std::io::Error>(())
    };

    GameVariant::BrightNights
      .fetch_releases(
        &client,
        resources_dir.path(),
        &repo,
        on_releases,
        &OS::Linux,
        &Arch::X64,
      )
      .await
      .map_err(|e| e.to_string())?;

    let lock = payloads.lock().map_err(|e| {
      std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Mutex poisoned: {e}"),
      )
    })?;

    // The GitHub payload (index 1) should have 0 installable releases because
    // the asset name doesn't match the Linux X64 platform strings.
    assert_eq!(lock.len(), 3);
    assert_eq!(
      lock[1].releases.len(),
      0,
      "Non-installable release should be filtered from the payload"
    );

    Ok(())
  }
}
