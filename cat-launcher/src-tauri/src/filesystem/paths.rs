use std::io;
use std::path::{Path, PathBuf};

use tokio::fs::{create_dir_all, read_dir};

use crate::filesystem::utils::get_safe_filename;
use crate::infra::utils::OS;
use crate::variants::GameVariant;

pub fn get_db_path(data_dir: &Path) -> PathBuf {
  data_dir.join("cat-launcher.db")
}

pub fn get_settings_path(resource_dir: &Path) -> PathBuf {
  resource_dir.join("settings.json")
}

#[derive(thiserror::Error, Debug)]
pub enum GetSchemaFilePathError {
  #[error("failed to get resource directory: {0}")]
  ResourceDir(#[from] tauri::Error),
}

pub fn get_schema_file_path(
  resources_dir: &Path,
) -> Result<PathBuf, GetSchemaFilePathError> {
  let schema_dir = resources_dir.join("schemas");
  Ok(schema_dir.join("schema.sql"))
}

pub fn get_releases_dir(resources_dir: &Path) -> PathBuf {
  resources_dir.join("releases")
}

pub fn get_default_releases_file_path(
  variant: &GameVariant,
  resources_dir: &Path,
) -> PathBuf {
  resources_dir
    .join("releases")
    .join(format!("{}.json", variant.id()))
}

pub fn get_releases_cache_filepath(
  variant: &GameVariant,
  cache_dir: &Path,
) -> PathBuf {
  cache_dir
    .join("Releases")
    .join(format!("{}.json", variant.id()))
}

#[derive(thiserror::Error, Debug)]
pub enum AssetDownloadDirError {
  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),
}

pub async fn get_or_create_asset_download_dir(
  variant: &GameVariant,
  data_dir: &Path,
) -> Result<PathBuf, AssetDownloadDirError> {
  let dir = data_dir.join("Assets").join(variant.id());

  create_dir_all(&dir).await?;

  Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum GetAutomaticBackupsDirError {
  #[error("failed to create backup directory: {0}")]
  DirFailed(#[from] io::Error),
}

pub async fn get_or_create_automatic_backups_dir(
  data_dir: &Path,
) -> Result<PathBuf, GetAutomaticBackupsDirError> {
  let dir = data_dir.join("Backups").join("Automatic");
  create_dir_all(&dir).await?;

  Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum AssetExtractionDirError {
  #[error("failed to create directory: {0}")]
  CreateDirectory(#[from] io::Error),
}

pub async fn get_or_create_asset_installation_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
) -> Result<PathBuf, AssetExtractionDirError> {
  let safe_dir_name = get_safe_filename(release_version);
  let dir = data_dir
    .join("Assets")
    .join(variant.id())
    .join(&safe_dir_name);

  create_dir_all(&dir).await?;

  Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum GetGameExecutableDirError {
  #[error("game directory not found")]
  GameDirectory,

  #[error("failed to read game directory")]
  Read(#[from] io::Error),

  #[error("game directory doesn't have game installation")]
  NoInstallation,

  #[error("failed to get asset extraction dir: {0}")]
  AssetExtractionDir(#[from] AssetExtractionDirError),
}

pub async fn get_game_executable_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetGameExecutableDirError> {
  let installation_dir = get_or_create_asset_installation_dir(
    variant,
    release_version,
    data_dir,
  )
  .await?;

  if os == &OS::Windows {
    return Ok(installation_dir);
  }

  if os == &OS::Mac {
    return Ok(
      installation_dir
        .join("Cataclysm.app")
        .join("Contents")
        .join("MacOS"),
    );
  }

  // On Linux, the game directory is located one directory under
  // the installation directory.
  let mut dir = read_dir(installation_dir).await?;
  while let Some(entry) = dir.next_entry().await? {
    let file_name = entry.file_name();
    if file_name
      .to_string_lossy()
      .to_lowercase()
      .starts_with("cataclysm")
      && entry.file_type().await?.is_dir()
    {
      return Ok(entry.path());
    }
  }

  Err(GetGameExecutableDirError::NoInstallation)
}

pub async fn get_game_resources_dir(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetGameExecutableDirError> {
  match (variant, os) {
    (_, OS::Mac) => {
      let installation_dir = get_or_create_asset_installation_dir(
        variant,
        release_version,
        data_dir,
      )
      .await?;
      let resources_dir = installation_dir
        .join("Cataclysm.app")
        .join("Contents")
        .join("Resources");
      Ok(resources_dir)
    }
    _ => {
      get_game_executable_dir(variant, release_version, data_dir, os)
        .await
    }
  }
}

pub fn get_game_executable_filenames(
  variant: &GameVariant,
  os: &OS,
) -> &'static [&'static str] {
  match (variant, os) {
    (g, OS::Windows) => match g {
      GameVariant::BrightNights => &["cataclysm-bn-tiles.exe"],
      GameVariant::DarkDaysAhead => &["cataclysm-tiles.exe"],
      GameVariant::TheLastGeneration => {
        &["cataclysm-tlg-tiles.exe", "cataclysm-tiles.exe"]
      }
    },

    (_, OS::Linux) => &["cataclysm-launcher"],
    (_, OS::Mac) => &["Cataclysm.sh"],
  }
}

#[derive(thiserror::Error, Debug)]
pub enum GetExecutablePathError {
  #[error("launcher file does not exist")]
  DoesNotExist,

  #[error("failed to get launcher directory: {0}")]
  LauncherDirectory(#[from] GetGameExecutableDirError),
}

pub async fn get_game_executable_filepath(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<PathBuf, GetExecutablePathError> {
  let dir = match get_game_executable_dir(
    variant,
    release_version,
    data_dir,
    os,
  )
  .await
  {
    Ok(dir) => dir,
    Err(GetGameExecutableDirError::NoInstallation) => {
      return Err(GetExecutablePathError::DoesNotExist)
    }
    Err(err) => {
      return Err(GetExecutablePathError::LauncherDirectory(err))
    }
  };

  let filenames = get_game_executable_filenames(variant, os);

  for filename in filenames {
    let filepath = dir.join(filename);

    match tokio::fs::metadata(&filepath).await {
      Ok(metadata) => {
        if metadata.is_file() {
          return Ok(filepath);
        }
      }
      Err(_) => continue,
    }
  }

  Err(GetExecutablePathError::DoesNotExist)
}

#[derive(thiserror::Error, Debug)]
pub enum GetVersionExecutableDirError {
  #[error("failed to get asset download dir: {0}")]
  AssetDownloadDir(#[from] AssetDownloadDirError),

  #[error("failed to get asset extraction dir: {0}")]
  AssetExtractionDir(#[from] AssetExtractionDirError),

  #[error("failed to get game executable dir: {0}")]
  GameExecutableDir(#[from] GetGameExecutableDirError),
}

pub async fn get_game_save_dirs(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<Vec<PathBuf>, GetGameExecutableDirError> {
  let dirs = &["save"];

  let executable_dir =
    get_game_executable_dir(variant, release_version, data_dir, os)
      .await?;
  Ok(dirs.iter().map(|d| executable_dir.join(d)).collect())
}

pub async fn get_game_save_and_config_dirs(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<Vec<PathBuf>, GetGameExecutableDirError> {
  let dirs = &[
    "achievements",
    "config",
    "graveyard",
    "memorial",
    "save",
    "templates",
  ];

  let executable_dir =
    get_game_executable_dir(variant, release_version, data_dir, os)
      .await?;
  Ok(dirs.iter().map(|d| executable_dir.join(d)).collect())
}

#[derive(thiserror::Error, Debug)]
pub enum GetTipFilePathsError {
  #[error("failed to get game executable dir: {0}")]
  GetGameExecutableDir(#[from] GetGameExecutableDirError),
}

pub async fn get_tip_file_paths(
  variant: &GameVariant,
  release_version: &str,
  data_dir: &Path,
  os: &OS,
) -> Result<Vec<PathBuf>, GetTipFilePathsError> {
  let resources_dir =
    get_game_resources_dir(variant, release_version, data_dir, os)
      .await?;

  let hints_path = resources_dir
    .join("data")
    .join("json")
    .join("npcs")
    .join("hints.json");

  let tips_path = match variant {
    GameVariant::BrightNights => {
      resources_dir.join("data").join("raw").join("tips.json")
    }
    GameVariant::DarkDaysAhead | GameVariant::TheLastGeneration => {
      resources_dir.join("data").join("core").join("tips.json")
    }
  };

  Ok(vec![tips_path, hints_path])
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserGameDataDirError {
  #[error("failed to create user data directory: {0}")]
  DirFailed(#[from] io::Error),
}

pub async fn get_or_create_user_game_data_dir(
  variant: &GameVariant,
  data_dir: &Path,
) -> Result<PathBuf, GetUserGameDataDirError> {
  let dir = data_dir.join("UserData").join(variant.id());
  create_dir_all(&dir).await?;

  Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum GetManualBackupsDirError {
  #[error("failed to create backup directory: {0}")]
  DirFailed(#[from] io::Error),
}

pub async fn get_or_create_manual_backups_dir(
  data_dir: &Path,
) -> Result<PathBuf, GetManualBackupsDirError> {
  let dir = data_dir.join("Backups").join("Manual");
  create_dir_all(&dir).await?;

  Ok(dir)
}

#[derive(thiserror::Error, Debug)]
pub enum GetAutomaticBackupArchivePathError {
  #[error("failed to create backup directory: {0}")]
  DirFailed(#[from] GetAutomaticBackupsDirError),
}

pub async fn get_or_create_automatic_backup_archive_filepath(
  variant: &GameVariant,
  id: i64,
  version: &str,
  timestamp: u64,
  data_dir: &Path,
) -> Result<PathBuf, GetAutomaticBackupArchivePathError> {
  let backup_dir =
    get_or_create_automatic_backups_dir(data_dir).await?;

  Ok(backup_dir.join(format!(
    "{}_{}_{}_{}.zip",
    id,
    variant.id(),
    get_safe_filename(version),
    timestamp
  )))
}

#[derive(thiserror::Error, Debug)]
pub enum GetManualBackupArchivePathError {
  #[error("failed to create backup directory: {0}")]
  DirFailed(#[from] GetManualBackupsDirError),
}

pub async fn get_or_create_manual_backup_archive_filepath(
  id: i64,
  name: &str,
  data_dir: &Path,
) -> Result<PathBuf, GetManualBackupArchivePathError> {
  let backup_dir = get_or_create_manual_backups_dir(data_dir).await?;

  Ok(backup_dir.join(format!(
    "{}_{}.zip",
    id,
    get_safe_filename(name)
  )))
}

#[derive(thiserror::Error, Debug)]
pub enum GetOrCreateDirectoryError {
  #[error("failed to create directory: {0}")]
  DirFailed(#[from] io::Error),
}

pub async fn get_or_create_directory(
  prefix: &Path,
  dir: &str,
) -> Result<PathBuf, GetOrCreateDirectoryError> {
  let dir_path = prefix.join(dir);
  create_dir_all(&dir_path).await?;

  Ok(dir_path)
}
