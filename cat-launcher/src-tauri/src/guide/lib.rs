use serde_json::Value;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::{read_dir, read_to_string};

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::fetch_releases::repository::{
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::filesystem::paths::{
  get_game_resources_dir, GetGameExecutableDirError,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::game_release::utils::gh_release_to_game_release;
use crate::guide::types::{GuideEntityDetail, GuideEntry};
use crate::infra::utils::OS;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(Debug, Error)]
pub enum GuideError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  #[error("failed to get cached releases: {0}")]
  GetCachedReleases(#[from] ReleasesRepositoryError),

  #[error("failed to get game resources dir: {0}")]
  GetGameResourcesDir(#[from] GetGameExecutableDirError),

  #[error("failed to get installation status: {0}")]
  GetInstallationStatus(#[from] GetInstallationStatusError),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("serde json error: {0}")]
  SerdeJson(#[from] serde_json::Error),

  #[error("entity not found")]
  NotFound,
}

pub async fn get_active_or_installed_version(
  variant: &GameVariant,
  data_dir: &Path,
  os: &OS,
  active_release_repository: &(dyn ActiveReleaseRepository
      + Send
      + Sync),
  releases_repository: &(dyn ReleasesRepository + Send + Sync),
) -> Result<Option<String>, GuideError> {
  if let Some(active_release) = active_release_repository
    .get_active_release(variant)
    .await?
  {
    return Ok(Some(active_release));
  }

  let gh_releases =
    releases_repository.get_cached_releases(variant).await?;
  let releases: Vec<GameRelease> = gh_releases
    .iter()
    .map(|r| gh_release_to_game_release(r, variant))
    .collect();

  for release in releases {
    if release.get_installation_status(os, data_dir).await?
      == GameReleaseStatus::ReadyToPlay
    {
      return Ok(Some(release.version));
    }
  }

  Ok(None)
}

async fn get_all_json_files(
  dir: PathBuf,
) -> Result<Vec<PathBuf>, std::io::Error> {
  let mut files = Vec::new();
  let mut dirs = vec![dir];

  while let Some(current_dir) = dirs.pop() {
    let mut entries = read_dir(current_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
      let path = entry.path();
      if path.is_dir() {
        dirs.push(path);
      } else if path.extension().and_then(|s| s.to_str())
        == Some("json")
      {
        files.push(path);
      }
    }
  }

  Ok(files)
}

pub async fn search_guide(
  query: String,
  variant: GameVariant,
  data_dir: &Path,
  os: &OS,
  active_release_repository: &(dyn ActiveReleaseRepository
      + Send
      + Sync),
  releases_repository: &(dyn ReleasesRepository + Send + Sync),
) -> Result<Vec<GuideEntry>, GuideError> {
  let version = get_active_or_installed_version(
    &variant,
    data_dir,
    os,
    active_release_repository,
    releases_repository,
  )
  .await?;

  let version = match version {
    Some(v) => v,
    None => return Ok(vec![]),
  };

  let resources_dir =
    get_game_resources_dir(&variant, &version, data_dir, os).await?;
  let json_dir = resources_dir.join("data").join("json");

  if !json_dir.exists() {
    return Ok(vec![]);
  }

  let json_files = get_all_json_files(json_dir).await?;
  let mut entries = Vec::new();
  let query_lower = query.to_lowercase();

  for file in json_files {
    let content = read_to_string(file).await?;
    let json: Value = match serde_json::from_str(&content) {
      Ok(j) => j,
      Err(_) => continue,
    };

    let items = if json.is_array() {
      json.as_array().unwrap().clone()
    } else {
      vec![json]
    };

    for item in items {
      if let Some(obj) = item.as_object() {
        let id = obj
          .get("id")
          .and_then(|v| v.as_str())
          .or_else(|| obj.get("abstract").and_then(|v| v.as_str()));

        if let Some(id_str) = id {
          let name = obj.get("name").and_then(|v| {
            if v.is_string() {
              v.as_str().map(|s| s.to_string())
            } else if v.is_object() {
              v.get("str")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
            } else {
              None
            }
          });

          let entry_type = obj
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

          if id_str.to_lowercase().contains(&query_lower)
            || name
              .as_ref()
              .map(|n| n.to_lowercase().contains(&query_lower))
              .unwrap_or(false)
          {
            entries.push(GuideEntry {
              id: id_str.to_string(),
              name,
              entry_type,
            });
          }
        }
      }
    }

    if entries.len() > 100 {
      break;
    }
  }

  Ok(entries)
}

pub async fn get_guide_entity(
  id: String,
  variant: GameVariant,
  data_dir: &Path,
  os: &OS,
  active_release_repository: &(dyn ActiveReleaseRepository
      + Send
      + Sync),
  releases_repository: &(dyn ReleasesRepository + Send + Sync),
) -> Result<GuideEntityDetail, GuideError> {
  let version = get_active_or_installed_version(
    &variant,
    data_dir,
    os,
    active_release_repository,
    releases_repository,
  )
  .await?;

  let version = match version {
    Some(v) => v,
    None => return Err(GuideError::NotFound),
  };

  let resources_dir =
    get_game_resources_dir(&variant, &version, data_dir, os).await?;
  let json_dir = resources_dir.join("data").join("json");

  if !json_dir.exists() {
    return Err(GuideError::NotFound);
  }

  let json_files = get_all_json_files(json_dir).await?;

  for file in json_files {
    let content = read_to_string(file).await?;
    let json: Value = match serde_json::from_str(&content) {
      Ok(j) => j,
      Err(_) => continue,
    };

    let items = if json.is_array() {
      json.as_array().unwrap().clone()
    } else {
      vec![json]
    };

    for item in items {
      if let Some(obj) = item.as_object() {
        let entry_id = obj
          .get("id")
          .and_then(|v| v.as_str())
          .or_else(|| obj.get("abstract").and_then(|v| v.as_str()));

        if let Some(id_str) = entry_id {
          if id_str == id {
            return Ok(GuideEntityDetail { raw_json: item });
          }
        }
      }
    }
  }

  Err(GuideError::NotFound)
}
