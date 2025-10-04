use std::path::Path;

use reqwest::Client;

use crate::fetch_releases::utils::{
    get_cached_releases, get_release_status, merge_releases, select_releases_for_cache,
    write_cached_releases, GetReleaseStatusError,
};
use crate::game_release::game_release::{GameRelease, ReleaseType};
use crate::infra::github::utils::{fetch_github_releases, GitHubReleaseFetchError};
use crate::infra::utils::get_github_repo_for_variant;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchReleasesError {
    #[error("failed to fetch releases: {0}")]
    Fetch(#[from] GitHubReleaseFetchError),

    #[error("failed to get release status: {0}")]
    Status(#[from] GetReleaseStatusError),
}

impl GameVariant {
    pub(crate) async fn fetch_releases(
        &self,
        os: &str,
        client: &Client,
        cache_dir: &Path,
        data_dir: &Path,
    ) -> Result<Vec<GameRelease>, FetchReleasesError> {
        let repo = get_github_repo_for_variant(self);

        let fetched_releases = fetch_github_releases(client, repo).await?;
        let cached_releases = get_cached_releases(&self, cache_dir);

        let all_releases = merge_releases(&fetched_releases, &cached_releases);
        let to_cache = select_releases_for_cache(&all_releases);

        let _ = write_cached_releases(&self, &to_cache, cache_dir);

        let game_releases = to_cache
            .into_iter()
            .map(|r| {
                Ok(GameRelease {
                    variant: *self,
                    version: r.tag_name.clone(),
                    release_type: if r.prerelease {
                        ReleaseType::Experimental
                    } else {
                        ReleaseType::Stable
                    },
                    status: get_release_status(self, &r.tag_name, &r.assets, os, data_dir)?,
                })
            })
            .collect::<Result<Vec<_>, GetReleaseStatusError>>()?;

        Ok(game_releases)
    }
}
