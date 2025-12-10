use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum PlayTimeRepositoryError {
    #[error("Failed to log play time: {0}")]
    LogPlayTime(Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to get play time for version: {0}")]
    GetPlayTimeForVersion(Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to get play time for variant: {0}")]
    GetPlayTimeForVariant(Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to get total play time: {0}")]
    GetTotalPlayTime(Box<dyn std::error::Error + Send + Sync>),

    #[error("Task join error: {0}")]
    JoinError(Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid duration: {0}")]
    InvalidDuration(i64),
}

#[async_trait]
pub trait PlayTimeRepository: Send + Sync {
    async fn log_play_time(
        &self,
        game_variant: &GameVariant,
        version: &str,
        duration_in_seconds: i64,
    ) -> Result<(), PlayTimeRepositoryError>;

    async fn get_play_time_for_version(
        &self,
        game_variant: &GameVariant,
        version: &str,
    ) -> Result<i64, PlayTimeRepositoryError>;

    async fn get_play_time_for_variant(
        &self,
        game_variant: &GameVariant,
    ) -> Result<i64, PlayTimeRepositoryError>;

    async fn get_total_play_time(&self) -> Result<i64, PlayTimeRepositoryError>;
}
