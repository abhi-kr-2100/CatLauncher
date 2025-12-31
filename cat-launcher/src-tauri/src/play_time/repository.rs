use async_trait::async_trait;
use r2d2::Error as R2d2Error;
use rusqlite::Error as RusqliteError;
use tokio::task::JoinError;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum PlayTimeRepositoryError {
  #[error("Database connection error: {0}")]
  Connection(#[from] R2d2Error),

  #[error("Database query error: {0}")]
  Query(#[from] RusqliteError),

  #[error("Task join error: {0}")]
  Join(#[from] JoinError),

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
}
