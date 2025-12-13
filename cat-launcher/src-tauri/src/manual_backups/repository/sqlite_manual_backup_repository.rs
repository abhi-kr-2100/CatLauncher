use std::str::FromStr;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::manual_backups::repository::manual_backup_repository::{
  ManualBackupEntry, ManualBackupRepository,
  ManualBackupRepositoryError,
};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteManualBackupRepository {
  pool: Pool,
}

impl SqliteManualBackupRepository {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl ManualBackupRepository for SqliteManualBackupRepository {
  async fn add_manual_backup_entry(
    &self,
    name: &str,
    game_variant: &GameVariant,
    timestamp: u64,
    notes: Option<String>,
  ) -> Result<i64, ManualBackupRepositoryError> {
    let pool = self.pool.clone();
    let name = name.to_string();
    let game_variant = game_variant.to_string();

    task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| ManualBackupRepositoryError::Add(Box::new(e)))?;
            let id = conn.query_row(
                "INSERT INTO manual_backups (name, game_variant, timestamp, notes) VALUES (?1, ?2, ?3, ?4) RETURNING id",
                rusqlite::params![name, game_variant, timestamp, notes],
                |row| row.get(0),
            ).map_err(|e| ManualBackupRepositoryError::Add(Box::new(e)))?;
            Ok(id)
        })
        .await
        .map_err(|e| ManualBackupRepositoryError::Add(Box::new(e)))?
  }

  async fn get_manual_backups_sorted_by_timestamp(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<ManualBackupEntry>, ManualBackupRepositoryError> {
    let pool = self.pool.clone();
    let game_variant = game_variant.to_string();

    task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn.prepare(
                "SELECT id, name, game_variant, timestamp, notes FROM manual_backups WHERE game_variant = ?1 ORDER BY timestamp ASC",
            ).map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?;
            let backups = stmt
                .query_map(rusqlite::params![game_variant], |row| {
                    let id = row.get(0)?;
                    let name: String = row.get(1)?;
                    let game_variant_str: String = row.get(2)?;
                    let game_variant = GameVariant::from_str(&game_variant_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;
                    Ok(ManualBackupEntry {
                        id,
                        name,
                        game_variant,
                        timestamp: row.get(3)?,
                        notes: row.get(4)?,
                    })
                })
                .map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?
                .collect::<Result<Vec<ManualBackupEntry>, _>>()
                .map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?;
            Ok(backups)
        })
        .await
        .map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?
  }

  async fn get_manual_backup_entry(
    &self,
    id: i64,
  ) -> Result<ManualBackupEntry, ManualBackupRepositoryError> {
    let pool = self.pool.clone();

    task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn.prepare(
                "SELECT id, name, game_variant, timestamp, notes FROM manual_backups WHERE id = ?1",
            ).map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?;
            let backup = stmt
                .query_row(rusqlite::params![id], |row| {
                    let id = row.get(0)?;
                    let name: String = row.get(1)?;
                    let game_variant_str: String = row.get(2)?;
                    let game_variant = GameVariant::from_str(&game_variant_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;
                    Ok(ManualBackupEntry {
                        id,
                        name,
                        game_variant,
                        timestamp: row.get(3)?,
                        notes: row.get(4)?,
                    })
                });

            match backup {
                Ok(backup) => Ok(backup),
                Err(rusqlite::Error::QueryReturnedNoRows) => Err(ManualBackupRepositoryError::NotFound(id)),
                Err(e) => Err(ManualBackupRepositoryError::Get(Box::new(e))),
            }
        })
        .await
        .map_err(|e| ManualBackupRepositoryError::Get(Box::new(e)))?
  }

  async fn delete_manual_backup_entry(
    &self,
    id: i64,
  ) -> Result<(), ManualBackupRepositoryError> {
    let pool = self.pool.clone();

    task::spawn_blocking(move || {
      let conn = pool.get().map_err(|e| {
        ManualBackupRepositoryError::Delete(Box::new(e))
      })?;
      conn
        .execute(
          "DELETE FROM manual_backups WHERE id = ?1",
          rusqlite::params![id],
        )
        .map_err(|e| {
          ManualBackupRepositoryError::Delete(Box::new(e))
        })?;
      Ok(())
    })
    .await
    .map_err(|e| ManualBackupRepositoryError::Delete(Box::new(e)))?
  }
}
