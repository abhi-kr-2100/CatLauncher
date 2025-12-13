use async_trait::async_trait;
use serde::Serialize;
use ts_rs::TS;

use crate::variants::GameVariant;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ManualBackupEntry {
  pub id: i64,
  pub name: String,
  pub game_variant: GameVariant,
  pub timestamp: u64,
  pub notes: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum ManualBackupRepositoryError {
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
pub trait ManualBackupRepository: Send + Sync {
  async fn add_manual_backup_entry(
    &self,
    name: &str,
    game_variant: &GameVariant,
    timestamp: u64,
    notes: Option<String>,
  ) -> Result<i64, ManualBackupRepositoryError>;

  async fn get_manual_backups_sorted_by_timestamp(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<ManualBackupEntry>, ManualBackupRepositoryError>;

  async fn get_manual_backup_entry(
    &self,
    id: i64,
  ) -> Result<ManualBackupEntry, ManualBackupRepositoryError>;

  async fn delete_manual_backup_entry(
    &self,
    id: i64,
  ) -> Result<(), ManualBackupRepositoryError>;
}
