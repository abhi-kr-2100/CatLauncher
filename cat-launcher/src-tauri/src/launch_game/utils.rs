use std::path::{Path, PathBuf};
use std::{fs::read_dir, io};

#[derive(thiserror::Error, Debug)]
pub enum GetExecutablePathError {
    #[error("installation directory does not exist")]
    DoesNotExist,

    #[error("failed to get executable path: {0}")]
    Get(#[from] io::Error),

    #[error("unsupported OS: {0}")]
    UnsupportedOS(String),
}

pub fn get_executable_path(
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

    let launcher_path = match os {
        "linux" | "macos" => launcher_dir.join("cataclysm-launcher"),
        "windows" => launcher_dir.join("cataclysm-launcher.exe"),
        _ => return Err(GetExecutablePathError::UnsupportedOS(os.to_string())),
    };

    if !launcher_path.exists() || !launcher_path.is_file() {
        return Err(GetExecutablePathError::DoesNotExist);
    }

    Ok(launcher_path)
}
