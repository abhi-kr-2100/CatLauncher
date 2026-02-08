use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::utils::OS;
use crate::manual_backups::manual_backups::{
  create_manual_backup, CreateManualBackupError,
};
use crate::manual_backups::repository::manual_backup_repository::ManualBackupRepository;
use crate::variants::GameVariant;
use tokio::fs;

#[derive(thiserror::Error, Debug)]
pub enum ImportGameDataError {
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to create backup: {0}")]
  Backup(#[from] CreateManualBackupError),

  #[error("failed to extract archive: {0}")]
  ArchiveExtraction(#[from] ExtractionError),

  #[error("failed to copy directory: {0}")]
  CopyDir(#[from] CopyDirError),

  #[error("failed to identify file type: {0}")]
  Io(#[from] std::io::Error),

  #[error("source path does not exist")]
  SourcePathDoesNotExist,
}

pub async fn import_game_data(
  variant: &GameVariant,
  data_dir: &Path,
  source_path: PathBuf,
  backup_repository: &impl ManualBackupRepository,
  os: &OS,
) -> Result<(), ImportGameDataError> {
  if !source_path.exists() {
    return Err(ImportGameDataError::SourcePathDoesNotExist);
  }

  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;

  let backup_dirs = vec![
    "achievements".to_string(),
    "config".to_string(),
    "graveyard".to_string(),
    "memorial".to_string(),
    "save".to_string(),
    "templates".to_string(),
  ];

  let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs();

  create_manual_backup(
    "Pre-Import Backup",
    variant,
    Some(format!(
      "Backup before importing game data from {:?}",
      source_path
    )),
    data_dir,
    timestamp,
    backup_repository,
    Some(backup_dirs.clone()),
  )
  .await?;

  // Determine if source is directory or file (zip)
  let metadata = fs::metadata(&source_path).await?;
  let source_dir_to_use = if metadata.is_file() {
    // Extract to a temp directory
    let temp_dir = std::env::temp_dir()
      .join(format!("cat-launcher-import-{}", timestamp));
    if temp_dir.exists() {
      fs::remove_dir_all(&temp_dir).await?;
    }
    extract_archive(&source_path, &temp_dir, os).await?;
    temp_dir
  } else {
    source_path.clone()
  };

  // Copy relevant directories
  for dir_name in &backup_dirs {
    let src_dir = source_dir_to_use.join(dir_name);
    if src_dir.exists() {
      copy_dir_all(&src_dir, &user_data_dir.join(dir_name), os)
        .await?;
    }
  }

  if metadata.is_file() {
    let _ = fs::remove_dir_all(&source_dir_to_use).await;
  }

  Ok(())
}
