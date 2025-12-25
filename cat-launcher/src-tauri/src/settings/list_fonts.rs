use std::io;
use std::path::{Path, PathBuf};

use strum::{IntoEnumIterator, IntoStaticStr};

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, get_system_font_dirs,
  GetUserGameDataDirError,
};
use crate::infra::utils::{OSNotSupportedError, OS};
use crate::settings::Font;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ListFontsError {
  #[error("failed to get OS: {0}")]
  OSNotSupported(#[from] OSNotSupportedError),

  #[error("failed to read directory: {0}")]
  ReadDirectory(#[from] io::Error),

  #[error("failed to get user game data dir: {0}")]
  UserGameDataDirectory(#[from] GetUserGameDataDirError),
}

pub async fn list_fonts(
  data_dir: &Path,
  os: &OS,
) -> Result<Vec<Font>, ListFontsError> {
  let mut fonts = Vec::new();

  // Get fonts from user game data dir (check all variants)
  for variant in GameVariant::iter() {
    let user_game_data_dir =
      get_or_create_user_game_data_dir(&variant, data_dir).await?;
    let user_font_dir = user_game_data_dir.join("font");
    if tokio::fs::try_exists(&user_font_dir).await.unwrap_or(false) {
      fonts.extend(collect_fonts_from_dir(user_font_dir).await?);
    }
  }

  // Get fonts from system font directories
  let system_font_dirs = get_system_font_dirs(os);
  for dir in system_font_dirs {
    if tokio::fs::try_exists(&dir).await.unwrap_or(false) {
      fonts.extend(collect_fonts_from_dir(dir).await?);
    }
  }

  // Remove duplicates based on font name
  let mut seen = std::collections::HashSet::new();
  fonts.retain(|font| seen.insert(font.name.clone()));

  Ok(fonts)
}

fn collect_fonts_from_dir(
  dir: PathBuf,
) -> std::pin::Pin<
  Box<
    dyn std::future::Future<Output = Result<Vec<Font>, io::Error>>
      + Send,
  >,
> {
  Box::pin(async move {
    let mut fonts = Vec::new();

    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
      let path = entry.path();
      let metadata = match tokio::fs::symlink_metadata(&path).await {
        Ok(m) => m,
        Err(_) => continue,
      };

      if metadata.is_symlink() {
        continue;
      }

      if !metadata.is_file() {
        fonts.extend(collect_fonts_from_dir(path).await?);
        continue;
      }

      if !path.extension().is_some_and(|ext| {
        ext.eq_ignore_ascii_case("ttf")
          || ext.eq_ignore_ascii_case("otf")
          || ext.eq_ignore_ascii_case("ttc")
          || ext.eq_ignore_ascii_case("otc")
      }) {
        continue;
      }

      let data = match tokio::fs::read(&path).await {
        Ok(data) => data,
        Err(_) => continue,
      };

      let n_fonts =
        ttf_parser::fonts_in_collection(&data).unwrap_or(1);
      for i in 0..n_fonts {
        let face = match ttf_parser::Face::parse(&data, i) {
          Ok(face) => face,
          Err(_) => continue,
        };

        if !face.is_monospaced() {
          continue;
        }

        let name = face
          .names()
          .into_iter()
          .find(|name| {
            name.name_id == ttf_parser::name_id::FAMILY
              && name.is_unicode()
          })
          .and_then(|name| name.to_string())
          .unwrap_or_else(|| {
            path
              .file_stem()
              .map(|s| s.to_string_lossy().to_string())
              .unwrap_or_default()
          });

        fonts.push(Font {
          name,
          location: path.to_string_lossy().to_string(),
        });
      }
    }

    Ok(fonts)
  })
}
