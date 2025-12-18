use thiserror::Error;

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::fetch_releases::repository::{
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::filesystem::paths::{
  get_tip_file_paths, GetTipFilePathsError,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::game_release::utils::gh_release_to_game_release;
use crate::game_tips::types::Tip;
use crate::infra::utils::OS;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

#[derive(Debug, Error)]
pub enum GetAllTipsForVariantError {
  #[error("failed to get tip file paths: {0}")]
  GetTipFilePaths(#[from] GetTipFilePathsError),

  #[error("serde json error: {0}")]
  SerdeJson(#[from] serde_json::Error),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  #[error("tokio io error: {0}")]
  Tokio(#[from] tokio::io::Error),

  #[error("failed to get installation status: {0}")]
  GetInstallationStatus(#[from] GetInstallationStatusError),

  #[error("failed to get cached releases: {0}")]
  GetCachedReleases(#[from] ReleasesRepositoryError),
}

async fn get_tips_from_version(
  variant: &GameVariant,
  version: &str,
  data_dir: &std::path::Path,
  os: &OS,
) -> Result<Vec<String>, GetAllTipsForVariantError> {
  let tip_file_paths =
    get_tip_file_paths(variant, version, data_dir, os).await?;
  let mut all_tips: Vec<String> = Vec::new();

  for path in tip_file_paths {
    match tokio::fs::read_to_string(path).await {
      Ok(tips_file_content) => {
        if !tips_file_content.is_empty() {
          let tips: Vec<Tip> =
            serde_json::from_str(&tips_file_content)?;
          all_tips.extend(tips.into_iter().flat_map(|tip| tip.text));
        }
      }
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        // File not found, just skip it
      }
      Err(e) => return Err(e.into()),
    }
  }

  Ok(all_tips)
}

pub async fn get_all_tips_for_variant(
  variant: &GameVariant,
  data_dir: &std::path::Path,
  os: &OS,
  active_release_repository: &(dyn ActiveReleaseRepository
      + Send
      + Sync),
  releases_repository: &(dyn ReleasesRepository + Send + Sync),
) -> Result<Vec<String>, GetAllTipsForVariantError> {
  if let Some(active_release) = active_release_repository
    .get_active_release(variant)
    .await?
  {
    let tips =
      get_tips_from_version(variant, &active_release, data_dir, os)
        .await?;
    return Ok(tips);
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
      let tips = get_tips_from_version(
        variant,
        &release.version,
        data_dir,
        os,
      )
      .await?;
      return Ok(tips);
    }
  }

  Ok(vec![])
}
