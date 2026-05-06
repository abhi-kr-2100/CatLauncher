use std::path::Path;

use crate::filesystem::paths::{
  GetAutomaticBackupArchivePathError, GetUserGameDataDirError,
  get_or_create_automatic_backup_archive_filepath,
  get_or_create_user_game_data_dir,
};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::utils::OS;
use crate::launch_game::repository::{
  BackupRepository, BackupRepositoryError,
};
use crate::variants::GameVariant;

/// Errors that can occur when listing backups.
#[derive(thiserror::Error, Debug)]
pub enum ListBackupsError {
  /// Failed to retrieve backup entries from the repository.
  #[error("failed to get backup entries: {0}")]
  Get(#[from] BackupRepositoryError),
}

/// Retrieves a list of all backups for a specific game variant, sorted by timestamp.
pub async fn list_backups(
  game_variant: &GameVariant,
  backup_repository: &impl BackupRepository,
) -> Result<
  Vec<crate::launch_game::repository::BackupEntry>,
  ListBackupsError,
> {
  let backups = backup_repository
    .get_backups_sorted_by_timestamp(game_variant)
    .await?;
  Ok(backups)
}

/// Errors that can occur when deleting a backup.
#[derive(thiserror::Error, Debug)]
pub enum DeleteBackupError {
  /// Failed to retrieve the backup entry.
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  /// Failed to construct the file path to the backup archive.
  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  /// Failed to remove the backup file from the filesystem.
  #[error("failed to remove backup file: {0}")]
  RemoveBackupFile(#[from] std::io::Error),
}

/// Deletes a backup from both the database and the filesystem.
/// 
/// If the file removal fails, it attempts to re-insert the backup entry back into the database.
pub async fn delete_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
) -> Result<(), DeleteBackupError> {
  let backup = backup_repository.get_backup_entry(id).await?;
  let path = get_or_create_automatic_backup_archive_filepath(
    &backup.game_variant,
    backup.id,
    &backup.release_version,
    backup.timestamp,
    data_dir,
  )
  .await?;

  backup_repository.delete_backup_entry(id).await?;

  if let Err(e) = tokio::fs::remove_file(path).await {
    // If we fail to delete the file, we should re-insert the backup entry
    // to avoid having an orphaned file.
    let _ = backup_repository
      .add_backup_entry(
        &backup.game_variant,
        &backup.release_version,
        backup.timestamp,
      )
      .await;
    return Err(DeleteBackupError::RemoveBackupFile(e));
  }

  Ok(())
}

/// Errors that can occur when restoring a backup.
#[derive(thiserror::Error, Debug)]
pub enum RestoreBackupError {
  /// Failed to retrieve the backup entry.
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  /// Failed to construct the file path to the backup archive.
  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  /// Failed to determine the user game data directory.
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  /// Failed to extract the backup archive.
  #[error("failed to extract archive: {0}")]
  Extract(#[from] ExtractionError),
}

/// Restores a backup by extracting its archive into the user's game data directory.
pub async fn restore_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
  os: &OS,
) -> Result<(), RestoreBackupError> {
  let backup = backup_repository.get_backup_entry(id).await?;
  let archive_path = get_or_create_automatic_backup_archive_filepath(
    &backup.game_variant,
    backup.id,
    &backup.release_version,
    backup.timestamp,
    data_dir,
  )
  .await?;

  let user_data_dir =
    get_or_create_user_game_data_dir(&backup.game_variant, data_dir)
      .await?;

  extract_archive(&archive_path, &user_data_dir, os).await?;

  Ok(())
}
