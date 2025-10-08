use std::path::Path;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::fetch_releases::utils::get_assets;
use crate::game_release::utils::get_platform_asset_substr;
use crate::infra::github::asset::GitHubAsset;
use crate::variants::GameVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, TS)]
pub enum ReleaseType {
    Stable,
    Experimental,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct GameRelease {
    pub variant: GameVariant,
    pub version: String,
    pub release_type: ReleaseType,
    pub status: GameReleaseStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum GameReleaseStatus {
    NotAvailable,
    NotDownloaded,
    Corrupted,
    NotInstalled,
    ReadyToPlay,
    Unknown,
}

impl GameRelease {
    pub fn get_asset(&self, os: &str, cache_dir: &Path) -> Option<GitHubAsset> {
        let assets = get_assets(self, cache_dir);

        get_platform_asset_substr(&self.variant, os)
            .and_then(|substring| assets.into_iter().find(|a| a.name.contains(substring)))
    }
}
