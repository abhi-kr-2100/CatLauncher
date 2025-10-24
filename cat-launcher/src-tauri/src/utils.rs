use tauri::{App, Listener, Manager};

use crate::infra::autoupdate::update::run_updater;
use crate::repository::file_last_played_repository::FileLastPlayedVersionRepository;
use crate::repository::file_releases_repository::FileReleasesRepository;

pub fn autoupdate(app: &App) {
    let handle = app.handle();
    let handle_for_closure = handle.clone();
    handle.once("frontend-ready", move |_event| {
        let handle = handle_for_closure.clone();
        tauri::async_runtime::spawn(async move {
            run_updater(handle).await;
        });
    });
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("failed to get system directory: {0}")]
    SystemDir(#[from] tauri::Error),
}

pub fn manage_repositories(app: &App) -> Result<(), RepositoryError> {
    let cache_dir = app.path().app_cache_dir()?;
    let data_dir = app.path().app_local_data_dir()?;

    app.manage(FileReleasesRepository::new(&cache_dir));
    app.manage(FileLastPlayedVersionRepository::new(&data_dir));

    Ok(())
}
