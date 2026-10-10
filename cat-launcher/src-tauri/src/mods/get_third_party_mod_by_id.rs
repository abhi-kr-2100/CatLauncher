use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError as RepoError, ModsRepository,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  #[error("failed to get mod from repository: {0}")]
  Repository(#[from] RepoError),
}

pub async fn get_third_party_mod_by_id(
  mod_id: &str,
  variant: &GameVariant,
  mods_repository: &impl ModsRepository,
) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
  let m = mods_repository
    .get_third_party_mod_by_id(mod_id, variant)
    .await?;

  Ok(m)
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
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
  use crate::mods::types::{ModActivity, ModInstallation};

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  fn create_test_mod(
    id: &str,
    github_url: Option<&str>,
  ) -> ThirdPartyMod {
    ThirdPartyMod {
      id: id.to_string(),
      name: format!("Mod Name {id}"),
      description: format!("Description for {id}"),
      category: "Content".to_string(),
      installation: ModInstallation {
        download_url: "https://example.com/mod.zip".to_string(),
        modinfo: "modinfo.json".to_string(),
      },
      activity: github_url.map(|github| ModActivity::GithubCommit {
        github: github.to_string(),
      }),
    }
  }

  #[tokio::test]
  async fn test_get_third_party_mod_by_id_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let test_mod = create_test_mod(
      "arcana",
      Some("https://github.com/chaosvolt/cdda-arcana-mod"),
    );
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let fetched =
      get_third_party_mod_by_id("arcana", &variant, &repo).await?;
    assert_eq!(fetched.id, "arcana");
    assert_eq!(fetched.name, "Mod Name arcana");
    assert_eq!(fetched.description, "Description for arcana");
    assert_eq!(fetched.category, "Content");
    assert_eq!(
      fetched.installation.download_url,
      "https://example.com/mod.zip"
    );
    assert_eq!(fetched.installation.modinfo, "modinfo.json");
    match fetched.activity {
      Some(ModActivity::GithubCommit { github }) => {
        assert_eq!(
          github,
          "https://github.com/chaosvolt/cdda-arcana-mod"
        );
      }
      None => return Err("expected github activity".into()),
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_third_party_mod_by_id_without_activity()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let test_mod = create_test_mod("no-activity-mod", None);
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let fetched =
      get_third_party_mod_by_id("no-activity-mod", &variant, &repo)
        .await?;
    assert_eq!(fetched.id, "no-activity-mod");
    assert!(fetched.activity.is_none());

    Ok(())
  }

  #[tokio::test]
  async fn test_get_third_party_mod_by_id_not_found() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result =
      get_third_party_mod_by_id("non-existent", &variant, &repo)
        .await;
    match result {
      Err(GetThirdPartyModByIdError::Repository(
        RepoError::NotFound(id, var),
      )) => {
        assert_eq!(id, "non-existent");
        assert_eq!(var, variant.to_string());
      }
      other => {
        return Err(
          format!("expected NotFound error, got: {:?}", other).into(),
        );
      }
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_third_party_mod_by_id_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());

    let test_mod = create_test_mod("dda-mod", None);
    repo
      .save_third_party_mods(
        &GameVariant::DarkDaysAhead,
        vec![test_mod],
      )
      .await?;

    // Exists in DDA
    assert!(
      get_third_party_mod_by_id(
        "dda-mod",
        &GameVariant::DarkDaysAhead,
        &repo
      )
      .await
      .is_ok()
    );

    // Fails in BN
    assert!(matches!(
      get_third_party_mod_by_id(
        "dda-mod",
        &GameVariant::BrightNights,
        &repo
      )
      .await,
      Err(GetThirdPartyModByIdError::Repository(
        RepoError::NotFound(_, _)
      ))
    ));

    // Fails in TLG
    assert!(matches!(
      get_third_party_mod_by_id(
        "dda-mod",
        &GameVariant::TheLastGeneration,
        &repo
      )
      .await,
      Err(GetThirdPartyModByIdError::Repository(
        RepoError::NotFound(_, _)
      ))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_get_third_party_mod_by_id_all_variants() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let mod_id = format!("mod-{}", variant);
      let test_mod = create_test_mod(&mod_id, None);
      repo.save_third_party_mods(&variant, vec![test_mod]).await?;

      let fetched =
        get_third_party_mod_by_id(&mod_id, &variant, &repo).await?;
      assert_eq!(fetched.id, mod_id);
    }

    Ok(())
  }
}
