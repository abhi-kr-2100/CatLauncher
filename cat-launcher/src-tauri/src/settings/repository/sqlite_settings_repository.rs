use std::num::{NonZeroU16, NonZeroUsize};

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::settings::fonts::get_font_from_file;
use crate::settings::repository::settings_repository::{
  SettingsRepository, SettingsRepositoryError,
};
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

    let (max_backups, parallel_requests, font_path) =
      task::spawn_blocking(move || {
        let conn = pool
          .get()
          .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

        let mut stmt = conn
          .prepare(
            "SELECT max_backups, parallel_requests, font_path FROM settings WHERE _id = 1",
          )
          .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

        let mut rows = stmt
          .query([])
          .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

        if let Some(row) = rows
          .next()
          .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?
        {
          let max_backups: usize = row
            .get(0)
            .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;
          let parallel_requests: u16 = row
            .get(1)
            .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;
          let font_path: Option<String> = row
            .get(2)
            .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))?;

          Ok((Some(max_backups), Some(parallel_requests), font_path))
        } else {
          Ok((None, None, None))
        }
      })
      .await
      .map_err(|e| SettingsRepositoryError::Get(Box::new(e)))??;

    if let (Some(max_backups), Some(parallel_requests)) =
      (max_backups, parallel_requests)
    {
      let font = if let Some(path) = font_path {
        get_font_from_file(std::path::Path::new(&path)).await.ok()
      } else {
        None
      };

      Ok(Settings {
        max_backups: NonZeroUsize::new(max_backups).ok_or_else(
          || {
            SettingsRepositoryError::Get(Box::new(
              std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "max_backups must be non-zero",
              ),
            ))
          },
        )?,
        parallel_requests: NonZeroU16::new(parallel_requests)
          .ok_or_else(|| {
            SettingsRepositoryError::Get(Box::new(
              std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "parallel_requests must be non-zero",
              ),
            ))
          })?,
        font,
      })
    } else {
      let default_settings = Settings::default();
      Ok(default_settings)
    }
  }

  async fn save_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SettingsRepositoryError> {
    let pool = self.pool.clone();
    let settings = settings.clone();

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| SettingsRepositoryError::Save(Box::new(e)))?;

      conn
        .execute(
          "INSERT OR REPLACE INTO settings (_id, max_backups, parallel_requests, font_path) VALUES (1, ?1, ?2, ?3)",
          rusqlite::params![
            settings.max_backups.get(),
            settings.parallel_requests.get(),
            settings.font.as_ref().map(|f| &f.path)
          ],
        )
        .map_err(|e| SettingsRepositoryError::Save(Box::new(e)))?;

      Ok(())
    })
    .await
    .map_err(|e| SettingsRepositoryError::Save(Box::new(e)))?
  }
}
