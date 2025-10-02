use std::path::PathBuf;

use tauri::{command, AppHandle, Manager};

use crate::game_release::game_release::GameRelease;
use crate::install_release::error::InstallReleaseError;

#[command]
pub async fn install_release(
    app_handle: AppHandle,
    release: GameRelease,
) -> Result<PathBuf, InstallReleaseError> {
    let cache_dir = app_handle.path().app_cache_dir()?;
    let data_dir = app_handle.path().app_local_data_dir()?;

    release.install_release(&cache_dir, &data_dir).await
}
