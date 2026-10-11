use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_helper::run_db;
use crate::soundpacks::repository::installed_soundpacks_repository::{
  InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::variants::GameVariant;

pub struct SqliteInstalledSoundpacksRepository {
  pool: r2d2::Pool<SqliteConnectionManager>,
}

impl SqliteInstalledSoundpacksRepository {
  pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl InstalledSoundpacksRepository
  for SqliteInstalledSoundpacksRepository
{
  async fn add_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError> {
    let pool = self.pool.clone();
    let soundpack_id = soundpack_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "INSERT OR IGNORE INTO installed_soundpacks (soundpack_id, game_variant) VALUES (?1, ?2)",
        [&soundpack_id, &variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledSoundpacksRepositoryError::Add)
  }

  async fn delete_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError> {
    let pool = self.pool.clone();
    let soundpack_id_clone = soundpack_id.to_string();
    let variant_name_clone = game_variant.to_string();

    let rows_affected = run_db(pool, move |conn| {
      let count = conn.execute(
        "DELETE FROM installed_soundpacks WHERE soundpack_id = ?1 AND game_variant = ?2",
        [&soundpack_id_clone, &variant_name_clone],
      )?;
      Ok::<usize, rusqlite::Error>(count)
    })
    .await
    .map_err(InstalledSoundpacksRepositoryError::Delete)?;

    if rows_affected == 0 {
      return Err(InstalledSoundpacksRepositoryError::NotFound(
        soundpack_id.to_string(),
        game_variant.to_string(),
      ));
    }

    Ok(())
  }

  async fn delete_all_installed_soundpacks(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError> {
    let pool = self.pool.clone();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      conn.execute(
        "DELETE FROM installed_soundpacks WHERE game_variant = ?1",
        [&variant_name],
      )?;
      Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(InstalledSoundpacksRepositoryError::DeleteAll)
  }

  async fn is_soundpack_installed(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledSoundpacksRepositoryError> {
    let pool = self.pool.clone();
    let soundpack_id = soundpack_id.to_string();
    let variant_name = game_variant.to_string();

    run_db(pool, move |conn| {
      let mut stmt = conn.prepare(
        "SELECT 1 FROM installed_soundpacks WHERE soundpack_id = ?1 AND game_variant = ?2",
      )?;
      stmt.exists([&soundpack_id, &variant_name])
    })
    .await
    .map_err(InstalledSoundpacksRepositoryError::IsInstalled)
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
  async fn test_sqlite_installed_soundpacks_repository_add_and_is_installed()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    assert!(!repo.is_soundpack_installed("pack1", &variant).await?);

    repo.add_installed_soundpack("pack1", &variant).await?;
    assert!(repo.is_soundpack_installed("pack1", &variant).await?);

    // Idempotent add
    repo.add_installed_soundpack("pack1", &variant).await?;
    assert!(repo.is_soundpack_installed("pack1", &variant).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_soundpacks_repository_delete()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    repo.add_installed_soundpack("pack_del", &variant).await?;
    assert!(repo.is_soundpack_installed("pack_del", &variant).await?);

    repo
      .delete_installed_soundpack("pack_del", &variant)
      .await?;
    assert!(
      !repo.is_soundpack_installed("pack_del", &variant).await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_soundpacks_repository_delete_not_found()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let result =
      repo.delete_installed_soundpack("missing", &variant).await;
    assert!(matches!(
      result,
      Err(InstalledSoundpacksRepositoryError::NotFound(id, var)) if id == "missing" && var == variant.to_string()
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_soundpacks_repository_delete_all()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let dda = GameVariant::DarkDaysAhead;
    let bn = GameVariant::BrightNights;

    repo.add_installed_soundpack("p1", &dda).await?;
    repo.add_installed_soundpack("p2", &dda).await?;
    repo.add_installed_soundpack("p3", &bn).await?;

    assert!(repo.is_soundpack_installed("p1", &dda).await?);
    assert!(repo.is_soundpack_installed("p2", &dda).await?);
    assert!(repo.is_soundpack_installed("p3", &bn).await?);

    repo.delete_all_installed_soundpacks(&dda).await?;

    assert!(!repo.is_soundpack_installed("p1", &dda).await?);
    assert!(!repo.is_soundpack_installed("p2", &dda).await?);
    assert!(repo.is_soundpack_installed("p3", &bn).await?);

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_soundpacks_repository_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());

    repo
      .add_installed_soundpack(
        "shared_id",
        &GameVariant::DarkDaysAhead,
      )
      .await?;

    assert!(
      repo
        .is_soundpack_installed(
          "shared_id",
          &GameVariant::DarkDaysAhead
        )
        .await?
    );
    assert!(
      !repo
        .is_soundpack_installed(
          "shared_id",
          &GameVariant::BrightNights
        )
        .await?
    );
    assert!(
      !repo
        .is_soundpack_installed(
          "shared_id",
          &GameVariant::TheLastGeneration
        )
        .await?
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_sqlite_installed_soundpacks_repository_all_variants()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let pack_id = format!("pack-{}", variant);
      assert!(
        !repo.is_soundpack_installed(&pack_id, &variant).await?
      );

      repo.add_installed_soundpack(&pack_id, &variant).await?;
      assert!(repo.is_soundpack_installed(&pack_id, &variant).await?);

      repo.delete_installed_soundpack(&pack_id, &variant).await?;
      assert!(
        !repo.is_soundpack_installed(&pack_id, &variant).await?
      );
    }

    Ok(())
  }
}
