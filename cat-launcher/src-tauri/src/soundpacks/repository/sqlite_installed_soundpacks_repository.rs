use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;

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
impl InstalledSoundpacksRepository for SqliteInstalledSoundpacksRepository {
    async fn add_installed_soundpack(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let soundpack_id = soundpack_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            conn.execute(
                "INSERT OR IGNORE INTO installed_soundpacks (soundpack_id, game_variant) VALUES (?1, ?2)",
                [&soundpack_id, &variant_name],
            )
            .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            Ok(())
        })
        .await
        .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?
    }

    async fn get_installed_soundpacks_for_variant(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<String>, InstalledSoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            let mut stmt = conn
                .prepare("SELECT soundpack_id FROM installed_soundpacks WHERE game_variant = ?1")
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            let soundpack_ids = stmt
                .query_map([&variant_name], |row| row.get::<_, String>(0))
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            Ok(soundpack_ids)
        })
        .await
        .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?
    }

    async fn delete_installed_soundpack(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let soundpack_id = soundpack_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            let rows_affected = conn
                .execute(
                    "DELETE FROM installed_soundpacks WHERE soundpack_id = ?1 AND game_variant = ?2",
                    [&soundpack_id, &variant_name],
                )
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            if rows_affected == 0 {
                return Err(InstalledSoundpacksRepositoryError::NotFound(soundpack_id, variant_name));
            }

            Ok(())
        })
        .await
        .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?
    }

    async fn is_soundpack_installed(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<bool, InstalledSoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let soundpack_id = soundpack_id.to_string();
        let variant_name = game_variant.to_string();

        tokio::task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            let mut stmt = conn
                .prepare("SELECT 1 FROM installed_soundpacks WHERE soundpack_id = ?1 AND game_variant = ?2")
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            let exists = stmt
                .exists([&soundpack_id, &variant_name])
                .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?;

            Ok(exists)
        })
        .await
        .map_err(|e| InstalledSoundpacksRepositoryError::Database(e.to_string()))?
    }
}