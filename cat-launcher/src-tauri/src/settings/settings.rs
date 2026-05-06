use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

use crate::settings::types::{ColorTheme, Font};

/// Represents the application-wide settings.
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[derive(Default)]
pub struct Settings {
  /// The selected font for the game.
  pub font: Option<Font>,
  /// The selected color theme for the game.
  pub color_theme: Option<ColorTheme>,
}

/// Errors that can occur when loading settings from a file.
#[derive(Debug, Error)]
pub enum LoadSettingsError {
  /// An error occurred while opening the settings file.
  #[error("Could not open settings.json")]
  OpenFile(#[source] std::io::Error),

  /// An error occurred while parsing the settings file.
  #[error("Could not parse settings.json")]
  Parse(#[from] serde_json::Error),
}
