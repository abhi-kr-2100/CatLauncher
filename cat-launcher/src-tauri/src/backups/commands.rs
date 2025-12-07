use serde::ser::SerializeStruct;
use serde::Serialize;
use strum::IntoStaticStr;
use tauri::{Manager, State};

use crate::backups::backups::{
    delete_backup, list_backups, restore_backup, DeleteBackupError, ListBackupsError,
    RestoreBackupError,
};
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::launch_game::repository::BackupEntry;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ListBackupsCommandError {
    #[error("failed to get backups: {0}")]
    Get(#[from] ListBackupsError),
}

impl Serialize for ListBackupsCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ListBackupsCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn list_backups_for_variant(
    variant: GameVariant,
    backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<Vec<BackupEntry>, ListBackupsCommandError> {
    let backups = list_backups(&variant, backup_repository.inner()).await?;
    Ok(backups)
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum DeleteBackupCommandError {
    #[error("failed to delete backup: {0}")]
    Delete(#[from] DeleteBackupError),
    #[error("failed to get data directory: {0}")]
    DataDir(#[from] tauri::Error),
}

impl Serialize for DeleteBackupCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DeleteBackupCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn delete_backup_by_id(
    id: i64,
    app_handle: tauri::AppHandle,
    backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), DeleteBackupCommandError> {
    let data_dir = app_handle.path().app_data_dir()?;
    delete_backup(id, &data_dir, backup_repository.inner()).await?;
    Ok(())
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum RestoreBackupCommandError {
    #[error("failed to restore backup: {0}")]
    Restore(#[from] RestoreBackupError),
    #[error("failed to get data directory: {0}")]
    DataDir(#[from] tauri::Error),
    #[error("unsupported OS: {0}")]
    UnsupportedOS(#[from] OSNotSupportedError),
}

impl Serialize for RestoreBackupCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RestoreBackupCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn restore_backup_by_id(
    id: i64,
    app_handle: tauri::AppHandle,
    backup_repository: State<'_, SqliteBackupRepository>,
) -> Result<(), RestoreBackupCommandError> {
    let data_dir = app_handle.path().app_data_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;
    restore_backup(id, &data_dir, backup_repository.inner(), &os).await?;
    Ok(())
}
