use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::infra::utils::OS;
use crate::settings::paths::get_font_directories;
use crate::settings::types::Font;

#[derive(thiserror::Error, Debug)]
pub enum GetFontError {
  #[error("failed to read font file: {0}")]
  Read(#[from] std::io::Error),
  #[error("failed to parse font file: {0}")]
  Parse(#[from] ttf_parser::FaceParsingError),
  #[error("font is not monospaced")]
  NotMonospaced,
  #[error("font name not found")]
  NameNotFound,
}

pub async fn get_font_from_file(
  path: &Path,
) -> Result<Font, GetFontError> {
  let data = tokio::fs::read(path).await?;
  let face = ttf_parser::Face::parse(&data, 0)?;

  if !face.is_monospaced() {
    return Err(GetFontError::NotMonospaced);
  }

  let name = face
    .names()
    .into_iter()
    .find(|name| {
      name.name_id == ttf_parser::name_id::FULL_NAME
        && name.is_unicode()
    })
    .or_else(|| {
      face.names().into_iter().find(|name| {
        name.name_id == ttf_parser::name_id::FAMILY
          && name.is_unicode()
      })
    })
    .ok_or(GetFontError::NameNotFound)?;

  let name_str =
    name.to_string().ok_or(GetFontError::NameNotFound)?;

  Ok(Font {
    name: name_str,
    path: path.to_string_lossy().into_owned(),
  })
}

async fn get_fonts_in_dir_recursive(dir: PathBuf) -> Vec<Font> {
  let mut fonts = Vec::new();
  let mut dirs = vec![dir];

  while let Some(current_dir) = dirs.pop() {
    let mut entries = match tokio::fs::read_dir(current_dir).await {
      Ok(e) => e,
      Err(_) => continue,
    };

    loop {
      match entries.next_entry().await {
        Ok(Some(entry)) => {
          let path = entry.path();
          if let Ok(file_type) = entry.file_type().await {
            if file_type.is_dir() {
              dirs.push(path);
            } else if let Some(ext) =
              path.extension().and_then(|e| e.to_str())
            {
              let ext = ext.to_lowercase();
              if ext == "ttf" || ext == "otf" {
                if let Ok(font) = get_font_from_file(&path).await {
                  fonts.push(font);
                }
              }
            }
          }
        }
        Ok(None) => break,
        Err(e) => {
          eprintln!("Error reading directory entry: {}", e);
          continue;
        }
      }
    }
  }
  fonts
}

pub async fn get_all_fonts(os: OS) -> Vec<Font> {
  let dirs = get_font_directories(&os);
  let mut fonts = HashSet::new();

  for dir in dirs {
    if tokio::fs::try_exists(&dir).await.unwrap_or(false) {
      let dir_fonts = get_fonts_in_dir_recursive(dir).await;
      for font in dir_fonts {
        fonts.insert(font);
      }
    }
  }

  let mut fonts_vec: Vec<Font> = fonts.into_iter().collect();
  fonts_vec.sort_by(|a, b| a.name.cmp(&b.name));
  fonts_vec
}
