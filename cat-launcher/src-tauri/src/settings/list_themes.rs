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

#[derive(thiserror::Error, Debug)]
pub enum GetThemesForVariantError {
  #[error("failed to get active release: {0}")]
  Repository(#[from] ActiveReleaseRepositoryError),

  #[error("failed to get game resources dir: {0}")]
  GameResourcesDirectory(#[from] GetGameExecutableDirError),

  #[error("failed to read directory: {0}")]
  ReadDirectory(#[from] io::Error),
}

pub async fn list_themes(
  data_dir: PathBuf,
  repository: &impl ActiveReleaseRepository,
  os: &OS,
) -> Result<Vec<ColorTheme>, ListThemesError> {
  let mut themes = Vec::new();

  for variant in GameVariant::iter() {
    match get_themes_for_variant(&variant, &data_dir, repository, os)
      .await
    {
      Ok(variant_themes) => themes.extend(variant_themes),
      Err(GetThemesForVariantError::Repository(e)) => {
        return Err(ListThemesError::ActiveRelease(e))
      }
      Err(GetThemesForVariantError::GameResourcesDirectory(e)) => {
        return Err(ListThemesError::GameResourcesDirectory(e))
      }
      Err(GetThemesForVariantError::ReadDirectory(e)) => {
        return Err(ListThemesError::ReadDirectory(e))
      }
    }
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
) -> Result<Vec<ColorTheme>, GetThemesForVariantError> {
  let release = match repository.get_active_release(variant).await {
    Ok(Some(release)) => release,
    Ok(None) => return Ok(Vec::new()),
    Err(e) => return Err(GetThemesForVariantError::Repository(e)),
  };

  let resources_dir =
    get_game_resources_dir(variant, &release, data_dir, os)
      .await
      .map_err(GetThemesForVariantError::GameResourcesDirectory)?;

  let themes_dir =
    resources_dir.join("data").join("raw").join("color_themes");

  match tokio::fs::try_exists(&themes_dir).await {
    Ok(true) => {}
    Ok(false) => return Ok(Vec::new()),
    Err(e) => return Err(GetThemesForVariantError::ReadDirectory(e)),
  }

  let mut entries = tokio::fs::read_dir(themes_dir)
    .await
    .map_err(GetThemesForVariantError::ReadDirectory)?;

  let mut themes = Vec::new();
  while let Ok(Some(entry)) = entries.next_entry().await {
    if let Some(theme) = parse_theme_file(&entry.path()).await {
      themes.push(theme);
    }
  }

  Ok(themes)
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
