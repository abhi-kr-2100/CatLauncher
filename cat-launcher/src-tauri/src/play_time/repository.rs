use async_trait::async_trait;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
pub struct PlayTime {
    pub game_variant: String,
    pub version: String,
    pub duration_in_seconds: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum PlayTimeRepositoryError {
    #[error("Failed to log play time: {0}")]
    LogPlayTime(String),
    #[error("Failed to get play time for version: {0}")]
    GetPlayTimeForVersion(String),
    #[error("Failed to get play time for variant: {0}")]
    GetPlayTimeForVariant(String),
    #[error("Failed to get total play time: {0}")]
    GetTotalPlayTime(String),
    #[error("Task join error: {0}")]
    JoinError(String),
}

#[async_trait]
pub trait PlayTimeRepository: Send + Sync {
    async fn log_play_time(
        &self,
        game_variant: String,
        version: String,
        duration_in_seconds: i64,
    ) -> Result<(), PlayTimeRepositoryError>;
    async fn get_play_time_for_version(
        &self,
        game_variant: String,
        version: String,
    ) -> Result<i64, PlayTimeRepositoryError>;
    async fn get_play_time_for_variant(
        &self,
        game_variant: String,
    ) -> Result<i64, PlayTimeRepositoryError>;
    async fn get_total_play_time(&self) -> Result<i64, PlayTimeRepositoryError>;
}
