mod support;

use cat_launcher_lib::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use cat_launcher_lib::fetch_releases::repository::ReleasesRepository;
use cat_launcher_lib::infra::github::release::GitHubRelease;
use cat_launcher_lib::variants::GameVariant;
use chrono::Utc;
use support::db::TestDatabase;
use support::mock_github::{MockGitHubApi, github_not_found_response, github_release_response};

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

  let mock_api = MockGitHubApi::start().await;
  let client = mock_api.create_rewire_client();

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

  let mock_api = MockGitHubApi::start().await;
  let response = github_release_response(
    12345,
    "v2.0.0",
    Some("Fresh release notes from GitHub"),
    false,
  );
  let mock = mock_api.mock_release_by_tag(
    "CleverRaven/Cataclysm-DDA",
    "v2.0.0",
    response,
  );
  mock_api.register(mock).await;

  let client = mock_api.create_rewire_client();

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

  let mock_api = MockGitHubApi::start().await;
  let response = github_release_response(
    12345,
    "v3.0.0",
    Some("Body was missing, fetched from GitHub"),
    false,
  );
  let mock = mock_api.mock_release_by_tag(
    "CleverRaven/Cataclysm-DDA",
    "v3.0.0",
    response,
  );
  mock_api.register(mock).await;

  let client = mock_api.create_rewire_client();

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

  let mock_api = MockGitHubApi::start().await;
  let response = github_release_response(
    99999,
    "v4.0.0",
    Some("Release notes to be cached"),
    false,
  );
  let mock = mock_api.mock_release_by_tag(
    "CleverRaven/Cataclysm-DDA",
    "v4.0.0",
    response,
  );
  mock_api.register(mock).await;

  let client = mock_api.create_rewire_client();

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

  let mock_api = MockGitHubApi::start().await;
  let mock = mock_api.mock_release_by_tag(
    "CleverRaven/Cataclysm-DDA",
    "nonexistent-tag",
    github_not_found_response(),
  );
  mock_api.register(mock).await;

  let client = mock_api.create_rewire_client();

  let result = GameVariant::DarkDaysAhead
    .fetch_release_notes("nonexistent-tag", &client, &repo)
    .await;

  assert!(result.is_err());
}

#[tokio::test]
async fn works_with_different_game_variants() {
  let db = TestDatabase::new();
  let repo = SqliteReleasesRepository::new(db.pool.clone());

  let mock_api = MockGitHubApi::start().await;

  let bn_response = github_release_response(
    111,
    "bn-v1.0",
    Some("Bright Nights notes"),
    false,
  );
  let bn_mock = mock_api.mock_release_by_tag(
    "cataclysmbnteam/Cataclysm-BN",
    "bn-v1.0",
    bn_response,
  );
  mock_api.register(bn_mock).await;

  let tlg_response = github_release_response(
    222,
    "tlg-v1.0",
    Some("The Last Generation notes"),
    false,
  );
  let tlg_mock = mock_api.mock_release_by_tag(
    "Cataclysm-TLG/Cataclysm-TLG",
    "tlg-v1.0",
    tlg_response,
  );
  mock_api.register(tlg_mock).await;

  let client = mock_api.create_rewire_client();

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
