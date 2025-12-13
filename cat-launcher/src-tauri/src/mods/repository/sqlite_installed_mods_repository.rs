use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

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

    tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            conn.execute(
                "INSERT OR IGNORE INTO installed_mods (mod_id, game_variant) VALUES (?1, ?2)",
                [&mod_id, &variant_name],
            )
            .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?
  }

  async fn delete_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            let rows_affected = conn
                .execute(
                    "DELETE FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
                    [&mod_id, &variant_name],
                )
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            if rows_affected == 0 {
                return Err(InstalledModsRepositoryError::NotFound(mod_id, variant_name));
            }

            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?
  }

  async fn is_mod_installed(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            let mut stmt = conn
                .prepare("SELECT 1 FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2")
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            let exists = stmt
                .exists([&mod_id, &variant_name])
                .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?;

            Ok(exists)
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Database(e.to_string()))?
  }
}
