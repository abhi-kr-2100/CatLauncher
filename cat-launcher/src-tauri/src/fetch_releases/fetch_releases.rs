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
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::{Arch, OS};
  use crate::variants::GameVariant;
  use chrono::Utc;
  use github_mock_api::MockServer;
  use std::collections::HashMap;
  use std::sync::Arc;
  use std::sync::Mutex;

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

  fn create_mock_github_release(id: u64, tag: &str) -> github_mock_api::Release {
    let mut release = github_mock_api::Release::new("cataclysmbnteam", "Cataclysm-BN", tag)
      .created_at("2024-01-01T00:00:00Z");
    release.id = id;
    release
  }

  fn create_mock_github_asset(name: &str) -> serde_json::Value {
    serde_json::json!({
      "url": "https://api.github.com/repos/cataclysmbnteam/Cataclysm-BN/releases/assets/1",
      "browser_download_url": "https://example.com/download",
      "id": 1,
      "node_id": "mock_node_id_1",
      "name": name,
      "state": "uploaded",
      "content_type": "application/zip",
      "size": 1024,
      "download_count": 0,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z",
      "uploader": {
        "login": "cataclysmbnteam",
        "id": 1,
        "node_id": "mock_node_id_1",
        "avatar_url": "https://avatars.githubusercontent.com/u/1?v=4",
        "html_url": "https://github.com/cataclysmbnteam",
        "followers_url": "https://api.github.com/users/cataclysmbnteam/followers",
        "following_url": "https://api.github.com/users/cataclysmbnteam/following{/other_user}",
        "gists_url": "https://api.github.com/users/cataclysmbnteam/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/cataclysmbnteam/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/cataclysmbnteam/subscriptions",
        "organizations_url": "https://api.github.com/users/cataclysmbnteam/orgs",
        "repos_url": "https://api.github.com/users/cataclysmbnteam/repos",
        "events_url": "https://api.github.com/users/cataclysmbnteam/events{/privacy}",
        "received_events_url": "https://api.github.com/users/cataclysmbnteam/received_events",
        "type": "User",
        "site_admin": false
      }
    })
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
  async fn test_fetch_releases_full_flow() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Setup Cache
    let cached_release = create_github_release_with_assets(
      1,
      "v1-cached",
      vec!["cbn-windows-tiles-x64.zip"],
    );
    repo
      .update_cached_releases(&variant, &[cached_release])
      .await?;

    // Setup GitHub Mock
    let mut gh_release = create_mock_github_release(2, "v2-github");
    let asset_json = create_mock_github_asset("cbn-windows-tiles-x64.zip");
    let asset = serde_json::from_value(asset_json)?;
    gh_release.assets.push(asset);
    server
      .add_release("cataclysmbnteam", "Cataclysm-BN", gh_release)
      .await;

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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    assert_eq!(payloads.len(), 3);

    // 1. Cached
    let p0 = payloads.get(0).ok_or("Missing payload 0")?;
    assert_eq!(p0.status, ReleasesUpdateStatus::Fetching);
    assert!(p0.releases.iter().any(|r| r.version == "v1-cached"));

    // 2. GitHub
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.status, ReleasesUpdateStatus::Fetching);
    assert!(p1.releases.iter().any(|r| r.version == "v2-github"));

    // 3. Default
    let p2 = payloads.get(2).ok_or("Missing payload 2")?;
    assert_eq!(p2.status, ReleasesUpdateStatus::Success);

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_initial_cache_emission() -> Result<(), Box<dyn std::error::Error>> {
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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    assert!(!payloads.is_empty());
    let p0 = payloads.get(0).ok_or("Missing payload 0")?;
    assert_eq!(p0.status, ReleasesUpdateStatus::Fetching);
    assert!(p0.releases.iter().any(|r| r.version == "v1-cached"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_github_fetch_and_emission() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut gh_release = create_mock_github_release(2, "v2-github");
    let asset_json = create_mock_github_asset("cbn-windows-tiles-x64.zip");
    let asset = serde_json::from_value(asset_json)?;
    gh_release.assets.push(asset);
    server
      .add_release("cataclysmbnteam", "Cataclysm-BN", gh_release)
      .await;

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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    // Index 1 should be GitHub emission
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.status, ReleasesUpdateStatus::Fetching);
    assert!(p1.releases.iter().any(|r| r.version == "v2-github"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_database_update_after_github_fetch() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut gh_release = create_mock_github_release(2, "v2-github");
    let asset_json = create_mock_github_asset("cbn-windows-tiles-x64.zip");
    let asset = serde_json::from_value(asset_json)?;
    gh_release.assets.push(asset);
    server
      .add_release("cataclysmbnteam", "Cataclysm-BN", gh_release)
      .await;

    variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let cached = repo.get_cached_releases(&variant).await?;
    assert!(cached.iter().any(|r| r.tag_name == "v2-github"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_default_releases_emission() -> Result<(), Box<dyn std::error::Error>> {
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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    // Index 2 should be default releases
    let p2 = payloads.get(2).ok_or("Missing payload 2")?;
    assert_eq!(p2.status, ReleasesUpdateStatus::Success);
    assert!(!p2.releases.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_platform_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Release for Windows
    let mut win_release = create_mock_github_release(2, "v-win");
    let win_asset_json = create_mock_github_asset("cbn-windows-tiles-x64.zip");
    let win_asset = serde_json::from_value(win_asset_json)?;
    win_release.assets.push(win_asset);
    server
      .add_release("cataclysmbnteam", "Cataclysm-BN", win_release)
      .await;

    // Release for Linux
    let mut lin_release = create_mock_github_release(3, "v-lin");
    let lin_asset_json = create_mock_github_asset("cbn-linux-tiles-x64.tar.gz");
    let lin_asset = serde_json::from_value(lin_asset_json)?;
    lin_release.assets.push(lin_asset);
    server
      .add_release("cataclysmbnteam", "Cataclysm-BN", lin_release)
      .await;

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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    let github_payload = payloads.get(1).ok_or("Missing payload 1")?;
    assert!(github_payload.releases.iter().any(|r| r.version == "v-win"));
    assert!(!github_payload.releases.iter().any(|r| r.version == "v-lin"));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_pagination_handling() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Add 110 releases to GitHub (should fetch up to 100)
    for i in 1..=110 {
      let tag = format!("v{}", i);
      let mut gh_release = create_mock_github_release(i as u64, &tag);
      gh_release.created_at =
        format!("2024-01-01T{:02}:{:02}:{:02}Z", (i / 3600) % 24, (i / 60) % 60, i % 60);
      let asset_json = create_mock_github_asset("cbn-windows-tiles-x64.zip");
      let asset = serde_json::from_value(asset_json)?;
      gh_release.assets.push(asset);
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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert_eq!(p1.releases.len(), 100);

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_github_api_error_handling() -> Result<(), Box<dyn std::error::Error>> {
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
        &OS::Windows,
        &Arch::X64,
      )
      .await;

    assert!(result.is_err());
    let err = result.err().ok_or("Expected error")?;
    assert!(matches!(err, FetchReleasesError::Fetch(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_repository_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let (_server, client, _test_db) = setup_test_context().await?;

    // Create a database without the required schema to trigger a repository error
    let test_db_no_schema = TestDatabase::builder()
      .with_schema_initializer(|_, _| Ok(()))
      .build()?;

    let repo = SqliteReleasesRepository::new(test_db_no_schema.pool().clone());
    let variant = GameVariant::BrightNights;
    let resources_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let result = variant
      .fetch_releases(
        &client,
        &resources_dir,
        &repo,
        |_| Ok::<(), std::io::Error>(()),
        &OS::Windows,
        &Arch::X64,
      )
      .await;

    assert!(result.is_err());
    let err = result.err().ok_or("Expected error")?;
    assert!(matches!(err, FetchReleasesError::Repository(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_callback_error_handling() -> Result<(), Box<dyn std::error::Error>> {
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
        &OS::Windows,
        &Arch::X64,
      )
      .await;

    assert!(result.is_err());
    let err = result.err().ok_or("Expected error")?;
    assert!(matches!(err, FetchReleasesError::Send(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_fetch_releases_variant_specificity() -> Result<(), Box<dyn std::error::Error>> {
    let (server, client, test_db) = setup_test_context().await?;
    let repo = SqliteReleasesRepository::new(test_db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let resources_dir =
      std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Setup GitHub Mock for DDA
    let mut gh_release = github_mock_api::Release::new("CleverRaven", "Cataclysm-DDA", "v-dda");
    // Ensure ID is not too large for i64 if needed, though 187090088 fits.
    // Let's use a safe small ID for mock.
    gh_release.id = 12345;
    let asset_json = create_mock_github_asset("windows-with-graphics-and-sounds");
    let asset = serde_json::from_value(asset_json)?;
    gh_release.assets.push(asset);
    server
      .add_release("CleverRaven", "Cataclysm-DDA", gh_release)
      .await;

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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .push(payload);
          Ok::<(), std::io::Error>(())
        },
        &OS::Windows,
        &Arch::X64,
      )
      .await?;

    let payloads = received_payloads
      .lock()
      .map_err(|e| e.to_string())?;
    let p1 = payloads.get(1).ok_or("Missing payload 1")?;
    assert!(p1.releases.iter().any(|r| r.version == "v-dda"));
    assert_eq!(p1.variant, GameVariant::DarkDaysAhead);

    Ok(())
  }
}
