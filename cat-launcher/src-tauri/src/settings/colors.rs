use std::collections::HashMap;
use std::path::Path;

use strum::IntoEnumIterator;
use tokio::fs;

use crate::active_release::repository::active_release_repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::filesystem::paths::get_game_resources_dir;
use crate::infra::utils::OS;
use crate::settings::types::ColorTheme;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetColorThemesError {
  #[error("failed to get game resources directory: {0}")]
  ResourcesDir(
    #[from] crate::filesystem::paths::GetGameExecutableDirError,
  ),

  #[error("failed to read directory: {0}")]
  ReadDir(#[from] std::io::Error),

  #[error("failed to get active release: {0}")]
  ActiveRelease(#[from] ActiveReleaseRepositoryError),
}

pub async fn get_available_color_themes(
  data_dir: &Path,
  resource_dir: &Path,
  active_release_repo: &SqliteActiveReleaseRepository,
  os: &OS,
) -> Result<Vec<ColorTheme>, GetColorThemesError> {
  let mut themes_map = HashMap::new();

  // 1. Get bundled themes
  let bundled_themes_dir =
    resource_dir.join("content").join("themes");
  if fs::try_exists(&bundled_themes_dir).await.unwrap_or(false) {
    let bundled_themes =
      get_themes_from_dir(&bundled_themes_dir).await?;
    themes_map
      .extend(bundled_themes.into_iter().map(|t| (t.id.clone(), t)));
  }

  // 2. Get game themes (prioritize over bundled)
  for variant in GameVariant::iter() {
    let active_release =
      active_release_repo.get_active_release(&variant).await?;

    if let Some(version) = active_release {
      match get_game_resources_dir(&variant, &version, data_dir, os)
        .await
      {
        Ok(resources_dir) => {
          let themes_dir = resources_dir
            .join("data")
            .join("raw")
            .join("color_themes");
          if fs::try_exists(&themes_dir).await.unwrap_or(false) {
            let game_themes =
              get_themes_from_dir(&themes_dir).await?;
            themes_map.extend(
              game_themes.into_iter().map(|t| (t.id.clone(), t)),
            );
          }
        }
        Err(_) => continue,
      }
    }
  }

  let mut themes: Vec<ColorTheme> =
    themes_map.into_values().collect();
  themes.sort_by(|a, b| a.name.cmp(&b.name));

  Ok(themes)
}

async fn get_themes_from_dir(
  dir: &Path,
) -> Result<Vec<ColorTheme>, GetColorThemesError> {
  let mut themes = Vec::new();
  let mut entries = fs::read_dir(dir).await?;

  while let Some(entry) = entries.next_entry().await? {
    let path = entry.path();
    if let Some(theme) = ColorTheme::from_path(&path) {
      themes.push(theme);
    }
  }

  themes.sort_by(|a, b| a.name.cmp(&b.name));

  Ok(themes)
}
