use std::num::NonZeroU16;
use std::path::{Path, PathBuf};

use downloader::Download;
use thiserror::Error;
use tokio::fs::{create_dir_all, remove_dir_all};

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::http_client::{create_downloader, HTTP_CLIENT};
use crate::infra::utils::OS;
use crate::mods::get_mod_details::{get_mod_details, GetModDetailsError};
use crate::mods::repository::{InstalledModsRepository, InstalledModsRepositoryError};
use crate::mods::validation::{validate_mod_id, InvalidModIdError};
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(Error, Debug)]
pub enum DownloadModError {
    #[error("downloader creation failed: {0}")]
    DownloaderCreation(#[from] downloader::Error),

    #[error("no download result found")]
    NoDownloadResult,
}

async fn download_mod(
    download_url: &str,
    download_dir: &Path,
    parallel_requests: NonZeroU16,
) -> Result<PathBuf, DownloadModError> {
    let mut downloader = create_downloader(HTTP_CLIENT.clone(), download_dir, parallel_requests)?;

    let dl = Download::new(download_url);
    let results = downloader.async_download(&[dl]).await?;

    if let Some(res) = results.into_iter().next() {
        match res {
            Ok(summary) => Ok(summary.file_name),
            Err(e) => Err(DownloadModError::DownloaderCreation(e)),
        }
    } else {
        Err(DownloadModError::NoDownloadResult)
    }
}

#[derive(Error, Debug)]
pub enum InstallModError {
    #[error("invalid mod ID: {0}")]
    InvalidModId(#[from] InvalidModIdError),

    #[error("failed to get mod details: {0}")]
    ModDetails(#[from] GetModDetailsError),

    #[error("failed to download mod: {0}")]
    Download(#[from] DownloadModError),

    #[error("failed to extract archive: {0}")]
    Extraction(#[from] ExtractionError),

    #[error("failed to copy mod directory: {0}")]
    Copy(#[from] CopyDirError),

    #[error("failed to get user game data directory: {0}")]
    UserDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to create directory: {0}")]
    CreateDir(std::io::Error),

    #[error("failed to clean up temp directory: {0}")]
    Cleanup(std::io::Error),

    #[error("source directory does not exist: {0}")]
    SourceNotFound(PathBuf),

    #[error("failed to add mod to repository: {0}")]
    Repository(#[from] InstalledModsRepositoryError),
}

pub async fn install_third_party_mod(
    variant: &GameVariant,
    mod_id: &str,
    data_dir: &Path,
    resource_dir: &Path,
    temp_dir: &Path,
    os: &OS,
    settings: &Settings,
    installed_mods_repository: &dyn InstalledModsRepository,
) -> Result<(), InstallModError> {
    // Validate mod_id to prevent path traversal and ensure safety
    validate_mod_id(mod_id)?;

    let mod_details = get_mod_details(variant, mod_id, resource_dir).await?;

    let user_data_dir = get_or_create_user_game_data_dir(variant, data_dir).await?;

    let mods_dir = user_data_dir.join("mods");
    create_dir_all(&mods_dir)
        .await
        .map_err(InstallModError::CreateDir)?;

    let download_dir = temp_dir.join("cat-launcher-mod-downloads");
    create_dir_all(&download_dir)
        .await
        .map_err(InstallModError::CreateDir)?;

    let zip_path = download_mod(
        &mod_details.installation.download_url,
        &download_dir,
        settings.parallel_requests,
    )
    .await?;

    let extraction_dir = temp_dir.join("cat-launcher-mod-extracted");
    create_dir_all(&extraction_dir)
        .await
        .map_err(InstallModError::CreateDir)?;

    extract_archive(&zip_path, &extraction_dir, os).await?;

    let source_dir = extraction_dir.join(&mod_details.installation.mod_dir);
    let target_dir = mods_dir.join(&mod_details.id);

    let source_exists = tokio::fs::metadata(&source_dir)
        .await
        .map(|_| true)
        .unwrap_or(false);

    if !source_exists {
        return Err(InstallModError::SourceNotFound(source_dir));
    }

    let target_exists = tokio::fs::metadata(&target_dir)
        .await
        .map(|_| true)
        .unwrap_or(false);

    if target_exists {
        remove_dir_all(&target_dir)
            .await
            .map_err(InstallModError::Cleanup)?;
    }

    copy_dir_all(&source_dir, &target_dir, os).await?;

    cleanup_temp_files(&zip_path, &extraction_dir).await?;

    installed_mods_repository.add_installed_mod(mod_id, variant).await?;

    Ok(())
}

async fn cleanup_temp_files(zip_path: &Path, extracted_dir: &Path) -> Result<(), InstallModError> {
    let zip_exists = tokio::fs::metadata(zip_path)
        .await
        .map(|_| true)
        .unwrap_or(false);

    if zip_exists {
        tokio::fs::remove_file(zip_path)
            .await
            .map_err(InstallModError::Cleanup)?;
    }

    let dir_exists = tokio::fs::metadata(extracted_dir)
        .await
        .map(|_| true)
        .unwrap_or(false);

    if dir_exists {
        remove_dir_all(extracted_dir)
            .await
            .map_err(InstallModError::Cleanup)?;
    }

    Ok(())
}
