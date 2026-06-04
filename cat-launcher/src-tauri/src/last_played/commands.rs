use tauri::{command, State};

use cat_macros::CommandErrorSerialize;

use crate::last_played::last_played::LastPlayedError;
use crate::last_played::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::variants::GameVariant;

/// Errors that can occur when executing the last played version command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum LastPlayedCommandError {
    /// Failed to retrieve the last played version from the repository.
    #[error("failed to get last played version: {0}")]
    GetLastPlayedVersion(#[from] LastPlayedError),

    /// A required system directory could not be found.
    #[error("failed to get system directory: {0}")]
    SystemDirectory(#[from] tauri::Error),
}

/// Tauri command to retrieve the last played version for a game variant.
///
/// Returns the version string if it exists in the repository.
#[command]
pub async fn get_last_played_version(
    variant: GameVariant,
    repository: State<'_, SqliteLastPlayedVersionRepository>,
) -> Result<Option<String>, LastPlayedCommandError> {
    let last_played_version = variant.get_last_played_version(&*repository).await?;

    Ok(last_played_version)
}
