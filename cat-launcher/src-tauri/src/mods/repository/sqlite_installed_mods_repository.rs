use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::mods::repository::{InstalledModsRepository, InstalledModsRepositoryError};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub struct SqliteInstalledModsRepository {
    pool: Pool,
}

impl SqliteInstalledModsRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstalledModsRepository for SqliteInstalledModsRepository {
    async fn add_installed_mod(&self, mod_id: &str, game_variant: &GameVariant) -> Result<(), InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let mod_id = mod_id.to_string();
        let game_variant = game_variant.id().to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Add(Box::new(e)))?;
            conn.execute(
                "INSERT OR IGNORE INTO installed_mods (mod_id, game_variant) VALUES (?1, ?2)",
                rusqlite::params![mod_id, game_variant],
            )
            .map_err(|e| InstalledModsRepositoryError::Add(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Add(Box::new(e)))?
    }

    async fn get_all_installed_mods(&self, game_variant: &GameVariant) -> Result<Vec<String>, InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let game_variant = game_variant.id().to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?;
            let mut stmt = conn
                .prepare("SELECT mod_id FROM installed_mods WHERE game_variant = ?1")
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?;
            let mods = stmt
                .query_map(rusqlite::params![game_variant], |row| row.get(0))
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?
                .collect::<Result<Vec<String>, _>>()
                .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?;
            Ok(mods)
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Get(Box::new(e)))?
    }

    async fn is_mod_installed(&self, mod_id: &str, game_variant: &GameVariant) -> Result<bool, InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let mod_id = mod_id.to_string();
        let game_variant = game_variant.id().to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Check(Box::new(e)))?;
            let mut stmt = conn
                .prepare("SELECT 1 FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2")
                .map_err(|e| InstalledModsRepositoryError::Check(Box::new(e)))?;
            let exists = stmt
                .exists(rusqlite::params![mod_id, game_variant])
                .map_err(|e| InstalledModsRepositoryError::Check(Box::new(e)))?;
            Ok(exists)
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Check(Box::new(e)))?
    }

    async fn delete_installed_mod(&self, mod_id: &str, game_variant: &GameVariant) -> Result<(), InstalledModsRepositoryError> {
        let pool = self.pool.clone();
        let mod_id = mod_id.to_string();
        let game_variant = game_variant.id().to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?;
            conn.execute(
                "DELETE FROM installed_mods WHERE mod_id = ?1 AND game_variant = ?2",
                rusqlite::params![mod_id, game_variant],
            )
            .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| InstalledModsRepositoryError::Delete(Box::new(e)))?
    }
}
