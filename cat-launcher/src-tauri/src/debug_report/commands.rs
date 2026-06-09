use chrono::Local;
use tauri::{AppHandle, Manager, command};
use cat_macros::CommandErrorSerialize;
use crate::filesystem::paths::get_db_path;
use crate::infra::archive::ArchiveCreationError;
use crate::debug_report::lib::create_debug_report_impl;

#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum CreateDebugReportError {
  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),

  #[error("failed to get downloads directory")]
  DownloadsDir,

  #[error("failed to create zip archive: {0}")]
  ArchiveCreation(#[from] ArchiveCreationError),
}

#[command]
pub async fn create_debug_report(
  app_handle: AppHandle,
) -> Result<String, CreateDebugReportError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let downloads_dir = app_handle.path().download_dir()
    .map_err(|_| CreateDebugReportError::DownloadsDir)?;

  let db_path = get_db_path(&data_dir);
  let settings_path = data_dir.join("settings.json");

  let version = app_handle.package_info().version.to_string();
  let timestamp = Local::now().format("%Y%m%d-%H%M%S");
  let zip_name = format!("cat-launcher-debug-report-v{}-{}.zip", version, timestamp);
  let zip_path = downloads_dir.join(zip_name);

  create_debug_report_impl(&db_path, &settings_path, &zip_path).await?;

  Ok(zip_path.to_string_lossy().to_string())
}
