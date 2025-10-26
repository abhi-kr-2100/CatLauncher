use std::str::FromStr;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::repository::backup_repository::{BackupEntry, BackupRepository, BackupRepositoryError};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteBackupRepository {
    pool: Pool,
}

impl SqliteBackupRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BackupRepository for SqliteBackupRepository {
    async fn add_backup_entry(
        &self,
        game_variant: &GameVariant,
        release_version: &str,
        timestamp: u64,
    ) -> Result<i64, BackupRepositoryError> {
        let pool = self.pool.clone();
        let game_variant = game_variant.to_string();
        let release_version = release_version.to_string();

        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| BackupRepositoryError::Add(Box::new(e)))?;
            let id = conn.query_row(
                "INSERT INTO backups (game_variant, release_version, timestamp) VALUES (?1, ?2, ?3) RETURNING id",
                rusqlite::params![game_variant, release_version, timestamp],
                |row| row.get(0),
            ).map_err(|e| BackupRepositoryError::Add(Box::new(e)))?;
            Ok(id)
        })
        .await
        .map_err(|e| BackupRepositoryError::Add(Box::new(e)))?
    }

    async fn get_backups_sorted_by_timestamp(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<BackupEntry>, BackupRepositoryError> {
        let pool = self.pool.clone();
        let game_variant = game_variant.to_string();

        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| BackupRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn.prepare(
                "SELECT id, game_variant, release_version, timestamp FROM backups WHERE game_variant = ?1 ORDER BY timestamp ASC",
            ).map_err(|e| BackupRepositoryError::Get(Box::new(e)))?;
            let backups = stmt
                .query_map(rusqlite::params![game_variant], |row| {
                    let id = row.get(0)?;
                    let game_variant_str: String = row.get(1)?;
                    let game_variant = GameVariant::from_str(&game_variant_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;
                    Ok(BackupEntry {
                        id,
                        game_variant,
                        release_version: row.get(2)?,
                        timestamp: row.get(3)?,
                    })
                })
                .map_err(|e| BackupRepositoryError::Get(Box::new(e)))?
                .collect::<Result<Vec<BackupEntry>, _>>()
                .map_err(|e| BackupRepositoryError::Get(Box::new(e)))?;
            Ok(backups)
        })
        .await
        .map_err(|e| BackupRepositoryError::Get(Box::new(e)))?
    }

    async fn delete_backup_entry(&self, id: i64) -> Result<(), BackupRepositoryError> {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| BackupRepositoryError::Delete(Box::new(e)))?;
            conn.execute("DELETE FROM backups WHERE id = ?1", rusqlite::params![id])
                .map_err(|e| BackupRepositoryError::Delete(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| BackupRepositoryError::Delete(Box::new(e)))?
    }
}
