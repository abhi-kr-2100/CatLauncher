use std::env::consts::OS;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use strum::IntoStaticStr;
use tauri::{AppHandle, Emitter, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::launch_game::launch_game::{
  launch_and_monitor_game, GameEvent, LaunchGameError,
};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::variants::GameVariant;

/// Errors that can occur when executing the launch game command.
#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum LaunchGameCommandError {
  /// Error that occurred during the game launch or monitoring process.
  #[error("failed to launch game: {0}")]
  LaunchGame(#[from] LaunchGameError),

  /// The system directory required for the operation was not found.
  #[error("system directory not found: {0}")]
  SystemDirectoryNotFound(#[from] tauri::Error),

  /// Failed to retrieve the current system time.
  #[error("failed to get system time: {0}")]
  SystemTime(#[from] SystemTimeError),

  /// The current operating system is not supported.
  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),
}

/// Tauri command to launch and monitor a game instance.
///
/// This command resolves necessary paths, gets the current system time,
/// and starts the game monitoring process. It also sets up an event emitter
/// to forward game events to the frontend.
#[command]
#[allow(clippy::too_many_arguments)]
pub async fn launch_game(
  app_handle: AppHandle,
  variant: GameVariant,
  release_id: &str,
  world: Option<&str>,
  releases_repository: State<'_, SqliteReleasesRepository>,
  backup_repository: State<'_, SqliteBackupRepository>,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
) -> Result<(), LaunchGameCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resource_dir = app_handle.path().resource_dir()?;

  let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

  let os = get_os_enum(OS)?;

  let emitter = app_handle.clone();
  let on_game_event = move |event: GameEvent| {
    let emitter = emitter.clone();
    async move {
      // We cannot handle emit errors
      let _ = emitter.emit("game-event", event);
    }
  };

  launch_and_monitor_game(
    &variant,
    release_id,
    world,
    &os,
    time,
    &data_dir,
    &resource_dir,
    &*releases_repository,
    backup_repository.inner().clone(),
    &*active_release_repository,
    on_game_event,
  )
  .await?;

  Ok(())
}
