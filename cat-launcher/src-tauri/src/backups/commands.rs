use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::backups::backups::{
  DeleteBackupError, ListBackupsError, RestoreBackupError,
  delete_backup, list_backups, restore_backup,
};
use crate::infra::utils::{OSNotSupportedError, get_os_enum};
use crate::launch_game::repository::BackupEntry;
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::variants::GameVariant;

/// Errors that can occur when executing the list backups command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum ListBackupsCommandError {
  /// Failed to retrieve the list of backups.
  #[error("failed to get backups: {0}")]
  Get(#[from] ListBackupsError),
}

/// Tauri command to list all backups for a specific game variant.
#[tauri::command]
pub async fn list_backups_for_variant(
  variant: GameVariant,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<Vec<BackupEntry>, ListBackupsCommandError> {
  let backups =
    list_backups(&variant, backup_repository.inner()).await?;
  Ok(backups)
}

/// Errors that can occur when executing the delete backup command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum DeleteBackupCommandError {
  /// Failed to delete the backup.
  #[error("failed to delete backup: {0}")]
  Delete(#[from] DeleteBackupError),
  /// Failed to access the app local data directory.
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),
}

/// Tauri command to delete a backup by its ID.
#[tauri::command]
pub async fn delete_backup_by_id(
  id: i64,
  app_handle: tauri::AppHandle,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), DeleteBackupCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  delete_backup(id, &data_dir, backup_repository.inner()).await?;
  Ok(())
}

/// Errors that can occur when executing the restore backup command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum RestoreBackupCommandError {
  /// Failed to restore the backup.
  #[error("failed to restore backup: {0}")]
  Restore(#[from] RestoreBackupError),
  /// Failed to access the app local data directory.
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),
  /// The current operating system is not supported.
  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),
}

/// Tauri command to restore a backup by its ID.
#[tauri::command]
pub async fn restore_backup_by_id(
  id: i64,
  app_handle: tauri::AppHandle,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), RestoreBackupCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;
  restore_backup(id, &data_dir, backup_repository.inner(), &os)
    .await?;
  Ok(())
}
