use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::settings::types::{ColorTheme, Font};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Settings {
  pub font: Option<Font>,
  pub font_size: i32,
  pub font_width: i32,
  pub font_height: i32,
  pub map_font_size: i32,
  pub map_font_width: i32,
  pub map_font_height: i32,
  pub overmap_font_size: i32,
  pub overmap_font_width: i32,
  pub overmap_font_height: i32,
  pub color_theme: Option<ColorTheme>,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      font: None,
      font_size: 16,
      font_width: 8,
      font_height: 16,
      map_font_size: 16,
      map_font_width: 16,
      map_font_height: 16,
      overmap_font_size: 16,
      overmap_font_width: 16,
      overmap_font_height: 16,
      color_theme: None,
    }
  }
}
