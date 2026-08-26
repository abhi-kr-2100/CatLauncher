use std::path::Path;

use crate::filesystem::paths::{
  GetAutomaticBackupArchivePathError, GetUserGameDataDirError,
  get_or_create_automatic_backup_archive_filepath,
  get_or_create_user_game_data_dir,
};
use crate::infra::archive::{ExtractionError, extract_archive};
use crate::infra::utils::OS;
use crate::launch_game::repository::{
  BackupRepository, BackupRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListBackupsError {
  #[error("failed to get backup entries: {0}")]
  Get(#[from] BackupRepositoryError),
}

pub async fn list_backups(
  game_variant: &GameVariant,
  backup_repository: &impl BackupRepository,
) -> Result<
  Vec<crate::launch_game::repository::BackupEntry>,
  ListBackupsError,
> {
  let backups = backup_repository
    .get_backups_sorted_by_timestamp(game_variant)
    .await?;
  Ok(backups)
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteBackupError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  #[error("failed to remove backup file: {0}")]
  RemoveBackupFile(#[from] std::io::Error),

  #[error("failed to re-insert backup entry: {0}")]
  Reinsert(BackupRepositoryError),
}

pub async fn delete_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
) -> Result<(), DeleteBackupError> {
  let backup = backup_repository.get_backup_entry(id).await?;
  let path = get_or_create_automatic_backup_archive_filepath(
    &backup.game_variant,
    backup.id,
    &backup.release_version,
    backup.timestamp,
    data_dir,
  )
  .await?;

  backup_repository.delete_backup_entry(id).await?;

  if let Err(e) = tokio::fs::remove_file(path).await
    && e.kind() != std::io::ErrorKind::NotFound
  {
    if let Err(reinsert_err) = backup_repository
      .reinsert_backup_entry(
        backup.id,
        &backup.game_variant,
        &backup.release_version,
        backup.timestamp,
      )
      .await
    {
      return Err(DeleteBackupError::Reinsert(reinsert_err));
    }
    return Err(DeleteBackupError::RemoveBackupFile(e));
  }

  Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RestoreBackupError {
  #[error("failed to get backup entry: {0}")]
  Get(#[from] BackupRepositoryError),

  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to extract archive: {0}")]
  Extract(#[from] ExtractionError),

  #[error("backup archive file does not exist")]
  ArchiveFileMissing,

  #[error("failed to delete backup entry: {0}")]
  Delete(BackupRepositoryError),
}

pub async fn restore_backup(
  id: i64,
  data_dir: &Path,
  backup_repository: &impl BackupRepository,
  os: &OS,
) -> Result<(), RestoreBackupError> {
  let backup = backup_repository.get_backup_entry(id).await?;
  let archive_path = get_or_create_automatic_backup_archive_filepath(
    &backup.game_variant,
    backup.id,
    &backup.release_version,
    backup.timestamp,
    data_dir,
  )
  .await?;

  if let Err(e) = tokio::fs::metadata(&archive_path).await
    && e.kind() == std::io::ErrorKind::NotFound
  {
    backup_repository
      .delete_backup_entry(id)
      .await
      .map_err(RestoreBackupError::Delete)?;
    return Err(RestoreBackupError::ArchiveFileMissing);
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
  use std::path::PathBuf;

  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
  use tempfile::TempDir;

  use crate::infra::testing::test_zip::create_test_zip;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  async fn setup_backup_test()
  -> TestResult<(TestDatabase, SqliteBackupRepository, TempDir)> {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;
    Ok((db, repo, temp_data))
  }

  async fn add_backup_with_archive(
    repo: &SqliteBackupRepository,
    variant: &GameVariant,
    version: &str,
    timestamp: u64,
    data_dir: &Path,
  ) -> TestResult<(i64, PathBuf)> {
    let id =
      repo.add_backup_entry(variant, version, timestamp).await?;
    let archive_path =
      get_or_create_automatic_backup_archive_filepath(
        variant, id, version, timestamp, data_dir,
      )
      .await?;
    Ok((id, archive_path))
  }

  #[tokio::test]
  async fn test_list_backups() -> TestResult {
    let (_db, repo, _temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let b1 =
        repo.add_backup_entry(&variant, "v1.0.0", 1000).await?;
      let b2 =
        repo.add_backup_entry(&variant, "v1.0.0", 2000).await?;

      let backups = list_backups(&variant, &repo).await?;
      assert_eq!(backups.len(), 2);
      assert_eq!(backups[0].id, b1);
      assert_eq!(backups[1].id, b2);
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_delete_backup_success() -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let (id, archive_path) = add_backup_with_archive(
        &repo,
        &variant,
        "v1.0.0",
        1000,
        temp_data.path(),
      )
      .await?;
      tokio::fs::write(&archive_path, b"zip data").await?;
      assert!(archive_path.exists());

      delete_backup(id, temp_data.path(), &repo).await?;

      assert!(!archive_path.exists());
      let remaining =
        repo.get_backups_sorted_by_timestamp(&variant).await?;
      assert!(remaining.is_empty());
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_delete_backup_already_deleted_file_removes_entry()
  -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let (id, archive_path) = add_backup_with_archive(
        &repo,
        &variant,
        "v1.0.0",
        1000,
        temp_data.path(),
      )
      .await?;

      if archive_path.exists() {
        tokio::fs::remove_file(&archive_path).await?;
      }

      delete_backup(id, temp_data.path(), &repo).await?;

      let remaining =
        repo.get_backups_sorted_by_timestamp(&variant).await?;
      assert!(
        remaining.is_empty(),
        "Backup entry should be removed when file is already deleted, not reinserted"
      );
    }

    Ok(())
  }

  async fn assert_delete_failure_reinserts_entry(
    repo: &SqliteBackupRepository,
    variant: &GameVariant,
    data_dir: &Path,
  ) -> TestResult {
    let b1 = repo.add_backup_entry(variant, "v1.0.0", 1000).await?;
    let b2 = repo.add_backup_entry(variant, "v2.0.0", 2000).await?;
    let archive_path =
      get_or_create_automatic_backup_archive_filepath(
        variant, b1, "v1.0.0", 1000, data_dir,
      )
      .await?;

    tokio::fs::create_dir_all(&archive_path).await?;

    let result = delete_backup(b1, data_dir, repo).await;
    assert!(matches!(
      result,
      Err(DeleteBackupError::RemoveBackupFile(_))
    ));

    let remaining =
      repo.get_backups_sorted_by_timestamp(variant).await?;
    assert_eq!(
      remaining.len(),
      2,
      "Backup entry should be reinserted when file deletion fails due to non-NotFound error"
    );
    assert_eq!(
      remaining[0].id, b1,
      "Reinserted entry must preserve the original id returned by add_backup_entry"
    );
    assert_eq!(remaining[0].release_version, "v1.0.0");
    assert_eq!(
      remaining[1].id, b2,
      "Second backup entry must remain intact"
    );

    tokio::fs::remove_dir_all(&archive_path).await?;
    Ok(())
  }

  #[tokio::test]
  async fn test_delete_backup_file_deletion_failure_reinserts_entry()
  -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      assert_delete_failure_reinserts_entry(
        &repo,
        &variant,
        temp_data.path(),
      )
      .await?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_backup_success() -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let (id, archive_path) = add_backup_with_archive(
        &repo,
        &variant,
        "v1.0.0",
        1000,
        temp_data.path(),
      )
      .await?;

      let zip_bytes = create_test_zip(&[(
        "save/world.json",
        b"{\"name\":\"World1\"}",
      )])?;
      tokio::fs::write(&archive_path, zip_bytes).await?;

      restore_backup(id, temp_data.path(), &repo, &OS::Linux).await?;

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let restored_file = user_data.join("save").join("world.json");
      assert!(restored_file.exists());
      let content = tokio::fs::read_to_string(restored_file).await?;
      assert_eq!(content, "{\"name\":\"World1\"}");
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_restore_backup_missing_archive_returns_error()
  -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let (id, archive_path) = add_backup_with_archive(
        &repo,
        &variant,
        "v1.0.0",
        1000,
        temp_data.path(),
      )
      .await?;

      if archive_path.exists() {
        tokio::fs::remove_file(&archive_path).await?;
      }

      let result =
        restore_backup(id, temp_data.path(), &repo, &OS::Linux).await;
      assert!(matches!(
        result,
        Err(RestoreBackupError::ArchiveFileMissing)
      ));

      let remaining =
        repo.get_backups_sorted_by_timestamp(&variant).await?;
      assert!(
        remaining.is_empty(),
        "Entry should be deleted when the archive file is missing"
      );
    }

    Ok(())
  }

  async fn assert_restore_creates_missing_user_game_data_dir(
    repo: &SqliteBackupRepository,
    variant: &GameVariant,
    data_dir: &Path,
  ) -> TestResult {
    let (id, archive_path) = add_backup_with_archive(
      repo, variant, "v1.0.0", 1000, data_dir,
    )
    .await?;

    let zip_bytes = create_test_zip(&[(
      "save/world.json",
      b"{\"name\":\"World1\"}",
    )])?;
    tokio::fs::write(&archive_path, zip_bytes).await?;

    let user_data_dir =
      get_or_create_user_game_data_dir(variant, data_dir).await?;
    if user_data_dir.exists() {
      tokio::fs::remove_dir_all(&user_data_dir).await?;
    }
    assert!(
      !user_data_dir.exists(),
      "user_data_dir must not exist before restore"
    );

    restore_backup(id, data_dir, repo, &OS::Linux).await?;

    assert!(
      user_data_dir.exists() && user_data_dir.is_dir(),
      "user_data_dir should be created during restore"
    );
    let restored_file = user_data_dir.join("save").join("world.json");
    assert!(restored_file.exists());
    Ok(())
  }

  #[tokio::test]
  async fn test_restore_backup_creates_missing_user_game_data_dir()
  -> TestResult {
    let (_db, repo, temp_data) = setup_backup_test().await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      assert_restore_creates_missing_user_game_data_dir(
        &repo,
        &variant,
        temp_data.path(),
      )
      .await?;
    }

    Ok(())
  }
}
