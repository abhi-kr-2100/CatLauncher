use chrono::Local;
use tauri::{AppHandle, Manager, command};
use cat_macros::CommandErrorSerialize;
use crate::filesystem::paths::get_db_path;
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};

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
  // User settings are stored in app_local_data_dir/settings.json
  let settings_path = data_dir.join("settings.json");

  let version = app_handle.package_info().version.to_string();
  let timestamp = Local::now().format("%Y%m%d-%H%M%S");
  let zip_name = format!("cat-launcher-debug-report-v{}-{}.zip", version, timestamp);
  let zip_path = downloads_dir.join(zip_name);

  let mut paths_to_include = Vec::new();
  if db_path.exists() {
    paths_to_include.push(db_path.clone());
  }
  if settings_path.exists() {
    paths_to_include.push(settings_path.clone());
  }

  // We use the common parent of data_dir and resources_dir as a base path if possible,
  // but since they might be far apart, we can just use the root and include full paths
  // OR we can just pass an empty path as source_dir if create_zip_archive handles absolute paths in paths_to_include.
  // Looking at create_zip_archive, it expects paths_to_include to be relative to source_dir if we want a clean structure.
  // But if we want them at the root of the zip, we might need a different approach or call it for each file.

  // Let's refine create_zip_archive usage. If we use / as source_dir, strip_prefix might work if paths are absolute.
  // Actually, let's just use the data_dir as source_dir and include db_path relative to it,
  // and then maybe do the same for resources_dir if they are different.
  // But create_zip_archive only takes one source_dir.

  // Alternatively, let's create a temporary directory, copy the files there, and zip that.
  let temp_dir = std::env::temp_dir().join(format!("cat-launcher-report-{}", timestamp));
  tokio::fs::create_dir_all(&temp_dir).await.map_err(ArchiveCreationError::Io)?;

  if db_path.exists() {
    tokio::fs::copy(&db_path, temp_dir.join("cat-launcher.db")).await.map_err(ArchiveCreationError::Io)?;
  }
  if settings_path.exists() {
    tokio::fs::copy(&settings_path, temp_dir.join("settings.json")).await.map_err(ArchiveCreationError::Io)?;
  }

  let paths_in_temp = vec![
    temp_dir.join("cat-launcher.db"),
    temp_dir.join("settings.json"),
  ];

  create_zip_archive(&temp_dir, &paths_in_temp, &zip_path).await?;

  // Clean up temp dir
  let _ = tokio::fs::remove_dir_all(&temp_dir).await;

  Ok(zip_path.to_string_lossy().to_string())
}
