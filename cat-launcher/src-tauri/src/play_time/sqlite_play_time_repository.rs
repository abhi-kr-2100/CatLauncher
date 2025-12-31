use async_trait::async_trait;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;
use tokio::task;

use crate::play_time::repository::{
  PlayTimeRepository, PlayTimeRepositoryError,
};
use crate::variants::GameVariant;

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
    game_variant: &GameVariant,
    version: &str,
    duration_in_seconds: i64,
  ) -> Result<(), PlayTimeRepositoryError> {
    if duration_in_seconds <= 0 {
      return Err(PlayTimeRepositoryError::InvalidDuration(
        duration_in_seconds,
      ));
    }

    let pool = self.pool.clone();
    let game_variant_id = game_variant.id();
    let version = version.to_owned();
    task::block_in_place(move || {
      let conn = pool.get()?;
      conn.execute(
                "INSERT INTO play_time (game_variant, version, duration_in_seconds)
                    VALUES (?1, ?2, ?3)
                    ON CONFLICT(game_variant, version)
                    DO UPDATE SET duration_in_seconds = duration_in_seconds + excluded.duration_in_seconds",
                rusqlite::params![game_variant_id, version, duration_in_seconds],
            )?;
      Ok(())
    })
  }

  async fn get_play_time_for_version(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<i64, PlayTimeRepositoryError> {
    let pool = self.pool.clone();
    let game_variant_id = game_variant.id();
    let version = version.to_owned();
    let duration = task::spawn_blocking(move || -> BlockingTaskResult<i64> {
      let conn = pool.get()?;
      let duration: Option<i64> = conn
        .query_row(
          "SELECT duration_in_seconds FROM play_time WHERE game_variant = ?1 AND version = ?2",
          rusqlite::params![game_variant_id, version],
          |row| row.get(0),
        )
        .optional()?;
      Ok(duration.unwrap_or(0))
    })
    .await??;
    Ok(duration)
  }

  async fn get_play_time_for_variant(
    &self,
    game_variant: &GameVariant,
  ) -> Result<i64, PlayTimeRepositoryError> {
    let pool = self.pool.clone();
    let game_variant_id = game_variant.id();
    let sum = task::spawn_blocking(move || -> BlockingTaskResult<i64> {
      let conn = pool.get()?;
      let sum: i64 = conn
        .query_row(
          "SELECT COALESCE(SUM(duration_in_seconds), 0) FROM play_time WHERE game_variant = ?1",
          rusqlite::params![game_variant_id],
          |row| row.get(0),
        )?;
      Ok(sum)
    })
    .await??;
    Ok(sum)
  }
}

type BlockingTaskResult<T> = Result<T, PlayTimeRepositoryError>;
