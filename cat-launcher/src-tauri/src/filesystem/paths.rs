use std::io;
use std::path::{Path, PathBuf};

use tokio::fs::{create_dir_all, read_dir};

use crate::filesystem::utils::get_safe_filename;
use crate::infra::utils::OS;
use crate::variants::GameVariant;

pub fn get_default_releases_file_path(variant: &GameVariant, resources_dir: &Path) -> PathBuf {
    resources_dir
        .join("releases")
        .join(format!("{}.json", variant.id()))
}

pub fn get_releases_cache_filepath(variant: &GameVariant, cache_dir: &Path) -> PathBuf {
    cache_dir
        .join("Releases")
        .join(format!("{}.json", variant.id()))
}

#[derive(thiserror::Error, Debug)]
pub enum AssetDownloadDirError {
    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),
}

pub async fn get_or_create_asset_download_dir(
    variant: &GameVariant,
    data_dir: &Path,
) -> Result<PathBuf, AssetDownloadDirError> {
    let dir = data_dir.join("Assets").join(variant.id());

    create_dir_all(&dir).await?;

    Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum AssetExtractionDirError {
    #[error("failed to create directory: {0}")]
    CreateDirectory(#[from] io::Error),
}

pub async fn get_or_create_asset_installation_dir(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
) -> Result<PathBuf, AssetExtractionDirError> {
    let safe_dir_name = get_safe_filename(&release_version);
    let dir = data_dir
        .join("Assets")
        .join(variant.id())
        .join(&safe_dir_name);

    create_dir_all(&dir).await?;

    Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum LastPlayedFileError {
    #[error("failed to create directory: {0}")]
    CreateDir(#[from] std::io::Error),
}

pub async fn get_last_played_filepath(
    variant: &GameVariant,
    data_dir: &Path,
) -> Result<PathBuf, LastPlayedFileError> {
    let directory = data_dir.join("LastPlayed").join(variant.id());
    create_dir_all(&directory).await?;

    let file_path = directory.join("last_played_versions.json");

    Ok(file_path)
}

#[derive(thiserror::Error, Debug)]
pub enum GetGameExecutableDirError {
    #[error("game directory not found")]
    GameDirectory,

    #[error("failed to read game directory")]
    Read(#[from] io::Error),

    #[error("game directory doesn't have game installation")]
    NoInstallation,

    #[error("failed to get asset extraction dir: {0}")]
    AssetExtractionDir(#[from] AssetExtractionDirError),
}

pub async fn get_game_executable_dir(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
    os: &OS,
) -> Result<PathBuf, GetGameExecutableDirError> {
    let installation_dir =
        get_or_create_asset_installation_dir(variant, release_version, data_dir).await?;

    if os == &OS::Windows {
        return Ok(installation_dir);
    }

    if os == &OS::MacOS {
        return Ok(installation_dir.join("Cataclysm.app/Contents/MacOS"));
    }

    // On Linux, the game directory is located one directory under
    // the installation directory.
    let mut dir = read_dir(installation_dir).await?;
    while let Some(entry) = dir.next_entry().await? {
        let file_name = entry.file_name();
        if file_name
            .to_string_lossy()
            .to_lowercase()
            .starts_with("cataclysm")
            && entry.file_type().await?.is_dir()
        {
            return Ok(entry.path());
        }
    }

    Err(GetGameExecutableDirError::NoInstallation)
}

pub fn get_game_executable_filename(variant: &GameVariant, os: &OS) -> &'static str {
    match (variant, os) {
        (g, OS::Windows) => match g {
            GameVariant::BrightNights => "cataclysm-bn-tiles.exe",
            GameVariant::DarkDaysAhead => "cataclysm-tiles.exe",
            GameVariant::TheLastGeneration => "cataclysm-tiles.exe",
        },

        (_, OS::Linux) => "cataclysm-launcher",
        (_, OS::MacOS) => "Cataclysm.sh",
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetExecutablePathError {
    #[error("launcher file does not exist")]
    DoesNotExist,

    #[error("failed to get launcher directory: {0}")]
    LauncherDirectory(#[from] GetGameExecutableDirError),
}

pub async fn get_game_executable_filepath(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
    os: &OS,
) -> Result<PathBuf, GetExecutablePathError> {
    let dir = match get_game_executable_dir(variant, release_version, data_dir, os).await {
        Ok(dir) => dir,
        Err(GetGameExecutableDirError::NoInstallation) => {
            return Err(GetExecutablePathError::DoesNotExist)
        }
        Err(err) => return Err(GetExecutablePathError::LauncherDirectory(err)),
    };

    let filename = get_game_executable_filename(variant, os);
    let filepath = dir.join(filename);

    match tokio::fs::metadata(&filepath).await {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(GetExecutablePathError::DoesNotExist);
            }
        }
        Err(_) => return Err(GetExecutablePathError::DoesNotExist),
    }

    Ok(filepath)
}

#[derive(thiserror::Error, Debug)]
pub enum GetVersionExecutableDirError {
    #[error("failed to get asset download dir: {0}")]
    AssetDownloadDir(#[from] AssetDownloadDirError),

    #[error("failed to get asset extraction dir: {0}")]
    AssetExtractionDir(#[from] AssetExtractionDirError),

    #[error("failed to get game executable dir: {0}")]
    GameExecutableDir(#[from] GetGameExecutableDirError),
}

pub async fn get_game_save_dirs(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
    os: &OS,
) -> Result<Vec<PathBuf>, GetGameExecutableDirError> {
    let dirs = &["achievements", "config", "memorial", "save", "templates"];

    let executable_dir = get_game_executable_dir(variant, release_version, data_dir, os).await?;
    Ok(dirs.iter().map(|d| executable_dir.join(d)).collect())
}

#[derive(thiserror::Error, Debug)]
pub enum GetBackupArchivePathError {
    #[error("failed to create backup directory: {0}")]
    DirFailed(#[from] io::Error),

    #[error("failed to get asset extraction dir: {0}")]
    AssetExtractionDir(#[from] AssetExtractionDirError),

    #[error("failed to get game executable dir: {0}")]
    GameExecutableDir(#[from] GetGameExecutableDirError),
}

pub async fn get_or_create_backup_archive_filepath(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
    timestamp: u64,
    os: &OS,
) -> Result<PathBuf, GetBackupArchivePathError> {
    let executable_dir = get_game_executable_dir(variant, release_version, data_dir, os).await?;
    let backup_dir = executable_dir.join("backups");
    tokio::fs::create_dir_all(&backup_dir).await?;

    Ok(backup_dir.join(format!("{}.zip", timestamp)))
}
