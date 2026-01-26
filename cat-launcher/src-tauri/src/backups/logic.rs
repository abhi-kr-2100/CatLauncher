use std::path::Path;

use crate::filesystem::paths::{
  get_or_create_automatic_backup_archive_filepath,
  get_or_create_user_game_data_dir,
  GetAutomaticBackupArchivePathError, GetUserGameDataDirError,
};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::utils::OS;
use crate::launch_game::repository::{
  BackupRepository, BackupRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListBackupsForVariantError {
  #[error("failed to get backup entries: {0}")]
  Get(#[from] BackupRepositoryError),
}

pub async fn list_backups_for_variant(
  game_variant: &GameVariant,
  backup_repository: &impl BackupRepository,
) -> Result<
  Vec<crate::launch_game::repository::BackupEntry>,
  ListBackupsForVariantError,
> {
  let backups = backup_repository
    .get_backups_sorted_by_timestamp(game_variant)
    .await?;
  Ok(backups)
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteBackupByIdError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  #[error("failed to remove backup file: {0}")]
  RemoveBackupFile(#[from] std::io::Error),
}

pub async fn delete_backup_by_id(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
) -> Result<(), DeleteBackupByIdError> {
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
    return Err(DeleteBackupByIdError::RemoveBackupFile(e));
  }

  Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RestoreBackupByIdError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to extract archive: {0}")]
  Extract(#[from] ExtractionError),
}

pub async fn restore_backup_by_id(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
  os: &OS,
) -> Result<(), RestoreBackupByIdError> {
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
