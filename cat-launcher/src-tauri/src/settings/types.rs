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
