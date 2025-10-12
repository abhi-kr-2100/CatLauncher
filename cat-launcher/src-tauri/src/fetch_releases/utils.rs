use std::cmp::Reverse;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use tokio::fs;
use tokio::fs::create_dir_all;

use crate::fetch_releases::fetch_releases::{ReleasesUpdatePayload, ReleasesUpdateStatus};
use crate::filesystem::paths::get_releases_cache_filepath;
use crate::game_release::game_release::{GameRelease, GameReleaseStatus, ReleaseType};
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{read_from_file, write_to_file, WriteToFileError};
use crate::variants::GameVariant;

pub async fn get_cached_releases(variant: &GameVariant, cache_dir: &Path) -> Vec<GitHubRelease> {
    let cache_file = get_releases_cache_filepath(variant, cache_dir);

    match fs::metadata(&cache_file).await {
        Ok(metadata) if metadata.is_file() => {}
        _ => return Vec::new(),
    };

    match read_from_file::<Vec<GitHubRelease>>(&cache_file).await {
        Ok(releases) => releases,
        _ => Vec::new(),
    }
}

pub async fn get_default_releases(
    variant: &GameVariant,
    default_releases_dir: &Path,
) -> Vec<GitHubRelease> {
    let default_releases_file = default_releases_dir.join(format!("{}.json", variant.id()));
    if !default_releases_file.is_file() {
        return Vec::new();
    }

    match read_from_file::<Vec<GitHubRelease>>(&default_releases_file).await {
        Ok(releases) => releases,
        _ => Vec::new(),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WriteCacheError {
    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),

    #[error("failed to cache releases: {0}")]
    Cache(#[from] WriteToFileError),
}

pub async fn write_cached_releases(
    variant: &GameVariant,
    releases: &[GitHubRelease],
    cache_dir: &Path,
) -> Result<(), WriteCacheError> {
    let cache_path = get_releases_cache_filepath(variant, cache_dir);

    if let Some(parent) = cache_path.parent() {
        create_dir_all(parent).await?;
    }

    Ok(write_to_file(&cache_path, &releases).await?)
}

pub fn select_releases_for_cache(releases: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let (non_prereleases, mut prereleases): (Vec<&GitHubRelease>, Vec<&GitHubRelease>) =
        releases.iter().partition(|r| !r.prerelease);

    prereleases.sort_by_key(|r| Reverse(r.created_at));

    non_prereleases
        .into_iter()
        .take(100)
        .cloned()
        .chain(prereleases.into_iter().take(100).cloned())
        .collect()
}

pub fn merge_releases(r1: &[GitHubRelease], r2: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let map: HashMap<u64, GitHubRelease> = r2
        .iter()
        .chain(r1.iter())
        .map(|r| (r.id, r.clone()))
        .collect();

    map.into_values().collect()
}

pub async fn get_assets(release: &GameRelease, cache_dir: &Path) -> Vec<GitHubAsset> {
    let cached_releases = get_cached_releases(&release.variant, cache_dir).await;
    let maybe_release = cached_releases
        .iter()
        .find(|r| r.tag_name == release.version);

    if let Some(release) = maybe_release {
        release.assets.clone()
    } else {
        Vec::new()
    }
}

pub fn get_releases_payload(
    variant: &GameVariant,
    gh_releases: &[GitHubRelease],
    status: ReleasesUpdateStatus,
) -> ReleasesUpdatePayload {
    let releases = gh_releases
        .iter()
        .map(|r| {
            let release_type = if r.prerelease {
                ReleaseType::Experimental
            } else {
                ReleaseType::Stable
            };

            GameRelease {
                variant: *variant,
                release_type,
                version: r.tag_name.clone(),
                created_at: r.created_at,
                status: GameReleaseStatus::Unknown,
            }
        })
        .collect();

    ReleasesUpdatePayload {
        variant: *variant,
        releases,
        status,
    }
}
