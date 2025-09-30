use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use super::github_fetch::GithubRelease;
use crate::infra::utils::{
    get_github_repo_for_variant, get_safe_filename, read_from_file, write_to_file,
};
use crate::variants::GameVariant;

pub fn get_cached_releases(variant: &GameVariant) -> Vec<GithubRelease> {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo);

    if !cache_path.exists() {
        return Vec::new();
    }

    match read_from_file::<Vec<GithubRelease>>(&cache_path) {
        Ok(releases) => releases,
        _ => Vec::new(),
    }
}

pub fn write_cached_releases(variant: &GameVariant, releases: &[GithubRelease]) -> () {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo);

    if let Some(parent) = cache_path.parent() {
        let _ = create_dir_all(parent);
    }

    let _ = write_to_file(&cache_path, &releases);
}

pub fn select_releases_for_cache(releases: &[GithubRelease]) -> Vec<GithubRelease> {
    let (non_prereleases, mut prereleases): (Vec<&GithubRelease>, Vec<&GithubRelease>) =
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
    let path = format!("CatLauncherCache/Releases/{}.json", safe);

    PathBuf::from(path)
}

pub fn merge_releases(fetched: &[GithubRelease], cached: &[GithubRelease]) -> Vec<GithubRelease> {
    let map: HashMap<u64, GithubRelease> = cached
        .iter()
        .chain(fetched.iter())
        .map(|r| (r.id, r.clone()))
        .collect();

    map.into_values().collect()
}
