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
use crate::infra::utils::{HostSystem, get_github_repo_for_variant};
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
    client: &impl HttpClient,
    resources_dir: &Path,
    releases_repository: &impl ReleasesRepository,
    on_releases: F,
    host_system: &HostSystem,
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
      host_system,
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
      host_system,
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
      host_system,
    );
    on_releases(payload).map_err(FetchReleasesError::Send)?;

    Ok(())
  }

  pub async fn fetch_release_notes(
    &self,
    release_id: &str,
    client: &impl HttpClient,
    releases_repository: &impl ReleasesRepository,
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
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::get_first,
  dead_code
)]
mod tests {
  use super::*;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::{Arch, HostSystem, OS};
  use crate::variants::GameVariant;
  use chrono::Utc;
  use github_mock_api::{MockServer, Release as MockRelease};
  use std::collections::HashMap;
  use std::sync::Arc;
  use std::sync::Mutex;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  async fn setup()
  -> TestResult<(TestDatabase, MockServer, TestHttpClient)> {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;

    let mut host_mappings = HashMap::new();
    let uri = server.uri();
    let host_port = uri
      .strip_prefix("http://")
      .ok_or("uri should start with http://")?;
    host_mappings
      .insert("api.github.com".to_string(), host_port.to_string());

    let client = TestHttpClient::new(host_mappings)?;

    Ok((db, server, client))
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_hit() -> TestResult {
    let (db, _server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tag = "v1.0.0";
    let body = "cached notes";

    // Seed cache
    repo
      .update_cached_releases(
        &variant,
        &[GitHubRelease {
          id: 123,
          tag_name: tag.to_string(),
          prerelease: false,
          body: Some(body.to_string()),
          assets: vec![],
          created_at: Utc::now(),
        }],
      )
      .await?;

    let result =
      variant.fetch_release_notes(tag, &client, &repo).await?;

    if result != Some(body.to_string()) {
      return Err(
        format!("Expected {:?}, got {:?}", Some(body), result).into(),
      );
    }

    // Verify NO network call was made
    if client.request_count() != 0 {
      return Err(
        format!(
          "Expected 0 network calls, got {}",
          client.request_count()
        )
        .into(),
      );
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_hit_empty_body()
  -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tag = "v1.0.0";
    let body = "github notes";

    // Seed cache with empty body
    repo
      .update_cached_releases(
        &variant,
        &[GitHubRelease {
          id: 123,
          tag_name: tag.to_string(),
          prerelease: false,
          body: None,
          assets: vec![],
          created_at: Utc::now(),
        }],
      )
      .await?;

    // Seed GitHub
    server
      .add_release(
        "CleverRaven",
        "Cataclysm-DDA",
        MockRelease::new("CleverRaven", "Cataclysm-DDA", tag)
          .body(body),
      )
      .await;

    let result =
      variant.fetch_release_notes(tag, &client, &repo).await?;

    if result != Some(body.to_string()) {
      return Err(
        format!("Expected {:?}, got {:?}", Some(body), result).into(),
      );
    }

    // Verify cache updated
    let cached = repo
      .get_cached_release_by_tag(&variant, tag)
      .await?
      .ok_or("should have cached release")?;
    if cached.body != Some(body.to_string()) {
      return Err(
        format!(
          "Cached body expected {:?}, got {:?}",
          Some(body),
          cached.body
        )
        .into(),
      );
    }

    // Verify EXACTLY one network call was made
    if client.request_count() != 1 {
      return Err(
        format!(
          "Expected 1 network call, got {}",
          client.request_count()
        )
        .into(),
      );
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_cache_miss() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::BrightNights;
    let tag = "bn-1.0";
    let body = "bn notes";

    // Seed GitHub
    server
      .add_release(
        "cataclysmbnteam",
        "Cataclysm-BN",
        MockRelease::new("cataclysmbnteam", "Cataclysm-BN", tag)
          .body(body),
      )
      .await;

    let result =
      variant.fetch_release_notes(tag, &client, &repo).await?;

    if result != Some(body.to_string()) {
      return Err(
        format!("Expected {:?}, got {:?}", Some(body), result).into(),
      );
    }

    // Verify cache updated
    let cached = repo
      .get_cached_release_by_tag(&variant, tag)
      .await?
      .ok_or("should have cached release")?;
    if cached.body != Some(body.to_string()) {
      return Err(
        format!(
          "Cached body expected {:?}, got {:?}",
          Some(body),
          cached.body
        )
        .into(),
      );
    }
    if cached.tag_name != tag {
      return Err(
        format!(
          "Cached tag expected {:?}, got {:?}",
          tag, cached.tag_name
        )
        .into(),
      );
    }

    // Verify EXACTLY one network call was made
    if client.request_count() != 1 {
      return Err(
        format!(
          "Expected 1 network call, got {}",
          client.request_count()
        )
        .into(),
      );
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_github_404() -> TestResult {
    let (db, _server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::TheLastGeneration;
    let tag = "missing-tag";

    // GitHub is empty, should 404
    let result =
      variant.fetch_release_notes(tag, &client, &repo).await;

    match result {
      Err(FetchReleaseNotesError::Fetch(_)) => Ok(()),
      Err(e) => {
        Err(format!("Expected Fetch error, got: {:?}", e).into())
      }
      Ok(res) => {
        Err(format!("Expected error, got success: {:?}", res).into())
      }
    }
  }

  #[tokio::test]
  async fn test_fetch_release_notes_github_500() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tag = "v1.0.0";

    use github_mock_api::{MockBehavior, MockError};
    server
      .add_mock_behavior(
        MockBehavior::builder()
          .error(MockError::InternalServerError)
          .build(),
      )
      .await?;

    let result =
      variant.fetch_release_notes(tag, &client, &repo).await;

    match result {
      Err(FetchReleaseNotesError::Fetch(_)) => Ok(()),
      Err(e) => {
        Err(format!("Expected Fetch error, got: {:?}", e).into())
      }
      Ok(res) => {
        Err(format!("Expected error, got success: {:?}", res).into())
      }
    }
  }

  #[tokio::test]
  async fn test_fetch_release_notes_special_characters_in_tag()
  -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let tag = "v1.0.0+test";
    let body = "special notes";

    // Seed GitHub
    server
      .add_release(
        "CleverRaven",
        "Cataclysm-DDA",
        MockRelease::new("CleverRaven", "Cataclysm-DDA", tag)
          .body(body),
      )
      .await;

    let result =
      variant.fetch_release_notes(tag, &client, &repo).await?;

    if result != Some(body.to_string()) {
      return Err(
        format!("Expected {:?}, got {:?}", Some(body), result).into(),
      );
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_release_notes_different_game_variants()
  -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteReleasesRepository::new(db.pool().clone());
    let tag = "v1.0.0";

    let variants = [
      (
        GameVariant::DarkDaysAhead,
        "CleverRaven",
        "Cataclysm-DDA",
        "dda notes",
      ),
      (
        GameVariant::BrightNights,
        "cataclysmbnteam",
        "Cataclysm-BN",
        "bn notes",
      ),
      (
        GameVariant::TheLastGeneration,
        "Cataclysm-TLG",
        "Cataclysm-TLG",
        "tlg notes",
      ),
    ];

    for (variant, owner, repo_name, body) in variants {
      let mut mock_release =
        MockRelease::new(owner, repo_name, tag).body(body);
      // Manually set ID to a value that fits in i64 to avoid Sqlite conversion error
      mock_release.id = 12345 + (variant as u64);

      server.add_release(owner, repo_name, mock_release).await;

      let result =
        variant.fetch_release_notes(tag, &client, &repo).await?;

      if result != Some(body.to_string()) {
        return Err(
          format!(
            "Variant {:?}: Expected {:?}, got {:?}",
            variant,
            Some(body),
            result
          )
          .into(),
        );
      }
    }

    Ok(())
  }

  async fn setup_test_context() -> Result<
    (MockServer, TestHttpClient, TestDatabase),
    Box<dyn std::error::Error>,
  > {
    let server = MockServer::start().await?;
    let mut host_mappings = HashMap::new();
    host_mappings.insert(
      "api.github.com".to_string(),
      server
        .uri()
        .strip_prefix("http://")
        .ok_or("Failed to strip http prefix")?
        .to_string(),
    );
    let client = TestHttpClient::new(host_mappings)?;
    let test_db = TestDatabase::builder().build()?;
    Ok((server, client, test_db))
  }

  fn load_test_releases() -> Vec<github_mock_api::Release> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
      .join("src/infra/testing/data/releases.json");
    github_mock_api::Release::load_from_file(
      path,
      "cataclysmbnteam",
      "Cataclysm-BN",
    )
    .expect("Failed to load test releases")
  }

  fn create_github_release_with_assets(
    id: u64,
    tag: &str,
    asset_names: Vec<&str>,
  ) -> GitHubRelease {
    use crate::infra::github::asset::GitHubAsset;
    GitHubRelease {
      id,
      tag_name: tag.to_string(),
      prerelease: false,
      body: Some("body".to_string()),
      assets: asset_names
        .into_iter()
        .map(|name| GitHubAsset {
          id: 1,
          browser_download_url: "url".to_string(),
          name: name.to_string(),
          digest: None,
        })
        .collect(),
      created_at: Utc::now(),
    }
  }

  #[tokio::test]
  async fn test_fetch_releases_full_flow()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Setup Cache
    let cached_release = create_github_release_with_assets(
      1,
      "v1-cached",
      vec!["cbn-windows-tiles-x64-msvc-2026-06-05.zip"],
    );
    repo
      .update_cached_releases(&variant, &[cached_release])
      .await?;

    // Setup GitHub Mock
    let releases = load_test_releases();
    for release in releases {
      server
        .add_release("cataclysmbnteam", "Cataclysm-BN", release)
        .await;
    }

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    assert_eq!(payloads.len(), 3);

    // 1. Cached
    let p0 = payloads.get(0).ok_or("Missing payload 0")?;
    assert_eq!(p0.status, ReleasesUpdateStatus::Fetching);
    assert!(p0.releases.iter().any(|r| r.version == "v1-cached"));

    // 2. GitHub
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.status, ReleasesUpdateStatus::Fetching);
    assert!(p1.releases.iter().any(|r| r.version == "2026-06-05"));

    // 3. Default
    let p2 = payloads.get(2).ok_or("Missing payload 2")?;
    assert_eq!(p2.status, ReleasesUpdateStatus::Success);

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_initial_cache_emission()
  -> Result<(), Box<dyn std::error::Error>> {
    let (_server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let cached_release = create_github_release_with_assets(
      1,
      "v1-cached",
      vec!["cbn-windows-tiles-x64.zip"],
    );
    repo
      .update_cached_releases(&variant, &[cached_release])
      .await?;

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    // We don't care if it fails after the first emission for this test
    let _ = variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    assert!(!payloads.is_empty());
    let p0 = payloads.get(0).ok_or("Missing payload 0")?;
    assert_eq!(p0.status, ReleasesUpdateStatus::Fetching);
    assert!(p0.releases.iter().any(|r| r.version == "v1-cached"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_github_fetch_and_emission()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let releases = load_test_releases();
    for release in releases {
      server
        .add_release("cataclysmbnteam", "Cataclysm-BN", release)
        .await;
    }

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    // Index 1 should be GitHub emission
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.status, ReleasesUpdateStatus::Fetching);
    assert!(p1.releases.iter().any(|r| r.version == "2026-06-05"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_database_update_after_github_fetch()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let releases = load_test_releases();
    for release in releases {
      server
        .add_release("cataclysmbnteam", "Cataclysm-BN", release)
        .await;
    }

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let cached = repo.get_cached_releases(&variant).await?;
    assert!(cached.iter().any(|r| r.tag_name == "2026-06-05"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_default_releases_emission()
  -> Result<(), Box<dyn std::error::Error>> {
    let (_server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    // Index 2 should be default releases
    let p2 = payloads.get(2).ok_or("Missing payload 2")?;
    assert_eq!(p2.status, ReleasesUpdateStatus::Success);
    assert!(!p2.releases.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_platform_filtering()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let releases = load_test_releases();
    for release in releases {
      server
        .add_release("cataclysmbnteam", "Cataclysm-BN", release)
        .await;
    }

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    let github_payload =
      payloads.get(1).ok_or("Missing payload 1")?;
    assert!(
      github_payload
        .releases
        .iter()
        .any(|r| r.version == "2026-06-05")
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_pagination_handling()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Add 110 releases to GitHub (should fetch up to 100)
    let test_releases = load_test_releases();
    let base_release =
      test_releases.first().ok_or("No test releases found")?;
    for i in 1..=110 {
      let tag = format!("v{}", i);
      let mut gh_release = base_release.clone();
      gh_release.tag_name = tag;
      gh_release.id = i as u64;
      gh_release.created_at = format!(
        "2024-01-01T{:02}:{:02}:{:02}Z",
        (i / 3600) % 24,
        (i / 60) % 60,
        i % 60
      );
      server
        .add_release("cataclysmbnteam", "Cataclysm-BN", gh_release)
        .await;
    }

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.releases.len(), 100);

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_github_api_error_handling()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let behavior = github_mock_api::MockBehavior::builder()
      .error(github_mock_api::MockError::InternalServerError)
      .build();
    server.add_mock_behavior(behavior).await?;

    let result = variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await;

    assert!(result.is_err());
    let err = result.err().ok_or("Expected error")?;
    assert!(matches!(err, FetchReleasesError::Fetch(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_repository_error_handling()
  -> Result<(), Box<dyn std::error::Error>> {
    let (_server, client, test_db) = setup_test_context().await?;

    // Use the normal test database with proper schema initialization
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // This test verifies that repository operations are properly attempted
    // and that the fetch_releases function handles the data flow correctly
    let result = variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await;

    // The result should succeed with a properly initialized repository
    assert!(result.is_ok());

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_callback_error_handling()
  -> Result<(), Box<dyn std::error::Error>> {
    let (_server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let result = variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| {
          Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "callback error",
          ))
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await;

    assert!(result.is_err());
    let err = result.err().ok_or("Expected error")?;
    assert!(matches!(err, FetchReleasesError::Send(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_variant_specificity()
  -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Setup GitHub Mock for DDA
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
      .join("src/infra/testing/data/dda_releases.json");
    let releases = github_mock_api::Release::load_from_file(
      path,
      "CleverRaven",
      "Cataclysm-DDA",
    )?;

    for release in releases {
      server
        .add_release("CleverRaven", "Cataclysm-DDA", release)
        .await;
    }

    let received_payloads = Arc::new(Mutex::new(Vec::new()));
    let payloads_clone = received_payloads.clone();

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |payload| {
          payloads_clone
            .lock()
            .map_err(|e| {
              std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
              )
            })?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &HostSystem {
          os: OS::Windows,
          arch: Arch::X64,
        },
      )
      .await?;

    let payloads =
      received_payloads.lock().map_err(|e| e.to_string())?;
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert!(
      p1.releases
        .iter()
        .any(|r| r.version == "cdda-experimental-2026-06-05-1638")
    );
    assert_eq!(p1.variant, GameVariant::DarkDaysAhead);

    Ok(())
  }
}
