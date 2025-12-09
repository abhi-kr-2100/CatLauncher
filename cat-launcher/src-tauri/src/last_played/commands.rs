use coded_error::CodedError;
use strum::IntoStaticStr;
use tauri::{command, State};
use thiserror::Error;

use crate::last_played::last_played::LastPlayedError;
use crate::last_played::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::variants::GameVariant;

#[derive(Error, Debug, IntoStaticStr, CodedError)]
pub enum LastPlayedCommandError {
    #[error("failed to get last played version: {0}")]
    GetLastPlayedVersion(#[from] LastPlayedError),

    #[error("failed to get system directory: {0}")]
    SystemDirectory(#[from] tauri::Error),
}

#[command]
pub async fn get_last_played_version(
    variant: GameVariant,
    repository: State<'_, SqliteLastPlayedVersionRepository>,
) -> Result<Option<String>, LastPlayedCommandError> {
    let last_played_version = variant.get_last_played_version(&*repository).await?;

    Ok(last_played_version)
}
