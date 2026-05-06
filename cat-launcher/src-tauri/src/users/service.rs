use uuid::Uuid;

use crate::users::repository::users_repository::{
  UsersRepository, UsersRepositoryError,
};

/// Errors that can occur in the user service.
#[derive(thiserror::Error, Debug)]
pub enum GetOrCreateUserIdError {
  /// An error occurred in the underlying repository.
  #[error("failed to get user id: {0}")]
  GetUserId(#[from] UsersRepositoryError),
}

/// Retrieves the current user ID or generates and persists a new one if it doesn't exist.
pub async fn get_or_create_user_id(
  repo: &impl UsersRepository,
) -> Result<String, GetOrCreateUserIdError> {
  let new_id = Uuid::new_v4().to_string();
  Ok(repo.get_or_create_user(&new_id).await?)
}
