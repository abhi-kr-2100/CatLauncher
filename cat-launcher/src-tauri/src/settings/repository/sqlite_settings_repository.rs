use std::error::Error;
use std::path::{Path, PathBuf};

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

pub async fn load_font_settings_from_game_options(
  data_dir: &Path,
) -> Result<crate::settings::types::FontSettings, GetSettingsError> {
  use crate::settings::types::FontSettings;
  use crate::settings::update_font_files::load_font_settings_from_options;
  use crate::variants::GameVariant;
  use strum::IntoEnumIterator;

  let mut font_settings = FontSettings::default();

  // Try to load from each game variant's options.json
  for variant in GameVariant::iter() {
    let config_dir = data_dir.join(variant.to_string());
    if config_dir.exists() {
      match load_font_settings_from_options(&config_dir).await {
        Ok(settings) => {
          font_settings = settings;
          break;
        }
        Err(e) => {
          eprintln!("Failed to load font settings from options.json for {}: {}", variant, e);
          continue;
        }
      }
    }
  }

  Ok(font_settings)
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
  async fn get_settings(&self) -> Result<Settings, GetSettingsError> {
    let pool = self.pool.clone();

    // First try to load from game options.json files
    // Use a default path for now - this will be enhanced later
    let mut data_dir = PathBuf::from(".");
    if let Ok(home_dir) = std::env::var("HOME") {
      data_dir = PathBuf::from(home_dir);
      data_dir.push(".cataclysm-dda");
    }

    let font_settings_from_options =
      load_font_settings_from_game_options(&data_dir).await?;

    let (font_path, font_settings, theme_path) =
      task::spawn_blocking(move || {
        let conn = pool.get().map_err(map_get_error)?;

        // Get Font Path
        let mut stmt = conn
          .prepare("SELECT font_path FROM settings WHERE _id = 1")
          .map_err(map_get_error)?;
        let mut rows = stmt.query([]).map_err(map_get_error)?;
        let font_path =
          if let Some(row) = rows.next().map_err(map_get_error)? {
            row.get::<usize, Option<String>>(0).map_err(map_get_error)?
          } else {
            None
          };

        // Get Font Sizes from separate table
        let mut stmt = conn
          .prepare("SELECT font_size, font_width, font_height, map_font_size, map_font_width, map_font_height, overmap_font_size, overmap_font_width, overmap_font_height FROM font_size_settings WHERE _id = 1")
          .map_err(map_get_error)?;
        let mut rows = stmt.query([]).map_err(map_get_error)?;
        let font_settings =
          if let Some(row) = rows.next().map_err(map_get_error)? {
            crate::settings::types::FontSettings {
              font_size: row.get::<usize, i32>(0).map_err(map_get_error)?,
              font_width: row.get::<usize, i32>(1).map_err(map_get_error)?,
              font_height: row.get::<usize, i32>(2).map_err(map_get_error)?,
              map_font_size: row.get::<usize, i32>(3).map_err(map_get_error)?,
              map_font_width: row.get::<usize, i32>(4).map_err(map_get_error)?,
              map_font_height: row.get::<usize, i32>(5).map_err(map_get_error)?,
              overmap_font_size: row.get::<usize, i32>(6).map_err(map_get_error)?,
              overmap_font_width: row.get::<usize, i32>(7).map_err(map_get_error)?,
              overmap_font_height: row.get::<usize, i32>(8).map_err(map_get_error)?,
            }
          } else {
            crate::settings::types::FontSettings::default()
          };

        // Try to load font settings from options.json if available
        // Note: This is handled in a separate async context since we can't await in blocking context
        // The font_size_settings table will be used as the primary source

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

        Ok((
          font_path,
          font_settings,
          theme_path,
        ))
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

    // Use values from options.json if available, otherwise use database values
    // Check if any options.json value is different from defaults
    let use_options_values = font_settings_from_options
      != crate::settings::types::FontSettings::default();

    let final_font_settings = if use_options_values {
      font_settings_from_options
    } else {
      font_settings
    };

    Ok(Settings {
      font,
      font_size: final_font_settings.font_size,
      font_width: final_font_settings.font_width,
      font_height: final_font_settings.font_height,
      map_font_size: final_font_settings.map_font_size,
      map_font_width: final_font_settings.map_font_width,
      map_font_height: final_font_settings.map_font_height,
      overmap_font_size: final_font_settings.overmap_font_size,
      overmap_font_width: final_font_settings.overmap_font_width,
      overmap_font_height: final_font_settings.overmap_font_height,
      color_theme,
    })
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
        rusqlite::params![
          settings.font.as_ref().map(|f| &f.path),
        ],
      )
      .map_err(map_save_error)?;

      tx.execute(
        "INSERT OR REPLACE INTO font_size_settings (_id, font_size, font_width, font_height, map_font_size, map_font_width, map_font_height, overmap_font_size, overmap_font_width, overmap_font_height) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
          settings.font_size,
          settings.font_width,
          settings.font_height,
          settings.map_font_size,
          settings.map_font_width,
          settings.map_font_height,
          settings.overmap_font_size,
          settings.overmap_font_width,
          settings.overmap_font_height
        ],
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
