mod game_release;
mod github_fetch;

pub mod commands;

use async_trait::async_trait;
use game_release::{GameRelease, ReleaseType};
use github_fetch::{fetch_github_releases, GithubRelease};
use crate::infra::http_client::HTTP_CLIENT;
use crate::variants::GameVariant;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to fetch GitHub releases: {0}")]
    Github(#[from] github_fetch::GithubFetchError),
}

#[async_trait]
pub trait FetchReleasesAsync {
    async fn fetch(&self) -> Result<Vec<GameRelease>, Error>;
}

#[async_trait]
impl FetchReleasesAsync for GameVariant {
    async fn fetch(&self) -> Result<Vec<GameRelease>, Error> {
        let (repo, variant_enum) = match self {
            GameVariant::DarkDaysAhead => ("CleverRaven/Cataclysm-DDA", GameVariant::DarkDaysAhead),
            GameVariant::BrightNights => ("cataclysmbnteam/Cataclysm-BN", GameVariant::BrightNights),
            GameVariant::TheLastGeneration => ("Cataclysm-TLG/Cataclysm-TLG", GameVariant::TheLastGeneration),
        };
    
        let releases: Vec<GithubRelease> = fetch_github_releases(&HTTP_CLIENT, repo).await?;
        let game_releases = releases
            .into_iter()
            .map(|r| GameRelease {
                variant: variant_enum.clone(),
                version: r.tag_name,
                release_type: if r.prerelease { ReleaseType::Experimental } else { ReleaseType::Stable },
            })
            .collect();
        Ok(game_releases)
    }
}
