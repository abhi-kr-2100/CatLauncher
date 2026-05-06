use std::path::Path;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents a font available on the system.
#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct Font {
  /// The display name of the font.
  pub name: String,
  /// The absolute path to the font file.
  pub path: String,
}

/// Represents a color theme for the game.
#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct ColorTheme {
  /// A unique identifier for the theme.
  pub id: String,
  /// The display name of the theme.
  pub name: String,
  /// The absolute path to the theme's configuration file.
  pub path: String,
}

impl ColorTheme {
  /// Attempts to create a `ColorTheme` from a file path.
  ///
  /// The filename must start with `base_colors-` or `base_colors_` and end with `.json`.
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
