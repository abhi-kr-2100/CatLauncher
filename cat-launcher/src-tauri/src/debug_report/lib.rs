use std::path::Path;
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};

/// Creates a debug report .zip archive containing the database and settings files
/// located in the specified `data_dir`. The archive is saved at `zip_path`.
pub async fn create_debug_report_impl(
  data_dir: &Path,
  zip_path: &Path,
) -> Result<(), ArchiveCreationError> {
  let db_path = data_dir.join("cat-launcher.db");
  let settings_path = data_dir.join("settings.json");

  let mut paths_to_include = Vec::new();
  if db_path.exists() {
    paths_to_include.push(db_path);
  }
  if settings_path.exists() {
    paths_to_include.push(settings_path);
  }

  create_zip_archive(data_dir, &paths_to_include, zip_path).await?;

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
    let data_dir = tempdir().unwrap();
    let db_path = data_dir.path().join("cat-launcher.db");
    let settings_path = data_dir.path().join("settings.json");

    File::create(&db_path).unwrap().write_all(b"db content").unwrap();
    File::create(&settings_path).unwrap().write_all(b"settings content").unwrap();

    let target_dir = tempdir().unwrap();
    let zip_path = target_dir.path().join("report.zip");

    create_debug_report_impl(data_dir.path(), &zip_path).await.unwrap();

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
    let data_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let zip_path = target_dir.path().join("report.zip");

    create_debug_report_impl(data_dir.path(), &zip_path).await.unwrap();

    assert!(zip_path.exists());

    // Verify zip is empty but exists
    let file = File::open(&zip_path).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    assert_eq!(archive.len(), 0);
  }
}
