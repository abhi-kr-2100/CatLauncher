use std::cmp::Reverse;
use std::collections::HashMap;
use std::env::consts::OS;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::game_release::game_release::{GameRelease, GameReleaseStatus};
use crate::game_release::utils::get_platform_asset_substr;
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{
    get_github_repo_for_variant, get_safe_filename, read_from_file, write_to_file, WriteToFileError,
};
use crate::install_release::utils::{
    get_asset_download_dir, get_asset_extraction_dir, AssetDownloadDirError,
    AssetExtractionDirError,
};
use crate::launch_game::utils::{get_executable_path, GetExecutablePathError};
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

#[derive(thiserror::Error, Debug)]
pub enum GetReleaseStatusError {
    #[error("failed to get asset download directory: {0}")]
    AssetDownloadDir(#[from] AssetDownloadDirError),

    #[error("failed to get asset extraction directory: {0}")]
    AssetExtractionDir(#[from] AssetExtractionDirError),

    #[error("failed to get executable directory: {0}")]
    ExecutableDir(#[from] GetExecutablePathError),

    #[error("failed to verify asset: {0}")]
    Verify(#[from] DigestComputationError),
}

pub fn get_release_status(
    variant: &GameVariant,
    version: &str,
    assets: &[GitHubAsset],
    os: &str,
    data_dir: &Path,
) -> Result<GameReleaseStatus, GetReleaseStatusError> {
    let asset = get_platform_asset_substr(&variant, OS)
        .and_then(|substring| assets.into_iter().find(|a| a.name.contains(substring)));

    if asset.is_none() {
        return Ok(GameReleaseStatus::NotAvailable);
    }
    let asset = asset.unwrap();

    let download_dir = get_asset_download_dir(&variant, &data_dir)?;

    let asset_file = download_dir.join(&asset.name);
    if !asset_file.exists() {
        return Ok(GameReleaseStatus::NotDownloaded);
    }

    let is_uncorrupted = uncorrupted(&asset_file, &asset.digest)?;
    if !is_uncorrupted {
        return Ok(GameReleaseStatus::NotDownloaded);
    }

    let extraction_dir = get_asset_extraction_dir(version, &download_dir)?;
    let executable_path = get_executable_path(os, &extraction_dir)?;
    if !executable_path.exists() {
        return Ok(GameReleaseStatus::NotInstalled);
    }

    Ok(GameReleaseStatus::ReadyToPlay)
}

#[derive(thiserror::Error, Debug)]
pub enum DigestComputationError {
    #[error("failed to compute digest: {0}")]
    Compute(#[from] io::Error),
}

pub fn uncorrupted(path: &Path, digest: &str) -> Result<bool, DigestComputationError> {
    let parts: Vec<&str> = digest.split(':').collect();
    if parts.len() != 2 || parts[0] != "sha256" {
        return Ok(false);
    }
    let expected_hash = parts[1].to_ascii_lowercase();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(false),
    };

    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let actual_hash = hasher.finalize();

    Ok(format!("{:x}", actual_hash) == expected_hash)
}
