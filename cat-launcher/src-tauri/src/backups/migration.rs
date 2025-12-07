use std::path::{Path, PathBuf};

use strum::IntoEnumIterator;

use crate::filesystem::paths::{
    get_or_create_automatic_backup_archive_filepath, get_or_create_user_game_data_dir,
};
use crate::launch_game::repository::BackupRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum MigrationError {
    #[error("failed to get backup entries: {0}")]
    GetBackups(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to get new backup path: {0}")]
    GetNewPath(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to get old backup path: {0}")]
    GetOldPath(Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to move backup file: {0}")]
    MoveFile(#[from] std::io::Error),

    #[error("failed to get filename from path")]
    MissingFilename,
}

pub async fn migrate_older_automatic_backups(
    data_dir: &Path,
    backup_repository: &impl BackupRepository,
) -> Result<(), MigrationError> {
    for variant in GameVariant::iter() {
        let backups = backup_repository
            .get_backups_sorted_by_timestamp(&variant)
            .await
            .map_err(|e| MigrationError::GetBackups(Box::new(e)))?;

        for backup in backups {
            let new_path = get_or_create_automatic_backup_archive_filepath(
                &variant,
                backup.id,
                &backup.release_version,
                backup.timestamp,
                data_dir,
            )
            .await
            .map_err(|e| MigrationError::GetNewPath(Box::new(e)))?;

            if new_path.exists() {
                continue;
            }

            let old_path = get_old_backup_path(&variant, data_dir, &new_path).await?;

            if old_path.exists() {
                if let Some(parent) = new_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::rename(old_path, new_path).await?;
            }
        }
    }

    Ok(())
}

async fn get_old_backup_path(
    variant: &GameVariant,
    data_dir: &Path,
    new_path: &Path,
) -> Result<PathBuf, MigrationError> {
    // Old path: {user_game_data_dir}/backups/{filename}
    let user_data_dir = get_or_create_user_game_data_dir(variant, data_dir)
        .await
        .map_err(|e| MigrationError::GetOldPath(Box::new(e)))?;

    let filename = new_path.file_name().ok_or(MigrationError::MissingFilename)?;
    Ok(user_data_dir.join("backups").join(filename))
}
