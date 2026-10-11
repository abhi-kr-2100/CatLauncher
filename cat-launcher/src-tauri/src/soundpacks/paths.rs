use std::path::{Path, PathBuf};

use crate::filesystem::paths::{
  GetGameExecutableDirError, get_game_resources_dir,
};
use crate::infra::utils::OS;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetStockSoundpacksDirError {
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
}

pub async fn get_stock_soundpacks_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetStockSoundpacksDirError> {
  let game_resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  Ok(game_resources_dir.join("data").join("sound"))
}

pub fn get_soundpacks_resource_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("content").join("soundpacks.json")
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
  use crate::filesystem::paths::get_or_create_asset_installation_dir;
  use tempfile::tempdir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[test]
  fn test_get_soundpacks_resource_path() {
    let resource_dir = Path::new("/mock/resources");
    assert_eq!(
      get_soundpacks_resource_path(resource_dir),
      PathBuf::from("/mock/resources/content/soundpacks.json")
    );
  }

  #[tokio::test]
  async fn test_get_stock_soundpacks_dir_windows() -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let version = "v0.H";

    let result = get_stock_soundpacks_dir(
      &variant,
      version,
      temp.path(),
      &OS::Windows,
    )
    .await?;

    let expected_install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp.path(),
    )
    .await?;

    assert_eq!(
      result,
      expected_install_dir.join("data").join("sound")
    );
    Ok(())
  }

  #[tokio::test]
  async fn test_get_stock_soundpacks_dir_mac() -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let version = "v0.H";

    let result = get_stock_soundpacks_dir(
      &variant,
      version,
      temp.path(),
      &OS::Mac,
    )
    .await?;

    let expected_install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp.path(),
    )
    .await?;

    assert_eq!(
      result,
      expected_install_dir
        .join("Cataclysm.app")
        .join("Contents")
        .join("Resources")
        .join("data")
        .join("sound")
    );
    Ok(())
  }

  #[tokio::test]
  async fn test_get_stock_soundpacks_dir_linux() -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let version = "v0.H";

    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp.path(),
    )
    .await?;
    let game_folder = install_dir.join("cataclysm-dda-v0.H");
    tokio::fs::create_dir_all(&game_folder).await?;

    let result = get_stock_soundpacks_dir(
      &variant,
      version,
      temp.path(),
      &OS::Linux,
    )
    .await?;

    assert_eq!(result, game_folder.join("data").join("sound"));
    Ok(())
  }

  #[tokio::test]
  async fn test_get_stock_soundpacks_dir_linux_no_installation()
  -> TestResult {
    let temp = tempdir()?;
    let variant = GameVariant::DarkDaysAhead;
    let version = "v0.H";

    let result = get_stock_soundpacks_dir(
      &variant,
      version,
      temp.path(),
      &OS::Linux,
    )
    .await;

    assert!(matches!(
      result,
      Err(GetStockSoundpacksDirError::GameResourcesDir(
        GetGameExecutableDirError::NoInstallation
      ))
    ));
    Ok(())
  }
}
