use super::repository::{PlayTimeRepository, PlayTimeRepositoryError};
use async_trait::async_trait;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

#[derive(Clone)]
pub struct SqlitePlayTimeRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl SqlitePlayTimeRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlayTimeRepository for SqlitePlayTimeRepository {
    async fn log_play_time(
        &self,
        game_variant: String,
        version: String,
        duration_in_seconds: i64,
    ) -> Result<(), PlayTimeRepositoryError> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| PlayTimeRepositoryError::LogPlayTime(e.to_string()))?;
            conn.execute(
                "INSERT INTO play_time (game_variant, version, duration_in_seconds) VALUES (?1, ?2, ?3)",
                rusqlite::params![game_variant, version, duration_in_seconds],
            )
            .map_err(|e| PlayTimeRepositoryError::LogPlayTime(e.to_string()))?;
            Ok(())
        })
        .await
        .map_err(|e| PlayTimeRepositoryError::JoinError(e.to_string()))?
    }

    async fn get_play_time_for_version(
        &self,
        game_variant: String,
        version: String,
    ) -> Result<i64, PlayTimeRepositoryError> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| PlayTimeRepositoryError::GetPlayTimeForVersion(e.to_string()))?;
            let sum: Option<i64> = conn
                .query_row(
                    "SELECT SUM(duration_in_seconds) FROM play_time WHERE game_variant = ?1 AND version = ?2",
                    rusqlite::params![game_variant, version],
                    |row| row.get(0),
                )
                .map_err(|e| PlayTimeRepositoryError::GetPlayTimeForVersion(e.to_string()))?;
            Ok(sum.unwrap_or(0))
        })
        .await
        .map_err(|e| PlayTimeRepositoryError::JoinError(e.to_string()))?
    }

    async fn get_play_time_for_variant(
        &self,
        game_variant: String,
    ) -> Result<i64, PlayTimeRepositoryError> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| PlayTimeRepositoryError::GetPlayTimeForVariant(e.to_string()))?;
            let sum: Option<i64> = conn
                .query_row(
                    "SELECT SUM(duration_in_seconds) FROM play_time WHERE game_variant = ?1",
                    rusqlite::params![game_variant],
                    |row| row.get(0),
                )
                .map_err(|e| PlayTimeRepositoryError::GetPlayTimeForVariant(e.to_string()))?;
            Ok(sum.unwrap_or(0))
        })
        .await
        .map_err(|e| PlayTimeRepositoryError::JoinError(e.to_string()))?
    }

    async fn get_total_play_time(&self) -> Result<i64, PlayTimeRepositoryError> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| PlayTimeRepositoryError::GetTotalPlayTime(e.to_string()))?;
            let sum: Option<i64> = conn
                .query_row(
                    "SELECT SUM(duration_in_seconds) FROM play_time",
                    rusqlite::params![],
                    |row| row.get(0),
                )
                .map_err(|e| PlayTimeRepositoryError::GetTotalPlayTime(e.to_string()))?;
            Ok(sum.unwrap_or(0))
        })
        .await
        .map_err(|e| PlayTimeRepositoryError::JoinError(e.to_string()))?
    }
}
