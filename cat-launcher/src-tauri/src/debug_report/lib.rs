use std::path::{Path, PathBuf};
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};

/// Creates a debug report .zip archive containing the database and settings files
/// located in the specified `data_dir`. The archive is saved in `downloads_dir`
/// with a name based on the `version` and `timestamp`.
/// Returns the path to the created .zip archive.
pub async fn create_debug_report_impl(
  data_dir: &Path,
  downloads_dir: &Path,
  version: &str,
  timestamp: &str,
) -> Result<PathBuf, ArchiveCreationError> {
  let db_path = data_dir.join("cat-launcher.db");
  let settings_path = data_dir.join("settings.json");

  let zip_name = format!("cat-launcher-debug-report-v{}-{}.zip", version, timestamp);
  let zip_path = downloads_dir.join(zip_name);

  let mut paths_to_include = Vec::new();
  if db_path.exists() {
    paths_to_include.push(db_path);
  }
  if settings_path.exists() {
    paths_to_include.push(settings_path);
  }

  create_zip_archive(data_dir, &paths_to_include, &zip_path).await?;

  Ok(zip_path)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Write;
  use tempfile::tempdir;

  #[tokio::test]
  async fn test_create_debug_report_impl() {
    let data_dir = tempdir().unwrap();
    let db_path = data_dir.path().join("cat-launcher.db");
    let settings_path = data_dir.path().join("settings.json");

    File::create(&db_path).unwrap().write_all(b"db content").unwrap();
    File::create(&settings_path).unwrap().write_all(b"settings content").unwrap();

    let downloads_dir = tempdir().unwrap();

    let zip_path = create_debug_report_impl(
      data_dir.path(),
      downloads_dir.path(),
      "0.1.0",
      "20230101"
    ).await.unwrap();

    assert!(zip_path.exists());
    assert_eq!(zip_path.file_name().unwrap(), "cat-launcher-debug-report-v0.1.0-20230101.zip");

    // Verify zip contents
    let file = File::open(&zip_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    assert_eq!(archive.len(), 2);
    assert!(archive.by_name("cat-launcher.db").is_ok());
    assert!(archive.by_name("settings.json").is_ok());
  }

  #[tokio::test]
  async fn test_create_debug_report_impl_missing_files() {
    let data_dir = tempdir().unwrap();
    let downloads_dir = tempdir().unwrap();

    let zip_path = create_debug_report_impl(
      data_dir.path(),
      downloads_dir.path(),
      "0.1.0",
      "20230101"
    ).await.unwrap();

    assert!(zip_path.exists());

    // Verify zip is empty but exists
    let file = File::open(&zip_path).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    assert_eq!(archive.len(), 0);
  }
}
