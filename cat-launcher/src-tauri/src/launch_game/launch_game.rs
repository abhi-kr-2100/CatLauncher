use std::future::Future;
use std::io;
use std::path::Path;
use std::process::Stdio;

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::task::JoinError;
use ts_rs::TS;

use crate::filesystem::paths::{
    get_game_executable_filepath, AssetDownloadDirError, AssetExtractionDirError,
    GetExecutablePathError,
};
use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::last_played::last_played::LastPlayedError;
use crate::launch_game::utils::{backup_and_copy_save_files, BackupAndCopyError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum LaunchGameError {
    #[error("download directory not found: {0}")]
    DownloadDir(#[from] AssetDownloadDirError),

    #[error("game directory not found: {0}")]
    GameDir(#[from] AssetExtractionDirError),

    #[error("executable not found: {0}")]
    Executable(#[from] GetExecutablePathError),

    #[error("executable directory not found")]
    ExecutableDir,

    #[error("failed to launch game: {0}")]
    Launch(#[from] io::Error),

    #[error("failed to get last played version: {0}")]
    LastPlayed(#[from] LastPlayedError),

    #[error("failed to backup and copy saves: {0}")]
    BackupAndCopy(#[from] BackupAndCopyError),

    #[error("failed to get stdout from child process")]
    Stdout,

    #[error("failed to get stderr from child process")]
    Stderr,

    #[error("failed to obtain release: {0}")]
    Release(#[from] GetReleaseError),

    #[error("failed to wait for subtasks: {0}")]
    Subtasks(#[from] JoinError),
}

#[derive(Serialize, Clone, TS)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum GameEvent {
    Log(String),
    Exit(GameExitPayload),
    Error(GameErrorPayload),
}

#[derive(Serialize, Clone, serde::Deserialize, TS)]
#[ts(export)]
pub struct GameErrorPayload {
    pub message: String,
}

#[derive(Serialize, Clone, serde::Deserialize, TS)]
#[ts(export)]
pub struct GameExitPayload {
    pub code: Option<i32>,
}

impl GameRelease {
    pub async fn prepare_launch(
        &self,
        os: &str,
        timestamp: u64,
        data_dir: &Path,
    ) -> Result<Command, LaunchGameError> {
        let executable_path =
            get_game_executable_filepath(&self.variant, &self.version, os, data_dir).await?;

        let executable_dir = executable_path
            .parent()
            .ok_or(LaunchGameError::ExecutableDir)?
            .to_path_buf();

        let last_played_version = self
            .variant
            .get_last_played_version(data_dir)
            .await?
            .unwrap_or(self.version.clone()); // If no version is found, use the current version

        backup_and_copy_save_files(
            &last_played_version,
            &self.version,
            &self.variant,
            data_dir,
            timestamp,
        )
        .await?;

        self.variant
            .set_last_played_version(&self.version, data_dir)
            .await?;

        let mut command = Command::new(executable_path);

        command
            .current_dir(executable_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Ok(command)
    }
}

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
        while let Some(line) = stdout_reader.next_line().await.transpose() {
            if let Ok(line) = line {
                on_game_event_clone(GameEvent::Log(line)).await;
            }
        }
    });

    let on_game_event_clone = on_game_event.clone();
    let stderr_task = tokio::spawn(async move {
        while let Some(line) = stderr_reader.next_line().await.transpose() {
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

pub async fn launch_and_monitor_game<F, Fut>(
    variant: &GameVariant,
    release_id: &str,
    os: &str,
    timestamp: u64,
    cache_dir: &Path,
    data_dir: &Path,
    resource_dir: &Path,
    on_game_event: F,
) -> Result<(), LaunchGameError>
where
    F: Fn(GameEvent) -> Fut + Send + Sync + 'static + Clone,
    Fut: Future<Output = ()> + Send,
{
    let release =
        get_release_by_id(variant, release_id, os, cache_dir, data_dir, resource_dir).await?;

    let command = release.prepare_launch(os, timestamp, data_dir).await?;

    let on_game_event_for_error = on_game_event.clone();

    // It's important to not await the task here, as it be blocking.
    // run_game_and_monitor streams to the frontend.
    tokio::spawn(async move {
        if let Err(e) = run_game_and_monitor(command, on_game_event).await {
            let error_payload = GameErrorPayload {
                message: e.to_string(),
            };
            on_game_event_for_error(GameEvent::Error(error_payload)).await;
        }
    });

    Ok(())
}
