use std::fs::File;
use std::io;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::filesystem::paths::{
    get_game_executable_filepath, get_or_create_asset_download_dir, AssetDownloadDirError,
    AssetExtractionDirError, GetExecutablePathError,
};
use crate::game_release::game_release::{GameRelease, GameReleaseStatus};

#[derive(thiserror::Error, Debug)]
pub enum GetInstallationStatusError {
    #[error("failed to get asset download directory: {0}")]
    AssetDownloadDir(#[from] AssetDownloadDirError),

    #[error("failed to get asset extraction directory: {0}")]
    AssetExtractionDir(#[from] AssetExtractionDirError),

    #[error("failed to get executable directory: {0}")]
    ExecutableDir(#[from] GetExecutablePathError),

    #[error("failed to verify asset: {0}")]
    Verify(#[from] DigestComputationError),
}

impl GameRelease {
    pub async fn get_installation_status(
        &self,
        os: &str,
        cache_dir: &Path,
        data_dir: &Path,
    ) -> Result<GameReleaseStatus, GetInstallationStatusError> {
        let asset = match self.get_asset(os, cache_dir) {
            Some(asset) => asset,
            None => return Ok(GameReleaseStatus::NotAvailable),
        };

        let download_dir = get_or_create_asset_download_dir(&self.variant, &data_dir)?;

        let asset_file = download_dir.join(&asset.name);
        if !asset_file.exists() {
            return Ok(GameReleaseStatus::NotDownloaded);
        }

        // Checksum verification is very slow; skip it for now
        // let is_uncorrupted = uncorrupted(&asset_file, &asset.digest)?;
        // if !is_uncorrupted {
        //     return Ok(GameReleaseStatus::Corrupted);
        // }
        //
        // Since we don't verify the integrity of the downloaded file, if downloaded asset is present
        // but the installation dir or executable file is missing, we still consider the asset to not
        // be downloaded.

        let executable_path =
            match get_game_executable_filepath(&self.variant, &self.version, os, data_dir) {
                Ok(path) => path,
                Err(GetExecutablePathError::DoesNotExist) => {
                    return Ok(GameReleaseStatus::NotDownloaded)
                }
                Err(e) => return Err(GetInstallationStatusError::ExecutableDir(e)),
            };

        if !executable_path.exists() {
            return Ok(GameReleaseStatus::NotDownloaded);
        }

        Ok(GameReleaseStatus::ReadyToPlay)
    }
}

#[allow(dead_code)]
pub fn uncorrupted(path: &Path, digest: &str) -> Result<bool, DigestComputationError> {
    let parts: Vec<&str> = digest.split(':').collect();
    if parts.len() != 2 || parts[0] != "sha256" {
        return Err(DigestComputationError::InvalidFormat(digest.to_string()));
    }
    let expected_hash = parts[1].to_ascii_lowercase();

    let mut file = File::open(path)?;

    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let actual_hash = hasher.finalize();

    Ok(format!("{:x}", actual_hash) == expected_hash)
}

#[derive(thiserror::Error, Debug)]
pub enum DigestComputationError {
    #[error("failed to compute digest: {0}")]
    Compute(#[from] io::Error),
    #[error("invalid digest format: {0}")]
    InvalidFormat(String),
}
