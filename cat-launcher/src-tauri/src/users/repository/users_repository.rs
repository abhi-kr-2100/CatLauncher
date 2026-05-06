use async_trait::async_trait;

/// Errors that can occur when interacting with the users repository.
#[derive(Debug, thiserror::Error)]
pub enum UsersRepositoryError {
  /// An error occurred while retrieving the user.
  #[error("failed to get user: {0}")]
  Get(#[source] Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while creating a new user.
  #[error("failed to create user: {0}")]
  Create(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A repository for managing user data.
#[async_trait]
pub trait UsersRepository: Send + Sync {
  /// Retrieves the existing user ID or creates a new one with the provided `id` if none exists.
  async fn get_or_create_user(
    &self,
    id: &str,
  ) -> Result<String, UsersRepositoryError>;
}
