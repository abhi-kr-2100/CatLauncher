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
