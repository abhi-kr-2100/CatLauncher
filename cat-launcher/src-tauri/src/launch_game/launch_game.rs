use std::io;
use std::path::Path;
use std::process::Command;

use crate::game_release::GameRelease;
use crate::install_release::utils::{
    get_asset_download_dir, get_asset_extraction_dir, AssetDownloadDirError,
    AssetExtractionDirError,
};
use crate::launch_game::utils::{get_executable_path, GetExecutablePathError};

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
}

impl GameRelease {
    pub fn launch_game(&self, os: &str, data_dir: &Path) -> Result<(), LaunchGameError> {
        let download_dir = get_asset_download_dir(&self.variant, data_dir)?;
        let game_dir = get_asset_extraction_dir(&self.version, &download_dir)?;

        let executable_path = get_executable_path(&self.variant, os, &game_dir)?;
        let executable_dir = executable_path
            .parent()
            .ok_or(LaunchGameError::ExecutableDir)?;

        Command::new(&executable_path)
            .current_dir(executable_dir)
            .spawn()?;

        Ok(())
    }
}
