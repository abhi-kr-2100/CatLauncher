use std::path::Path;

use strum::IntoEnumIterator;

use crate::filesystem::paths::GetUserGameDataDirError;
use crate::settings::paths::{
  get_or_create_user_config_dir, GetOrCreateUserConfigDirError,
};
use crate::settings::Settings;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum UpdateColorFilesError {
  #[error("failed to get user game data directory: {0}")]
  UserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to get or create user config directory: {0}")]
  GetOrCreateUserConfigDir(#[from] GetOrCreateUserConfigDirError),

  #[error("failed to copy color theme file: {0}")]
  Copy(#[from] std::io::Error),
}

pub async fn update_color_files(
  data_dir: &Path,
  settings: &Settings,
) -> Result<(), UpdateColorFilesError> {
  let selected_theme = &settings.color_theme;

  for variant in GameVariant::iter() {
    let config_dir =
      get_or_create_user_config_dir(&variant, data_dir).await?;
    let target_path = config_dir.join("base_colors.json");

    if let Some(theme) = selected_theme {
      tokio::fs::copy(&theme.path, &target_path).await?;
    } else {
      // Removing the file, resets the theme to the default
      match tokio::fs::remove_file(&target_path).await {
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(UpdateColorFilesError::Copy(e)),
      }
    }
  }

  Ok(())
}
