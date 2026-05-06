use async_trait::async_trait;

use crate::variants::GameVariant;

/// Errors that can occur when interacting with the play time repository.
#[derive(thiserror::Error, Debug)]
pub enum PlayTimeRepositoryError {
  /// An error occurred while logging play time.
  #[error("Failed to log play time: {0}")]
  LogPlayTime(Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while retrieving play time for a specific version.
  #[error("Failed to get play time for version: {0}")]
  GetPlayTimeForVersion(Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while retrieving play time for a specific variant.
  #[error("Failed to get play time for variant: {0}")]
  GetPlayTimeForVariant(Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while retrieving the total play time across all variants.
  #[error("Failed to get total play time: {0}")]
  GetTotalPlayTime(Box<dyn std::error::Error + Send + Sync>),

  /// An error occurred while waiting for a background task to complete.
  #[error("Task join error: {0}")]
  JoinError(Box<dyn std::error::Error + Send + Sync>),

  /// The provided duration is invalid (e.g., negative).
  #[error("Invalid duration: {0}")]
  InvalidDuration(i64),
}

/// A repository for managing play time data.
#[async_trait]
pub trait PlayTimeRepository: Send + Sync {
  /// Logs play time for a specific version of a game variant.
  async fn log_play_time(
    &self,
    game_variant: &GameVariant,
    version: &str,
    duration_in_seconds: i64,
  ) -> Result<(), PlayTimeRepositoryError>;

  /// Retrieves the play time for a specific version of a game variant.
  async fn get_play_time_for_version(
    &self,
    game_variant: &GameVariant,
    version: &str,
  ) -> Result<i64, PlayTimeRepositoryError>;

  /// Retrieves the total play time for a specific game variant.
  async fn get_play_time_for_variant(
    &self,
    game_variant: &GameVariant,
  ) -> Result<i64, PlayTimeRepositoryError>;
}
