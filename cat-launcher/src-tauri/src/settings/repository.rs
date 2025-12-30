use async_trait::async_trait;
use sqlx::SqlitePool;
use thiserror::Error;

use crate::settings::Settings;

#[derive(Debug, Error)]
pub enum SettingsRepositoryError {
    #[error("Could not find settings")]
    NotFound,
    #[error(transparent)]
    Other(#[from] sqlx::Error),
}

#[async_trait]
pub trait SettingsRepository {
    async fn get_settings(&self) -> Result<Settings, SettingsRepositoryError>;
    async fn update_settings(&self, settings: &Settings) -> Result<(), SettingsRepositoryError>;
}

#[derive(Debug, Clone)]
pub struct SqliteSettingsRepository {
    pool: SqlitePool,
}

impl SqliteSettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn get_settings(&self) -> Result<Settings, SettingsRepositoryError> {
        let settings = sqlx::query_as::<_, Settings>("SELECT max_backups, parallel_requests FROM settings WHERE id = 1")
            .fetch_one(&self.pool)
            .await?;

        Ok(settings)
    }

    async fn update_settings(&self, settings: &Settings) -> Result<(), SettingsRepositoryError> {
        sqlx::query("UPDATE settings SET max_backups = ?, parallel_requests = ? WHERE id = 1")
            .bind(settings.max_backups.get() as i64)
            .bind(settings.parallel_requests.get() as i64)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
