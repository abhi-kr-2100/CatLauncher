use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::active_release::repository::{
  ActiveReleaseRepository, GetActiveReleaseError,
  SetActiveReleaseError,
};
use crate::variants::game_variant::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub struct SqliteActiveReleaseRepository {
  pool: Pool,
}

impl SqliteActiveReleaseRepository {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl ActiveReleaseRepository for SqliteActiveReleaseRepository {
  async fn get_active_release(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Option<String>, GetActiveReleaseError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;

    task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| GetActiveReleaseError::Get(Box::new(e)))?;
            let mut stmt = conn
                .prepare("SELECT version FROM active_release WHERE game_variant = ?1")
                .map_err(|e| GetActiveReleaseError::Get(Box::new(e)))?;

            let mut rows = stmt
                .query_map([game_variant.to_string()], |row| row.get(0))
                .map_err(|e| GetActiveReleaseError::Get(Box::new(e)))?;

            if let Some(row) = rows.next() {
                row.map_err(|e| GetActiveReleaseError::Get(Box::new(e)))
                    .map(Some)
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| GetActiveReleaseError::Get(Box::new(e)))?
  }

  async fn set_active_release(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<(), SetActiveReleaseError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;
    let version = version.to_string();

    task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| SetActiveReleaseError::Set(Box::new(e)))?;
            conn.execute(
                "INSERT OR REPLACE INTO active_release (game_variant, version) VALUES (?1, ?2)",
                (game_variant.to_string(), version),
            )
            .map_err(|e| SetActiveReleaseError::Set(Box::new(e)))?;

            Ok(())
        })
        .await
        .map_err(|e| SetActiveReleaseError::Set(Box::new(e)))?
  }
}
