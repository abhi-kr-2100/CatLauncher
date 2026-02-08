use strum::IntoStaticStr;
use tauri::{Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::backups::logic::{
  delete_backup_by_id as delete_backup_by_id_business,
  list_backups_for_variant as list_backups_for_variant_business,
  restore_backup_by_id as restore_backup_by_id_business,
  DeleteBackupByIdError, ListBackupsForVariantError,
  RestoreBackupByIdError,
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
  Get(#[from] ListBackupsForVariantError),
}

#[tauri::command]
pub async fn list_backups_for_variant(
  variant: GameVariant,
  backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<Vec<BackupEntry>, ListBackupsCommandError> {
  let backups = list_backups_for_variant_business(
    &variant,
    backup_repository.inner(),
  )
  .await?;
  Ok(backups)
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum DeleteBackupCommandError {
  #[error("failed to delete backup: {0}")]
  Delete(#[from] DeleteBackupByIdError),
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
  delete_backup_by_id_business(
    id,
    &data_dir,
    backup_repository.inner(),
  )
  .await?;
  Ok(())
}

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum RestoreBackupCommandError {
  #[error("failed to restore backup: {0}")]
  Restore(#[from] RestoreBackupByIdError),
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
  restore_backup_by_id_business(
    id,
    &data_dir,
    backup_repository.inner(),
    &os,
  )
  .await?;
  Ok(())
}
