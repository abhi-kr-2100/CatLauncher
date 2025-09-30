use std::path::PathBuf;
use std::thread;

use tauri::command;
use tokio::sync::oneshot;

use crate::game_release::game_release::GameRelease;
use crate::install_release::error::InstallReleaseError;

#[command]
pub async fn install_release(release: GameRelease) -> Result<PathBuf, InstallReleaseError> {
    // The downloader crate currently uses internal non-Send closures in its async path.
    // Tauri requires command futures to be Send, so run the non-Send async work on a
    // dedicated thread with its own runtime and await the join handle here. That
    // keeps the tauri-facing future Send while allowing the downloader to use
    // non-Send internals.
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();

        if rt.is_err() {
            return;
        }
        let rt = rt.unwrap();

        let res = rt.block_on(async move { release.install_release().await });

        // Ignore send errors; receiver might be dropped if the caller went away.
        let _ = tx.send(res);
    });

    // Await the oneshot receiver asynchronously to avoid blocking the async
    // runtime thread.
    match rx.await {
        Ok(r) => r,
        Err(_) => Err(InstallReleaseError::Unknown),
    }
}
