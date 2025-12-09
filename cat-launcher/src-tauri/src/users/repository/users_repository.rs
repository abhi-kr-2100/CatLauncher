use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum UsersRepositoryError {
    #[error("failed to get user: {0}")]
    Get(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to create user: {0}")]
    Create(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_or_create_user(&self, id: &str) -> Result<String, UsersRepositoryError>;
}
