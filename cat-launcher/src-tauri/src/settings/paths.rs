use std::io;
use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

#[derive(Debug, thiserror::Error)]
pub enum GetOrCreateUserConfigDirError {
  #[error("Failed to get or create user game data directory")]
  GameDataDir(#[from] GetUserGameDataDirError),

  #[error("Failed to create config directory: {0}")]
  CreateDirFailed(#[from] io::Error),
}

pub async fn get_or_create_user_config_dir(
  variant: &GameVariant,
  data_dir: &Path,
) -> Result<PathBuf, GetOrCreateUserConfigDirError> {
  let user_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  let config_dir = user_dir.join("config");
  tokio::fs::create_dir_all(&config_dir).await?;
  Ok(config_dir)
}

pub fn get_font_directories(os: &OS) -> Vec<PathBuf> {
  let mut paths = Vec::new();
  let home = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .ok();

  match os {
    OS::Linux => {
      if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        paths.push(PathBuf::from(xdg).join("fonts"));
      }
      if let Some(h) = &home {
        paths.push(PathBuf::from(h).join(".local/share/fonts"));
        paths.push(PathBuf::from(h).join(".fonts"));
      }
      paths.push(PathBuf::from("/usr/share/fonts"));
      paths.push(PathBuf::from("/usr/local/share/fonts"));
    }
    OS::Mac => {
      if let Some(h) = &home {
        paths.push(PathBuf::from(h).join("Library/Fonts"));
      }
      paths.push(PathBuf::from("/Library/Fonts"));
      paths.push(PathBuf::from("/System/Library/Fonts"));
    }
    OS::Windows => {
      let windir = std::env::var("WINDIR")
        .or_else(|_| std::env::var("SYSTEMROOT"))
        .unwrap_or_else(|_| "C:\\Windows".to_string());
      paths.push(PathBuf::from(windir).join("Fonts"));
      if let Some(h) = &home {
        paths.push(
          PathBuf::from(h)
            .join("AppData\\Local\\Microsoft\\Windows\\Fonts"),
        );
      }
    }
  }
  paths
}
