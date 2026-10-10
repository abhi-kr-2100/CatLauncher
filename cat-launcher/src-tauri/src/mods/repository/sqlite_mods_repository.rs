use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError, ListCachedThirdPartyModsError,
  ModsRepository, SaveThirdPartyModsError,
};
use crate::mods::types::{
  ModActivity, ModInstallation, ThirdPartyMod,
};
use crate::variants::GameVariant;

const ACTIVITY_TYPE_GITHUB_COMMIT: &str = "github_commit";

fn insert_third_party_mod_tx(
  tx: &rusqlite::Transaction,
  variant_name: &str,
  m: &ThirdPartyMod,
) -> Result<(), SaveThirdPartyModsError> {
  let activity_type = match &m.activity {
    Some(ModActivity::GithubCommit { .. }) => {
      ACTIVITY_TYPE_GITHUB_COMMIT
    }
    None => "",
  };

  tx.execute(
    "INSERT INTO mods (id, game_variant, name, description, category, download_url, modinfo_path, activity_type)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
     ON CONFLICT(id, game_variant) DO UPDATE SET
       name = excluded.name,
       description = excluded.description,
       category = excluded.category,
       download_url = excluded.download_url,
       modinfo_path = excluded.modinfo_path,
       activity_type = excluded.activity_type",
    [
      &m.id,
      variant_name,
      &m.name,
      &m.description,
      &m.category,
      &m.installation.download_url,
      &m.installation.modinfo,
      activity_type,
    ],
  )
  .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?;

  match &m.activity {
    Some(ModActivity::GithubCommit { github }) => {
      if github.is_empty() {
        return Err(SaveThirdPartyModsError::InvalidData(format!(
          "GitHub URL required for mod {} in variant {}",
          m.id, variant_name
        )));
      }
      tx.execute(
        "INSERT INTO github_commit_mod_activity (mod_id, game_variant, github_url)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(mod_id, game_variant) DO UPDATE SET
           github_url = excluded.github_url",
        [&m.id, variant_name, github],
      )
      .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?;
    }
    None => {}
  }

  Ok(())
}

fn load_activity(
  conn: &rusqlite::Connection,
  mod_id: &str,
  variant_name: &str,
  activity_type: &str,
) -> Result<Option<ModActivity>, GetThirdPartyModByIdError> {
  if activity_type == ACTIVITY_TYPE_GITHUB_COMMIT {
    let mut stmt = conn
      .prepare(
        "SELECT github_url FROM github_commit_mod_activity WHERE mod_id = ?1 AND game_variant = ?2",
      )
      .map_err(|e| GetThirdPartyModByIdError::Get(Box::new(e)))?;

    let github: String = stmt
      .query_row([mod_id, variant_name], |row| row.get(0))
      .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
          GetThirdPartyModByIdError::InconsistentData(format!(
            "GitHub activity data missing for mod {} in variant {}",
            mod_id, variant_name
          ))
        }
        _ => GetThirdPartyModByIdError::Get(Box::new(e)),
      })?;

    Ok(Some(ModActivity::GithubCommit { github }))
  } else if activity_type.is_empty() {
    Ok(None)
  } else {
    Err(GetThirdPartyModByIdError::UnsupportedActivityType(
      activity_type.to_string(),
    ))
  }
}

pub struct SqliteModsRepository {
  pool: r2d2::Pool<SqliteConnectionManager>,
}

impl SqliteModsRepository {
  pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl ModsRepository for SqliteModsRepository {
  async fn save_third_party_mods(
    &self,
    variant: &GameVariant,
    mods: Vec<ThirdPartyMod>,
  ) -> Result<(), SaveThirdPartyModsError> {
    let pool = self.pool.clone();
    let variant_name = variant.to_string();

    tokio::task::spawn_blocking(move || {
      let mut conn = pool
        .get()
        .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?;

      let tx = conn
        .transaction()
        .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?;

      for m in mods {
        insert_third_party_mod_tx(&tx, &variant_name, &m)?;
      }

      tx.commit()
        .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?;

      Ok(())
    })
    .await
    .map_err(|e| SaveThirdPartyModsError::Save(Box::new(e)))?
  }

  async fn get_third_party_mod_by_id(
    &self,
    mod_id: &str,
    variant: &GameVariant,
  ) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = variant.to_string();

    tokio::task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| GetThirdPartyModByIdError::Get(Box::new(e)))?;

      let mut stmt = conn
        .prepare(
          "SELECT name, description, category, download_url, modinfo_path, activity_type
           FROM mods WHERE id = ?1 AND game_variant = ?2",
        )
        .map_err(|e| GetThirdPartyModByIdError::Get(Box::new(e)))?;

      let row_data = stmt
        .query_row([&mod_id, &variant_name], |row| {
          Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
          ))
        })
        .map_err(|e| match e {
          rusqlite::Error::QueryReturnedNoRows => {
            GetThirdPartyModByIdError::NotFound(
              mod_id.clone(),
              variant_name.clone(),
            )
          }
          _ => GetThirdPartyModByIdError::Get(Box::new(e)),
        })?;

      let (
        name,
        description,
        category,
        download_url,
        modinfo,
        activity_type,
      ) = row_data;

      let activity = load_activity(&conn, &mod_id, &variant_name, &activity_type)?;

      let category = category.unwrap_or_else(|| "Unknown".to_string());

      Ok(ThirdPartyMod {
        id: mod_id,
        name,
        description,
        category,
        installation: ModInstallation {
          download_url,
          modinfo,
        },
        activity,
      })
    })
    .await
    .map_err(|e| GetThirdPartyModByIdError::Get(Box::new(e)))?
  }

  async fn get_third_party_mods(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<ThirdPartyMod>, ListCachedThirdPartyModsError> {
    let pool = self.pool.clone();
    let variant_name = variant.to_string();

    tokio::task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;

      let mut stmt = conn
        .prepare(
          "SELECT id, name, description, category, download_url, modinfo_path, activity_type
           FROM mods WHERE game_variant = ?1",
        )
        .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;

      let mut mods = Vec::new();
      let mut rows = stmt.query([&variant_name]).map_err(|e| {
        ListCachedThirdPartyModsError::Get(Box::new(e))
      })?;

      while let Some(row) = rows.next().map_err(|e| {
        ListCachedThirdPartyModsError::Get(Box::new(e))
      })? {
        let id: String = row
          .get(0)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let name: String = row
          .get(1)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let description: String = row
          .get(2)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let category: Option<String> = row
          .get(3)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let download_url: String = row
          .get(4)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let modinfo: String = row
          .get(5)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;
        let activity_type: String = row
          .get(6)
          .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?;

        let activity = load_activity(&conn, &id, &variant_name, &activity_type)
          .map_err(|e| match e {
            GetThirdPartyModByIdError::InconsistentData(msg) => {
              ListCachedThirdPartyModsError::InconsistentData(msg)
            }
            GetThirdPartyModByIdError::UnsupportedActivityType(type_) => {
              ListCachedThirdPartyModsError::UnsupportedActivityType(type_)
            }
            _ => {
              ListCachedThirdPartyModsError::Get(Box::new(e))
            }
          })?;

        let category = category.unwrap_or_else(|| "Unknown".to_string());

        mods.push(ThirdPartyMod {
          id,
          name,
          description,
          category,
          installation: ModInstallation {
            download_url,
            modinfo,
          },
          activity,
        });
      }

      Ok(mods)
    })
    .await
    .map_err(|e| ListCachedThirdPartyModsError::Get(Box::new(e)))?
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
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::types::ModInstallation;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  fn create_mod(id: &str, github: Option<&str>) -> ThirdPartyMod {
    ThirdPartyMod {
      id: id.to_string(),
      name: format!("Name {id}"),
      description: format!("Desc {id}"),
      category: "items".to_string(),
      installation: ModInstallation {
        download_url: "https://example.com/mod.zip".to_string(),
        modinfo: "modinfo.json".to_string(),
      },
      activity: github.map(|g| ModActivity::GithubCommit {
        github: g.to_string(),
      }),
    }
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_save_and_get() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let test_mod = create_mod(
      "sample",
      Some("https://github.com/owner/sample-repo"),
    );
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let fetched =
      repo.get_third_party_mod_by_id("sample", &variant).await?;
    assert_eq!(fetched.id, "sample");
    assert_eq!(fetched.name, "Name sample");
    assert_eq!(
      fetched.installation.download_url,
      "https://example.com/mod.zip"
    );

    match fetched.activity {
      Some(ModActivity::GithubCommit { github }) => {
        assert_eq!(github, "https://github.com/owner/sample-repo");
      }
      None => panic!("Expected github activity"),
    }

    let all = repo.get_third_party_mods(&variant).await?;
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, "sample");

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_upsert() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let mut test_mod = create_mod("mod_upsert", None);
    repo
      .save_third_party_mods(&variant, vec![test_mod.clone()])
      .await?;

    let fetched1 = repo
      .get_third_party_mod_by_id("mod_upsert", &variant)
      .await?;
    assert_eq!(fetched1.name, "Name mod_upsert");

    test_mod.name = "Updated Name".to_string();
    test_mod.description = "Updated Description".to_string();
    repo.save_third_party_mods(&variant, vec![test_mod]).await?;

    let fetched2 = repo
      .get_third_party_mod_by_id("mod_upsert", &variant)
      .await?;
    assert_eq!(fetched2.name, "Updated Name");
    assert_eq!(fetched2.description, "Updated Description");

    let all = repo.get_third_party_mods(&variant).await?;
    assert_eq!(all.len(), 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_empty_github_url_error()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let test_mod = create_mod("bad_github", Some(""));
    let result =
      repo.save_third_party_mods(&variant, vec![test_mod]).await;

    assert!(matches!(
      result,
      Err(SaveThirdPartyModsError::InvalidData(_))
    ));

    let all = repo.get_third_party_mods(&variant).await?;
    assert!(all.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_not_found() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result =
      repo.get_third_party_mod_by_id("missing", &variant).await;
    assert!(matches!(
      result,
      Err(GetThirdPartyModByIdError::NotFound(id, var)) if id == "missing" && var == variant.to_string()
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_inconsistent_activity_data()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    // Insert directly into `mods` table with github_commit activity_type,
    // but without corresponding row in `github_commit_mod_activity` table
    {
      let conn = db.pool().get()?;
      conn.execute(
        "INSERT INTO mods (id, game_variant, name, description, category, download_url, modinfo_path, activity_type)
         VALUES ('broken', ?1, 'Broken', 'Desc', 'items', 'https://example.com/mod.zip', 'modinfo.json', 'github_commit')",
        [variant.to_string()],
      )?;
    }

    let result =
      repo.get_third_party_mod_by_id("broken", &variant).await;
    assert!(matches!(
      result,
      Err(GetThirdPartyModByIdError::InconsistentData(_))
    ));

    let list_result = repo.get_third_party_mods(&variant).await;
    assert!(matches!(
      list_result,
      Err(ListCachedThirdPartyModsError::InconsistentData(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_mods_repository_unsupported_activity_type()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    // Insert directly with unknown activity_type
    {
      let conn = db.pool().get()?;
      conn.execute(
        "INSERT INTO mods (id, game_variant, name, description, category, download_url, modinfo_path, activity_type)
         VALUES ('weird', ?1, 'Weird', 'Desc', 'items', 'https://example.com/mod.zip', 'modinfo.json', 'alien_activity')",
        [variant.to_string()],
      )?;
    }

    let result =
      repo.get_third_party_mod_by_id("weird", &variant).await;
    assert!(matches!(
      result,
      Err(GetThirdPartyModByIdError::UnsupportedActivityType(t)) if t == "alien_activity"
    ));

    let list_result = repo.get_third_party_mods(&variant).await;
    assert!(matches!(
      list_result,
      Err(ListCachedThirdPartyModsError::UnsupportedActivityType(t)) if t == "alien_activity"
    ));

    Ok(())
  }
}
