use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_helper::run_db;
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

pub struct SqliteInstalledModsRepository {
  pool: r2d2::Pool<SqliteConnectionManager>,
}

impl SqliteInstalledModsRepository {
  pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl InstalledModsRepository for SqliteInstalledModsRepository {
  async fn add_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "INSERT OR IGNORE INTO installed_mods (mod_id, game_variant) VALUES (?1, ?2)",
        [&mod_id, &variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledModsRepositoryError::Add)
  }

  async fn delete_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id_clone = mod_id.to_string();
    let variant_name_clone = game_variant.to_string();

    let rows_affected = run_db(pool, move |conn| {
      let count = conn.execute(
        "DELETE FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
        [&mod_id_clone, &variant_name_clone],
      )?;
      Ok::<usize, rusqlite::Error>(count)
    })
    .await
    .map_err(InstalledModsRepositoryError::Delete)?;

    if rows_affected == 0 {
      return Err(InstalledModsRepositoryError::NotFound(
        mod_id.to_string(),
        game_variant.to_string(),
      ));
    }

    Ok(())
  }

  async fn delete_all_installed_mods(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "DELETE FROM installed_mods WHERE game_variant = ?1",
        [&variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledModsRepositoryError::DeleteAll)
  }

  async fn is_mod_installed(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      let mut stmt = conn.prepare(
        "SELECT 1 FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
      )?;
      stmt.exists([&mod_id, &variant_name])
    })
    .await
    .map_err(InstalledModsRepositoryError::IsInstalled)
  }
}
