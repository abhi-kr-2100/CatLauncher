use std::env::consts::OS;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use strum::IntoStaticStr;
use tauri::State;
use tauri::{command, AppHandle, Emitter, Manager};

use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::infra::command_error::SerializableError;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::last_played::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::launch_game::launch_game::{launch_and_monitor_game, GameEvent, LaunchGameError};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum LaunchGameCommandError {
    #[error("failed to launch game: {0}")]
    LaunchGame(#[from] LaunchGameError),

    #[error("system directory not found: {0}")]
    SystemDirectoryNotFound(#[from] tauri::Error),

    #[error("failed to get system time: {0}")]
    SystemTime(#[from] SystemTimeError),

    #[error("failed to get OS enum: {0}")]
    Os(#[from] OSNotSupportedError),
}

#[command]
pub async fn launch_game(
    app_handle: AppHandle,
    variant: GameVariant,
    release_id: &str,
    settings: State<'_, Settings>,
    releases_repository: State<'_, SqliteReleasesRepository>,
    last_played_repository: State<'_, SqliteLastPlayedVersionRepository>,
    backup_repository: State<'_, SqliteBackupRepository>,
    play_time_repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<(), SerializableError> {
    let result = launch_game_inner(
        app_handle,
        variant,
        release_id,
        settings,
        releases_repository,
        last_played_repository,
        backup_repository,
        play_time_repository,
    )
    .await;
    result.map_err(SerializableError::from)
}

pub async fn launch_game_inner(
    app_handle: AppHandle,
    variant: GameVariant,
    release_id: &str,
    settings: State<'_, Settings>,
    releases_repository: State<'_, SqliteReleasesRepository>,
    last_played_repository: State<'_, SqliteLastPlayedVersionRepository>,
    backup_repository: State<'_, SqliteBackupRepository>,
    play_time_repository: State<'_, SqlitePlayTimeRepository>,
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
        &os,
        time,
        &data_dir,
        &resource_dir,
        &*releases_repository,
        &*last_played_repository,
        backup_repository.inner().clone(),
        play_time_repository.inner().clone(),
        on_game_event,
        &settings,
    )
    .await?;

    Ok(())
}

impl From<LaunchGameCommandError> for SerializableError {
    fn from(error: LaunchGameCommandError) -> Self {
        SerializableError::new(error)
    }
}
