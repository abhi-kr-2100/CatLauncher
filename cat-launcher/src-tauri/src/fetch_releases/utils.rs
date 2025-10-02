use std::cmp::Reverse;
use std::collections::HashMap;
use std::env::consts::OS;
use std::fs::create_dir_all;
use std::io;
use std::path::{Path, PathBuf};

use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::get_platform_asset_substr;
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{
    get_github_repo_for_variant, get_safe_filename, read_from_file, write_to_file, WriteToFileError,
};
use crate::install_release::utils::get_asset_download_dir;
use crate::variants::GameVariant;

pub fn get_cached_releases(variant: &GameVariant, cache_dir: &Path) -> Vec<GitHubRelease> {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo, cache_dir);

    if !cache_path.exists() {
        return Vec::new();
    }

    match read_from_file::<Vec<GitHubRelease>>(&cache_path) {
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

pub fn write_cached_releases(
    variant: &GameVariant,
    releases: &[GitHubRelease],
    cache_dir: &Path,
) -> Result<(), WriteCacheError> {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo, cache_dir);

    if let Some(parent) = cache_path.parent() {
        create_dir_all(parent)?;
    }

    Ok(write_to_file(&cache_path, &releases)?)
}

pub fn select_releases_for_cache(releases: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let (non_prereleases, mut prereleases): (Vec<&GitHubRelease>, Vec<&GitHubRelease>) =
        releases.iter().partition(|r| !r.prerelease);

    prereleases.sort_by_key(|r| Reverse(r.created_at));

    non_prereleases
        .into_iter()
        .take(100)
        .cloned()
        .chain(prereleases.into_iter().take(10).cloned())
        .collect()
}

pub fn get_cache_path_for_repo(repo: &str, cache_dir: &Path) -> PathBuf {
    let safe = get_safe_filename(repo);
    cache_dir.join("Releases").join(format!("{}.json", safe))
}

pub fn merge_releases(fetched: &[GitHubRelease], cached: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let map: HashMap<u64, GitHubRelease> = cached
        .iter()
        .chain(fetched.iter())
        .map(|r| (r.id, r.clone()))
        .collect();

    map.into_values().collect()
}

pub fn get_assets(release: &GameRelease, cache_dir: &Path) -> Vec<GitHubAsset> {
    let cached_releases = get_cached_releases(&release.variant, cache_dir);
    let maybe_release = cached_releases
        .iter()
        .find(|r| r.tag_name == release.version);

    if let Some(release) = maybe_release {
        release.assets.clone()
    } else {
        Vec::new()
    }
}

pub fn is_release_ready_to_play(
    variant: &GameVariant,
    assets: &[GitHubAsset],
    data_dir: &Path,
) -> bool {
    let asset = get_platform_asset_substr(variant, OS)
        .and_then(|substring| assets.into_iter().find(|a| a.name.contains(substring)));

    if asset.is_none() {
        return false;
    }
    let asset = asset.unwrap();

    let dir = match get_asset_download_dir(&variant, data_dir) {
        Ok(dir) => dir,
        Err(_) => return false,
    };
    let filepath = dir.join(&asset.name);

    filepath.exists()
}
