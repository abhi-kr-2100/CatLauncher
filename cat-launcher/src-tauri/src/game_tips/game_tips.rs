use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::fetch_releases::repository::{ReleasesRepository, ReleasesRepositoryError};
use crate::filesystem::paths::{get_tip_file_paths, GetTipFilePathsError};
use crate::game_release::game_release::{GameRelease, GameReleaseStatus};
use crate::infra::utils::OS;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::last_played::last_played::LastPlayedError;
use crate::last_played::repository::LastPlayedVersionRepository;
use crate::variants::GameVariant;

#[derive(Debug, Deserialize, Clone)]
pub struct Tip {
    pub text: Vec<String>,
}

#[derive(Debug, Error)]
pub enum GetAllTipsForVariantError {
    #[error("Failed to get last played version: {0}")]
    LastPlayed(#[from] LastPlayedError),

    #[error("Failed to get tip file paths: {0}")]
    GetTipFilePaths(#[from] GetTipFilePathsError),

    #[error("Tokio IO error: {0}")]
    Tokio(#[from] tokio::io::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("failed to get releases: {0}")]
    GetReleases(#[from] ReleasesRepositoryError),

    #[error("Failed to get installation status: {0}")]
    GetInstallationStatus(#[from] GetInstallationStatusError),
}

async fn get_tips_from_version(
    variant: &GameVariant,
    version: &str,
    data_dir: &Path,
    os: &OS,
) -> Result<Vec<String>, GetAllTipsForVariantError> {
    let tip_file_paths = get_tip_file_paths(variant, version, data_dir, os).await?;
    let mut all_tips: Vec<String> = Vec::new();

    for path in tip_file_paths {
        if path.exists() {
            let tips_file_content = tokio::fs::read_to_string(path).await?;
            if !tips_file_content.is_empty() {
                let tips: Vec<Tip> = serde_json::from_str(&tips_file_content)?;
                all_tips.extend(tips.into_iter().flat_map(|tip| tip.text));
            }
        }
    }

    Ok(all_tips)
}

pub async fn get_all_tips_for_variant(
    variant: &GameVariant,
    data_dir: &Path,
    os: &OS,
    last_played_repository: &dyn LastPlayedVersionRepository,
    releases_repository: &dyn ReleasesRepository,
) -> Result<Vec<String>, GetAllTipsForVariantError> {
    if let Some(last_played_version) = variant
        .get_last_played_version(last_played_repository)
        .await?
    {
        let tips = get_tips_from_version(variant, &last_played_version, data_dir, os).await?;
        return Ok(tips);
    };

    let gh_releases = releases_repository.get_cached_releases(variant).await?;

    let releases = gh_releases.iter().map(|r| {
        let release_type = variant.determine_release_type(&r.tag_name, r.prerelease);

        GameRelease {
            variant: *variant,
            release_type,
            version: r.tag_name.clone(),
            created_at: r.created_at,
            status: GameReleaseStatus::Unknown,
        }
    });

    for release in releases {
        if release.get_installation_status(os, data_dir).await? == GameReleaseStatus::ReadyToPlay {
            let tips = get_tips_from_version(variant, &release.version, data_dir, os).await?;
            return Ok(tips);
        }
    }

    Ok(vec![])
}
