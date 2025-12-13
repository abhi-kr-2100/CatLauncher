use std::env::consts::OS;
use std::sync::Arc;

use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::infra::download::Downloader;
use crate::infra::utils::{get_arch_enum, get_os_enum, ArchNotSupportedError, OSNotSupportedError};
use crate::install_release::channel_reporter::ChannelReporter;
use crate::install_release::install_release::ReleaseInstallationError;
use crate::install_release::installation_progress_payload::InstallationProgressPayload;
use crate::variants::GameVariant;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum InstallReleaseCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("installation failed: {0}")]
  Install(#[from] ReleaseInstallationError<tauri::Error>),

  #[error("failed to obtain release: {0}")]
  Release(#[from] GetReleaseError),

  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),

  #[error("failed to get arch enum: {0}")]
  Arch(#[from] ArchNotSupportedError),
}

#[command]
pub async fn install_release(
  app_handle: AppHandle,
  variant: GameVariant,
  release_id: &str,
  releases_repository: State<'_, SqliteReleasesRepository>,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  downloader: State<'_, Downloader>,
  on_download_progress: Channel,
) -> Result<GameRelease, InstallReleaseCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resource_dir = app_handle.path().resource_dir()?;

  let os = get_os_enum(OS)?;
  let arch = get_arch_enum(std::env::consts::ARCH)?;

  let mut release = get_release_by_id(
    &variant,
    release_id,
    &os,
    &data_dir,
    &resource_dir,
    &*releases_repository,
  )
  .await?;

  let on_status_update = {
    let app_handle = app_handle.clone();
    move |payload: InstallationProgressPayload| {
      let app_handle = app_handle.clone();
      async move { app_handle.emit("installation-status-update", payload) }
    }
  };

  let progress = Arc::new(ChannelReporter::new(on_download_progress));

  release
    .install_release(
      &downloader,
      &os,
      &arch,
      &data_dir,
      &resource_dir,
      &*releases_repository,
      &*active_release_repository,
      on_status_update,
      progress,
    )
    .await?;

  Ok(release)
}
