use std::collections::HashMap;
use std::num::NonZeroU16;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use ts_rs::TS;

use crate::constants::{
  DEFAULT_MAX_BACKUPS, DEFAULT_PARALLEL_REQUESTS, MAX_BACKUPS,
  MAX_PARALLEL_REQUESTS, MIN_PARALLEL_REQUESTS,
};
use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::settings::apply_settings::SettingsUpdateError;
use crate::settings::{ColorDefWrapper, FontsConfig, ThemeColors};
use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SettingsData {
  #[ts(type = "number")]
  pub max_backups: usize,

  #[ts(type = "number")]
  pub parallel_requests: NonZeroU16,

  pub games: HashMap<String, crate::settings::GameSettings>,

  #[ts(optional)]
  pub font: Option<String>,

  #[ts(optional)]
  pub theme: Option<ThemeColors>,
}

impl Default for SettingsData {
  fn default() -> Self {
    Self {
      max_backups: DEFAULT_MAX_BACKUPS,
      parallel_requests: NonZeroU16::new(DEFAULT_PARALLEL_REQUESTS)
        .expect("DEFAULT_PARALLEL_REQUESTS should be non-zero"),
      games: HashMap::new(),
      font: None,
      theme: None,
    }
  }
}

#[derive(Clone)]
pub struct Settings {
  settings_path: PathBuf,
  data_dir: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("failed to parse JSON: {0}")]
  ParseJson(#[from] serde_json::Error),

  #[error("failed to get user game data dir: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
}

impl Settings {
  pub fn new(settings_path: PathBuf, data_dir: PathBuf) -> Self {
    Self {
      settings_path,
      data_dir,
    }
  }

  async fn load_data(&self) -> Result<SettingsData, SettingsError> {
    if self.settings_path.exists() {
      let content =
        tokio::fs::read_to_string(&self.settings_path).await?;
      Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
      Ok(SettingsData::default())
    }
  }

  async fn write_data(
    &self,
    data: &SettingsData,
  ) -> Result<(), SettingsError> {
    let json = serde_json::to_string_pretty(data)?;
    tokio::fs::write(&self.settings_path, json).await?;
    Ok(())
  }

  pub async fn get_data(
    &self,
  ) -> Result<SettingsData, SettingsError> {
    self.load_data().await
  }

  pub async fn update_max_backups(
    &self,
    max_backups: usize,
  ) -> Result<(), SettingsUpdateError> {
    if max_backups > MAX_BACKUPS {
      return Err(SettingsUpdateError::MaxBackupsInvalid);
    }

    let mut data = self
      .load_data()
      .await
      .map_err(SettingsUpdateError::Settings)?;
    data.max_backups = max_backups;
    self
      .write_data(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;

    Ok(())
  }

  pub async fn update_parallel_requests(
    &self,
    parallel_requests: u16,
  ) -> Result<(), SettingsUpdateError> {
    if !(MIN_PARALLEL_REQUESTS..=MAX_PARALLEL_REQUESTS)
      .contains(&parallel_requests)
    {
      return Err(SettingsUpdateError::ParallelRequestsInvalid);
    }

    let parallel_requests_nz = NonZeroU16::new(parallel_requests)
      .ok_or(SettingsUpdateError::ParallelRequestsInvalid)?;

    let mut data = self
      .load_data()
      .await
      .map_err(SettingsUpdateError::Settings)?;
    data.parallel_requests = parallel_requests_nz;
    self
      .write_data(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;

    Ok(())
  }

  pub async fn update_font(
    &self,
    font_location: Option<String>,
  ) -> Result<(), SettingsUpdateError> {
    if let Some(loc) = &font_location {
      self
        .install_font(loc)
        .await
        .map_err(SettingsUpdateError::Settings)?;
    }

    let mut data = self
      .load_data()
      .await
      .map_err(SettingsUpdateError::Settings)?;
    data.font = font_location.clone();
    self
      .write_data(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;
    self
      .write_fonts_config(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;

    Ok(())
  }

  pub async fn update_theme(
    &self,
    theme: Option<ThemeColors>,
  ) -> Result<(), SettingsUpdateError> {
    let mut data = self
      .load_data()
      .await
      .map_err(SettingsUpdateError::Settings)?;
    data.theme = theme.clone();
    self
      .write_data(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;
    self
      .write_theme_config(&data)
      .await
      .map_err(SettingsUpdateError::Settings)?;

    Ok(())
  }

  async fn write_fonts_config(
    &self,
    data: &SettingsData,
  ) -> Result<(), SettingsError> {
    if let Some(font_location) = &data.font {
      let source_path = std::path::PathBuf::from(font_location);
      let font_name = source_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown.ttf".to_string());

      for variant in GameVariant::iter() {
        let user_game_data_dir =
          get_or_create_user_game_data_dir(&variant, &self.data_dir)
            .await?;

        let config_dir = user_game_data_dir.join("config");
        tokio::fs::create_dir_all(&config_dir).await?;

        let fonts_json_path = config_dir.join("fonts.json");
        let font_path = format!("data/font/{}", font_name);

        let fonts_config = FontsConfig {
          typeface: vec![
            font_path.clone(),
            "data/font/unifont.ttf".to_string(),
          ],
          map_typeface: vec![
            font_path.clone(),
            "data/font/unifont.ttf".to_string(),
          ],
          overmap_typeface: vec![
            font_path,
            "data/font/unifont.ttf".to_string(),
          ],
        };

        let fonts_json = serde_json::to_string_pretty(&fonts_config)?;
        tokio::fs::write(&fonts_json_path, fonts_json).await?;
      }
    }
    Ok(())
  }

  async fn write_theme_config(
    &self,
    data: &SettingsData,
  ) -> Result<(), SettingsError> {
    if let Some(colors) = &data.theme {
      for variant in GameVariant::iter() {
        let user_game_data_dir =
          get_or_create_user_game_data_dir(&variant, &self.data_dir)
            .await?;

        let config_dir = user_game_data_dir.join("config");
        tokio::fs::create_dir_all(&config_dir).await?;

        let base_colors_path = config_dir.join("base_colors.json");
        let color_def = ColorDefWrapper {
          r#type: "colordef".to_string(),
          colors: colors.clone(),
        };

        let base_colors_json =
          serde_json::to_string_pretty(&vec![color_def])?;
        tokio::fs::write(&base_colors_path, base_colors_json).await?;
      }
    }
    Ok(())
  }

  async fn install_font(
    &self,
    font_location: &str,
  ) -> Result<(), SettingsError> {
    for variant in GameVariant::iter() {
      let user_game_data_dir =
        get_or_create_user_game_data_dir(&variant, &self.data_dir)
          .await?;

      let user_font_dir = user_game_data_dir.join("font");
      tokio::fs::create_dir_all(&user_font_dir).await?;

      let source_path = std::path::PathBuf::from(font_location);
      let font_name = source_path.file_name().ok_or_else(|| {
        std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Invalid font location",
        )
      })?;
      let dest_path = user_font_dir.join(font_name);

      if source_path != dest_path {
        tokio::fs::copy(&source_path, &dest_path).await?;
      }
    }

    Ok(())
  }

  pub async fn max_backups(&self) -> Result<usize, SettingsError> {
    Ok(self.load_data().await?.max_backups)
  }

  pub async fn parallel_requests(
    &self,
  ) -> Result<NonZeroU16, SettingsError> {
    Ok(self.load_data().await?.parallel_requests)
  }

  pub async fn games(
    &self,
  ) -> Result<
    HashMap<String, crate::settings::GameSettings>,
    SettingsError,
  > {
    Ok(self.load_data().await?.games)
  }

  pub async fn font(&self) -> Result<Option<String>, SettingsError> {
    Ok(self.load_data().await?.font)
  }

  pub async fn theme(
    &self,
  ) -> Result<Option<ThemeColors>, SettingsError> {
    Ok(self.load_data().await?.theme)
  }
}
