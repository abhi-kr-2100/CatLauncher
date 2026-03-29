use std::path::Path;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct Font {
  pub name: String,
  pub path: String,
}

#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct ColorTheme {
  pub id: String,
  pub name: String,
  pub path: String,
}

#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct FontSettings {
  pub font_size: i32,
  pub font_width: i32,
  pub font_height: i32,
  pub map_font_size: i32,
  pub map_font_width: i32,
  pub map_font_height: i32,
  pub overmap_font_size: i32,
  pub overmap_font_width: i32,
  pub overmap_font_height: i32,
}

impl Default for FontSettings {
  fn default() -> Self {
    Self {
      font_size: 16,
      font_width: 8,
      font_height: 16,
      map_font_size: 16,
      map_font_width: 16,
      map_font_height: 16,
      overmap_font_size: 16,
      overmap_font_width: 16,
      overmap_font_height: 16,
    }
  }
}

impl ColorTheme {
  pub fn from_path(path: &Path) -> Option<Self> {
    let filename = path.file_name()?.to_str()?;

    let id = filename
      .strip_prefix("base_colors-")
      .or_else(|| filename.strip_prefix("base_colors_"))?
      .strip_suffix(".json")?
      .to_string();

    Some(ColorTheme {
      id: id.clone(),
      name: id,
      path: path.to_string_lossy().into_owned(),
    })
  }
}
