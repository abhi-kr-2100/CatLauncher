use std::path::PathBuf;

use cat_macros::CommandErrorSerialize;
use strum::IntoStaticStr;
use tauri::{command, AppHandle, Manager, State};

use crate::import_export::export::{export_game_data, ExportGameDataError};
use crate::import_export::import::{import_game_data, ImportGameDataError};
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::variants::GameVariant;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum ImportExportCommandError {
  #[error("failed to get app data directory: {0}")]
  AppDataDir(#[from] tauri::Error),

  #[error("export failed: {0}")]
  Export(#[from] ExportGameDataError),

  #[error("import failed: {0}")]
  Import(#[from] ImportGameDataError),

  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),
}

#[command]
pub async fn export_game_data_command(
  app_handle: AppHandle,
  variant: GameVariant,
  destination_path: PathBuf,
) -> Result<(), ImportExportCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  export_game_data(&variant, &data_dir, destination_path).await?;
  Ok(())
}

#[command]
pub async fn import_game_data_command(
  app_handle: AppHandle,
  variant: GameVariant,
  source_path: PathBuf,
  backup_repository: State<'_, SqliteManualBackupRepository>,
) -> Result<(), ImportExportCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let os = get_os_enum(std::env::consts::OS)?;

  import_game_data(
    &variant,
    &data_dir,
    source_path,
    backup_repository.inner(),
    &os,
  )
  .await?;

  Ok(())
}
