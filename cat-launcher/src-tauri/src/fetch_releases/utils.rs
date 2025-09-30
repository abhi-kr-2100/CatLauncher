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

pub fn write_cached_releases(variant: &GameVariant, releases: &Vec<GithubRelease>) -> () {
    let repo = get_github_repo_for_variant(variant);
    let cache_path = get_cache_path_for_repo(repo);

    if let Some(parent) = cache_path.parent() {
        let _ = create_dir_all(parent);
    }

    let non_prereleases = releases.iter().filter(|r| !r.prerelease).take(100);
    let mut prereleases: Vec<&GithubRelease> = releases.iter().filter(|r| r.prerelease).collect();

    prereleases.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let prereleases_to_cache = prereleases.into_iter().take(10);

    let mut to_cache: Vec<GithubRelease> = non_prereleases.cloned().collect();
    to_cache.extend(prereleases_to_cache.cloned());

    let _ = write_to_file(&cache_path, &to_cache);
}

pub fn get_cache_path_for_repo(repo: &str) -> PathBuf {
    let safe = get_safe_filename(repo);
    let path = format!("CatLauncherCache/Releases/{}.json", safe);

    PathBuf::from(path)
}

pub fn merge_releases(
    fetched: Vec<GithubRelease>,
    cached: Vec<GithubRelease>,
) -> Vec<GithubRelease> {
    let mut map: HashMap<u64, GithubRelease> = HashMap::new();

    for r in cached {
        map.insert(r.id, r);
    }

    for r in fetched {
        map.insert(r.id, r);
    }

    map.values().cloned().collect()
}
