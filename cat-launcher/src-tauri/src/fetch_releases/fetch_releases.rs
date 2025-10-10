use std::path::Path;

use reqwest::Client;
use serde::Serialize;
use ts_rs::TS;

use crate::fetch_releases::utils::{
    get_cached_releases, merge_releases, select_releases_for_cache, write_cached_releases,
};
use crate::game_release::game_release::{GameRelease, GameReleaseStatus, ReleaseType};
use crate::infra::github::release::GitHubRelease;
use crate::infra::github::utils::{fetch_github_releases, GitHubReleaseFetchError};
use crate::infra::utils::get_github_repo_for_variant;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchReleasesError<E> {
    #[error("failed to fetch releases: {0}")]
    Fetch(#[from] GitHubReleaseFetchError),
    #[error("callback failed: {0}")]
    Callback(E),
}

#[derive(Clone, Serialize, TS)]
#[ts(export)]
pub enum ReleasesUpdateStatus {
    InProgress,
    Finished,
}

#[derive(Clone, Serialize, TS)]
#[ts(export)]
pub struct ReleasesUpdatePayload {
    pub variant: GameVariant,
    pub releases: Vec<GameRelease>,
    pub status: ReleasesUpdateStatus,
}

impl GameVariant {
    pub(crate) async fn fetch_releases<F, E>(
        &self,
        client: &Client,
        cache_dir: &Path,
        on_releases: F,
    ) -> Result<(), FetchReleasesError<E>>
    where
        F: Fn(ReleasesUpdatePayload) -> Result<(), E>,
    {
        let cached_releases = get_cached_releases(self, cache_dir);
        let cached_game_releases: Vec<GameRelease> =
            cached_releases.iter().map(|r| (r, *self).into()).collect();

        let payload = ReleasesUpdatePayload {
            variant: *self,
            releases: cached_game_releases,
            status: ReleasesUpdateStatus::InProgress,
        };
        on_releases(payload).map_err(FetchReleasesError::Callback)?;

        let repo = get_github_repo_for_variant(self);
        let fetched_releases = fetch_github_releases(client, repo).await?;

        let all_releases = merge_releases(&fetched_releases, &cached_releases);
        let to_cache = select_releases_for_cache(&all_releases);

        // Successfully writing to cache is not important. Ignore if an error happens.
        let _ = write_cached_releases(self, &to_cache, cache_dir);

        let game_releases: Vec<GameRelease> = to_cache.iter().map(|r| (r, *self).into()).collect();

        let payload = ReleasesUpdatePayload {
            variant: *self,
            releases: game_releases,
            status: ReleasesUpdateStatus::Finished,
        };
        on_releases(payload).map_err(FetchReleasesError::Callback)?;

        Ok(())
    }
}

impl<'a> From<(&'a GitHubRelease, GameVariant)> for GameRelease {
    fn from((r, variant): (&'a GitHubRelease, GameVariant)) -> Self {
        GameRelease {
            variant,
            version: r.tag_name.clone(),
            release_type: if r.prerelease {
                ReleaseType::Experimental
            } else {
                ReleaseType::Stable
            },
            status: GameReleaseStatus::Unknown,
            created_at: r.created_at,
        }
    }
}
