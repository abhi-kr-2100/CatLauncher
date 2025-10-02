use std::path::PathBuf;

use tauri::command;

use crate::game_release::game_release::GameRelease;
use crate::install_release::error::InstallReleaseError;

#[command]
pub async fn install_release(release: GameRelease) -> Result<PathBuf, InstallReleaseError> {
    release.install_release().await
}
