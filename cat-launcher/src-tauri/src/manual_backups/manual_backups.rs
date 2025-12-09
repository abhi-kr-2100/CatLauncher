use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
    get_or_create_manual_backup_archive_filepath, get_or_create_user_game_data_dir,
    GetManualBackupArchivePathError, GetUserGameDataDirError,
};
use crate::infra::archive::{create_zip_archive, extract_archive, ArchiveCreationError, ExtractionError};
use crate::infra::utils::OS;
use crate::manual_backups::repository::manual_backup_repository::{
    ManualBackupEntry, ManualBackupRepository, ManualBackupRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListManualBackupsError {
    #[error("failed to get backup entries: {0}")]
    Get(#[from] ManualBackupRepositoryError),
}

pub async fn list_manual_backups(
    game_variant: &GameVariant,
    backup_repository: &impl ManualBackupRepository,
) -> Result<Vec<ManualBackupEntry>, ListManualBackupsError> {
    let backups = backup_repository
        .get_manual_backups_sorted_by_timestamp(game_variant)
        .await?;
    Ok(backups)
}

#[derive(thiserror::Error, Debug)]
pub enum CreateManualBackupError {
    #[error("failed to add backup entry: {0}")]
    Add(#[from] ManualBackupRepositoryError),

    #[error("failed to get backup archive path: {0}")]
    BackupArchivePath(#[from] GetManualBackupArchivePathError),

    #[error("failed to create archive: {0}")]
    ArchiveCreation(#[from] ArchiveCreationError),

    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
}

pub async fn create_manual_backup(
    name: &str,
    game_variant: &GameVariant,
    notes: Option<String>,
    data_dir: &Path,
    timestamp: u64,
    backup_repository: &impl ManualBackupRepository,
) -> Result<i64, CreateManualBackupError> {
    let id = backup_repository
        .add_manual_backup_entry(name, game_variant, timestamp, notes)
        .await?;

    let user_data_dir = get_or_create_user_game_data_dir(game_variant, data_dir).await?;

    let dirs_to_backup = vec![user_data_dir.join("save")];
    let archive_path: PathBuf = get_or_create_manual_backup_archive_filepath(
        id,
        name,
        data_dir,
    )
    .await?;

    if let Err(e) = create_zip_archive(&user_data_dir, &dirs_to_backup, &archive_path).await {
        let _ = backup_repository.delete_manual_backup_entry(id).await;
        return Err(e.into());
    }

    Ok(id)
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteManualBackupError {
    #[error("failed to get backup entry: {0}")]
    Get(#[from] ManualBackupRepositoryError),

    #[error("failed to get backup archive path: {0}")]
    BackupArchivePath(#[from] GetManualBackupArchivePathError),

    #[error("failed to remove backup file: {0}")]
    RemoveBackupFile(#[from] std::io::Error),
}

pub async fn delete_manual_backup(
    id: i64,
    data_dir: &Path,
    backup_repository: &impl ManualBackupRepository,
) -> Result<(), DeleteManualBackupError> {
    let backup = backup_repository.get_manual_backup_entry(id).await?;
    let path: PathBuf = get_or_create_manual_backup_archive_filepath(
        backup.id,
        &backup.name,
        data_dir,
    )
    .await?;

    backup_repository.delete_manual_backup_entry(id).await?;

    if let Err(e) = tokio::fs::remove_file(path).await {
        // If we fail to delete the file, we should re-insert the backup entry
        // to avoid having an orphaned file.
        let _ = backup_repository
            .add_manual_backup_entry(
                &backup.name,
                &backup.game_variant,
                backup.timestamp,
                backup.notes,
            )
            .await;
        return Err(DeleteManualBackupError::RemoveBackupFile(e));
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RestoreManualBackupError {
    #[error("failed to get backup entry: {0}")]
    Get(#[from] ManualBackupRepositoryError),

    #[error("failed to get backup archive path: {0}")]
    BackupArchivePath(#[from] GetManualBackupArchivePathError),

    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to extract archive: {0}")]
    Extract(#[from] ExtractionError),
}

pub async fn restore_manual_backup(
    id: i64,
    data_dir: &Path,
    backup_repository: &impl ManualBackupRepository,
    os: &OS,
) -> Result<(), RestoreManualBackupError> {
    let backup = backup_repository.get_manual_backup_entry(id).await?;
    let archive_path: PathBuf = get_or_create_manual_backup_archive_filepath(
        backup.id,
        &backup.name,
        data_dir,
    )
    .await?;

    let user_data_dir = get_or_create_user_game_data_dir(&backup.game_variant, data_dir).await?;

    extract_archive(&archive_path, &user_data_dir, os).await?;

    Ok(())
}
