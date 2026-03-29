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

    update_options_json(&config_dir, settings).await?;
  }

  Ok(())
}

async fn update_options_json(
  config_dir: &Path,
  settings: &Settings,
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

  let mut blending_found = false;
  let mut size_found = false;
  let mut width_found = false;
  let mut height_found = false;
  let mut map_size_found = false;
  let mut map_width_found = false;
  let mut map_height_found = false;
  let mut overmap_size_found = false;
  let mut overmap_width_found = false;
  let mut overmap_height_found = false;

  for entry in options_array.iter_mut() {
    let entry_obj = entry
      .as_object_mut()
      .ok_or(EnsureFontBlendingError::BadOptionsJson)?;

    let name = entry_obj.get("name").and_then(|v| v.as_str());
    match name {
      Some("FONT_BLENDING") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String("true".to_string()),
        );
        blending_found = true;
      }
      Some("FONT_SIZE") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(settings.font_size.to_string()),
        );
        size_found = true;
      }
      Some("FONT_WIDTH") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(settings.font_width.to_string()),
        );
        width_found = true;
      }
      Some("FONT_HEIGHT") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(settings.font_height.to_string()),
        );
        height_found = true;
      }
      Some("MAP_FONT_SIZE") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.map_font_size.to_string(),
          ),
        );
        map_size_found = true;
      }
      Some("MAP_FONT_WIDTH") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.map_font_width.to_string(),
          ),
        );
        map_width_found = true;
      }
      Some("MAP_FONT_HEIGHT") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.map_font_height.to_string(),
          ),
        );
        map_height_found = true;
      }
      Some("OVERMAP_FONT_SIZE") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.overmap_font_size.to_string(),
          ),
        );
        overmap_size_found = true;
      }
      Some("OVERMAP_FONT_WIDTH") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.overmap_font_width.to_string(),
          ),
        );
        overmap_width_found = true;
      }
      Some("OVERMAP_FONT_HEIGHT") => {
        entry_obj.insert(
          "value".to_string(),
          serde_json::Value::String(
            settings.overmap_font_height.to_string(),
          ),
        );
        overmap_height_found = true;
      }
      _ => {}
    }
  }

  if !blending_found {
    options_array.push(serde_json::json!({
        "name": "FONT_BLENDING",
        "value": "true"
    }));
  }
  if !size_found {
    options_array.push(serde_json::json!({
        "name": "FONT_SIZE",
        "value": settings.font_size.to_string()
    }));
  }
  if !width_found {
    options_array.push(serde_json::json!({
        "name": "FONT_WIDTH",
        "value": settings.font_width.to_string()
    }));
  }
  if !height_found {
    options_array.push(serde_json::json!({
        "name": "FONT_HEIGHT",
        "value": settings.font_height.to_string()
    }));
  }
  if !map_size_found {
    options_array.push(serde_json::json!({
        "name": "MAP_FONT_SIZE",
        "value": settings.map_font_size.to_string()
    }));
  }
  if !map_width_found {
    options_array.push(serde_json::json!({
        "name": "MAP_FONT_WIDTH",
        "value": settings.map_font_width.to_string()
    }));
  }
  if !map_height_found {
    options_array.push(serde_json::json!({
        "name": "MAP_FONT_HEIGHT",
        "value": settings.map_font_height.to_string()
    }));
  }
  if !overmap_size_found {
    options_array.push(serde_json::json!({
        "name": "OVERMAP_FONT_SIZE",
        "value": settings.overmap_font_size.to_string()
    }));
  }
  if !overmap_width_found {
    options_array.push(serde_json::json!({
        "name": "OVERMAP_FONT_WIDTH",
        "value": settings.overmap_font_width.to_string()
    }));
  }
  if !overmap_height_found {
    options_array.push(serde_json::json!({
        "name": "OVERMAP_FONT_HEIGHT",
        "value": settings.overmap_font_height.to_string()
    }));
  }

  let content = serde_json::to_string_pretty(&options)?;
  tokio::fs::write(&options_path, content)
    .await
    .map_err(EnsureFontBlendingError::WriteOptionsJson)?;

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

pub async fn load_font_settings_from_options(
  config_dir: &Path,
) -> Result<
  crate::settings::types::FontSettings,
  EnsureFontBlendingError,
> {
  let options_path = config_dir.join("options.json");
  let content = match tokio::fs::read_to_string(&options_path).await {
    Ok(content) => content,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      return Ok(crate::settings::types::FontSettings::default());
    }
    Err(e) => {
      return Err(EnsureFontBlendingError::ReadOptionsJson(e))
    }
  };

  let options: serde_json::Value = serde_json::from_str(&content)?;

  let options_array = options
    .as_array()
    .ok_or(EnsureFontBlendingError::BadOptionsJson)?;

  let mut font_settings =
    crate::settings::types::FontSettings::default();

  for entry in options_array.iter() {
    let entry_obj = entry
      .as_object()
      .ok_or(EnsureFontBlendingError::BadOptionsJson)?;

    let name = entry_obj.get("name").and_then(|v| v.as_str());
    let value = entry_obj.get("value").and_then(|v| v.as_str());

    match name {
      Some("FONT_SIZE") => {
        if let Some(size) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.font_size = size;
        }
      }
      Some("FONT_WIDTH") => {
        if let Some(width) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.font_width = width;
        }
      }
      Some("FONT_HEIGHT") => {
        if let Some(height) =
          value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.font_height = height;
        }
      }
      Some("MAP_FONT_SIZE") => {
        if let Some(size) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.map_font_size = size;
        }
      }
      Some("MAP_FONT_WIDTH") => {
        if let Some(width) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.map_font_width = width;
        }
      }
      Some("MAP_FONT_HEIGHT") => {
        if let Some(height) =
          value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.map_font_height = height;
        }
      }
      Some("OVERMAP_FONT_SIZE") => {
        if let Some(size) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.overmap_font_size = size;
        }
      }
      Some("OVERMAP_FONT_WIDTH") => {
        if let Some(width) = value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.overmap_font_width = width;
        }
      }
      Some("OVERMAP_FONT_HEIGHT") => {
        if let Some(height) =
          value.and_then(|v| v.parse::<i32>().ok())
        {
          font_settings.overmap_font_height = height;
        }
      }
      _ => {}
    }
  }

  Ok(font_settings)
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
