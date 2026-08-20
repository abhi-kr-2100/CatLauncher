use std::io;
use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetUserGameDataDirError, get_or_create_user_game_data_dir,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

/// Errors that can occur when retrieving or creating the user configuration directory.
#[derive(Debug, thiserror::Error)]
pub enum GetOrCreateUserConfigDirError {
  /// An error occurred while retrieving the user game data directory.
  #[error("Failed to get or create user game data directory")]
  GameDataDir(#[from] GetUserGameDataDirError),

  /// An error occurred while creating the configuration directory.
  #[error("Failed to create config directory: {0}")]
  CreateDirFailed(#[from] io::Error),
}

/// Retrieves the path to the user's configuration directory for a specific game variant, creating it if it doesn't exist.
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

/// Returns a list of standard directories where fonts are typically stored on the given operating system.
pub fn get_font_directories(os: &OS) -> Vec<PathBuf> {
  get_font_directories_impl(os, |key| std::env::var(key).ok())
}

fn get_font_directories_impl(
  os: &OS,
  env_var: impl Fn(&str) -> Option<String>,
) -> Vec<PathBuf> {
  let env_lookup =
    |key: &str| env_var(key).filter(|s| !s.trim().is_empty());

  let mut paths = Vec::new();
  let home = match os {
    OS::Windows => {
      env_lookup("USERPROFILE").or_else(|| env_lookup("HOME"))
    }
    _ => env_lookup("HOME").or_else(|| env_lookup("USERPROFILE")),
  };

  match os {
    OS::Linux => {
      if let Some(xdg) = env_lookup("XDG_DATA_HOME") {
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
      let windir = env_lookup("WINDIR")
        .or_else(|| env_lookup("SYSTEMROOT"))
        .unwrap_or_else(|| "C:\\Windows".to_string());
      paths.push(PathBuf::from(windir).join("Fonts"));
      if let Some(h) = &home {
        paths.push(
          PathBuf::from(h)
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Windows")
            .join("Fonts"),
        );
      }
    }
  }
  paths
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_or_create_user_config_dir() -> TestResult {
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let config_dir =
        get_or_create_user_config_dir(&variant, temp_data.path())
          .await?;

      assert!(config_dir.exists());
      assert!(config_dir.is_dir());
      assert!(config_dir.ends_with("config"));
    }

    Ok(())
  }

  #[test]
  fn test_get_font_directories_linux() {
    let linux_fonts = get_font_directories(&OS::Linux);
    assert!(
      linux_fonts.contains(&PathBuf::from("/usr/share/fonts")),
      "Linux font directories should include /usr/share/fonts"
    );
    assert!(
      linux_fonts.contains(&PathBuf::from("/usr/local/share/fonts")),
      "Linux font directories should include /usr/local/share/fonts"
    );
  }

  #[test]
  fn test_get_font_directories_mac() {
    let mac_fonts = get_font_directories(&OS::Mac);
    assert!(
      mac_fonts.contains(&PathBuf::from("/Library/Fonts")),
      "Mac font directories should include /Library/Fonts"
    );
    assert!(
      mac_fonts.contains(&PathBuf::from("/System/Library/Fonts")),
      "Mac font directories should include /System/Library/Fonts"
    );
  }

  #[test]
  fn test_get_font_directories_windows() {
    let win_fonts =
      get_font_directories_impl(&OS::Windows, |key| match key {
        "WINDIR" => Some("C:\\Windows".to_string()),
        "USERPROFILE" => Some("C:\\Users\\testuser".to_string()),
        _ => None,
      });
    assert!(
      win_fonts.contains(&PathBuf::from("C:\\Windows").join("Fonts")),
      "Windows font directories should include WINDIR Fonts"
    );
  }

  #[test]
  fn test_get_font_directories_env_branches() {
    let linux_fonts =
      get_font_directories_impl(&OS::Linux, |key| match key {
        "XDG_DATA_HOME" => Some("/tmp/custom_xdg_data".to_string()),
        "HOME" => Some("/home/testuser".to_string()),
        _ => None,
      });
    assert!(
      linux_fonts
        .contains(&PathBuf::from("/tmp/custom_xdg_data/fonts")),
      "Linux fonts should contain XDG_DATA_HOME/fonts"
    );
    assert!(
      linux_fonts.contains(&PathBuf::from(
        "/home/testuser/.local/share/fonts"
      )),
      "Linux fonts should contain HOME/.local/share/fonts"
    );
    assert!(
      linux_fonts.contains(&PathBuf::from("/home/testuser/.fonts")),
      "Linux fonts should contain HOME/.fonts"
    );

    let win_fonts =
      get_font_directories_impl(&OS::Windows, |key| match key {
        "WINDIR" => Some("C:\\CustomWin".to_string()),
        "USERPROFILE" => Some("C:\\Users\\testuser".to_string()),
        _ => None,
      });
    let expected_windir_fonts =
      PathBuf::from("C:\\CustomWin").join("Fonts");
    let expected_user_fonts = PathBuf::from("C:\\Users\\testuser")
      .join("AppData")
      .join("Local")
      .join("Microsoft")
      .join("Windows")
      .join("Fonts");
    assert!(
      win_fonts.contains(&expected_windir_fonts),
      "Windows fonts should contain WINDIR font directory"
    );
    assert!(
      win_fonts.contains(&expected_user_fonts),
      "Windows fonts should contain USERPROFILE font directory"
    );
  }

  #[test]
  fn test_get_font_directories_empty_env_fallback() {
    let linux_fonts =
      get_font_directories_impl(&OS::Linux, |key| match key {
        "XDG_DATA_HOME" => Some("".to_string()),
        "HOME" => Some("   ".to_string()),
        _ => None,
      });
    assert!(
      linux_fonts.contains(&PathBuf::from("/usr/share/fonts")),
      "Empty environment variables should fallback to system fonts"
    );
    assert!(
      !linux_fonts.iter().any(|p| p == &PathBuf::from("fonts")),
      "Empty environment variables should not generate relative paths"
    );

    let win_fonts =
      get_font_directories_impl(&OS::Windows, |key| match key {
        "WINDIR" => Some("".to_string()),
        "USERPROFILE" => Some("C:\\Users\\testuser".to_string()),
        "HOME" => Some("C:\\Users\\otheruser".to_string()),
        _ => None,
      });
    let expected_windir_fonts =
      PathBuf::from("C:\\Windows").join("Fonts");
    let expected_user_fonts = PathBuf::from("C:\\Users\\testuser")
      .join("AppData")
      .join("Local")
      .join("Microsoft")
      .join("Windows")
      .join("Fonts");
    assert!(
      win_fonts.contains(&expected_windir_fonts),
      "Empty WINDIR should fallback to C:\\Windows\\Fonts"
    );
    assert!(
      win_fonts.contains(&expected_user_fonts),
      "USERPROFILE should take precedence over HOME on Windows"
    );
  }
}
