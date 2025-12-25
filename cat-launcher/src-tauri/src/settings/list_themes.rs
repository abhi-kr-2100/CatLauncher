use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};

use serde_json;
use strum::{IntoEnumIterator, IntoStaticStr};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::filesystem::paths::{
  get_game_resources_dir, GetGameExecutableDirError,
};
use crate::infra::utils::{OSNotSupportedError, OS};
use crate::settings::{ColorDefWrapper, ColorTheme};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ListThemesError {
  #[error("failed to get OS: {0}")]
  OSNotSupported(#[from] OSNotSupportedError),

  #[error("failed to read directory: {0}")]
  ReadDirectory(#[from] io::Error),

  #[error("failed to get game resources dir: {0}")]
  GameResourcesDirectory(#[from] GetGameExecutableDirError),

  #[error("failed to parse theme file: {0}")]
  ParseFile(#[from] serde_json::Error),

  #[error("failed to get active release: {0}")]
  ActiveRelease(#[from] ActiveReleaseRepositoryError),
}

pub async fn list_themes(
  data_dir: PathBuf,
  repository: &impl ActiveReleaseRepository,
  os: &OS,
) -> Result<Vec<ColorTheme>, ListThemesError> {
  let mut themes = Vec::new();

  for variant in GameVariant::iter() {
    let variant_themes =
      get_themes_for_variant(&variant, &data_dir, repository, os)
        .await;
    themes.extend(variant_themes);
  }

  // Remove duplicates
  let mut seen = HashSet::new();
  themes.retain(|theme| seen.insert(theme.name.clone()));

  Ok(themes)
}

async fn get_themes_for_variant(
  variant: &GameVariant,
  data_dir: &Path,
  repository: &impl ActiveReleaseRepository,
  os: &OS,
) -> Vec<ColorTheme> {
  let release = match repository.get_active_release(variant).await {
    Ok(Some(release)) => release,
    _ => return Vec::new(),
  };

  let resources_dir =
    match get_game_resources_dir(variant, &release, data_dir, os)
      .await
    {
      Ok(dir) => dir,
      _ => return Vec::new(),
    };

  let themes_dir =
    resources_dir.join("data").join("raw").join("color_themes");

  if !tokio::fs::try_exists(&themes_dir).await.unwrap_or(false) {
    return Vec::new();
  }

  let mut entries = match tokio::fs::read_dir(themes_dir).await {
    Ok(entries) => entries,
    _ => return Vec::new(),
  };

  let mut themes = Vec::new();
  while let Ok(Some(entry)) = entries.next_entry().await {
    if let Some(theme) = parse_theme_file(&entry.path()).await {
      themes.push(theme);
    }
  }

  themes
}

async fn parse_theme_file(path: &Path) -> Option<ColorTheme> {
  let metadata = tokio::fs::metadata(path).await.ok()?;
  if !metadata.is_file() {
    return None;
  }

  match path.extension() {
    Some(ext) if ext.eq_ignore_ascii_case("json") => {}
    _ => return None,
  }

  let name = path.file_stem()?.to_string_lossy().to_string();
  let content = tokio::fs::read_to_string(path).await.ok()?;
  let color_defs: Vec<ColorDefWrapper> =
    serde_json::from_str(&content).ok()?;

  // A color def file contains only one color definition, despite containing an array.
  color_defs.first().map(|color_def| ColorTheme {
    name,
    colors: color_def.colors.clone(),
  })
}
