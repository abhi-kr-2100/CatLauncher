use std::collections::HashMap;
use std::path::Path;

use strum::IntoEnumIterator;

use crate::filesystem::paths::GetUserGameDataDirError;
use crate::settings::consts::FALLBACK_FONTS;
use crate::settings::paths::{
  get_or_create_user_config_dir, GetOrCreateUserConfigDirError,
};
use crate::settings::types::Font;
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum EnsureFontBlendingError {
  #[error("failed to read existing options.json: {0}")]
  ReadOptionsJson(#[source] std::io::Error),

  #[error("failed to serialize JSON: {0}")]
  Json(#[from] serde_json::Error),

  #[error("failed to write options.json: {0}")]
  WriteOptionsJson(#[source] std::io::Error),

  #[error("bad options.json file")]
  BadOptionsJson,
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateFontFilesError {
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get or create user config directory: {0}")]
  GetOrCreateUserConfigDir(#[from] GetOrCreateUserConfigDirError),

  #[error("failed to read existing fonts.json: {0}")]
  ReadFontsJson(#[source] std::io::Error),

  #[error("failed to serialize JSON: {0}")]
  Json(#[from] serde_json::Error),

  #[error("failed to write fonts.json: {0}")]
  WriteFontsJson(#[source] std::io::Error),

  #[error("failed to ensure font blending: {0}")]
  EnsureFontBlending(#[from] EnsureFontBlendingError),
}

pub async fn update_font_files(
  data_dir: &Path,
  settings: &Settings,
) -> Result<(), UpdateFontFilesError> {
  let selected_font = &settings.font;

  for variant in GameVariant::iter() {
    let config_dir =
      get_or_create_user_config_dir(&variant, data_dir).await?;

    let fonts_json_path = config_dir.join("fonts.json");

    let mut fonts_map: HashMap<String, Vec<String>> =
      match tokio::fs::read_to_string(&fonts_json_path).await {
        Ok(content) => {
          serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!(
              "Failed to parse fonts.json at {:?}: {}",
              fonts_json_path, e
            );
            create_default_fonts_map(&variant)
          })
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
          create_default_fonts_map(&variant)
        }
        Err(e) => return Err(UpdateFontFilesError::ReadFontsJson(e)),
      };

    let updated_list = get_updated_typeface_list(selected_font);
    let supported_categories =
      variant.supported_typeface_categories();

    for category in supported_categories {
      fonts_map.insert(category.to_string(), updated_list.clone());
    }

    let content = serde_json::to_string_pretty(&fonts_map)?;
    if let Err(e) = tokio::fs::write(&fonts_json_path, content).await
    {
      return Err(UpdateFontFilesError::WriteFontsJson(e));
    }

    ensure_font_blending(&config_dir).await?;
  }

  Ok(())
}

async fn ensure_font_blending(
  config_dir: &Path,
) -> Result<(), EnsureFontBlendingError> {
  let options_path = config_dir.join("options.json");
  let content = match tokio::fs::read_to_string(&options_path).await {
    Ok(content) => content,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      return Ok(());
    }
    Err(e) => {
      return Err(EnsureFontBlendingError::ReadOptionsJson(e))
    }
  };

  let mut options: serde_json::Value =
    serde_json::from_str(&content)?;

  let options_array = options
    .as_array_mut()
    .ok_or(EnsureFontBlendingError::BadOptionsJson)?;

  let mut modified = false;
  for entry in options_array {
    let entry_obj = entry
      .as_object_mut()
      .ok_or(EnsureFontBlendingError::BadOptionsJson)?;

    if entry_obj.get("name").and_then(|v| v.as_str())
      == Some("FONT_BLENDING")
    {
      entry_obj.insert(
        "value".to_string(),
        serde_json::Value::String("true".to_string()),
      );
      modified = true;
      break;
    }
  }

  if modified {
    let content = serde_json::to_string_pretty(&options)?;
    tokio::fs::write(&options_path, content)
      .await
      .map_err(EnsureFontBlendingError::WriteOptionsJson)?;
  }

  Ok(())
}

fn create_default_fonts_map(
  variant: &GameVariant,
) -> HashMap<String, Vec<String>> {
  let fallbacks: Vec<String> =
    FALLBACK_FONTS.iter().map(|s| s.to_string()).collect();

  let mut map = HashMap::new();
  for category in variant.supported_typeface_categories() {
    map.insert(category.to_string(), fallbacks.clone());
  }
  map
}

fn get_updated_typeface_list(
  selected_font: &Option<Font>,
) -> Vec<String> {
  let mut new_list = Vec::new();

  if let Some(font) = selected_font {
    new_list.push(font.path.clone());
  }

  for font in FALLBACK_FONTS {
    new_list.push(font.to_string());
  }

  new_list
}
