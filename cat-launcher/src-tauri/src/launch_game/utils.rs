use std::path::Path;

use crate::filesystem::paths::{
    get_or_create_user_data_backup_archive_filepath, get_or_create_user_game_data_dir,
    GetUserDataBackupArchivePathError, GetUserGameDataDirError,
};
use crate::infra::archive::{create_zip_archive, ArchiveCreationError};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum BackupError {
    #[error("failed to get backup archive path: {0}")]
    BackupArchivePath(#[from] GetUserDataBackupArchivePathError),

    #[error("failed to create archive: {0}")]
    ArchiveCreation(#[from] ArchiveCreationError),

    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
}

pub async fn backup_save_files(
    variant: &GameVariant,
    data_dir: &Path,
    timestamp: u64,
) -> Result<(), BackupError> {
    let user_data_dir = get_or_create_user_game_data_dir(variant, data_dir).await?;

    let dirs_to_backup = vec![user_data_dir.join("save")];
    let archive_path =
        get_or_create_user_data_backup_archive_filepath(variant, data_dir, timestamp).await?;

    create_zip_archive(&user_data_dir, &dirs_to_backup, &archive_path).await?;

    Ok(())
}
