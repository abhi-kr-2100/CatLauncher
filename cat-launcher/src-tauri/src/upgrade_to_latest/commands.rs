use std::env::consts::OS;
use std::sync::Arc;

use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::fetch_releases::ReleasesUpdatePayload;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_release::game_release::GameRelease;
use crate::infra::download::Downloader;
use crate::infra::installation_progress_monitor::channel_reporter::ChannelReporter;
use crate::infra::utils::{get_arch_enum, get_os_enum, ArchNotSupportedError, OSNotSupportedError};
use crate::upgrade_to_latest::upgrade_to_latest::UpgradeToLatestError;
use crate::variants::GameVariant;
use reqwest::Client;

#[derive(
  thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize,
)]
pub enum UpgradeToLatestCommandError {
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("upgrade failed: {0}")]
  Upgrade(#[from] UpgradeToLatestError),

  #[error("failed to get OS enum: {0}")]
  Os(#[from] OSNotSupportedError),

  #[error("failed to get arch enum: {0}")]
  Arch(#[from] ArchNotSupportedError),
}

#[command]
pub async fn upgrade_to_latest(
  app_handle: AppHandle,
  variant: GameVariant,
  releases_repository: State<'_, SqliteReleasesRepository>,
  active_release_repository: State<'_, SqliteActiveReleaseRepository>,
  downloader: State<'_, Downloader>,
  client: State<'_, Client>,
  on_download_progress: Channel,
) -> Result<GameRelease, UpgradeToLatestCommandError> {
  let data_dir = app_handle.path().app_local_data_dir()?;
  let resource_dir = app_handle.path().resource_dir()?;

  let os = get_os_enum(OS)?;
  let arch = get_arch_enum(std::env::consts::ARCH)?;

  let progress = Arc::new(ChannelReporter::new(on_download_progress));

  let on_releases = move |payload: ReleasesUpdatePayload| {
    app_handle.emit("releases-update", payload)?;
    Ok::<(), tauri::Error>(())
  };

  let release = variant
    .upgrade_to_latest(
      &client,
      &downloader,
      &os,
      &arch,
      &data_dir,
      &resource_dir,
      &*releases_repository,
      &*active_release_repository,
      progress,
      on_releases,
    )
    .await?;

  Ok(release)
}
