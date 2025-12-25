use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;
use tokio::task;

use crate::mods::repository::cached_mods_repository::{
  CachedModsRepository, CachedModsRepositoryError,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub struct SqliteCachedModsRepository {
  pool: Pool,
}

impl SqliteCachedModsRepository {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl CachedModsRepository for SqliteCachedModsRepository {
  async fn get_cached_mods(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<ThirdPartyMod>, CachedModsRepositoryError> {
    let pool = self.pool.clone();
    let variant = *variant;

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?;

      let mut stmt = conn
        .prepare(
          "SELECT mod_id, name, description, category, download_url, modinfo, activity_type, github 
           FROM cached_mods WHERE game_variant = ?1",
        )
        .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?;

      let rows = stmt
        .query_map([variant.to_string()], |row| {
          Ok(ThirdPartyMod {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            category: row.get(3)?,
            installation: crate::mods::types::ModInstallation {
              download_url: row.get(4)?,
              modinfo: row.get(5)?,
            },
            activity: crate::mods::types::ModActivity {
              activity_type: row.get(6)?,
              github: row.get(7)?,
            },
          })
        })
        .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?;

      let mut mods = Vec::new();
      for row in rows {
        mods.push(row.map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?);
      }

      Ok(mods)
    })
    .await
    .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?
  }

  async fn get_cached_mod_by_id(
    &self,
    variant: &GameVariant,
    mod_id: &str,
  ) -> Result<Option<ThirdPartyMod>, CachedModsRepositoryError> {
    let pool = self.pool.clone();
    let variant = *variant;
    let mod_id = mod_id.to_string();

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?;

      let result = conn
        .query_row(
          "SELECT mod_id, name, description, category, download_url, modinfo, activity_type, github 
           FROM cached_mods WHERE game_variant = ?1 AND mod_id = ?2",
          (variant.to_string(), mod_id),
          |row| {
            Ok(ThirdPartyMod {
              id: row.get(0)?,
              name: row.get(1)?,
              description: row.get(2)?,
              category: row.get(3)?,
              installation: crate::mods::types::ModInstallation {
                download_url: row.get(4)?,
                modinfo: row.get(5)?,
              },
              activity: crate::mods::types::ModActivity {
                activity_type: row.get(6)?,
                github: row.get(7)?,
              },
            })
          },
        )
        .optional()
        .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?;

      Ok(result)
    })
    .await
    .map_err(|e| CachedModsRepositoryError::Get(Box::new(e)))?
  }

  async fn update_cached_mods(
    &self,
    variant: &GameVariant,
    mods: &[ThirdPartyMod],
  ) -> Result<(), CachedModsRepositoryError> {
    let pool = self.pool.clone();
    let variant = *variant;
    let mods = mods.to_vec();

    task::spawn_blocking(move || {
      let mut conn = pool
        .get()
        .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;

      let tx = conn
        .transaction()
        .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;

      tx.execute(
        "DELETE FROM cached_mods WHERE game_variant = ?1",
        [variant.to_string()],
      )
      .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;

      let mut stmt = tx
        .prepare(
          "INSERT INTO cached_mods (mod_id, game_variant, name, description, category, download_url, modinfo, activity_type, github) 
           VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )
        .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;

      for third_party_mod in mods {
        stmt
          .execute((
            &third_party_mod.id,
            variant.to_string(),
            &third_party_mod.name,
            &third_party_mod.description,
            &third_party_mod.category,
            &third_party_mod.installation.download_url,
            &third_party_mod.installation.modinfo,
            &third_party_mod.activity.activity_type,
            &third_party_mod.activity.github,
          ))
          .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;
      }

      drop(stmt);

      tx.commit()
        .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?;

      Ok(())
    })
    .await
    .map_err(|e| CachedModsRepositoryError::Update(Box::new(e)))?
  }
}
