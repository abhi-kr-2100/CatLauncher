mod support;

use std::collections::HashMap;

use cat_launcher_lib::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use cat_launcher_lib::fetch_releases::repository::ReleasesRepository;
use cat_launcher_lib::infra::endpoint::Endpoint;
use cat_launcher_lib::infra::github::release::GitHubRelease;
use cat_launcher_lib::variants::GameVariant;
use chrono::Utc;
use github_mock_api::{MockServer, Release};
use support::db::TestDatabase;
use support::rewire_client::RewireClient;

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
