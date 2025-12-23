pub mod sqlite;

use async_trait::async_trait;

use crate::backups::types::BackupEntry;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum BackupRepositoryError {
  #[error("failed to add backup entry: {0}")]
  Add(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to get backup entries: {0}")]
  Get(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to delete backup entry: {0}")]
  Delete(Box<dyn std::error::Error + Send + Sync>),

  #[error("backup entry with id {0} not found")]
  NotFound(i64),
}

#[async_trait]
pub trait BackupRepository: Send + Sync {
  async fn add_backup_entry(
    &self,
    game_variant: &GameVariant,
    release_version: &str,
    timestamp: u64,
  ) -> Result<i64, BackupRepositoryError>;

  async fn get_backups_sorted_by_timestamp(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<BackupEntry>, BackupRepositoryError>;

  async fn get_backup_entry(
    &self,
    id: i64,
  ) -> Result<BackupEntry, BackupRepositoryError>;

  async fn delete_backup_entry(
    &self,
    id: i64,
  ) -> Result<(), BackupRepositoryError>;
}
