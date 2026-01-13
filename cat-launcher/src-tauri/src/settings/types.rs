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
    let filename = path.file_name().and_then(|n| n.to_str())?;

    if filename.starts_with("base_colors-")
      && filename.ends_with(".json")
    {
      let id = filename
        .trim_start_matches("base_colors-")
        .trim_end_matches(".json")
        .to_string();

      Some(ColorTheme {
        id: id.clone(),
        name: id,
        path: path.to_string_lossy().into_owned(),
      })
    } else {
      None
    }
  }
}
