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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::constants::PARALLEL_REQUESTS;
  use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
  use crate::filesystem::paths::{
    get_game_executable_filenames, get_or_create_asset_installation_dir,
  };
  use crate::game_release::game_release::GameReleaseStatus;
  use crate::infra::testing::http_client::TestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::{Arch, OS};
  use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
  use chrono::Utc;
  use downloader::progress::Reporter;
  use github_mock_api::{MockServer, Release as MockRelease};
  use std::collections::HashMap;
  use std::path::PathBuf;
  use std::sync::Arc;
  use tokio::sync::Mutex;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  struct TestReporter;
  impl Reporter for TestReporter {
    fn setup(&self, _max: Option<u64>, _message: &str) {}
    fn progress(&self, _current: u64) {}
    fn set_message(&self, _message: &str) {}
    fn done(&self) {}
  }

  async fn setup() -> TestResult<(
    TestDatabase,
    MockServer,
    Arc<TestHttpClient>,
    tempfile::TempDir,
    tempfile::TempDir,
  )> {
    let db = TestDatabase::builder().build()?;
    let server = MockServer::start().await?;

    let mut host_mappings = HashMap::new();
    let uri = server.uri();
    let host_port = uri
      .strip_prefix("http://")
      .ok_or("uri should start with http://")?;
    host_mappings
      .insert("api.github.com".to_string(), host_port.to_string());
    host_mappings
      .insert("github.com".to_string(), host_port.to_string());

    let client = Arc::new(TestHttpClient::new(host_mappings)?);
    let data_dir = tempfile::tempdir()?;
    let resource_dir = tempfile::tempdir()?;

    // Create releases directory in resource_dir for default releases
    tokio::fs::create_dir_all(resource_dir.path().join("releases"))
      .await?;

    Ok((db, server, client, data_dir, resource_dir))
  }

  fn get_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("src/infra/testing/fixtures")
  }

  #[tokio::test]
  async fn test_prepare_launch_with_world() -> TestResult {
    let (db, _server, _client, data_dir, _resource_dir) = setup().await?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let version = "0.F-3";
    let timestamp = 1234567890;
    let world = "test-world";

    // Create dummy executable
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      data_dir.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-test");
    tokio::fs::create_dir_all(&game_dir).await?;
    let exec_filename = get_game_executable_filenames(&variant, &OS::Linux)[0];
    let exec_path = game_dir.join(exec_filename);
    tokio::fs::write(&exec_path, "").await?;

    let release = GameRelease {
      variant,
      version: version.to_string(),
      body: None,
      release_type: crate::game_release::game_release::ReleaseType::Stable,
      status: GameReleaseStatus::ReadyToPlay,
      created_at: Utc::now(),
    };

    let command = release
      .prepare_launch(
        &OS::Linux,
        Some(world),
        timestamp,
        data_dir.path(),
        &backup_repo,
      )
      .await?;

    let args: Vec<_> = command.as_std().get_args().collect();
    assert!(args.contains(&std::ffi::OsStr::new("--world")));
    assert!(args.contains(&std::ffi::OsStr::new(world)));

    Ok(())
  }

  #[tokio::test]
  async fn test_run_game_and_monitor_stderr() -> TestResult {
    let mut command = Command::new("sh");
    command.arg("-c").arg("echo error message >&2").stdout(Stdio::piped()).stderr(Stdio::piped());

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    run_game_and_monitor(command, move |event| {
      let events_clone = events_clone.clone();
      async move {
        events_clone.lock().await.push(event);
      }
    })
    .await?;

    let events = events.lock().await;
    assert!(events.iter().any(|e| matches!(e, GameEvent::Log(s) if s == "error message")));
    assert!(events.iter().any(|e| matches!(e, GameEvent::Exit(payload) if payload.code == Some(0))));

    Ok(())
  }

  #[tokio::test]
  async fn test_run_game_and_monitor_exit_code() -> TestResult {
    let mut command = Command::new("sh");
    command.arg("-c").arg("exit 42").stdout(Stdio::piped()).stderr(Stdio::piped());

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    run_game_and_monitor(command, move |event| {
      let events_clone = events_clone.clone();
      async move {
        events_clone.lock().await.push(event);
      }
    })
    .await?;

    let events = events.lock().await;
    assert!(events.iter().any(|e| matches!(e, GameEvent::Exit(payload) if payload.code == Some(42))));

    Ok(())
  }

  #[tokio::test]
  async fn test_prepare_launch_executable_missing() -> TestResult {
    let (db, _server, _client, data_dir, _resource_dir) = setup().await?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let version = "0.F-3";
    let timestamp = 1234567890;

    let release = GameRelease {
      variant,
      version: version.to_string(),
      body: None,
      release_type: crate::game_release::game_release::ReleaseType::Stable,
      status: GameReleaseStatus::ReadyToPlay,
      created_at: Utc::now(),
    };

    let result = release
      .prepare_launch(
        &OS::Linux,
        None,
        timestamp,
        data_dir.path(),
        &backup_repo,
      )
      .await;

    assert!(matches!(result, Err(LaunchGameError::Executable(_))));

    Ok(())
  }

  #[tokio::test]
  async fn test_prepare_launch_success() -> TestResult {
    let (db, _server, _client, data_dir, _resource_dir) = setup().await?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let version = "0.F-3";
    let timestamp = 1234567890;

    // Create dummy executable
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      data_dir.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-test");
    tokio::fs::create_dir_all(&game_dir).await?;
    let exec_filename = get_game_executable_filenames(&variant, &OS::Linux)[0];
    let exec_path = game_dir.join(exec_filename);
    tokio::fs::write(&exec_path, "").await?;

    let release = GameRelease {
      variant,
      version: version.to_string(),
      body: None,
      release_type: crate::game_release::game_release::ReleaseType::Stable,
      status: GameReleaseStatus::ReadyToPlay,
      created_at: Utc::now(),
    };

    let command = release
      .prepare_launch(
        &OS::Linux,
        None,
        timestamp,
        data_dir.path(),
        &backup_repo,
      )
      .await?;

    assert_eq!(command.as_std().get_program(), exec_path.as_os_str());

    // Verify backup entry was created
    let backups = backup_repo.get_backups_sorted_by_timestamp(&variant).await?;
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].release_version, version);

    Ok(())
  }

  #[tokio::test]
  async fn test_run_game_and_monitor_success() -> TestResult {
    let mut command = Command::new("echo");
    command.arg("hello world").stdout(Stdio::piped()).stderr(Stdio::piped());

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    run_game_and_monitor(command, move |event| {
      let events_clone = events_clone.clone();
      async move {
        events_clone.lock().await.push(event);
      }
    })
    .await?;

    let events = events.lock().await;
    assert!(events.iter().any(|e| matches!(e, GameEvent::Log(s) if s == "hello world")));
    assert!(events.iter().any(|e| matches!(e, GameEvent::Exit(payload) if payload.code == Some(0))));

    Ok(())
  }

  #[tokio::test]
  async fn test_cleanup_backups_under_limit() -> TestResult {
    let (db, _server, _client, data_dir, _resource_dir) = setup().await?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    // Create 3 backups (limit is 5)
    for i in 0..3 {
        let timestamp = 1000 + i as u64;
        let version = format!("v{}", i);
        backup_repo.add_backup_entry(&variant, &version, timestamp).await?;
    }

    cleanup_old_backups(backup_repo.clone(), &variant, data_dir.path()).await?;

    let backups = backup_repo.get_backups_sorted_by_timestamp(&variant).await?;
    assert_eq!(backups.len(), 3);

    Ok(())
  }

  #[tokio::test]
  async fn test_cleanup_old_backups() -> TestResult {
    let (db, _server, _client, data_dir, _resource_dir) = setup().await?;
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    // Create more than MAX_BACKUPS
    let max = MAX_BACKUPS.get();
    for i in 0..max + 2 {
        let timestamp = 1000 + i as u64;
        let version = format!("v{}", i);
        let id = backup_repo.add_backup_entry(&variant, &version, timestamp).await?;

        // Create dummy backup file
        let backup_path = get_or_create_automatic_backup_archive_filepath(
            &variant, id, &version, timestamp, data_dir.path()
        ).await?;
        tokio::fs::create_dir_all(backup_path.parent().unwrap()).await?;
        tokio::fs::write(&backup_path, "").await?;
    }

    cleanup_old_backups(backup_repo.clone(), &variant, data_dir.path()).await?;

    let backups = backup_repo.get_backups_sorted_by_timestamp(&variant).await?;
    assert_eq!(backups.len(), max);

    // Verify oldest backups (v0, v1) are gone
    assert!(backups.iter().all(|b| b.release_version != "v0" && b.release_version != "v1"));

    Ok(())
  }

  #[tokio::test]
  async fn test_launch_and_monitor_game_release_not_found() -> TestResult {
    let (db, _server, _client, data_dir, resource_dir) = setup().await?;
    let releases_repo = SqliteReleasesRepository::new(db.pool().clone());
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let active_release_repo = SqliteActiveReleaseRepository::new(db.pool().clone());
    let variant = GameVariant::BrightNights;
    let tag = "non-existent-tag";

    let result = launch_and_monitor_game(
        &variant,
        tag,
        None,
        &OS::Linux,
        12345,
        data_dir.path(),
        resource_dir.path(),
        &releases_repo,
        backup_repo,
        &active_release_repo,
        |_| async {}
    ).await;

    assert!(matches!(result, Err(LaunchGameError::Release(GetReleaseError::NotFound(_)))));

    Ok(())
  }

  #[tokio::test]
  async fn test_full_launch_flow() -> TestResult {
    let (db, server, client, data_dir, resource_dir) = setup().await?;
    let releases_repo = SqliteReleasesRepository::new(db.pool().clone());
    let backup_repo = SqliteBackupRepository::new(db.pool().clone());
    let active_release_repo = SqliteActiveReleaseRepository::new(db.pool().clone());
    let variant = GameVariant::BrightNights;
    let version = "2026-06-07";
    let tag = version;
    let timestamp = Utc::now().timestamp() as u64;

    // 1. Setup GitHub Mock for release and asset
    let owner = "cataclysmbnteam";
    let repo_name = "Cataclysm-BN";
    let asset_name = "cbn-linux-tiles-x64-2026-06-07.tar.gz";
    let fixture_path = get_fixtures_dir().join("dummy-release.tar.gz");

    let download_url = format!("{}/{}/{}/releases/download/{}/{}", server.uri(), owner, repo_name, version, asset_name);

    let mock_release_json = serde_json::json!({
        "id": 12345,
        "tag_name": tag,
        "prerelease": false,
        "created_at": "2024-01-01T00:00:00Z",
        "assets": [
            {
                "id": 54321,
                "name": asset_name,
                "browser_download_url": download_url,
                "content_type": "application/gzip",
                "size": 0,
                "state": "uploaded",
                "download_count": 0,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
                "url": "",
                "node_id": ""
            }
        ],
        "url": "",
        "html_url": "",
        "assets_url": "",
        "upload_url": "",
        "node_id": "",
        "target_commitish": "main",
        "draft": false,
        "author": {
            "login": owner,
            "id": 1,
            "node_id": "",
            "avatar_url": "",
            "gravatar_id": "",
            "html_url": "",
            "followers_url": "",
            "following_url": "",
            "gists_url": "",
            "starred_url": "",
            "subscriptions_url": "",
            "organizations_url": "",
            "repos_url": "",
            "events_url": "",
            "received_events_url": "",
            "type": "User",
            "site_admin": false
        }
    });
    let mock_release: MockRelease = serde_json::from_value(mock_release_json)?;

    let mock_asset = github_mock_api::Asset::from_path(
      asset_name,
      fixture_path,
      "application/gzip",
    );

    server.add_release(owner, repo_name, mock_release).await;
    server.add_asset(owner, repo_name, version, mock_asset).await;

    // 2. Fetch releases to populate releases_repo
    // We need to use the TestHttpClient which knows about host mappings
    variant.fetch_releases(
        client.as_ref(),
        resource_dir.path(),
        &releases_repo,
        |_| Ok::<(), std::io::Error>(()),
        &OS::Linux,
        &Arch::X64
    ).await?;

    // We also need to make sure the Downloader uses the same host mappings if it's going to use the mock server.
    // However, the Downloader in install_release uses a standard create_http_client().
    // We should probably pass a client to install_release if we could, but it takes &Downloader which owns a Client.
    // Let's see if we can make Downloader use our client.

    let mut release = get_release_by_id(
        &variant,
        tag,
        &OS::Linux,
        data_dir.path(),
        resource_dir.path(),
        &releases_repo
    ).await?;

    // 3. Install release
    // Use the TestHttpClient's internal client because it might have some configuration,
    // but browser_download_url needs to be reachable.
    let downloader = crate::infra::download::Downloader::new(client.client().clone(), PARALLEL_REQUESTS);
    release.install_release(
        &downloader,
        &OS::Linux,
        &Arch::X64,
        data_dir.path(),
        resource_dir.path(),
        &releases_repo,
        &active_release_repo,
        Arc::new(TestReporter)
    ).await?;

    // 4. Launch and monitor
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    launch_and_monitor_game(
        &variant,
        tag,
        None,
        &OS::Linux,
        timestamp,
        data_dir.path(),
        resource_dir.path(),
        &releases_repo,
        backup_repo,
        &active_release_repo,
        move |event| {
            let events_clone = events_clone.clone();
            async move {
                events_clone.lock().await.push(event);
            }
        }
    ).await?;

    // Since launch_and_monitor_game spawns tasks, we might need a small wait or check for conditions
    // Wait for a bit for tasks to run
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Verify active release was set
    let active = active_release_repo.get_active_release(&variant).await?;
    assert_eq!(active, Some(tag.to_string()));

    Ok(())
  }
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
