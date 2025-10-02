use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::game_release::game_release::GameRelease;
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{
    get_github_repo_for_variant, get_safe_filename, read_from_file, write_to_file,
};
use crate::variants::GameVariant;

pub fn get_cached_releases(variant: &GameVariant) -> Vec<GitHubRelease> {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo);

    if !cache_path.exists() {
        return Vec::new();
    }

    match read_from_file::<Vec<GitHubRelease>>(&cache_path) {
        Ok(releases) => releases,
        _ => Vec::new(),
    }
}

pub fn write_cached_releases(variant: &GameVariant, releases: &[GitHubRelease]) {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo);

    if let Some(parent) = cache_path.parent() {
        let _ = create_dir_all(parent);
    }

    let _ = write_to_file(&cache_path, &releases);
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

pub fn get_cache_path_for_repo(repo: &str) -> PathBuf {
    let safe = get_safe_filename(repo);
    let mut path = PathBuf::from("CatLauncherCache");
    path.push("Releases");
    path.push(format!("{}.json", safe));

    path
}

pub fn merge_releases(fetched: &[GitHubRelease], cached: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let map: HashMap<u64, GitHubRelease> = cached
        .iter()
        .chain(fetched.iter())
        .map(|r| (r.id, r.clone()))
        .collect();

    map.into_values().collect()
}

pub fn get_assets(release: &GameRelease) -> Vec<GitHubAsset> {
    let cached_releases = get_cached_releases(&release.variant);
    let maybe_release = cached_releases
        .iter()
        .find(|r| r.tag_name == release.version);

    if let Some(release) = maybe_release {
        release.assets.clone()
    } else {
        Vec::new()
    }
}
