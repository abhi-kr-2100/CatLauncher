use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tokio::task;

use super::settings_repository::{
  SettingsRepository, SettingsRepositoryError,
};
use crate::settings::fonts::get_font_from_path;
use crate::settings::Settings;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteSettingsRepository {
  pool: Pool,
}

impl SqliteSettingsRepository {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
  async fn get_settings(
    &self,
  ) -> Result<Settings, SettingsRepositoryError> {
    let pool = self.pool.clone();

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

      let mut stmt = conn
        .prepare("SELECT max_backups, parallel_requests, font_path FROM settings WHERE _id = ?1")
        .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

      let (max_backups, parallel_requests, font_path) = match stmt.query_row([1], |row| {
        let max_backups: u16 = row.get(0)?;
        let parallel_requests: u16 = row.get(1)?;
        let font_path: Option<String> = row.get(2)?;
        Ok((max_backups, parallel_requests, font_path))
      }) {
        Ok(res) => res,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
          // If no row exists, return default settings
          let defaults = Settings::default();
          (defaults.max_backups, defaults.parallel_requests, None)
        }
        Err(e) => return Err(SettingsRepositoryError::Get(Box::new(e))),
      };

      let font = font_path
        .map(|p| get_font_from_path(&p))
        .transpose()
        .map_err(SettingsRepositoryError::FontLoad)?;

      Ok(Settings {
        max_backups,
        parallel_requests,
        font,
      })
    })
    .await
    .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?
  }

  async fn update_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SettingsRepositoryError> {
    let pool = self.pool.clone();
    let max_backups = settings.max_backups;
    let parallel_requests = settings.parallel_requests;
    let font_path = settings.font.as_ref().map(|f| f.path.clone());

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| SettingsRepositoryError::Update(Box::new(e)))?;

      conn
        .execute(
          "INSERT OR REPLACE INTO settings (_id, max_backups, parallel_requests, font_path) VALUES (?1, ?2, ?3, ?4)",
          params![1, max_backups, parallel_requests, font_path],
        )
        .map_err(|e| SettingsRepositoryError::Update(Box::new(e)))?;

      Ok(())
    })
    .await
    .map_err(|e| SettingsRepositoryError::Update(Box::new(e)))?
  }
}
