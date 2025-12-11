use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;
use tokio::task;

use crate::soundpacks::models::InstalledSoundpackMetadata;
use crate::soundpacks::repository::soundpacks_repository::{
    SoundpacksRepository, SoundpacksRepositoryError,
};
use crate::variants::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub struct SqliteSoundpacksRepository {
    pool: Pool,
}

impl SqliteSoundpacksRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SoundpacksRepository for SqliteSoundpacksRepository {
    async fn get_installed_soundpack(
        &self,
        variant: &GameVariant,
        soundpack_id: &str,
    ) -> Result<Option<InstalledSoundpackMetadata>, SoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let variant = variant.clone();
        let soundpack_id = soundpack_id.to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| SoundpacksRepositoryError::Get(Box::new(e)))?;

            let mut stmt = conn
                .prepare(
                    "SELECT soundpack_id, game_variant, installed_last_updated_time
                     FROM installed_soundpacks
                     WHERE game_variant = ?1 AND soundpack_id = ?2",
                )
                .map_err(|e| SoundpacksRepositoryError::Get(Box::new(e)))?;

            let result = stmt
                .query_row([variant.to_string(), soundpack_id], |row| {
                    let soundpack_id: String = row.get(0)?;
                    let variant_str: String = row.get(1)?;
                    let installed_last_updated_time: String = row.get(2)?;

                    let variant: GameVariant = variant_str
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?;

                    let installed_last_updated_time = installed_last_updated_time
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?;

                    Ok(InstalledSoundpackMetadata {
                        soundpack_id,
                        variant,
                        installed_last_updated_time,
                    })
                })
                .optional()
                .map_err(|e| SoundpacksRepositoryError::Get(Box::new(e)))?;

            Ok(result)
        })
        .await
        .map_err(|e| SoundpacksRepositoryError::Get(Box::new(e)))?
    }

    async fn save_installed_soundpack(
        &self,
        metadata: &InstalledSoundpackMetadata,
    ) -> Result<(), SoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let metadata = metadata.clone();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| SoundpacksRepositoryError::Save(Box::new(e)))?;

            conn.execute(
                "INSERT OR REPLACE INTO installed_soundpacks 
                 (soundpack_id, game_variant, installed_last_updated_time) 
                 VALUES (?1, ?2, ?3)",
                (
                    &metadata.soundpack_id,
                    metadata.variant.to_string(),
                    metadata.installed_last_updated_time.to_rfc3339(),
                ),
            )
            .map_err(|e| SoundpacksRepositoryError::Save(Box::new(e)))?;

            Ok(())
        })
        .await
        .map_err(|e| SoundpacksRepositoryError::Save(Box::new(e)))?
    }

    async fn delete_installed_soundpack(
        &self,
        variant: &GameVariant,
        soundpack_id: &str,
    ) -> Result<(), SoundpacksRepositoryError> {
        let pool = self.pool.clone();
        let variant = variant.clone();
        let soundpack_id = soundpack_id.to_string();

        task::spawn_blocking(move || {
            let conn = pool
                .get()
                .map_err(|e| SoundpacksRepositoryError::Delete(Box::new(e)))?;

            conn.execute(
                "DELETE FROM installed_soundpacks 
                 WHERE game_variant = ?1 AND soundpack_id = ?2",
                [variant.to_string(), soundpack_id],
            )
            .map_err(|e| SoundpacksRepositoryError::Delete(Box::new(e)))?;

            Ok(())
        })
        .await
        .map_err(|e| SoundpacksRepositoryError::Delete(Box::new(e)))?
    }
}
