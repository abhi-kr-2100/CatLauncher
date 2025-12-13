use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

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
impl InstalledTilesetsRepository for SqliteInstalledTilesetsRepository {
    async fn add_installed_tileset(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError> {
        let pool = self.pool.clone();
        let tileset_id = tileset_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            conn.execute(
                "INSERT OR IGNORE INTO installed_tilesets (tileset_id, game_variant) VALUES (?1, ?2)",
                [&tileset_id, &variant_name],
            )
            .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            Ok(())
        })
        .await
        .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?
    }

    async fn get_installed_tilesets_for_variant(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<String>, InstalledTilesetsRepositoryError> {
        let pool = self.pool.clone();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            let mut stmt = conn
                .prepare("SELECT tileset_id FROM installed_tilesets WHERE game_variant = ?1")
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            let tileset_ids = stmt
                .query_map([&variant_name], |row| row.get::<_, String>(0))
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            Ok(tileset_ids)
        })
        .await
        .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?
    }

    async fn delete_installed_tileset(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError> {
        let pool = self.pool.clone();
        let tileset_id = tileset_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            let rows_affected = conn
                .execute(
                    "DELETE FROM installed_tilesets WHERE tileset_id = ?1 AND game_variant = ?2",
                    [&tileset_id, &variant_name],
                )
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            if rows_affected == 0 {
                return Err(InstalledTilesetsRepositoryError::NotFound(tileset_id, variant_name));
            }

            Ok(())
        })
        .await
        .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?
    }

    async fn is_tileset_installed(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<bool, InstalledTilesetsRepositoryError> {
        let pool = self.pool.clone();
        let tileset_id = tileset_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            let mut stmt = conn
                .prepare("SELECT 1 FROM installed_tilesets WHERE tileset_id = ?1 AND game_variant = ?2")
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            let exists = stmt
                .exists([&tileset_id, &variant_name])
                .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?;

            Ok(exists)
        })
        .await
        .map_err(|e| InstalledTilesetsRepositoryError::Database(e.to_string()))?
    }
}
