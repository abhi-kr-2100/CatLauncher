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

  /// Failed to backup existing save files.
  #[error("failed to backup and copy saves: {0}")]
  Backup(#[from] BackupError),

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
    backup_repository: &dyn BackupRepository,
  ) -> Result<Command, LaunchGameError> {
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

    if let Err(e) = backup_save_files(
      &self.variant,
      backup_id,
      &self.version,
      timestamp,
      data_dir,
    )
    .await
    {
      let _ = backup_repository.delete_backup_entry(backup_id).await;
      return Err(e.into());
    }

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

    Ok(command)
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
  releases_repository: &dyn ReleasesRepository,
  backup_repository: impl BackupRepository + Clone + 'static,
  active_release_repository: &dyn ActiveReleaseRepository,
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

  let command = release
    .prepare_launch(
      os,
      world,
      timestamp,
      data_dir,
      &backup_repository,
    )
    .await?;

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
