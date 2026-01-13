use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

use crate::settings::types::{ColorTheme, Font};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[derive(Default)]
pub struct Settings {
  pub font: Option<Font>,
  pub color_theme: Option<ColorTheme>,
}

#[derive(Debug, Error)]
pub enum LoadSettingsError {
  #[error("Could not open settings.json")]
  OpenFile(#[source] std::io::Error),

  #[error("Could not parse settings.json")]
  Parse(#[from] serde_json::Error),
}
