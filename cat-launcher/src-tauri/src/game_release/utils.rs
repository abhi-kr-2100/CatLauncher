use std::path::Path;

use crate::fetch_releases::utils::{get_cached_releases, get_default_releases, merge_releases};
use crate::game_release::game_release::{GameReleaseStatus, ReleaseType};
use crate::game_release::GameRelease;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

pub fn get_platform_asset_substr(variant: &GameVariant, os: &str) -> Option<&'static str> {
    match (variant, os) {
        (GameVariant::DarkDaysAhead, "windows") => Some("windows-with-graphics-and-sounds"),
        (GameVariant::DarkDaysAhead, "macos") => Some("osx-terminal-only"),
        (GameVariant::DarkDaysAhead, "linux") => Some("linux-with-graphics-and-sounds"),
        (GameVariant::BrightNights, "windows") => Some("windows-tiles"),
        (GameVariant::BrightNights, "macos") => Some("osx-tiles-arm"),
        (GameVariant::BrightNights, "linux") => Some("linux-tiles"),
        (GameVariant::TheLastGeneration, "windows") => Some("windows-tiles-sounds-x64-msvc"),
        (GameVariant::TheLastGeneration, "macos") => Some("osx-tiles-universal"),
        (GameVariant::TheLastGeneration, "linux") => Some("linux-tiles-sounds"),
        _ => None,
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetReleaseError {
    #[error("failed to get release status: {0}")]
    Status(#[from] GetInstallationStatusError),

    #[error("release with ID {0} not found")]
    NotFound(String),
}

pub async fn get_release_by_id(
    variant: &GameVariant,
    release_id: &str,
    os: &str,
    cache_dir: &Path,
    data_dir: &Path,
    resources_dir: &Path,
) -> Result<GameRelease, GetReleaseError> {
    let cached_releases = get_cached_releases(variant, cache_dir).await;
    let default_releases = get_default_releases(variant, resources_dir).await;
    let gh_releases = merge_releases(&cached_releases, &default_releases);

    let gh_release = match gh_releases.into_iter().find(|r| r.tag_name == release_id) {
        Some(r) => r,
        None => return Err(GetReleaseError::NotFound(release_id.into())),
    };

    let mut release = GameRelease {
        variant: variant.clone(),
        version: gh_release.tag_name,
        release_type: if gh_release.prerelease {
            ReleaseType::Experimental
        } else {
            ReleaseType::Stable
        },
        status: GameReleaseStatus::Unknown,
        created_at: gh_release.created_at,
    };
    release.status = release
        .get_installation_status(os, cache_dir, data_dir, resources_dir)
        .await?;

    Ok(release)
}
