use std::path::Path;

use tokio::fs;

use crate::filesystem::paths::{
  get_game_executable_filepath, AssetDownloadDirError,
  AssetExtractionDirError, GetExecutablePathError,
};
use crate::game_release::game_release::{
  GameRelease, GameReleaseStatus,
};
use crate::infra::utils::OS;

#[derive(thiserror::Error, Debug)]
pub enum GetInstallationStatusError {
  #[error("failed to get asset download directory: {0}")]
  AssetDownload(#[from] AssetDownloadDirError),

  #[error("failed to get asset extraction directory: {0}")]
  AssetExtraction(#[from] AssetExtractionDirError),

  #[error("failed to get executable directory: {0}")]
  Executable(#[from] GetExecutablePathError),
}

impl GameRelease {
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
        return Ok(GameReleaseStatus::NotDownloaded)
      }
      Err(e) => {
        return Err(GetInstallationStatusError::Executable(e))
      }
    };

    match fs::metadata(&executable_path).await {
      Ok(metadata) if metadata.is_file() => {}
      _ => return Ok(GameReleaseStatus::NotDownloaded),
    }

    Ok(GameReleaseStatus::ReadyToPlay)
  }
}
