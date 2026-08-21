use std::future::Future;
use std::io;
use std::path::Path;
use std::process::Stdio;

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::task::JoinError;
use tokio::task::JoinSet;
use ts_rs::TS;

use crate::active_release::repository::ActiveReleaseRepository;
use crate::constants::MAX_BACKUPS;
use crate::fetch_releases::repository::ReleasesRepository;
use crate::filesystem::paths::{
  AssetDownloadDirError, AssetExtractionDirError,
  GetAutomaticBackupArchivePathError, GetExecutablePathError,
  GetUserGameDataDirError, get_game_executable_filepath,
  get_or_create_automatic_backup_archive_filepath,
  get_or_create_user_game_data_dir,
};
use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::{
  GetReleaseError, get_release_by_id,
};
use crate::infra::utils::OS;
use crate::launch_game::repository::{
  BackupRepository, BackupRepositoryError,
};
use crate::launch_game::utils::{BackupError, backup_save_files};
use crate::variants::GameVariant;

/// Errors that can occur during the game launch process.
#[derive(thiserror::Error, Debug)]
pub enum LaunchGameError {
  /// Error related to the download directory.
  #[error("download directory not found: {0}")]
  DownloadDir(#[from] AssetDownloadDirError),

  /// Error related to the game extraction directory.
  #[error("game directory not found: {0}")]
  GameDir(#[from] AssetExtractionDirError),

  /// Error related to the game executable path.
  #[error("executable not found: {0}")]
  Executable(#[from] GetExecutablePathError),

  /// The directory containing the executable could not be determined.
  #[error("executable directory not found")]
  ExecutableDir,

  /// Failed to spawn the game process.
  #[error("failed to launch game: {0}")]
  Launch(#[from] io::Error),

  /// Error while interacting with the backup repository.
  #[error("failed to access backup repository: {0}")]
  BackupRepository(#[from] BackupRepositoryError),

  /// Error related to the user's game data directory.
  #[error("failed to get user data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  /// Failed to capture stdout from the game process.
  #[error("failed to get stdout from child process")]
  Stdout,

  /// Failed to capture stderr from the game process.
  #[error("failed to get stderr from child process")]
  Stderr,

  /// Failed to retrieve information about the game release.
  #[error("failed to obtain release: {0}")]
  Release(#[from] GetReleaseError),

  /// An error occurred while waiting for asynchronous subtasks to complete.
  #[error("failed to wait for subtasks: {0}")]
  Subtasks(#[from] JoinError),

  /// Failed to determine the path for a backup archive.
  #[error("failed to get backup archive path: {0}")]
  BackupArchivePath(#[from] GetAutomaticBackupArchivePathError),

  /// Failed to remove an old backup file.
  #[error("failed to remove backup file: {0}")]
  RemoveBackupFile(io::Error),
}

/// Events emitted during the game session.
#[derive(Serialize, Clone, TS)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum GameEvent {
  /// A log line captured from the game's stdout or stderr.
  Log(String),
  /// The game process has exited.
  Exit(GameExitPayload),
  /// An error occurred during the game session.
  Error(GameErrorPayload),
}

/// Payload for the `Error` game event.
#[derive(Serialize, Clone, serde::Deserialize, TS)]
#[ts(export)]
pub struct GameErrorPayload {
  /// The error message.
  pub message: String,
}

/// Payload for the `Exit` game event.
#[derive(Serialize, Clone, serde::Deserialize, TS)]
#[ts(export)]
pub struct GameExitPayload {
  /// The exit code of the game process, if available.
  pub code: Option<i32>,
}

impl GameRelease {
  /// Prepares the command and environment for launching this release.
  ///
  /// This includes setting up the executable path, creating a backup of save files,
  /// and configuring the command arguments (e.g., `--userdir`, `--world`).
  pub async fn prepare_launch(
    &self,
    os: &OS,
    world: Option<&str>,
    timestamp: u64,
    data_dir: &Path,
    backup_repository: &impl BackupRepository,
  ) -> Result<(Command, Option<BackupError>), LaunchGameError> {
    let executable_path = get_game_executable_filepath(
      &self.variant,
      &self.version,
      data_dir,
      os,
    )
    .await?;

    let executable_dir = executable_path
      .parent()
      .ok_or(LaunchGameError::ExecutableDir)?
      .to_path_buf();

    let backup_id = backup_repository
      .add_backup_entry(&self.variant, &self.version, timestamp)
      .await?;

    // A backup failure is not fatal: clear the entry and continue launching
    // the game. The failure is reported via a non-terminal Log event by the
    // caller.
    let backup_failure = match backup_save_files(
      &self.variant,
      backup_id,
      &self.version,
      timestamp,
      data_dir,
    )
    .await
    {
      Ok(()) => None,
      Err(e) => {
        if let Err(delete_error) =
          backup_repository.delete_backup_entry(backup_id).await
        {
          return Err(delete_error.into());
        }
        eprintln!("Failed to backup save files: {}", e);
        Some(e)
      }
    };

    let user_data_dir =
      get_or_create_user_game_data_dir(&self.variant, data_dir)
        .await?;
    let mut command = Command::new(executable_path);

    command
      .current_dir(executable_dir)
      .arg("--userdir")
      .arg(user_data_dir)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped());

    if let Some(world) = world {
      command.arg("--world").arg(world);
    }

    Ok((command, backup_failure))
  }
}

/// Runs the game process and monitors its output.
///
/// Spawns the command, captures stdout and stderr, and forwards logs
/// and the exit event via the provided `on_game_event` callback.
pub async fn run_game_and_monitor<F, Fut>(
  mut command: Command,
  on_game_event: F,
) -> Result<(), LaunchGameError>
where
  F: Fn(GameEvent) -> Fut + Send + Sync + 'static + Clone,
  Fut: Future<Output = ()> + Send,
{
  let mut child = command.spawn()?;

  let stdout = child.stdout.take().ok_or(LaunchGameError::Stdout)?;
  let stderr = child.stderr.take().ok_or(LaunchGameError::Stderr)?;

  let mut stdout_reader = BufReader::new(stdout).lines();
  let mut stderr_reader = BufReader::new(stderr).lines();

  let on_game_event_clone = on_game_event.clone();
  let stdout_task = tokio::spawn(async move {
    while let Some(line) = stdout_reader.next_line().await.transpose()
    {
      if let Ok(line) = line {
        on_game_event_clone(GameEvent::Log(line)).await;
      }
    }
  });

  let on_game_event_clone = on_game_event.clone();
  let stderr_task = tokio::spawn(async move {
    while let Some(line) = stderr_reader.next_line().await.transpose()
    {
      if let Ok(line) = line {
        on_game_event_clone(GameEvent::Log(line)).await;
      }
    }
  });

  let status = child.wait().await?;

  let stdout_task_result = stdout_task.await;
  let stderr_task_result = stderr_task.await;

  // Exit is emitted before waiting for other tasks to complete so that
  // an error does not prevent the exit event from being ever emitted.
  on_game_event(GameEvent::Exit(GameExitPayload {
    code: status.code(),
  }))
  .await;

  stdout_task_result?;
  stderr_task_result?;

  Ok(())
}

async fn cleanup_old_backups(
  backup_repository: impl BackupRepository + Clone + 'static,
  variant: &GameVariant,
  data_dir: &Path,
) -> Result<(), LaunchGameError> {
  let backups = backup_repository
    .get_backups_sorted_by_timestamp(variant)
    .await?;

  if backups.len() <= MAX_BACKUPS.get() {
    return Ok(());
  }

  let num_to_delete = backups.len() - MAX_BACKUPS.get();
  let backups_to_delete = backups.into_iter().take(num_to_delete);

  let mut set = JoinSet::new();

  for backup in backups_to_delete {
    let data_dir_clone = data_dir.to_owned();
    let variant_clone = *variant;
    let backup_repo_clone = backup_repository.clone();
    set.spawn(async move {
      let path_res = get_or_create_automatic_backup_archive_filepath(
        &variant_clone,
        backup.id,
        &backup.release_version,
        backup.timestamp,
        &data_dir_clone,
      )
      .await;

      if backup_repo_clone
        .delete_backup_entry(backup.id)
        .await
        .is_ok()
        && let Ok(path) = path_res
      {
        // file deletion fails is ignored.
        let _ = tokio::fs::remove_file(&path).await;
      }
    });
  }

  while set.join_next().await.is_some() {}

  Ok(())
}

/// High-level function to launch and monitor a game release.
///
/// This function coordinates retrieving the release information, preparing the launch
/// environment (including backups), and starting the monitoring task.
#[allow(clippy::too_many_arguments)]
pub async fn launch_and_monitor_game<F, Fut>(
  variant: &GameVariant,
  release_id: &str,
  world: Option<&str>,
  os: &OS,
  timestamp: u64,
  data_dir: &Path,
  resource_dir: &Path,
  releases_repository: &impl ReleasesRepository,
  backup_repository: impl BackupRepository + Clone + 'static,
  active_release_repository: &impl ActiveReleaseRepository,
  on_game_event: F,
) -> Result<(), LaunchGameError>
where
  F: Fn(GameEvent) -> Fut + Send + Sync + 'static + Clone,
  Fut: Future<Output = ()> + Send,
{
  let release = get_release_by_id(
    variant,
    release_id,
    os,
    data_dir,
    resource_dir,
    releases_repository,
  )
  .await?;

  // Ignore non-critical error where active release could not be set
  let _ = variant
    .set_active_release(release_id, active_release_repository)
    .await;

  let (command, backup_failure) = release
    .prepare_launch(
      os,
      world,
      timestamp,
      data_dir,
      &backup_repository,
    )
    .await?;

  if let Some(backup_error) = backup_failure {
    on_game_event(GameEvent::Log(format!(
      "Failed to backup save files: {backup_error}"
    )))
    .await;
  }

  let backup_repository_clone = backup_repository.clone();
  let variant_clone = *variant;
  let data_dir_clone = data_dir.to_path_buf();
  let on_game_event_for_cleanup = on_game_event.clone();
  tokio::spawn(async move {
    if let Err(e) = cleanup_old_backups(
      backup_repository_clone,
      &variant_clone,
      &data_dir_clone,
    )
    .await
    {
      eprintln!("Error cleaning up old backups: {}", e);
      let error_payload = GameErrorPayload {
        message: e.to_string(),
      };
      on_game_event_for_cleanup(GameEvent::Error(error_payload))
        .await;
    }
  });

  let on_game_event_for_error = on_game_event.clone();

  // It's important to not await the task here, as it be blocking.
  // run_game_and_monitor streams to the frontend.
  tokio::spawn(async move {
    let result = run_game_and_monitor(command, on_game_event).await;

    if let Err(e) = result {
      eprintln!("Error running game: {}", e);

      let error_payload = GameErrorPayload {
        message: e.to_string(),
      };
      on_game_event_for_error(GameEvent::Error(error_payload)).await;
    }
  });

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
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::filesystem::paths::{
    get_game_executable_dir, get_game_executable_filenames,
    get_or_create_asset_installation_dir, get_or_create_directory,
    get_or_create_user_game_data_dir,
  };
  use crate::game_release::game_release::{
    GameRelease, GameReleaseStatus, ReleaseType,
  };
  use crate::infra::github::release::GitHubRelease;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
  use crate::variants::GameVariant;
  use chrono::Utc;
  use std::sync::{Arc, Mutex};
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  fn create_test_release(
    variant: GameVariant,
    version: &str,
    status: GameReleaseStatus,
  ) -> GameRelease {
    GameRelease {
      variant,
      version: version.to_string(),
      body: Some("Test release notes".to_string()),
      release_type: ReleaseType::Experimental,
      status,
      created_at: Utc::now(),
    }
  }

  async fn setup_dummy_executable(
    variant: &GameVariant,
    version: &str,
    data_dir: &Path,
    os: &OS,
  ) -> TestResult<std::path::PathBuf> {
    let install_dir = get_or_create_asset_installation_dir(
      variant, version, data_dir,
    )
    .await?;
    if os == &OS::Linux {
      get_or_create_directory(&install_dir, "cataclysm-dda").await?;
    }

    let exec_dir =
      get_game_executable_dir(variant, version, data_dir, os).await?;
    tokio::fs::create_dir_all(&exec_dir).await?;

    let exec_filename = get_game_executable_filenames(variant, os)[0];
    let exec_path = exec_dir.join(exec_filename);

    let script: &[u8] = match os {
      OS::Windows => b"@echo off\necho hello from windows\n",
      OS::Linux => b"#!/bin/sh\necho hello from linux\nexit 0\n",
      OS::Mac => b"#!/bin/sh\necho hello from mac\nexit 0\n",
    };
    tokio::fs::write(&exec_path, script).await?;

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      let mut perms =
        tokio::fs::metadata(&exec_path).await?.permissions();
      perms.set_mode(0o755);
      tokio::fs::set_permissions(&exec_path, perms).await?;
    }

    Ok(exec_path)
  }

  #[tokio::test]
  async fn test_prepare_launch_success() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";
      let os = OS::Linux;

      let exec_path = setup_dummy_executable(
        &variant,
        version,
        temp_data.path(),
        &os,
      )
      .await?;

      let user_data_dir =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      tokio::fs::create_dir_all(user_data_dir.join("save")).await?;

      let release = create_test_release(
        variant,
        version,
        GameReleaseStatus::ReadyToPlay,
      );

      let (command, backup_failure) = release
        .prepare_launch(
          &os,
          Some("TestWorld"),
          1000,
          temp_data.path(),
          &backup_repo,
        )
        .await?;

      let std_cmd = command.as_std();
      assert_eq!(std_cmd.get_program(), exec_path);
      assert!(
        backup_failure.is_none(),
        "Backup should succeed when the save directory is present"
      );

      let backups = backup_repo
        .get_backups_sorted_by_timestamp(&variant)
        .await?;
      assert_eq!(backups.len(), 1);
      assert_eq!(backups[0].timestamp, 1000);
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_prepare_launch_executable_not_found() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let release = create_test_release(
        variant,
        "v1.0.0",
        GameReleaseStatus::ReadyToPlay,
      );

      let result = release
        .prepare_launch(
          &OS::Linux,
          None,
          1000,
          temp_data.path(),
          &backup_repo,
        )
        .await;

      assert!(matches!(result, Err(LaunchGameError::Executable(_))));
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_prepare_launch_backup_failure_clears_entry_but_launches()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    // Block Backups directory creation to force backup_save_files to fail
    tokio::fs::write(temp_data.path().join("Backups"), b"not a dir")
      .await?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";
      let os = OS::Linux;

      let exec_path = setup_dummy_executable(
        &variant,
        version,
        temp_data.path(),
        &os,
      )
      .await?;

      let release = create_test_release(
        variant,
        version,
        GameReleaseStatus::ReadyToPlay,
      );

      let (command, backup_failure) = release
        .prepare_launch(
          &os,
          None,
          2000,
          temp_data.path(),
          &backup_repo,
        )
        .await?;

      let std_cmd = command.as_std();
      assert_eq!(std_cmd.get_program(), exec_path);
      assert!(
        backup_failure.is_some(),
        "Backup failure should be reported as a non-fatal result"
      );

      let backups = backup_repo
        .get_backups_sorted_by_timestamp(&variant)
        .await?;
      assert!(
        backups.is_empty(),
        "Backup entry should be cleaned up on backup failure"
      );
    }

    Ok(())
  }

  #[cfg(unix)]
  fn create_shell_test_command() -> Command {
    let mut command = Command::new("sh");
    command
      .arg("-c")
      .arg("echo stdout_line && echo stderr_line >&2")
      .stdout(Stdio::piped())
      .stderr(Stdio::piped());
    command
  }

  #[cfg(unix)]
  async fn collect_game_events(
    command: Command,
  ) -> TestResult<Vec<GameEvent>> {
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    run_game_and_monitor(command, move |evt| {
      let events = events_clone.clone();
      async move {
        if let Ok(mut guard) = events.lock() {
          guard.push(evt);
        }
      }
    })
    .await?;

    Ok(events.lock().map_err(|e| e.to_string())?.clone())
  }

  #[tokio::test]
  #[cfg(unix)]
  async fn test_run_game_and_monitor_success() -> TestResult {
    let events =
      collect_game_events(create_shell_test_command()).await?;

    assert!(
      events.iter().any(
        |e| matches!(e, GameEvent::Log(line) if line == "stdout_line")
      ),
      "Should log stdout"
    );
    assert!(
      events.iter().any(
        |e| matches!(e, GameEvent::Log(line) if line == "stderr_line")
      ),
      "Should log stderr"
    );
    assert!(
      events
        .iter()
        .any(|e| matches!(e, GameEvent::Exit(payload) if payload.code == Some(0))),
      "Should emit Exit event with code 0"
    );

    Ok(())
  }

  async fn assert_cleanup_old_backups(
    backup_repo: &SqliteBackupRepository,
    variant: &GameVariant,
    data_dir: &Path,
  ) -> TestResult {
    let version = "v1.0.0";

    let total_backups = MAX_BACKUPS.get() + 3;
    let mut backup_info = Vec::new();
    for i in 0..total_backups {
      let ts = 1000 + i as u64;
      let id =
        backup_repo.add_backup_entry(variant, version, ts).await?;

      let archive_path =
        get_or_create_automatic_backup_archive_filepath(
          variant, id, version, ts, data_dir,
        )
        .await?;
      tokio::fs::write(&archive_path, b"dummy zip content").await?;
      backup_info.push((id, archive_path));
    }

    let backups_before =
      backup_repo.get_backups_sorted_by_timestamp(variant).await?;
    assert_eq!(backups_before.len(), total_backups);

    cleanup_old_backups(backup_repo.clone(), variant, data_dir)
      .await?;

    let backups_after =
      backup_repo.get_backups_sorted_by_timestamp(variant).await?;
    assert_eq!(
      backups_after.len(),
      MAX_BACKUPS.get(),
      "Should retain exactly MAX_BACKUPS"
    );

    let first_kept_ts = 1003;
    assert_eq!(
      backups_after.first().map(|b| b.timestamp),
      Some(first_kept_ts)
    );

    for (_, path) in &backup_info[..3] {
      assert!(
        !path.exists(),
        "Purged backup file should be removed from disk"
      );
    }
    for (_, path) in &backup_info[3..] {
      assert!(
        path.exists(),
        "Retained backup file should remain on disk"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_cleanup_old_backups() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      assert_cleanup_old_backups(
        &backup_repo,
        &variant,
        temp_data.path(),
      )
      .await?;
    }

    Ok(())
  }

  #[cfg(unix)]
  async fn wait_for_exit_event(
    events: &Arc<Mutex<Vec<GameEvent>>>,
  ) -> TestResult {
    tokio::time::timeout(
      tokio::time::Duration::from_secs(5),
      async {
        loop {
          if let Ok(guard) = events.lock()
            && guard.iter().any(|e| {
              matches!(e, GameEvent::Exit(payload) if payload.code == Some(0))
            })
          {
            break;
          }
          tokio::time::sleep(std::time::Duration::from_millis(50))
            .await;
        }
      },
    )
    .await
    .map_err(|_| "Timed out waiting for GameEvent::Exit with exit code 0")?;

    Ok(())
  }

  #[cfg(unix)]
  async fn assert_launch_and_monitor_full_flow(
    releases_repo: &SqliteReleasesRepository,
    active_repo: &SqliteActiveReleaseRepository,
    backup_repo: &SqliteBackupRepository,
    variant: GameVariant,
    data_dir: &Path,
    resource_dir: &Path,
  ) -> TestResult {
    let version = "v1.0.0";
    let os = OS::Linux;

    setup_dummy_executable(&variant, version, data_dir, &os).await?;

    let user_data_dir =
      get_or_create_user_game_data_dir(&variant, data_dir).await?;
    tokio::fs::create_dir_all(user_data_dir.join("save")).await?;

    releases_repo
      .update_cached_releases(
        &variant,
        &[GitHubRelease {
          id: 100,
          tag_name: version.to_string(),
          prerelease: false,
          body: Some("body".to_string()),
          assets: vec![],
          created_at: Utc::now(),
        }],
      )
      .await?;

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    launch_and_monitor_game(
      &variant,
      version,
      None,
      &os,
      5000,
      data_dir,
      resource_dir,
      releases_repo,
      backup_repo.clone(),
      active_repo,
      move |evt| {
        let events = events_clone.clone();
        async move {
          if let Ok(mut guard) = events.lock() {
            guard.push(evt);
          }
        }
      },
    )
    .await?;

    let active = active_repo.get_active_release(&variant).await?;
    assert_eq!(active, Some(version.to_string()));

    wait_for_exit_event(&events).await?;

    let guard = events.lock().map_err(|e| e.to_string())?;
    let has_error =
      guard.iter().any(|e| matches!(e, GameEvent::Error(_)));
    assert!(
      !has_error,
      "No GameEvent::Error should be emitted during a successful launch"
    );

    let has_stdout = guard.iter().any(|e| {
      matches!(e, GameEvent::Log(line) if line == "hello from linux")
    });
    assert!(
      has_stdout,
      "Should receive the stdout log line from the game process"
    );

    Ok(())
  }

  #[tokio::test]
  #[cfg(unix)]
  async fn test_launch_and_monitor_game_full_flow() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let releases_repo =
      SqliteReleasesRepository::new(db.pool().clone());
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;
    let temp_res = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      assert_launch_and_monitor_full_flow(
        &releases_repo,
        &active_repo,
        &backup_repo,
        variant,
        temp_data.path(),
        temp_res.path(),
      )
      .await?;
    }

    Ok(())
  }
}
