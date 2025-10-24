use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::repository::last_played_repository::{
    LastPlayedVersionRepository, LastPlayedVersionRepositoryError,
};
use crate::variants::game_variant::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub struct SqliteLastPlayedVersionRepository {
    pool: Pool,
}

impl SqliteLastPlayedVersionRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LastPlayedVersionRepository for SqliteLastPlayedVersionRepository {
    async fn get_last_played_version(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Option<String>, LastPlayedVersionRepositoryError> {
        let pool = self.pool.clone();
        let game_variant = game_variant.clone();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| LastPlayedVersionRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn
                .prepare("SELECT version FROM last_played_version WHERE game_variant = ?1")
                .map_err(|e| LastPlayedVersionRepositoryError::Get(Box::new(e)))?;

            let mut rows = stmt
                .query_map([game_variant.to_string()], |row| row.get(0))
                .map_err(|e| LastPlayedVersionRepositoryError::Get(Box::new(e)))?;

            if let Some(row) = rows.next() {
                row.map_err(|e| LastPlayedVersionRepositoryError::Get(Box::new(e)))
                    .map(Some)
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| LastPlayedVersionRepositoryError::Get(Box::new(e)))?
    }

    async fn set_last_played_version(
        &self,
        game_variant: &GameVariant,
        version: &str,
    ) -> Result<(), LastPlayedVersionRepositoryError> {
        let pool = self.pool.clone();
        let game_variant = game_variant.clone();
        let version = version.to_string();

        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| LastPlayedVersionRepositoryError::Set(Box::new(e)))?;
            conn.execute(
                "INSERT OR REPLACE INTO last_played_version (game_variant, version) VALUES (?1, ?2)",
                (game_variant.to_string(), version),
            )
            .map_err(|e| LastPlayedVersionRepositoryError::Set(Box::new(e)))?;

            Ok(())
        })
        .await
        .map_err(|e| LastPlayedVersionRepositoryError::Set(Box::new(e)))?
    }
}
