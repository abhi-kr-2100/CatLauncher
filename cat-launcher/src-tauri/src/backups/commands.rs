use strum::IntoStaticStr;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::backups::backups::{
  delete_backup, list_backups, restore_backup, DeleteBackupError,
  ListBackupsError, RestoreBackupError,
};
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::launch_game::repository::BackupEntry;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ListBackupsCommandError {
  #[error("failed to get backups: {0}")]
  Get(#[from] ListBackupsError),
}

#[tauri::command]
pub async fn list_backups_for_variant(
  variant: GameVariant,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<Vec<BackupEntry>, ListBackupsCommandError> {
  let repo = backup_repository.inner();
  let backups = list_backups(&variant, repo).await?;
  Ok(backups)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum DeleteBackupCommandError {
  #[error("failed to delete backup: {0}")]
  Delete(#[from] DeleteBackupError),
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),
}

#[tauri::command]
pub async fn delete_backup_by_id(
  id: i64,
  app_handle: tauri::AppHandle,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), DeleteBackupCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let repo = backup_repository.inner();

  delete_backup(id, &data_dir, repo).await?;
  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum RestoreBackupCommandError {
  #[error("failed to restore backup: {0}")]
  Restore(#[from] RestoreBackupError),
  #[error("failed to get data directory: {0}")]
  DataDir(#[from] tauri::Error),
  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),
}

#[tauri::command]
pub async fn restore_backup_by_id(
  id: i64,
  app_handle: tauri::AppHandle,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), RestoreBackupCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;
  let repo = backup_repository.inner();
  restore_backup(id, &data_dir, repo, &os).await?;
  Ok(())
}
