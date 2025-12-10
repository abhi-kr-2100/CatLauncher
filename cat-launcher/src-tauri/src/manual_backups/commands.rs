use serde::ser::SerializeStruct;
use serde::Serialize;
use std::time::SystemTimeError;
use strum::IntoStaticStr;
use tauri::{Manager, State};

use crate::analytics::helpers::track_event;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::manual_backups::manual_backups::{
    create_manual_backup, delete_manual_backup, list_manual_backups, restore_manual_backup,
    CreateManualBackupError, DeleteManualBackupError, ListManualBackupsError,
    RestoreManualBackupError,
};
use crate::manual_backups::repository::manual_backup_repository::ManualBackupEntry;
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ListManualBackupsCommandError {
    #[error("failed to get backups: {0}")]
    Get(#[from] ListManualBackupsError),
}

impl Serialize for ListManualBackupsCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ListManualBackupsCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn list_manual_backups_for_variant(
    variant: GameVariant,
    backup_repository: State<'_, SqliteManualBackupRepository>,
) -> Result<Vec<ManualBackupEntry>, ListManualBackupsCommandError> {
    let backups = list_manual_backups(&variant, backup_repository.inner()).await?;
    Ok(backups)
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum CreateManualBackupCommandError {
    #[error("failed to create backup: {0}")]
    Create(#[from] CreateManualBackupError),
    #[error("failed to get data directory: {0}")]
    DataDir(#[from] tauri::Error),
    #[error("unsupported OS: {0}")]
    UnsupportedOS(#[from] OSNotSupportedError),
    #[error("failed to get system time: {0}")]
    SystemTime(#[from] SystemTimeError),
}

impl Serialize for CreateManualBackupCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CreateManualBackupCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn create_manual_backup_for_variant(
    name: String,
    variant: GameVariant,
    notes: Option<String>,
    app_handle: tauri::AppHandle,
    backup_repository: State<'_, SqliteManualBackupRepository>,
) -> Result<i64, CreateManualBackupCommandError> {
    let handle = app_handle.clone();
    let variant_clone = variant.clone();
    tauri::async_runtime::spawn(async move {
        let mut props = std::collections::HashMap::new();
        props.insert("variant".to_string(), serde_json::json!(variant_clone));
        track_event(&handle, "backup:create_manual_backup_click", props).await;
    });

    let data_dir = app_handle.path().app_data_dir()?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let id = create_manual_backup(
        &name,
        &variant,
        notes,
        &data_dir,
        timestamp,
        backup_repository.inner(),
    )
    .await?;
    Ok(id)
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum DeleteManualBackupCommandError {
    #[error("failed to delete backup: {0}")]
    Delete(#[from] DeleteManualBackupError),
    #[error("failed to get data directory: {0}")]
    DataDir(#[from] tauri::Error),
}

impl Serialize for DeleteManualBackupCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DeleteManualBackupCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn delete_manual_backup_by_id(
    id: i64,
    app_handle: tauri::AppHandle,
    backup_repository: State<'_, SqliteManualBackupRepository>,
) -> Result<(), DeleteManualBackupCommandError> {
    let data_dir = app_handle.path().app_data_dir()?;
    delete_manual_backup(id, &data_dir, backup_repository.inner()).await?;
    Ok(())
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum RestoreManualBackupCommandError {
    #[error("failed to restore backup: {0}")]
    Restore(#[from] RestoreManualBackupError),
    #[error("failed to get data directory: {0}")]
    DataDir(#[from] tauri::Error),
    #[error("unsupported OS: {0}")]
    UnsupportedOS(#[from] OSNotSupportedError),
}

impl Serialize for RestoreManualBackupCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RestoreManualBackupCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn restore_manual_backup_by_id(
    id: i64,
    app_handle: tauri::AppHandle,
    backup_repository: State<'_, SqliteManualBackupRepository>,
) -> Result<(), RestoreManualBackupCommandError> {
    let data_dir = app_handle.path().app_data_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;
    restore_manual_backup(id, &data_dir, backup_repository.inner(), &os).await?;
    Ok(())
}
