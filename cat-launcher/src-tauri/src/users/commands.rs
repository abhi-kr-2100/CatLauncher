use tauri::State;

use cat_macros::CommandErrorSerialize;

use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::users::service::{
  GetOrCreateUserIdError, get_or_create_user_id,
};

/// Errors that can occur when retrieving the user ID via a command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum GetUserIdCommandError {
  /// An error occurred while getting or creating the user ID.
  #[error("failed to get or create user id: {0}")]
  GetOrCreateUserId(#[from] GetOrCreateUserIdError),
}

/// Retrieves the unique identifier for the current user, creating one if it doesn't exist.
#[tauri::command]
pub async fn get_user_id(
  repo: State<'_, SqliteUsersRepository>,
) -> Result<String, GetUserIdCommandError> {
  let user_id = get_or_create_user_id(repo.inner()).await?;
  Ok(user_id)
}
