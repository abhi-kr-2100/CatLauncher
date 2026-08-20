use std::path::Path;

use crate::filesystem::paths::{
  GetAutomaticBackupArchivePathError, GetUserGameDataDirError,
  get_or_create_automatic_backup_archive_filepath,
  get_or_create_user_game_data_dir,
};
use crate::infra::archive::{
  ArchiveCreationError, create_zip_archive,
};
use crate::variants::GameVariant;

/// Errors that can occur during the backup of game save files.
#[derive(thiserror::Error, Debug)]
pub enum BackupError {
  /// Failed to determine the path for the backup archive.
  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  /// Failed to create the zip archive for the backup.
  #[error("failed to create archive: {0}")]
  ArchiveCreation(#[from] ArchiveCreationError),

  /// Failed to locate or create the user's game data directory.
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
}

/// Backs up game save files for a specific game variant.
///
/// This function creates a zip archive containing the 'save' directory
/// from the user's game data folder.
pub async fn backup_save_files(
  variant: &GameVariant,
  id: i64,
  version: &str,
  timestamp: u64,
  data_dir: &Path,
) -> Result<(), BackupError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;

  let dirs_to_backup = vec![user_data_dir.join("save")];
  let archive_path = get_or_create_automatic_backup_archive_filepath(
    variant, id, version, timestamp, data_dir,
  )
  .await?;

  if let Err(e) =
    create_zip_archive(&user_data_dir, &dirs_to_backup, &archive_path)
      .await
  {
    // create_zip_archive may leave a partially written archive behind
    // after creating the destination file; remove it so no orphaned
    // file remains.
    let _ = tokio::fs::remove_file(&archive_path).await;
    return Err(e.into());
  }

  Ok(())
}
