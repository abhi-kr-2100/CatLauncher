use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::variants::links::Link;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Font {
  pub name: String,
  pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ColorTheme {
  pub name: String,
  pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorDefWrapper {
  pub r#type: String,
  #[serde(flatten)]
  pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontsConfig {
  pub typeface: Vec<String>,
  pub map_typeface: Vec<String>,
  pub overmap_typeface: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct GameSettings {
  pub name: String,
  pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export)]
#[allow(non_snake_case)]
pub struct ThemeColors {
  pub BLACK: [u8; 3],
  pub DGRAY: [u8; 3],
  pub GRAY: [u8; 3],
  pub WHITE: [u8; 3],
  pub MAGENTA: [u8; 3],
  pub LMAGENTA: [u8; 3],
  pub RED: [u8; 3],
  pub LRED: [u8; 3],
  pub BROWN: [u8; 3],
  pub YELLOW: [u8; 3],
  pub LGREEN: [u8; 3],
  pub GREEN: [u8; 3],
  pub LCYAN: [u8; 3],
  pub CYAN: [u8; 3],
  pub LBLUE: [u8; 3],
  pub BLUE: [u8; 3],
}
