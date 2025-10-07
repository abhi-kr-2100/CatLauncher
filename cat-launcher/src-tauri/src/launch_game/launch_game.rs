use std::io;
use std::path::Path;
use std::process::Command;

use crate::filesystem::paths::{get_game_executable_filepath, AssetDownloadDirError,
    AssetExtractionDirError, GetExecutablePathError,
};
use crate::game_release::GameRelease;
use crate::last_played::state::LastPlayedError;
use crate::launch_game::utils::{backup_and_copy_save_files, BackupAndCopyError};

#[derive(thiserror::Error, Debug)]
pub enum LaunchGameError {
    #[error("download directory not found: {0}")]
    DownloadDir(#[from] AssetDownloadDirError),

    #[error("game directory not found: {0}")]
    GameDir(#[from] AssetExtractionDirError),

    #[error("executable not found: {0}")]
    Executable(#[from] GetExecutablePathError),

    #[error("executable directory not found")]
    ExecutableDir,

    #[error("failed to launch game: {0}")]
    Launch(#[from] io::Error),

    #[error("failed to get last played version: {0}")]
    LastPlayed(#[from] LastPlayedError),

    #[error("failed to backup and copy saves: {0}")]
    BackupAndCopy(#[from] BackupAndCopyError),
}

impl GameRelease {
    pub async fn launch_game(
        &self,
        os: &str,
        timestamp: u64,
        data_dir: &Path,
    ) -> Result<(), LaunchGameError> {
        let executable_path =
            get_game_executable_filepath(&self.variant, &self.version, os, data_dir)?;
        let executable_dir = executable_path
            .parent()
            .ok_or(LaunchGameError::ExecutableDir)?;

        let last_played_version = self
            .variant
            .get_last_played_version(data_dir)?
            .unwrap_or(self.version.clone());

        backup_and_copy_save_files(
            &last_played_version,
            &self.version,
            &self.variant,
            &data_dir,
            timestamp,
        )
        .await?;

        Command::new(&executable_path)
            .current_dir(executable_dir)
            .spawn()?;

        self.variant
            .set_last_played_version(&self.version, data_dir)?;

        Ok(())
    }
}
