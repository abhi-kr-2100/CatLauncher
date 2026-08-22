use std::env::consts::{ARCH, OS};
use std::sync::Arc;

use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State, command};

use cat_macros::CommandErrorSerialize;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::{get_release_by_id, GetReleaseError};
use crate::infra::download::Downloader;
use crate::infra::installation_progress_monitor::channel_reporter::ChannelReporter;
use crate::infra::utils::{HostSystem, HostSystemError};
use crate::install_release::install_release::ReleaseInstallationError;

use crate::variants::GameVariant;

/// Errors that can occur when executing the `install_release` command.
#[derive(thiserror::Error, Debug, CommandErrorSerialize)]
pub enum InstallReleaseCommandError {
  /// The system's local data or resource directory could not be found.
  #[error("system directory not found: {0}")]
  SystemDir(#[from] tauri::Error),

  /// The installation process failed.
  #[error("installation failed: {0}")]
  Install(#[from] ReleaseInstallationError),

  /// Failed to retrieve the release information from the repository.
  #[error("failed to obtain release: {0}")]
  Release(#[from] GetReleaseError),

  /// The host system (OS/architecture) is not supported.
  #[error("failed to determine host system: {0}")]
  HostSystem(#[from] HostSystemError),
}

/// A Tauri command that installs a specific game release.
///
/// This command handles downloading the release asset and extracting it to the
/// appropriate directory, while reporting progress via a channel.
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

  let host_system = HostSystem::current(OS, ARCH)?;

  let mut release = get_release_by_id(
    &variant,
    release_id,
    &host_system.os,
    &data_dir,
    &resource_dir,
    &*releases_repository,
  )
  .await?;

  let progress = Arc::new(ChannelReporter::new(on_download_progress));

  release
    .install_release(
      &downloader,
      &host_system,
      &data_dir,
      &resource_dir,
      &*releases_repository,
      &*active_release_repository,
      progress,
    )
    .await?;

  Ok(release)
}
