use tauri::{AppHandle, Manager, command};

use crate::last_played_world::last_played_world::{
  GetLastPlayedWorldError,
  get_last_played_world as get_last_played_world_impl,
};
use crate::variants::GameVariant;
use cat_macros::CommandErrorSerialize;

/// Errors that can occur when retrieving the last played world via a command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetLastPlayedWorldCommandError {
  /// An error occurred while retrieving the last played world.
  #[error("failed to get last played world: {0}")]
  GetLastPlayedWorld(#[from] GetLastPlayedWorldError),

  /// An error occurred while accessing the application's local data directory.
  #[error("failed to get app local data directory: {0}")]
  AppLocalDataDir(#[from] tauri::Error),
}

/// Retrieves the name of the last world played for the specified game variant.
///
/// This command reads the `lastworld.json` file from the variant's configuration directory.
#[command]
pub async fn get_last_played_world(
  app_handle: AppHandle,
  variant: GameVariant,
) -> Result<Option<String>, GetLastPlayedWorldCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let last_played_world =
    get_last_played_world_impl(&data_dir, &variant).await?;

  Ok(last_played_world)
}
