use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub id: i64,
    pub game_variant: GameVariant,
    pub release_version: String,
    pub timestamp: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum BackupRepositoryError {
    #[error("failed to add backup entry: {0}")]
    Add(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to get backup entries: {0}")]
    Get(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to delete backup entry: {0}")]
    Delete(Box<dyn std::error::Error + Send + Sync>),
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

    async fn delete_backup_entry(&self, id: i64) -> Result<(), BackupRepositoryError>;
}
