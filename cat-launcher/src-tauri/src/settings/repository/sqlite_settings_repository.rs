use std::error::Error;
use std::num::{NonZeroU16, NonZeroUsize};

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::settings::fonts::get_font_from_file;
use crate::settings::repository::settings_repository::{
  GetSettingsError, SaveSettingsError, SettingsRepository,
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

fn map_get_error<E>(e: E) -> GetSettingsError
where
  E: Error + Send + Sync + 'static,
{
  GetSettingsError::Get(Box::new(e))
}

fn map_save_error<E>(e: E) -> SaveSettingsError
where
  E: Error + Send + Sync + 'static,
{
  SaveSettingsError::Save(Box::new(e))
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
  async fn get_settings(&self) -> Result<Settings, GetSettingsError> {
    let pool = self.pool.clone();

    let (max_backups, parallel_requests, font_path) =
      task::spawn_blocking(move || {
        let conn = pool.get().map_err(map_get_error)?;

        let mut stmt = conn
          .prepare(
            "SELECT max_backups, parallel_requests, font_path FROM settings WHERE _id = 1",
          )
          .map_err(map_get_error)?;

        let mut rows = stmt.query([]).map_err(map_get_error)?;

        if let Some(row) = rows.next().map_err(map_get_error)? {
          let max_backups: usize =
            row.get(0).map_err(map_get_error)?;
          let parallel_requests: u16 =
            row.get(1).map_err(map_get_error)?;
          let font_path: Option<String> =
            row.get(2).map_err(map_get_error)?;

          Ok((Some(max_backups), Some(parallel_requests), font_path))
        } else {
          Ok((None, None, None))
        }
      })
      .await
      .map_err(map_get_error)??;

    if let (Some(max_backups), Some(parallel_requests)) =
      (max_backups, parallel_requests)
    {
      let font = if let Some(path) = font_path {
        match get_font_from_file(std::path::Path::new(&path)).await {
          Ok(f) => Some(f),
          Err(e) => {
            eprintln!("Failed to load font from {}: {:?}", path, e);
            None
          }
        }
      } else {
        None
      };

      Ok(Settings {
        max_backups: NonZeroUsize::new(max_backups)
          .ok_or(GetSettingsError::InvalidMaxBackups)?,
        parallel_requests: NonZeroU16::new(parallel_requests)
          .ok_or(GetSettingsError::InvalidParallelRequests)?,
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
  ) -> Result<(), SaveSettingsError> {
    let pool = self.pool.clone();
    let settings = settings.clone();

    task::spawn_blocking(move || {
      let conn = pool.get().map_err(map_save_error)?;

      conn
        .execute(
          "INSERT OR REPLACE INTO settings (_id, max_backups, parallel_requests, font_path) VALUES (1, ?1, ?2, ?3)",
          rusqlite::params![
            settings.max_backups.get(),
            settings.parallel_requests.get(),
            settings.font.as_ref().map(|f| &f.path)
          ],
        )
        .map_err(map_save_error)?;

      Ok(())
    })
    .await
    .map_err(map_save_error)?
  }
}
