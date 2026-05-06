use std::path::Path;

use tokio::fs;

use crate::filesystem::paths::{
  AssetDownloadDirError, AssetExtractionDirError,
  GetExecutablePathError, get_game_executable_filepath,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::infra::utils::OS;

/// Errors that can occur when checking the installation status of a release.
#[derive(thiserror::Error, Debug)]
pub enum GetInstallationStatusError {
  /// Failed to determine the asset download directory.
  #[error("failed to get asset download directory: {0}")]
  AssetDownload(#[from] AssetDownloadDirError),

  /// Failed to determine the asset extraction directory.
  #[error("failed to get asset extraction directory: {0}")]
  AssetExtraction(#[from] AssetExtractionDirError),

  /// Failed to determine the game executable path.
  #[error("failed to get executable directory: {0}")]
  Executable(#[from] GetExecutablePathError),
}

impl GameRelease {
  /// Returns the current installation status of the game release.
  ///
  /// It checks for the existence of the game executable in the appropriate
  /// directory for the given operating system.
  pub async fn get_installation_status(
    &self,
    os: &OS,
    data_dir: &Path,
  ) -> Result<GameReleaseStatus, GetInstallationStatusError> {
    let executable_path = match get_game_executable_filepath(
      &self.variant,
      &self.version,
      data_dir,
      os,
    )
    .await
    {
      Ok(path) => path,
      Err(GetExecutablePathError::DoesNotExist) => {
        return Ok(GameReleaseStatus::NotDownloaded);
      }
      Err(e) => {
        return Err(GetInstallationStatusError::Executable(e));
      }
    };

    match fs::metadata(&executable_path).await {
      Ok(metadata) if metadata.is_file() => {}
      _ => return Ok(GameReleaseStatus::NotDownloaded),
    }

    Ok(GameReleaseStatus::ReadyToPlay)
  }
}
