use super::error::FetchReleasesError;
use super::game_release::{GameRelease, ReleaseType};
use super::github_fetch::{fetch_github_releases, GithubRelease};
use super::utils::{get_cached_releases, merge_releases, write_cached_releases};
use crate::fetch_releases::utils::select_releases_for_cache;
use crate::infra::http_client::HTTP_CLIENT;
use crate::infra::utils::get_github_repo_for_variant;
use crate::variants::GameVariant;

impl GameVariant {
    pub(crate) async fn fetch_releases(&self) -> Result<Vec<GameRelease>, FetchReleasesError> {
        let repo = get_github_repo_for_variant(self);

        let fetched_releases: Vec<GithubRelease> =
            fetch_github_releases(&HTTP_CLIENT, repo).await?;
        let cached_releases: Vec<GithubRelease> = get_cached_releases(&self);

        let all_releases = merge_releases(&fetched_releases, &cached_releases);
        let to_cache = select_releases_for_cache(&all_releases);
        write_cached_releases(&self, &to_cache);

        let game_releases = to_cache
            .into_iter()
            .map(|r| GameRelease {
                variant: *self,
                version: r.tag_name,
                release_type: if r.prerelease {
                    ReleaseType::Experimental
                } else {
                    ReleaseType::Stable
                },
            })
            .collect();

        Ok(game_releases)
    }
}
