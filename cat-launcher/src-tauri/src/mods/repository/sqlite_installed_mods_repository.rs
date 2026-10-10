use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_helper::run_db;
use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::variants::GameVariant;

pub struct SqliteInstalledModsRepository {
  pool: r2d2::Pool<SqliteConnectionManager>,
}

impl SqliteInstalledModsRepository {
  pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl InstalledModsRepository for SqliteInstalledModsRepository {
  async fn add_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "INSERT OR IGNORE INTO installed_mods (mod_id, game_variant) VALUES (?1, ?2)",
        [&mod_id, &variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledModsRepositoryError::Add)
  }

  async fn delete_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id_clone = mod_id.to_string();
    let variant_name_clone = game_variant.to_string();

    let rows_affected = run_db(pool, move |conn| {
      let count = conn.execute(
        "DELETE FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
        [&mod_id_clone, &variant_name_clone],
      )?;
      Ok::<usize, rusqlite::Error>(count)
    })
    .await
    .map_err(InstalledModsRepositoryError::Delete)?;

    if rows_affected == 0 {
      return Err(InstalledModsRepositoryError::NotFound(
        mod_id.to_string(),
        game_variant.to_string(),
      ));
    }

    Ok(())
  }

  async fn delete_all_installed_mods(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "DELETE FROM installed_mods WHERE game_variant = ?1",
        [&variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledModsRepositoryError::DeleteAll)
  }

  async fn is_mod_installed(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledModsRepositoryError> {
    let pool = self.pool.clone();
    let mod_id = mod_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      let mut stmt = conn.prepare(
        "SELECT 1 FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
      )?;
      stmt.exists([&mod_id, &variant_name])
    })
    .await
    .map_err(InstalledModsRepositoryError::IsInstalled)
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

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_sqlite_installed_mods_repository_add_and_is_installed()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    assert!(!repo.is_mod_installed("mod1", &variant).await?);

    repo.add_installed_mod("mod1", &variant).await?;
    assert!(repo.is_mod_installed("mod1", &variant).await?);

    // Idempotent add
    repo.add_installed_mod("mod1", &variant).await?;
    assert!(repo.is_mod_installed("mod1", &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_mods_repository_delete() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    repo.add_installed_mod("mod_del", &variant).await?;
    assert!(repo.is_mod_installed("mod_del", &variant).await?);

    repo.delete_installed_mod("mod_del", &variant).await?;
    assert!(!repo.is_mod_installed("mod_del", &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_mods_repository_delete_not_found()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result = repo.delete_installed_mod("missing", &variant).await;
    assert!(matches!(
      result,
      Err(InstalledModsRepositoryError::NotFound(id, var)) if id == "missing" && var == variant.to_string()
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_mods_repository_delete_all()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let dda = GameVariant::DarkDaysAhead;
    let bn = GameVariant::BrightNights;

    repo.add_installed_mod("m1", &dda).await?;
    repo.add_installed_mod("m2", &dda).await?;
    repo.add_installed_mod("m3", &bn).await?;

    assert!(repo.is_mod_installed("m1", &dda).await?);
    assert!(repo.is_mod_installed("m2", &dda).await?);
    assert!(repo.is_mod_installed("m3", &bn).await?);

    repo.delete_all_installed_mods(&dda).await?;

    assert!(!repo.is_mod_installed("m1", &dda).await?);
    assert!(!repo.is_mod_installed("m2", &dda).await?);
    assert!(repo.is_mod_installed("m3", &bn).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_mods_repository_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());

    repo
      .add_installed_mod("shared_id", &GameVariant::DarkDaysAhead)
      .await?;

    assert!(
      repo
        .is_mod_installed("shared_id", &GameVariant::DarkDaysAhead)
        .await?
    );
    assert!(
      !repo
        .is_mod_installed("shared_id", &GameVariant::BrightNights)
        .await?
    );
    assert!(
      !repo
        .is_mod_installed(
          "shared_id",
          &GameVariant::TheLastGeneration
        )
        .await?
    );

    Ok(())
  }
}
