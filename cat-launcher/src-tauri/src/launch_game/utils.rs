use std::path::{Path, PathBuf};
use std::{fs::read_dir, io};

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetExecutablePathError {
    #[error("installation directory does not exist")]
    DoesNotExist,

    #[error("failed to get executable path: {0}")]
    Get(#[from] io::Error),

    #[error("unsupported OS: {0}")]
    UnsupportedOS(#[from] LauncherFilenameError),
}

pub fn get_executable_path(
    variant: &GameVariant,
    os: &str,
    installation_dir: &Path,
) -> Result<PathBuf, GetExecutablePathError> {
    let launcher_dir = {
        if !installation_dir.is_dir() {
            return Err(GetExecutablePathError::DoesNotExist);
        }

        let first_subdir = read_dir(installation_dir)?
            .next()
            .ok_or(GetExecutablePathError::DoesNotExist)?;

        match first_subdir {
            Ok(entry) => entry.path(),
            Err(e) => return Err(GetExecutablePathError::Get(e)),
        }
    };

    let launcher_path = launcher_dir.join(get_launcher_filename(variant, os)?);

    if !launcher_path.exists() || !launcher_path.is_file() {
        return Err(GetExecutablePathError::DoesNotExist);
    }

    Ok(launcher_path)
}

#[derive(thiserror::Error, Debug)]
pub enum LauncherFilenameError {
    #[error("unsupported OS: {0}")]
    UnsupportedOS(String),
}

fn get_launcher_filename(
    variant: &GameVariant,
    os: &str,
) -> Result<&'static str, LauncherFilenameError> {
    match (variant, os) {
        (GameVariant::BrightNights | GameVariant::DarkDaysAhead, "linux" | "macos") => {
            Ok("cataclysm-launcher")
        }
        (GameVariant::BrightNights | GameVariant::DarkDaysAhead, "windows") => {
            Ok("cataclysm-launcher.exe")
        }

        (GameVariant::TheLastGeneration, "linux" | "macos") => Ok("cataclysm-tlg-tiles"),
        (GameVariant::TheLastGeneration, "windows") => Ok("cataclysm-tlg-tiles.exe"),

        _ => Err(LauncherFilenameError::UnsupportedOS(os.to_string())),
    }
}
