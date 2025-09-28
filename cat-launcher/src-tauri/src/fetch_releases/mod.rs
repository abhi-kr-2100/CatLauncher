pub mod game_release;
pub mod commands;

mod github_fetch;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;
use async_trait::async_trait;
use game_release::{GameRelease, ReleaseType};
use github_fetch::{fetch_github_releases, GithubRelease};
use crate::infra::http_client::HTTP_CLIENT;
use crate::infra::utils::{get_github_repo_for_variant, get_safe_filename, read_from_file, write_to_file};
use crate::variants::GameVariant;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to fetch GitHub releases: {0}")]
    Github(#[from] github_fetch::GithubFetchError)
}

#[async_trait]
pub trait FetchReleasesAsync {
    async fn fetch(&self) -> Result<Vec<GameRelease>, Error>;
}

#[async_trait]
impl FetchReleasesAsync for GameVariant {
    async fn fetch(&self) -> Result<Vec<GameRelease>, Error> {
        let repo = get_github_repo_for_variant(self);
    
        let fetched_releases: Vec<GithubRelease> = fetch_github_releases(&HTTP_CLIENT, repo).await?;
        let cached_releases: Vec<GithubRelease> = get_cached_releases(&self);

        let all_releases = merge_releases(fetched_releases, cached_releases);
        write_cached_releases(&self, &all_releases);

        let game_releases = all_releases
            .into_iter()
            .map(|r| GameRelease {
                variant: self.clone(),
                version: r.tag_name,
                release_type: if r.prerelease { ReleaseType::Experimental } else { ReleaseType::Stable },
            })
            .collect();

        Ok(game_releases)
    }
}

fn get_cached_releases(variant: &GameVariant) -> Vec<GithubRelease> {
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

fn write_cached_releases(variant: &GameVariant, releases: &Vec<GithubRelease>) -> () {
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

fn get_cache_path_for_repo(repo: &str) -> PathBuf {
    let safe = get_safe_filename(repo);
    let path = format!("CatLauncherCache/Releases/{}.json", safe);

    PathBuf::from(path)
}

fn merge_releases(fetched: Vec<GithubRelease>, cached: Vec<GithubRelease>) -> Vec<GithubRelease> {
    let mut map: HashMap<u64, GithubRelease> = HashMap::new();

    for r in cached {
        map.insert(r.id, r);
    }

    for r in fetched {
        map.insert(r.id, r);
    }

    map.values().cloned().collect()
}
