use std::{env, fs, io};
use tauri::{App, Manager};

use crate::{
  filesystem::utils::{copy_dir_all, CopyDirError},
  infra::utils::{get_os_enum, OSNotSupportedError},
};

#[derive(thiserror::Error, Debug)]
pub enum MigrateToLocalDataDirError {
  #[error("failed to get app directory: {0}")]
  GetAppDir(#[from] tauri::Error),

  #[error("failed to canonicalize path: {0}")]
  CanonicalizePath(#[from] io::Error),

  #[error("failed to get OS: {0}")]
  GetOs(#[from] OSNotSupportedError),

  #[error("failed to copy directory: {0}")]
  CopyDir(#[from] CopyDirError),

  #[error("failed to remove directory: {0}")]
  RemoveDir(io::Error),
}

pub fn migrate_to_local_data_dir(app: &App) {
  let handle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    if let Err(e) = migrate_to_local_data_dir_impl(&handle).await {
      eprintln!("Migration to local data directory failed: {}", e);
    }
  });
}

async fn migrate_to_local_data_dir_impl(
  handle: &tauri::AppHandle,
) -> Result<(), MigrateToLocalDataDirError> {
  let app_data_dir = handle.path().app_data_dir()?;
  let app_local_data_dir = handle.path().app_local_data_dir()?;

  let app_data_dir_canonical = fs::canonicalize(&app_data_dir)?;
  let app_local_data_dir_canonical =
    fs::canonicalize(&app_local_data_dir)?;

  if app_data_dir_canonical == app_local_data_dir_canonical {
    return Ok(());
  }

  if !app_data_dir.exists() {
    return Ok(());
  }

  let os = get_os_enum(env::consts::OS)?;
  copy_dir_all(&app_data_dir, &app_local_data_dir, &os).await?;
  tokio::fs::remove_dir_all(&app_data_dir)
    .await
    .map_err(MigrateToLocalDataDirError::RemoveDir)?;

  Ok(())
}
