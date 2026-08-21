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
/// The type of a game release.
pub enum ReleaseType {
  /// A stable release, well-tested and recommended for most players.
  Stable,
  /// A release candidate, potentially containing new features but still being tested.
  ReleaseCandidate,
  /// An experimental release, containing the latest changes but may be unstable.
  Experimental,
}

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS,
)]
#[ts(export)]
/// Represents a specific release of the game.
pub struct GameRelease {
  /// The game variant this release belongs to.
  pub variant: GameVariant,
  /// The version string (usually the Git tag).
  pub version: String,
  /// The release notes or description.
  pub body: Option<String>,
  /// The type of release (Stable, Experimental, etc.).
  pub release_type: ReleaseType,
  /// The current installation status on the local system.
  pub status: GameReleaseStatus,
  /// The date and time when the release was created.
  #[ts(type = "string")]
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, TS,
)]
#[ts(export)]
/// The possible installation statuses of a game release.
pub enum GameReleaseStatus {
  /// The release is not available for the current platform.
  NotAvailable,
  /// The release has not been downloaded yet.
  NotDownloaded,
  /// The downloaded asset is corrupted.
  Corrupted,
  /// The release has been downloaded but not yet installed/extracted.
  NotInstalled,
  /// The release is installed and ready to be played.
  ReadyToPlay,
  /// The status of the release is unknown.
  Unknown,
}

impl GameRelease {
  /// Attempts to find a compatible GitHub asset for this release based on the OS and architecture.
  pub async fn get_asset(
    &self,
    os: &OS,
    arch: &Arch,
    resources_dir: &Path,
    releases_repository: &impl ReleasesRepository,
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
