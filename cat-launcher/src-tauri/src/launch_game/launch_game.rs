use std::io;
use std::path::Path;
use std::process::Command;

use crate::game_release::GameRelease;
use crate::install_release::utils::{get_asset_download_dir, get_asset_extraction_dir};

#[derive(thiserror::Error, Debug)]
pub enum LaunchGameError {
    #[error("failed to launch game: {0}")]
    Failed(#[from] io::Error),

    #[error("executable directory not found")]
    ExecutableDirectoryNotFound,
}

impl GameRelease {
    pub fn launch_game(&self, data_dir: &Path) -> Result<(), LaunchGameError> {
        let download_dir = get_asset_download_dir(&self.variant, data_dir)?;
        let game_dir = get_asset_extraction_dir(self, &download_dir)?;

        let executable_path = match &self.variant {
            _ => game_dir
                .join("cataclysmbn-unstable")
                .join("cataclysm-bn-tiles"),
        };

        if let Some(executable_dir) = executable_path.parent() {
            Command::new(&executable_path)
                .current_dir(executable_dir)
                .spawn()?;
        } else {
            return Err(LaunchGameError::ExecutableDirectoryNotFound);
        }

        Ok(())
    }
}
