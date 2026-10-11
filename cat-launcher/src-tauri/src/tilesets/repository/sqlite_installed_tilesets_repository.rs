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

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use strum::IntoEnumIterator;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_add_and_is_installed()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    assert!(!repo.is_tileset_installed("pack1", &variant).await?);

    repo.add_installed_tileset("pack1", &variant).await?;
    assert!(repo.is_tileset_installed("pack1", &variant).await?);

    // Idempotent add
    repo.add_installed_tileset("pack1", &variant).await?;
    assert!(repo.is_tileset_installed("pack1", &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_delete()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    repo.add_installed_tileset("pack_del", &variant).await?;
    assert!(repo.is_tileset_installed("pack_del", &variant).await?);

    repo.delete_installed_tileset("pack_del", &variant).await?;
    assert!(!repo.is_tileset_installed("pack_del", &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_delete_not_found()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result =
      repo.delete_installed_tileset("missing", &variant).await;
    assert!(matches!(
      result,
      Err(InstalledTilesetsRepositoryError::NotFound(id, var)) if id == "missing" && var == variant.to_string()
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_delete_all()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());
    let dda = GameVariant::DarkDaysAhead;
    let bn = GameVariant::BrightNights;

    repo.add_installed_tileset("p1", &dda).await?;
    repo.add_installed_tileset("p2", &dda).await?;
    repo.add_installed_tileset("p3", &bn).await?;

    assert!(repo.is_tileset_installed("p1", &dda).await?);
    assert!(repo.is_tileset_installed("p2", &dda).await?);
    assert!(repo.is_tileset_installed("p3", &bn).await?);

    repo.delete_all_installed_tilesets(&dda).await?;

    assert!(!repo.is_tileset_installed("p1", &dda).await?);
    assert!(!repo.is_tileset_installed("p2", &dda).await?);
    assert!(repo.is_tileset_installed("p3", &bn).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    repo
      .add_installed_tileset("shared_id", &GameVariant::DarkDaysAhead)
      .await?;

    assert!(
      repo
        .is_tileset_installed(
          "shared_id",
          &GameVariant::DarkDaysAhead
        )
        .await?
    );
    assert!(
      !repo
        .is_tileset_installed("shared_id", &GameVariant::BrightNights)
        .await?
    );
    assert!(
      !repo
        .is_tileset_installed(
          "shared_id",
          &GameVariant::TheLastGeneration
        )
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_tilesets_repository_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledTilesetsRepository::new(db.pool().clone());

    for variant in GameVariant::iter() {
      let pack_id = format!("pack-{}", variant);
      assert!(!repo.is_tileset_installed(&pack_id, &variant).await?);

      repo.add_installed_tileset(&pack_id, &variant).await?;
      assert!(repo.is_tileset_installed(&pack_id, &variant).await?);

      repo.delete_installed_tileset(&pack_id, &variant).await?;
      assert!(!repo.is_tileset_installed(&pack_id, &variant).await?);
    }

    Ok(())
  }
}
