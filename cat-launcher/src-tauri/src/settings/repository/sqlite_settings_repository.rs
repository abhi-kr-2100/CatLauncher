use std::error::Error;
use std::path::Path;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::settings::fonts::get_font_from_file;
use crate::settings::repository::settings_repository::{
  GetSettingsError, SaveSettingsError, SettingsRepository,
};
use crate::settings::types::ColorTheme;
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

    let (font_path, theme_path) = task::spawn_blocking(move || {
      let conn = pool.get().map_err(map_get_error)?;

      // Get Font Path
      let mut stmt = conn
        .prepare("SELECT font_path FROM settings WHERE _id = 1")
        .map_err(map_get_error)?;
      let mut rows = stmt.query([]).map_err(map_get_error)?;
      let font_path: Option<String> =
        if let Some(row) = rows.next().map_err(map_get_error)? {
          row.get(0).map_err(map_get_error)?
        } else {
          None
        };

      // Get Color Theme Path
      let mut stmt = conn
        .prepare(
          "SELECT theme_path FROM color_settings WHERE _id = 1",
        )
        .map_err(map_get_error)?;
      let mut rows = stmt.query([]).map_err(map_get_error)?;
      let theme_path: Option<String> =
        if let Some(row) = rows.next().map_err(map_get_error)? {
          row.get(0).map_err(map_get_error)?
        } else {
          None
        };

      Ok((font_path, theme_path))
    })
    .await
    .map_err(map_get_error)??;

    let font = if let Some(path) = font_path {
      get_font_from_file(Path::new(&path)).await.ok()
    } else {
      None
    };

    let color_theme = theme_path.and_then(|path_str| {
      ColorTheme::from_path(Path::new(&path_str))
    });

    Ok(Settings { font, color_theme })
  }

  async fn save_settings(
    &self,
    settings: &Settings,
  ) -> Result<(), SaveSettingsError> {
    let pool = self.pool.clone();
    let settings = settings.clone();

    task::spawn_blocking(move || {
      let mut conn = pool.get().map_err(map_save_error)?;

      let tx = conn.transaction().map_err(map_save_error)?;

      tx.execute(
          "INSERT OR REPLACE INTO settings (_id, font_path) VALUES (1, ?1)",
          rusqlite::params![settings.font.as_ref().map(|f| &f.path)],
        )
        .map_err(map_save_error)?;

      tx.execute(
        "INSERT OR REPLACE INTO color_settings (_id, theme_path) VALUES (1, ?1)",
        rusqlite::params![settings.color_theme.as_ref().map(|t| &t.path)],
      )
      .map_err(map_save_error)?;

      tx.commit().map_err(map_save_error)?;

      Ok(())
    })
    .await
    .map_err(map_save_error)?
  }
}
