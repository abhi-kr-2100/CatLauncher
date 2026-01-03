use std::path::{Path, PathBuf};

use font_kit::font::Font as FontKitFont;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use walkdir::WalkDir;

use crate::infra::utils::OS;
use crate::settings::paths::get_font_directories;

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export)]
pub struct Font {
  pub name: String,
  pub path: String,
}

#[derive(thiserror::Error, Debug)]
pub enum FontError {
  #[error("failed to list fonts: {0}")]
  List(String),
}

fn get_fonts_in_directory(dir: PathBuf) -> Vec<Font> {
  let mut fonts = Vec::new();
  for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_file() {
      if let Ok(font) = FontKitFont::from_path(path, 0) {
        let name = font.full_name();
        let path_str = path.to_string_lossy().to_string();
        fonts.push(Font {
          name,
          path: path_str,
        });
      }
    }
  }
  fonts
}

pub fn get_available_fonts(os: &OS) -> Result<Vec<Font>, FontError> {
  let mut fonts = Vec::new();
  let directories = get_font_directories(os);

  for dir in directories {
    if dir.as_os_str().is_empty() || !dir.exists() {
      continue;
    }
    fonts.extend(get_fonts_in_directory(dir));
  }

  fonts.sort_by(|a, b| a.name.cmp(&b.name));
  fonts.dedup_by(|a, b| a.name == b.name);

  Ok(fonts)
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum GetFontFromPathError {
  #[error("font file does not exist")]
  DoesNotExist,
  #[error("failed to load font: {0}")]
  Load(String),
}

pub fn get_font_from_path(
  path: &str,
) -> Result<Font, GetFontFromPathError> {
  let path_obj = Path::new(path);
  if !path_obj.exists() {
    return Err(GetFontFromPathError::DoesNotExist);
  }
  match FontKitFont::from_path(path_obj, 0) {
    Ok(font) => Ok(Font {
      name: font.full_name(),
      path: path.to_string(),
    }),
    Err(e) => Err(GetFontFromPathError::Load(e.to_string())),
  }
}
