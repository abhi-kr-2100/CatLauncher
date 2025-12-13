use std::path::Path;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::fetch_releases::repository::ReleasesRepository;
use crate::fetch_releases::utils::get_assets;
use crate::game_release::utils::get_platform_asset_substrs;
use crate::infra::github::asset::GitHubAsset;
use crate::infra::utils::{Arch, OS};
use crate::variants::GameVariant;

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, TS,
)]
#[ts(export)]
pub enum ReleaseType {
  Stable,
  ReleaseCandidate,
  Experimental,
}

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS,
)]
#[ts(export)]
pub struct GameRelease {
  pub variant: GameVariant,
  pub version: String,
  pub release_type: ReleaseType,
  pub status: GameReleaseStatus,
  #[ts(type = "string")]
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS,
)]
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
  pub async fn get_asset(
    &self,
    os: &OS,
    arch: &Arch,
    resources_dir: &Path,
    releases_repository: &dyn ReleasesRepository,
  ) -> Option<GitHubAsset> {
    let assets =
      get_assets(self, resources_dir, releases_repository).await;
    let substrings =
      get_platform_asset_substrs(&self.variant, os, arch);

    substrings
      .iter()
      .find_map(|substr| {
        assets.iter().find(|a| a.name.contains(substr))
      })
      .cloned()
  }
}
