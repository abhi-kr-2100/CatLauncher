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
