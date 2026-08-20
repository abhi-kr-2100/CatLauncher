use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetManualBackupArchivePathError, GetUserGameDataDirError,
  get_or_create_manual_backup_archive_filepath,
  get_or_create_user_game_data_dir,
};
use crate::infra::archive::{
  ArchiveCreationError, ExtractionError, create_zip_archive,
  extract_archive,
};
use crate::infra::utils::OS;
use crate::manual_backups::repository::manual_backup_repository::{
  ManualBackupEntry, ManualBackupRepository,
  ManualBackupRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListManualBackupsError {
  #[error("failed to get backup entries: {0}")]
  Get(#[from] ManualBackupRepositoryError),
}

pub async fn list_manual_backups(
  game_variant: &GameVariant,
  backup_repository: &impl ManualBackupRepository,
) -> Result<Vec<ManualBackupEntry>, ListManualBackupsError> {
  let backups = backup_repository
    .get_manual_backups_sorted_by_timestamp(game_variant)
    .await?;
  Ok(backups)
}

#[derive(thiserror::Error, Debug)]
pub enum CreateManualBackupError {
  #[error("failed to add backup entry: {0}")]
  Add(#[from] ManualBackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetManualBackupArchivePathError),

  #[error("failed to create archive: {0}")]
  ArchiveCreation(#[from] ArchiveCreationError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),
}

pub async fn create_manual_backup(
  name: &str,
  game_variant: &GameVariant,
  notes: Option<String>,
  data_dir: &Path,
  timestamp: u64,
  backup_repository: &impl ManualBackupRepository,
) -> Result<i64, CreateManualBackupError> {
  let id = backup_repository
    .add_manual_backup_entry(name, game_variant, timestamp, notes)
    .await?;

  let user_data_dir =
    get_or_create_user_game_data_dir(game_variant, data_dir).await?;

  let dirs_to_backup = vec![user_data_dir.join("save")];
  let archive_path: PathBuf =
    get_or_create_manual_backup_archive_filepath(id, name, data_dir)
      .await?;

  if let Err(e) =
    create_zip_archive(&user_data_dir, &dirs_to_backup, &archive_path)
      .await
  {
    let _ = backup_repository.delete_manual_backup_entry(id).await;
    return Err(e.into());
  }

  Ok(id)
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteManualBackupError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] ManualBackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetManualBackupArchivePathError),

  #[error("failed to remove backup file: {0}")]
  RemoveBackupFile(#[from] std::io::Error),
}

pub async fn delete_manual_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl ManualBackupRepository,
) -> Result<(), DeleteManualBackupError> {
  let backup = backup_repository.get_manual_backup_entry(id).await?;
  let path: PathBuf = get_or_create_manual_backup_archive_filepath(
    backup.id,
    &backup.name,
    data_dir,
  )
  .await?;

  backup_repository.delete_manual_backup_entry(id).await?;

  if let Err(e) = tokio::fs::remove_file(path).await
    && e.kind() != std::io::ErrorKind::NotFound
  {
    // If we fail to delete the file for reasons other than NotFound,
    // re-insert the backup entry using its original id so the archive
    // path stays resolvable.
    let _ = backup_repository
      .reinsert_manual_backup_entry(
        backup.id,
        &backup.name,
        &backup.game_variant,
        backup.timestamp,
        backup.notes,
      )
      .await;
    return Err(DeleteManualBackupError::RemoveBackupFile(e));
  }

  Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RestoreManualBackupError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] ManualBackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetManualBackupArchivePathError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to extract archive: {0}")]
  Extract(#[from] ExtractionError),

  #[error("failed to delete backup entry: {0}")]
  Delete(ManualBackupRepositoryError),

  #[error("backup archive file does not exist")]
  ArchiveFileMissing,
}

pub async fn restore_manual_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl ManualBackupRepository,
  os: &OS,
) -> Result<(), RestoreManualBackupError> {
  let backup = backup_repository.get_manual_backup_entry(id).await?;
  let archive_path: PathBuf =
    get_or_create_manual_backup_archive_filepath(
      backup.id,
      &backup.name,
      data_dir,
    )
    .await?;

  if let Err(e) = tokio::fs::metadata(&archive_path).await
    && e.kind() == std::io::ErrorKind::NotFound
  {
    // The archive file is missing, so the backup can never be restored.
    // Delete the entry to avoid leaving an orphaned record behind, and
    // propagate the deletion failure if cleanup could not be performed.
    backup_repository
      .delete_manual_backup_entry(id)
      .await
      .map_err(RestoreManualBackupError::Delete)?;
    return Err(RestoreManualBackupError::ArchiveFileMissing);
  }

  let user_data_dir =
    get_or_create_user_game_data_dir(&backup.game_variant, data_dir)
      .await?;

  extract_archive(&archive_path, &user_data_dir, os).await?;

  Ok(())
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
  use tempfile::TempDir;

  use crate::infra::testing::test_zip::create_test_zip;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_create_and_list_manual_backups() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let save_dir = user_data.join("save");
      tokio::fs::create_dir_all(&save_dir).await?;
      tokio::fs::write(save_dir.join("file.txt"), b"save data")
        .await?;

      let id = create_manual_backup(
        "Backup1",
        &variant,
        Some("my notes".to_string()),
        temp_data.path(),
        1000,
        &repo,
      )
      .await?;

      let backups = list_manual_backups(&variant, &repo).await?;
      assert_eq!(backups.len(), 1);
      assert_eq!(backups[0].id, id);
      assert_eq!(backups[0].name, "Backup1");
      assert_eq!(backups[0].notes.as_deref(), Some("my notes"));
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_create_manual_backup_archive_failure_cleans_up_entry()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      tokio::fs::create_dir_all(user_data.join("save")).await?;

      let dummy_id = repo
        .add_manual_backup_entry("temp", &variant, 0, None)
        .await?;
      repo.delete_manual_backup_entry(dummy_id).await?;
      let next_id = dummy_id + 1;

      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          next_id,
          "BackupFail",
          temp_data.path(),
        )
        .await?;
      tokio::fs::create_dir_all(&archive_path).await?;
      assert!(
        archive_path.is_dir(),
        "Blocking archive path must be a directory"
      );

      let result = create_manual_backup(
        "BackupFail",
        &variant,
        None,
        temp_data.path(),
        1000,
        &repo,
      )
      .await;

      assert!(matches!(
        result,
        Err(CreateManualBackupError::ArchiveCreation(_))
      ));
      assert!(
        archive_path.is_dir(),
        "Pre-created blocking directory must remain after archive creation failure"
      );

      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert!(
        remaining.is_empty(),
        "Entry should be deleted if archive creation fails"
      );
      tokio::fs::remove_dir_all(&archive_path).await?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_delete_manual_backup_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      tokio::fs::create_dir_all(user_data.join("save")).await?;

      let id = create_manual_backup(
        "BackupDelete",
        &variant,
        None,
        temp_data.path(),
        1000,
        &repo,
      )
      .await?;

      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "BackupDelete",
          temp_data.path(),
        )
        .await?;
      assert!(archive_path.exists());

      delete_manual_backup(id, temp_data.path(), &repo).await?;

      assert!(!archive_path.exists());
      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert!(remaining.is_empty());
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_manual_backup_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "ManualRestore",
          &variant,
          2000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "ManualRestore",
          temp_data.path(),
        )
        .await?;

      let zip_bytes =
        create_test_zip(&[("save/data.txt", b"restored content")])?;
      tokio::fs::write(&archive_path, zip_bytes).await?;

      restore_manual_backup(id, temp_data.path(), &repo, &OS::Linux)
        .await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let restored_file = user_data.join("save").join("data.txt");
      assert!(restored_file.exists());
      assert_eq!(
        tokio::fs::read_to_string(restored_file).await?,
        "restored content"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_create_manual_backup_missing_save_dir() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      assert!(
        !user_data.join("save").exists(),
        "save directory must not exist before creating the backup"
      );

      let id = create_manual_backup(
        "BackupNoSave",
        &variant,
        None,
        temp_data.path(),
        1000,
        &repo,
      )
      .await?;

      let backups = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert_eq!(backups.len(), 1);
      assert_eq!(backups[0].id, id);

      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "BackupNoSave",
          temp_data.path(),
        )
        .await?;
      assert!(archive_path.is_file());
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_manual_backup_missing_user_data_dir()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "ManualRestoreNoDir",
          &variant,
          2000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "ManualRestoreNoDir",
          temp_data.path(),
        )
        .await?;
      let zip_bytes =
        create_test_zip(&[("save/data.txt", b"restored content")])?;
      tokio::fs::write(&archive_path, zip_bytes).await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      if user_data.exists() {
        tokio::fs::remove_dir_all(&user_data).await?;
      }
      assert!(
        !user_data.exists(),
        "user data directory must not exist before restore"
      );

      restore_manual_backup(id, temp_data.path(), &repo, &OS::Linux)
        .await?;

      assert!(user_data.is_dir());
      assert!(
        user_data.join("save").join("data.txt").exists(),
        "user data directory should be created during restore"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_delete_manual_backup_missing_archive_file()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "BackupDeleteMissing",
          &variant,
          1000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "BackupDeleteMissing",
          temp_data.path(),
        )
        .await?;
      assert!(!archive_path.exists());

      delete_manual_backup(id, temp_data.path(), &repo).await?;

      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert!(
        remaining.is_empty(),
        "Entry should be removed when the archive file is missing"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_delete_manual_backup_file_deletion_failure_reinserts_entry()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "BackupRollback",
          &variant,
          1000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "BackupRollback",
          temp_data.path(),
        )
        .await?;

      // Create a directory at archive_path so tokio::fs::remove_file fails with a non-NotFound error
      tokio::fs::create_dir_all(&archive_path).await?;

      let result =
        delete_manual_backup(id, temp_data.path(), &repo).await;
      assert!(matches!(
        result,
        Err(DeleteManualBackupError::RemoveBackupFile(_))
      ));

      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert_eq!(
        remaining.len(),
        1,
        "Backup entry should be reinserted when file deletion fails due to non-NotFound error"
      );
      assert_eq!(
        remaining[0].id, id,
        "Rollback must preserve the original backup id so the archive path stays resolvable"
      );

      tokio::fs::remove_dir_all(&archive_path).await?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_manual_backup_missing_archive_deletes_entry()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "ManualRestoreMissing",
          &variant,
          2000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "ManualRestoreMissing",
          temp_data.path(),
        )
        .await?;
      assert!(
        !archive_path.exists(),
        "archive file must not exist before restore"
      );

      let result = restore_manual_backup(
        id,
        temp_data.path(),
        &repo,
        &OS::Linux,
      )
      .await;
      assert!(matches!(
        result,
        Err(RestoreManualBackupError::ArchiveFileMissing)
      ));

      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert!(
        remaining.is_empty(),
        "Entry should be deleted when the archive file is missing"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_manual_backup_other_failure_preserves_entry()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteManualBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let id = repo
        .add_manual_backup_entry(
          "ManualRestoreCorrupt",
          &variant,
          2000,
          None,
        )
        .await?;
      let archive_path =
        get_or_create_manual_backup_archive_filepath(
          id,
          "ManualRestoreCorrupt",
          temp_data.path(),
        )
        .await?;
      tokio::fs::write(&archive_path, b"this is not a zip file")
        .await?;

      let result = restore_manual_backup(
        id,
        temp_data.path(),
        &repo,
        &OS::Linux,
      )
      .await;
      assert!(matches!(
        result,
        Err(RestoreManualBackupError::Extract(_))
      ));

      let remaining = repo
        .get_manual_backups_sorted_by_timestamp(&variant)
        .await?;
      assert_eq!(
        remaining.len(),
        1,
        "Entry should be preserved when extraction fails for a reason other than a missing file"
      );
    }

    Ok(())
  }
}
