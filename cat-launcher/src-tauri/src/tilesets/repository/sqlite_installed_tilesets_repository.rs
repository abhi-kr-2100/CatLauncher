use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_helper::run_db;
use crate::tilesets::repository::installed_tilesets_repository::{
  InstalledTilesetsRepository, InstalledTilesetsRepositoryError,
};
use crate::variants::GameVariant;

pub struct SqliteInstalledTilesetsRepository {
  pool: r2d2::Pool<SqliteConnectionManager>,
}

impl SqliteInstalledTilesetsRepository {
  pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl InstalledTilesetsRepository
  for SqliteInstalledTilesetsRepository
{
  async fn add_installed_tileset(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError> {
    let pool = self.pool.clone();
    let tileset_id = tileset_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "INSERT OR IGNORE INTO installed_tilesets (tileset_id, game_variant) VALUES (?1, ?2)",
        [&tileset_id, &variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledTilesetsRepositoryError::Add)
  }

  async fn delete_installed_tileset(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError> {
    let pool = self.pool.clone();
    let tileset_id_clone = tileset_id.to_string();
    let variant_name_clone = game_variant.to_string();

    let rows_affected = run_db(pool, move |conn| {
      let count = conn.execute(
        "DELETE FROM installed_tilesets WHERE tileset_id = ?1 AND game_variant = ?2",
        [&tileset_id_clone, &variant_name_clone],
      )?;
      Ok::<usize, rusqlite::Error>(count)
    })
    .await
    .map_err(InstalledTilesetsRepositoryError::Delete)?;

    if rows_affected == 0 {
      return Err(InstalledTilesetsRepositoryError::NotFound(
        tileset_id.to_string(),
        game_variant.to_string(),
      ));
    }

    Ok(())
  }

  async fn delete_all_installed_tilesets(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError> {
    let pool = self.pool.clone();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "DELETE FROM installed_tilesets WHERE game_variant = ?1",
        [&variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledTilesetsRepositoryError::DeleteAll)
  }

  async fn is_tileset_installed(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledTilesetsRepositoryError> {
    let pool = self.pool.clone();
    let tileset_id = tileset_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      let mut stmt = conn.prepare(
        "SELECT 1 FROM installed_tilesets WHERE tileset_id = ?1 AND game_variant = ?2",
      )?;
      stmt.exists([&tileset_id, &variant_name])
    })
    .await
    .map_err(InstalledTilesetsRepositoryError::IsInstalled)
  }
}
