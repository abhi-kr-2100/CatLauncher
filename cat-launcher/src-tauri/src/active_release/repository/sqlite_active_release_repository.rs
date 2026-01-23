use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
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
  ) -> Result<Option<String>, ActiveReleaseRepositoryError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;

    task::spawn_blocking(
      move || -> Result<Option<String>, ActiveReleaseRepositoryError> {
        let conn =
          pool.get().map_err(ActiveReleaseRepositoryError::GetFromPool)?;
        let mut stmt = conn
          .prepare("SELECT version FROM active_release WHERE game_variant = ?1")
          .map_err(ActiveReleaseRepositoryError::Get)?;

        let mut rows = stmt
          .query_map([game_variant.to_string()], |row| row.get(0))
          .map_err(ActiveReleaseRepositoryError::Get)?;

        if let Some(row) = rows.next() {
          row.map(Some).map_err(ActiveReleaseRepositoryError::Get)
        } else {
          Ok(None)
        }
      },
    )
    .await?
  }

  async fn set_active_release(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<(), ActiveReleaseRepositoryError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;
    let version = version.to_string();

    task::spawn_blocking(move || -> Result<(), ActiveReleaseRepositoryError> {
      let conn =
        pool.get().map_err(ActiveReleaseRepositoryError::SetFromPool)?;
      conn
        .execute(
          "INSERT OR REPLACE INTO active_release (game_variant, version) VALUES (?1, ?2)",
          (game_variant.to_string(), version),
        )
        .map_err(ActiveReleaseRepositoryError::Set)?;

      Ok(())
    })
    .await??;
    Ok(())
  }
}
