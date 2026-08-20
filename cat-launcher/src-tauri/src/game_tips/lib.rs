use thiserror::Error;

use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::fetch_releases::repository::{
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::filesystem::paths::{
  GetTipFilePathsError, get_tip_file_paths,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::game_release::utils::gh_release_to_game_release;
use crate::game_tips::types::Tip;
use crate::infra::utils::OS;
use crate::install_release::installation_status::status::GetInstallationStatusError;
use crate::variants::GameVariant;

/// Errors that can occur when retrieving game tips for a variant.
#[derive(Debug, Error)]
pub enum GetAllTipsForVariantError {
  /// Failed to determine the paths for the tips files.
  #[error("failed to get tip file paths: {0}")]
  GetTipFilePaths(#[from] GetTipFilePathsError),

  /// Failed to parse the tips JSON file.
  #[error("serde json error: {0}")]
  SerdeJson(#[from] serde_json::Error),

  /// Failed to retrieve the active release information.
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseRepositoryError),

  /// A tokio IO error occurred while reading the tips file.
  #[error("tokio io error: {0}")]
  Tokio(#[from] tokio::io::Error),

  /// Failed to check the installation status of a release.
  #[error("failed to get installation status: {0}")]
  GetInstallationStatus(#[from] GetInstallationStatusError),

  /// Failed to retrieve cached releases from the repository.
  #[error("failed to get cached releases: {0}")]
  GetCachedReleases(#[from] ReleasesRepositoryError),
}

/// Reads all tip files associated with a specific game version and collects the text.
async fn get_tips_from_version(
  variant: &GameVariant,
  version: &str,
  data_dir: &std::path::Path,
  os: &OS,
) -> Result<Vec<String>, GetAllTipsForVariantError> {
  let tip_file_paths =
    get_tip_file_paths(variant, version, data_dir, os).await?;
  let mut all_tips: Vec<String> = Vec::new();

  for path in tip_file_paths {
    match tokio::fs::read_to_string(path).await {
      Ok(tips_file_content) => {
        if !tips_file_content.is_empty() {
          let tips: Vec<Tip> =
            serde_json::from_str(&tips_file_content)?;
          all_tips.extend(tips.into_iter().flat_map(|tip| tip.text));
        }
      }
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        // File not found, just skip it
      }
      Err(e) => return Err(e.into()),
    }
  }

  Ok(all_tips)
}

/// Retrieves all game tips for the currently active or installed game release variant.
pub async fn get_all_tips_for_variant(
  variant: &GameVariant,
  data_dir: &std::path::Path,
  os: &OS,
  active_release_repository: &(
     dyn ActiveReleaseRepository + Send + Sync
   ),
  releases_repository: &(dyn ReleasesRepository + Send + Sync),
) -> Result<Vec<String>, GetAllTipsForVariantError> {
  if let Some(active_release) = active_release_repository
    .get_active_release(variant)
    .await?
  {
    let tips =
      get_tips_from_version(variant, &active_release, data_dir, os)
        .await?;
    return Ok(tips);
  }

  let gh_releases =
    releases_repository.get_cached_releases(variant).await?;
  let releases: Vec<GameRelease> = gh_releases
    .iter()
    .map(|r| gh_release_to_game_release(r, variant))
    .collect();

  for release in releases {
    if release.get_installation_status(os, data_dir).await?
      == GameReleaseStatus::ReadyToPlay
    {
      let tips = get_tips_from_version(
        variant,
        &release.version,
        data_dir,
        os,
      )
      .await?;
      return Ok(tips);
    }
  }

  Ok(vec![])
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::filesystem::paths::{
    get_game_executable_dir, get_game_executable_filenames,
    get_game_resources_dir, get_or_create_asset_installation_dir,
    get_or_create_directory,
  };
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::get_os_enum;
  use chrono::Utc;
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  async fn setup_game_resources_dir(
    variant: &GameVariant,
    version: &str,
    data_dir: &std::path::Path,
    os: &OS,
  ) -> TestResult {
    let install_dir = get_or_create_asset_installation_dir(
      variant, version, data_dir,
    )
    .await?;
    if os == &OS::Linux {
      get_or_create_directory(&install_dir, "cataclysm-dda").await?;
    }
    let resources_dir =
      get_game_resources_dir(variant, version, data_dir, os).await?;
    tokio::fs::create_dir_all(&resources_dir).await?;
    Ok(())
  }

  async fn setup_installed_release(
    variant: &GameVariant,
    version: &str,
    data_dir: &std::path::Path,
    os: &OS,
  ) -> TestResult {
    setup_game_resources_dir(variant, version, data_dir, os).await?;
    let exec_dir =
      get_game_executable_dir(variant, version, data_dir, os).await?;
    tokio::fs::create_dir_all(&exec_dir).await?;
    let exec_filename = get_game_executable_filenames(variant, os)[0];
    tokio::fs::write(exec_dir.join(exec_filename), b"dummy exec")
      .await?;
    Ok(())
  }

  async fn write_tips_file(
    variant: &GameVariant,
    version: &str,
    data_dir: &std::path::Path,
    os: &OS,
    tips_json: &str,
  ) -> TestResult {
    let tip_paths =
      get_tip_file_paths(variant, version, data_dir, os).await?;
    let tips_file_path = &tip_paths[0];
    if let Some(parent) = tips_file_path.parent() {
      tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(tips_file_path, tips_json).await?;
    Ok(())
  }

  #[tokio::test]
  async fn test_get_all_tips_none_available() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let tips = get_all_tips_for_variant(
        &variant,
        temp_data.path(),
        &OS::Linux,
        &active_repo,
        &releases_repo,
      )
      .await?;

      assert!(tips.is_empty());
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_get_all_tips_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";
      let current_os = get_os_enum(std::env::consts::OS)?;

      active_repo.set_active_release(&variant, version).await?;

      setup_game_resources_dir(
        &variant,
        version,
        temp_data.path(),
        &current_os,
      )
      .await?;

      let tips_json = r#"[
        { "type": "tip", "text": ["Always check your surroundings.", "Boil water before drinking."] }
      ]"#;
      write_tips_file(
        &variant,
        version,
        temp_data.path(),
        &current_os,
        tips_json,
      )
      .await?;

      let tips = get_all_tips_for_variant(
        &variant,
        temp_data.path(),
        &current_os,
        &active_repo,
        &releases_repo,
      )
      .await?;

      assert_eq!(tips.len(), 2);
      assert_eq!(tips[0], "Always check your surroundings.");
      assert_eq!(tips[1], "Boil water before drinking.");
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_all_tips_active_release_no_tips_file()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";
      let current_os = get_os_enum(std::env::consts::OS)?;

      active_repo.set_active_release(&variant, version).await?;

      setup_game_resources_dir(
        &variant,
        version,
        temp_data.path(),
        &current_os,
      )
      .await?;

      let tips = get_all_tips_for_variant(
        &variant,
        temp_data.path(),
        &current_os,
        &active_repo,
        &releases_repo,
      )
      .await?;

      assert!(tips.is_empty());
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_all_tips_fallback_installed_release() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v2.0.0";
      let current_os = get_os_enum(std::env::consts::OS)?;

      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 200,
            tag_name: version.to_string(),
            prerelease: false,
            body: None,
            assets: vec![],
            created_at: Utc::now(),
          }],
        )
        .await?;

      setup_installed_release(
        &variant,
        version,
        temp_data.path(),
        &current_os,
      )
      .await?;

      let tips_json = r#"[
        { "type": "tip", "text": ["Keep warm during winter."] }
      ]"#;
      write_tips_file(
        &variant,
        version,
        temp_data.path(),
        &current_os,
        tips_json,
      )
      .await?;

      let tips = get_all_tips_for_variant(
        &variant,
        temp_data.path(),
        &current_os,
        &active_repo,
        &releases_repo,
      )
      .await?;

      assert_eq!(tips.len(), 1);
      assert_eq!(tips[0], "Keep warm during winter.");
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_all_tips_fallback_installed_release_no_tips_file()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v2.0.0";
      let current_os = get_os_enum(std::env::consts::OS)?;

      releases_repo
        .update_cached_releases(
          &variant,
          &[GitHubRelease {
            id: 300,
            tag_name: version.to_string(),
            prerelease: false,
            body: None,
            assets: vec![],
            created_at: Utc::now(),
          }],
        )
        .await?;

      setup_installed_release(
        &variant,
        version,
        temp_data.path(),
        &current_os,
      )
      .await?;

      let tips = get_all_tips_for_variant(
        &variant,
        temp_data.path(),
        &current_os,
        &active_repo,
        &releases_repo,
      )
      .await?;

      assert!(tips.is_empty());
    }

    Ok(())
  }
}
