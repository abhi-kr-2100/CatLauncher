use std::io;
use std::path::Path;

use crate::filesystem::paths::{
    get_game_executable_dir, get_game_save_dirs, get_or_create_backup_archive_filepath,
    GetBackupArchivePathError, GetGameExecutableDirError, GetVersionExecutableDirError,
};
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum BackupAndCopyError {
    #[error("failed to backup save files: {0}")]
    Backup(#[from] BackupError),

    #[error("failed to copy save files: {0}")]
    Copy(#[from] SaveCopyError),
}

pub async fn backup_and_copy_save_files(
    from_version: &str,
    to_version: &str,
    variant: &GameVariant,
    data_dir: &Path,
    os: &OS,
    timestamp: u64,
) -> Result<(), BackupAndCopyError> {
    backup_save_files(variant, from_version, data_dir, os, timestamp).await?;

    // Don't need to copy save files if the versions are the same
    if from_version == to_version {
        return Ok(());
    }
    copy_save_files(from_version, to_version, variant, data_dir, os).await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum SaveCopyError {
    #[error("failed to get version executable dir: {0}")]
    VersionExecutableDir(#[from] GetVersionExecutableDirError),

    #[error("failed to get game executable dir: {0}")]
    GameExecutableDir(#[from] GetGameExecutableDirError),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid save directory path")]
    InvalidSaveDirPath,

    #[error("failed to copy directory: {0}")]
    Copy(#[from] CopyDirError),
}

async fn copy_save_files(
    from_version: &str,
    to_version: &str,
    variant: &GameVariant,
    data_dir: &Path,
    os: &OS,
) -> Result<(), SaveCopyError> {
    let to_dir = get_game_executable_dir(variant, to_version, data_dir, os).await?;

    let save_dirs = get_game_save_dirs(variant, from_version, data_dir, os).await?;

    for save_dir in save_dirs {
        if let Ok(metadata) = tokio::fs::metadata(&save_dir).await {
            if !metadata.is_dir() {
                continue;
            }
            let file_name = save_dir
                .file_name()
                .ok_or_else(|| SaveCopyError::InvalidSaveDirPath)?;
            let dest_path = to_dir.join(file_name);
            copy_dir_all(&save_dir, &dest_path, os).await?;
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum BackupError {
    #[error("failed to get version executable dir: {0}")]
    VersionExecutableDir(#[from] GetVersionExecutableDirError),

    #[error("failed to get game executable dir: {0}")]
    GameExecutableDir(#[from] GetGameExecutableDirError),

    #[error("failed to get backup archive path: {0}")]
    BackupArchivePath(#[from] GetBackupArchivePathError),

    #[error("failed to create archive: {0}")]
    ArchiveCreation(#[from] ArchiveCreationError),
}

async fn backup_save_files(
    variant: &GameVariant,
    version: &str,
    data_dir: &Path,
    os: &OS,
    timestamp: u64,
) -> Result<(), BackupError> {
    let executable_dir = get_game_executable_dir(variant, version, data_dir, os).await?;
    let dirs_to_backup = get_game_save_dirs(variant, version, data_dir, os).await?;
    let archive_path =
        get_or_create_backup_archive_filepath(variant, version, data_dir, timestamp, os).await?;

    create_zip_archive(&executable_dir, &dirs_to_backup, &archive_path).await?;

    Ok(())
}
