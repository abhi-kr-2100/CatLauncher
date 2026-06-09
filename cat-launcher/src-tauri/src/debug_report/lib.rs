use std::path::Path;
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};

/// Creates a debug report .zip archive containing the specified database and settings files.
/// The archive is saved at the specified `zip_path`.
pub async fn create_debug_report_impl(
  db_path: &Path,
  settings_path: &Path,
  zip_path: &Path,
) -> Result<(), ArchiveCreationError> {
  // Use tempfile for automatic cleanup
  let temp_dir = tempfile::tempdir().map_err(ArchiveCreationError::Io)?;
  let temp_path = temp_dir.path();

  let mut paths_in_temp = Vec::new();

  if db_path.exists() {
    let dest = temp_path.join("cat-launcher.db");
    tokio::fs::copy(db_path, &dest).await.map_err(ArchiveCreationError::Io)?;
    paths_in_temp.push(dest);
  }
  if settings_path.exists() {
    let dest = temp_path.join("settings.json");
    tokio::fs::copy(settings_path, &dest).await.map_err(ArchiveCreationError::Io)?;
    paths_in_temp.push(dest);
  }

  create_zip_archive(temp_path, &paths_in_temp, zip_path).await?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Write;
  use tempfile::tempdir;

  #[tokio::test]
  async fn test_create_debug_report_impl() {
    let source_dir = tempdir().unwrap();
    let db_path = source_dir.path().join("cat-launcher.db");
    let settings_path = source_dir.path().join("settings.json");

    File::create(&db_path).unwrap().write_all(b"db content").unwrap();
    File::create(&settings_path).unwrap().write_all(b"settings content").unwrap();

    let target_dir = tempdir().unwrap();
    let zip_path = target_dir.path().join("report.zip");

    create_debug_report_impl(&db_path, &settings_path, &zip_path).await.unwrap();

    assert!(zip_path.exists());

    // Verify zip contents
    let file = File::open(&zip_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    assert_eq!(archive.len(), 2);
    assert!(archive.by_name("cat-launcher.db").is_ok());
    assert!(archive.by_name("settings.json").is_ok());
  }

  #[tokio::test]
  async fn test_create_debug_report_impl_missing_files() {
    let source_dir = tempdir().unwrap();
    let db_path = source_dir.path().join("non-existent.db");
    let settings_path = source_dir.path().join("non-existent.json");

    let target_dir = tempdir().unwrap();
    let zip_path = target_dir.path().join("report.zip");

    create_debug_report_impl(&db_path, &settings_path, &zip_path).await.unwrap();

    assert!(zip_path.exists());

    // Verify zip is empty but exists
    let file = File::open(&zip_path).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    assert_eq!(archive.len(), 0);
  }
}
