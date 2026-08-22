use std::path::Path;

use crate::fetch_releases::repository::ReleasesRepository;
use crate::fetch_releases::utils::{
  get_default_releases, merge_releases,
};
use crate::game_release::GameRelease;
use crate::game_release::game_release::GameReleaseStatus;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{Arch, HostSystem, OS};
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

/// Returns a list of substrings that identify the correct game asset for the given platform and architecture.
pub fn get_platform_asset_substrs(
  variant: &GameVariant,
  host_system: &HostSystem,
) -> Vec<&'static str> {
  let HostSystem { os, arch } = host_system;
  match (variant, os, arch) {
    (GameVariant::DarkDaysAhead, OS::Windows, _) => {
      vec!["windows-with-graphics-and-sounds"]
    }
    (GameVariant::DarkDaysAhead, OS::Mac, _) => {
      vec!["osx-with-graphics", "osx-terminal-only"]
    }
    (GameVariant::DarkDaysAhead, OS::Linux, _) => {
      vec!["linux-with-graphics-and-sounds"]
    }

    (GameVariant::BrightNights, OS::Windows, _) => {
      vec!["windows-tiles"]
    }
    (GameVariant::BrightNights, OS::Mac, Arch::ARM64) => {
      vec!["osx-tiles-arm"]
    }
    (GameVariant::BrightNights, OS::Mac, Arch::X64) => {
      vec!["osx-tiles-x64"]
    }
    (GameVariant::BrightNights, OS::Linux, _) => vec!["linux-tiles"],

    (GameVariant::TheLastGeneration, OS::Windows, _) => {
      vec!["windows-tiles-sounds-x64-msvc"]
    }
    (GameVariant::TheLastGeneration, OS::Mac, _) => {
      vec!["osx-tiles-universal"]
    }
    (GameVariant::TheLastGeneration, OS::Linux, _) => {
      vec!["linux-tiles-sounds"]
    }
  }
}

/// Errors that can occur when retrieving a specific game release.
#[derive(thiserror::Error, Debug)]
pub enum GetReleaseError {
  /// An error occurred while retrieving the installation status of the release.
  #[error("failed to get release status: {0}")]
  Status(#[from] GetInstallationStatusError),

  /// The release with the specified ID was not found.
  #[error("release with ID {0} not found")]
  NotFound(String),
}

/// Converts a `GitHubRelease` into a `GameRelease` for a specific game variant.
pub fn gh_release_to_game_release(
  gh_release: &GitHubRelease,
  variant: &GameVariant,
) -> GameRelease {
  GameRelease {
    variant: *variant,
    version: gh_release.tag_name.clone(),
    body: gh_release.body.clone(),
    release_type: variant.determine_release_type(
      &gh_release.tag_name,
      gh_release.prerelease,
    ),
    status: GameReleaseStatus::Unknown,
    created_at: gh_release.created_at,
  }
}

/// Retrieves a specific game release by its ID (tag name).
///
/// This function merges cached and default releases to find the requested release
/// and populates its current installation status.
pub async fn get_release_by_id(
  variant: &GameVariant,
  release_id: &str,
  os: &OS,
  data_dir: &Path,
  resources_dir: &Path,
  releases_repository: &impl ReleasesRepository,
) -> Result<GameRelease, GetReleaseError> {
  let cached_releases = releases_repository
    .get_cached_releases(variant)
    .await
    .unwrap_or_default(); // It's okay if getting cached releases fails.
  // If the release is not found, this function
  // will return an error.

  let default_releases =
    get_default_releases(variant, resources_dir).await;
  let gh_releases =
    merge_releases(&cached_releases, &default_releases);

  let gh_release = match gh_releases
    .iter()
    .find(|r| r.tag_name == release_id)
  {
    Some(r) => r,
    None => return Err(GetReleaseError::NotFound(release_id.into())),
  };

  let mut release = gh_release_to_game_release(gh_release, variant);
  release.status =
    release.get_installation_status(os, data_dir).await?;

  Ok(release)
}
