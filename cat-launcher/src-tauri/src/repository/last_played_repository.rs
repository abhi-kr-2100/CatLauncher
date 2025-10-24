use std::error::Error;

use async_trait::async_trait;

use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum LastPlayedVersionRepositoryError {
    #[error("failed to get last played version: {0}")]
    Get(Box<dyn Error + Send + Sync>),

    #[error("failed to set last played version: {0}")]
    Set(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait LastPlayedVersionRepository: Send + Sync {
    async fn get_last_played_version(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Option<String>, LastPlayedVersionRepositoryError>;

    async fn set_last_played_version(
        &self,
        game_variant: &GameVariant,
        version: &str,
    ) -> Result<(), LastPlayedVersionRepositoryError>;
}
