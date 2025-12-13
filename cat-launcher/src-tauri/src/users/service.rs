use uuid::Uuid;

use crate::users::repository::users_repository::{
  UsersRepository, UsersRepositoryError,
};

#[derive(thiserror::Error, Debug)]
pub enum GetOrCreateUserIdError {
  #[error("failed to get user id: {0}")]
  GetUserId(#[from] UsersRepositoryError),
}

pub async fn get_or_create_user_id(
  repo: &impl UsersRepository,
) -> Result<String, GetOrCreateUserIdError> {
  let new_id = Uuid::new_v4().to_string();
  Ok(repo.get_or_create_user(&new_id).await?)
}
