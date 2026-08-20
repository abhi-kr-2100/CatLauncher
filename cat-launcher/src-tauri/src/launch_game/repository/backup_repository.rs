use async_trait::async_trait;
use serde::Serialize;
use ts_rs::TS;

use crate::variants::GameVariant;

/// Represents a single backup entry in the repository.
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct BackupEntry {
  /// Unique identifier for the backup entry.
  pub id: i64,
  /// The game variant this backup belongs to.
  pub game_variant: GameVariant,
  /// The version of the release when the backup was created.
  pub release_version: String,
  /// Unix timestamp when the backup was created.
  pub timestamp: u64,
}

/// Errors that can occur during backup repository operations.
#[derive(thiserror::Error, Debug)]
pub enum BackupRepositoryError {
  /// Failed to add a new backup entry.
  #[error("failed to add backup entry: {0}")]
  Add(Box<dyn std::error::Error + Send + Sync>),

  /// Failed to retrieve backup entries.
  #[error("failed to get backup entries: {0}")]
  Get(Box<dyn std::error::Error + Send + Sync>),

  /// Failed to delete a backup entry.
  #[error("failed to delete backup entry: {0}")]
  Delete(Box<dyn std::error::Error + Send + Sync>),

  /// The requested backup entry was not found.
  #[error("backup entry with id {0} not found")]
  NotFound(i64),
}

/// Trait defining the operations for managing game backups.
#[async_trait]
pub trait BackupRepository: Send + Sync {
  /// Adds a new backup entry to the repository.
  async fn add_backup_entry(
    &self,
    game_variant: &GameVariant,
    release_version: &str,
    timestamp: u64,
  ) -> Result<i64, BackupRepositoryError>;

  /// Re-inserts a backup entry using its original id.
  ///
  /// Used to roll back a deletion that failed on the filesystem so that the
  /// archive path derived from the id stays resolvable.
  async fn reinsert_backup_entry(
    &self,
    id: i64,
    game_variant: &GameVariant,
    release_version: &str,
    timestamp: u64,
  ) -> Result<(), BackupRepositoryError>;

  /// Retrieves all backup entries for a specific game variant, sorted by timestamp.
  async fn get_backups_sorted_by_timestamp(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<BackupEntry>, BackupRepositoryError>;

  /// Retrieves a specific backup entry by its ID.
  async fn get_backup_entry(
    &self,
    id: i64,
  ) -> Result<BackupEntry, BackupRepositoryError>;

  /// Deletes a backup entry from the repository by its ID.
  async fn delete_backup_entry(
    &self,
    id: i64,
  ) -> Result<(), BackupRepositoryError>;
}
