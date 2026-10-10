use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

use crate::infra::github::get_last_commit::{
  GetLastCommitError, get_last_commit,
};
use crate::infra::http_client::HttpClient;
use crate::mods::get_third_party_mod_by_id::{
  GetThirdPartyModByIdError, get_third_party_mod_by_id,
};
use crate::mods::repository::mods_repository::ModsRepository;
use crate::mods::types::ModActivity;
use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LastModActivity {
  pub timestamp: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum GetLastActivityForThirdPartyModError {
  #[error("failed to get last commit: {0}")]
  GetLastCommit(#[from] GetLastCommitError),

  #[error("failed to get mod: {0}")]
  GetMod(#[from] GetThirdPartyModByIdError),

  #[error("mod has no github activity configured")]
  NoGithubActivity,
}

pub fn extract_repo_from_github_url(url_str: &str) -> Option<String> {
  let parsed_url = Url::parse(url_str).ok()?;

  let path = parsed_url.path();
  let parts: Vec<&str> = path
    .trim_start_matches('/')
    .trim_end_matches('/')
    .trim_end_matches(".git")
    .split('/')
    .collect();

  let owner = parts.first()?;
  let repo = parts.get(1)?;

  if !owner.is_empty() && !repo.is_empty() {
    Some(format!("{}/{}", owner, repo))
  } else {
    None
  }
}

pub async fn get_last_activity_for_third_party_mod(
  mod_id: &str,
  variant: &GameVariant,
  client: &impl HttpClient,
  mods_repository: &impl ModsRepository,
) -> Result<LastModActivity, GetLastActivityForThirdPartyModError> {
  let mod_data =
    get_third_party_mod_by_id(mod_id, variant, mods_repository)
      .await?;

  match &mod_data.activity {
    Some(ModActivity::GithubCommit { github }) => {
      let repo = extract_repo_from_github_url(github).ok_or(
        GetLastActivityForThirdPartyModError::NoGithubActivity,
      )?;

      let last_commit = get_last_commit(&repo, client).await?;
      let last_commit_date = last_commit.commit.author.date;
      let timestamp = last_commit_date.timestamp_millis();

      Ok(LastModActivity { timestamp })
    }
    None => {
      Err(GetLastActivityForThirdPartyModError::NoGithubActivity)
    }
  }
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use std::collections::HashMap;

  use chrono::DateTime;
  use github_mock_api::{
    Commit, MockBehavior, MockError, MockServer,
  };

  use super::*;
  use crate::infra::github::get_last_commit::GetLastCommitError;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::get_third_party_mod_by_id::GetThirdPartyModByIdError;
  use crate::mods::repository::mods_repository::GetThirdPartyModByIdError as RepoError;
  use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
  use crate::mods::types::{ModInstallation, ThirdPartyMod};

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

  fn create_test_mod(
    id: &str,
    github_url: Option<&str>,
  ) -> ThirdPartyMod {
    ThirdPartyMod {
      id: id.to_string(),
      name: format!("Test Mod {id}"),
      description: format!("Description for {id}"),
      category: "Vehicles".to_string(),
      installation: ModInstallation {
        download_url: "https://example.com/mod.zip".to_string(),
        modinfo: "modinfo.json".to_string(),
      },
      activity: github_url.map(|url| ModActivity::GithubCommit {
        github: url.to_string(),
      }),
    }
  }

  fn create_commit_with_date(
    owner: &str,
    repo: &str,
    rfc3339_date: &str,
  ) -> Commit {
    let mut commit = Commit::new(owner, repo);
    if let Some(ref mut author) = commit.commit.author {
      author.date = rfc3339_date.to_string();
    }
    commit
  }

  #[tokio::test]
  async fn test_get_last_activity_success() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "magiclysm";
    let owner = "CleverRaven";
    let repo_name = "Magiclysm";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    server
      .add_commit(owner, repo_name, Commit::new(owner, repo_name))
      .await;

    let activity = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await?;

    // Default timestamp in MockServer Commit::new is 2024-01-01T00:00:00Z (1704067200000 ms)
    assert_eq!(activity.timestamp, 1704067200000);
    assert_eq!(client.request_count(), 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_with_custom_timestamp() -> TestResult
  {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "custom-time-mod";
    let owner = "modder";
    let repo_name = "custom-mod";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let custom_date = "2025-06-15T14:30:00Z";
    let expected_timestamp =
      DateTime::parse_from_rfc3339(custom_date)?.timestamp_millis();

    server
      .add_commit(
        owner,
        repo_name,
        create_commit_with_date(owner, repo_name, custom_date),
      )
      .await;

    let activity = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await?;

    assert_eq!(activity.timestamp, expected_timestamp);
    assert_eq!(client.request_count(), 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_multiple_commits_returns_first()
  -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "multi-commit-mod";
    let owner = "team";
    let repo_name = "multi-commit-repo";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let latest_date = "2025-09-01T10:00:00Z";
    let older_date = "2025-01-01T10:00:00Z";
    let expected_timestamp =
      DateTime::parse_from_rfc3339(latest_date)?.timestamp_millis();

    server
      .add_commit(
        owner,
        repo_name,
        create_commit_with_date(owner, repo_name, latest_date),
      )
      .await;
    server
      .add_commit(
        owner,
        repo_name,
        create_commit_with_date(owner, repo_name, older_date),
      )
      .await;

    let activity = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await?;

    assert_eq!(activity.timestamp, expected_timestamp);
    assert_eq!(client.request_count(), 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_url_variations() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let owner = "url-owner";
    let repo_name = "url-repo";

    server
      .add_commit(owner, repo_name, Commit::new(owner, repo_name))
      .await;

    let url_variations = [
      (
        "mod-standard",
        format!("https://github.com/{owner}/{repo_name}"),
      ),
      (
        "mod-git",
        format!("https://github.com/{owner}/{repo_name}.git"),
      ),
      (
        "mod-slash",
        format!("https://github.com/{owner}/{repo_name}/"),
      ),
      (
        "mod-git-slash",
        format!("https://github.com/{owner}/{repo_name}.git/"),
      ),
      (
        "mod-subpath",
        format!("https://github.com/{owner}/{repo_name}/tree/main"),
      ),
      ("mod-http", format!("http://github.com/{owner}/{repo_name}")),
    ];

    for (mod_id, url) in &url_variations {
      let test_mod = create_test_mod(mod_id, Some(url));
      repo.save_third_party_mods(&variant, vec![test_mod]).await?;

      let activity = get_last_activity_for_third_party_mod(
        mod_id, &variant, &client, &repo,
      )
      .await?;

      assert_eq!(activity.timestamp, 1704067200000);
    }

    assert_eq!(client.request_count(), url_variations.len());

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_all_game_variants() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let owner = "variant-owner";
    let repo_name = "variant-repo";

    server
      .add_commit(owner, repo_name, Commit::new(owner, repo_name))
      .await;

    let variants = [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ];

    for variant in &variants {
      let mod_id = format!("mod-for-{}", variant);
      let github_url =
        format!("https://github.com/{owner}/{repo_name}");
      let test_mod = create_test_mod(&mod_id, Some(&github_url));
      repo.save_third_party_mods(variant, vec![test_mod]).await?;

      let activity = get_last_activity_for_third_party_mod(
        &mod_id, variant, &client, &repo,
      )
      .await?;

      assert_eq!(activity.timestamp, 1704067200000);
    }

    assert_eq!(client.request_count(), variants.len());

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_mod_not_found() -> TestResult {
    let (db, _server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result = get_last_activity_for_third_party_mod(
      "non-existent-mod",
      &variant,
      &client,
      &repo,
    )
    .await;

    match result {
      Err(GetLastActivityForThirdPartyModError::GetMod(
        GetThirdPartyModByIdError::Repository(RepoError::NotFound(
          id,
          var_name,
        )),
      )) => {
        assert_eq!(id, "non-existent-mod");
        assert_eq!(var_name, variant.to_string());
      }
      other => {
        return Err(
          format!("Expected mod NotFound error, got: {:?}", other)
            .into(),
        );
      }
    }

    assert_eq!(client.request_count(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_variant_isolation() -> TestResult {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let mod_id = "dda-only-mod";
    let owner = "iso-owner";
    let repo_name = "iso-repo";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo
      .save_third_party_mods(
        &GameVariant::DarkDaysAhead,
        vec![test_mod],
      )
      .await?;

    server
      .add_commit(owner, repo_name, Commit::new(owner, repo_name))
      .await;

    // Query for BrightNights when mod was only added for DarkDaysAhead
    let result = get_last_activity_for_third_party_mod(
      mod_id,
      &GameVariant::BrightNights,
      &client,
      &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(GetLastActivityForThirdPartyModError::GetMod(
        GetThirdPartyModByIdError::Repository(RepoError::NotFound(
          _,
          _
        ))
      ))
    ));
    assert_eq!(client.request_count(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_no_activity_configured()
  -> TestResult {
    let (db, _server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "no-activity-mod";

    let test_mod = create_test_mod(mod_id, None);
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let result = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(GetLastActivityForThirdPartyModError::NoGithubActivity)
    ));
    assert_eq!(client.request_count(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_unparseable_github_urls()
  -> TestResult {
    let (db, _server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let invalid_urls = [
      ("mod-only-owner", "https://github.com/only-owner"),
      ("mod-empty-path", "https://github.com/"),
      ("mod-no-path", "https://github.com"),
      ("mod-not-url", "not-a-valid-url"),
    ];

    for (mod_id, invalid_url) in &invalid_urls {
      let test_mod = create_test_mod(mod_id, Some(invalid_url));
      repo.save_third_party_mods(&variant, vec![test_mod]).await?;

      let result = get_last_activity_for_third_party_mod(
        mod_id, &variant, &client, &repo,
      )
      .await;

      assert!(
        matches!(
          result,
          Err(GetLastActivityForThirdPartyModError::NoGithubActivity)
        ),
        "Expected NoGithubActivity for URL: {invalid_url}, got: {:?}",
        result
      );
    }

    assert_eq!(client.request_count(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_repo_has_no_commits() -> TestResult
  {
    let (db, _server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "empty-repo-mod";
    let owner = "empty-owner";
    let repo_name = "empty-repo";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let result = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await;

    assert!(matches!(
      result,
      Err(GetLastActivityForThirdPartyModError::GetLastCommit(
        GetLastCommitError::NoCommitsFound
      ))
    ));
    assert_eq!(client.request_count(), 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_last_activity_github_server_error() -> TestResult
  {
    let (db, server, client) = setup().await?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "server-error-mod";
    let owner = "error-owner";
    let repo_name = "error-repo";

    let github_url =
      format!("https://github.com/{owner}/{repo_name}");
    let test_mod = create_test_mod(mod_id, Some(&github_url));
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    server
      .add_commit(owner, repo_name, Commit::new(owner, repo_name))
      .await;

    server
      .add_mock_behavior(
        MockBehavior::builder()
          .error(MockError::InternalServerError)
          .build(),
      )
      .await?;

    let result = get_last_activity_for_third_party_mod(
      mod_id, &variant, &client, &repo,
    )
    .await;

    match result {
      Err(GetLastActivityForThirdPartyModError::GetLastCommit(
        GetLastCommitError::InvalidResponse(msg),
      )) => {
        assert!(
          msg.contains("500"),
          "Expected 500 in error message, got: {msg}"
        );
      }
      other => {
        return Err(
          format!("Expected InvalidResponse error, got: {:?}", other)
            .into(),
        );
      }
    }
    assert_eq!(client.request_count(), 1);

    Ok(())
  }

  #[test]
  fn test_extract_repo_from_github_url() {
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA.git"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA/"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA.git/"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA/tree/master"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "http://github.com/CleverRaven/Cataclysm-DDA"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA?ref=main"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );
    assert_eq!(
      extract_repo_from_github_url(
        "https://github.com/CleverRaven/Cataclysm-DDA#readme"
      ),
      Some("CleverRaven/Cataclysm-DDA".to_string())
    );

    // Invalid / edge cases
    assert_eq!(
      extract_repo_from_github_url("https://github.com/CleverRaven"),
      None
    );
    assert_eq!(
      extract_repo_from_github_url("https://github.com/"),
      None
    );
    assert_eq!(
      extract_repo_from_github_url("https://github.com"),
      None
    );
    assert_eq!(extract_repo_from_github_url("not-a-url"), None);
    assert_eq!(extract_repo_from_github_url(""), None);
  }
}
