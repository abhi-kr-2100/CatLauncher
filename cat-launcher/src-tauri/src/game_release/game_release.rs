use std::env::consts::OS;
use std::path::Path;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::fetch_releases::utils::get_assets;
use crate::game_release::error::GameReleaseError;
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
}

impl GameRelease {
    pub fn get_asset(&self, cache_dir: &Path) -> Result<GitHubAsset, GameReleaseError> {
        let assets = get_assets(self, cache_dir);

        let asset = match (self.variant, OS) {
            (GameVariant::DarkDaysAhead, "windows") => Some("windows-with-graphics-and-sounds"),
            (GameVariant::DarkDaysAhead, "macos") => Some("osx-terminal-only"),
            (GameVariant::DarkDaysAhead, "linux") => Some("linux-with-graphics-and-sounds"),
            (GameVariant::BrightNights, "windows") => Some("windows-tiles"),
            (GameVariant::BrightNights, "macos") => Some("osx-tiles-arm"),
            (GameVariant::BrightNights, "linux") => Some("linux-tiles"),
            (GameVariant::TheLastGeneration, "windows") => Some("windows-tiles-sounds-x64-msvc"),
            (GameVariant::TheLastGeneration, "macos") => Some("osx-tiles-universal"),
            (GameVariant::TheLastGeneration, "linux") => Some("linux-tiles-sounds"),
            _ => None,
        }
        .and_then(|substring| assets.into_iter().find(|a| a.name.contains(substring)));

        asset.ok_or(GameReleaseError::NoCompatibleAssetFound)
    }
}
