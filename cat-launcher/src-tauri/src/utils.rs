use std::path::Path;

use tauri::{
  App, AppHandle, Emitter, Listener, Manager, Runtime, WebviewWindow,
  WindowEvent,
};

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::constants::PARALLEL_REQUESTS;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::filesystem::paths::{
  get_db_path, get_schema_file_path, GetSchemaFilePathError,
};
use crate::infra::autoupdate::update::run_updater;
use crate::infra::download::Downloader;
use crate::infra::http_client::{
  create_http_client, HttpClientError, ReqwestHttpClient,
};
use crate::infra::repository::sqlite_pool::{
  create_sqlite_pool, CreateSqlitePoolError, SqlitePool,
};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::mods::lib::OnlineModRepositoryRegistry;
use crate::mods::online::bright_nights::BrightNightsModRepository;
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
use crate::theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository;
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;

pub const FRONTEND_READY_EVENT: &str = "frontend-ready";
pub const QUIT_REQUESTED_EVENT: &str = "quit-requested";

pub fn register_autoupdate(handle: &AppHandle) {
  let handle_for_closure = handle.clone();
  handle.once(FRONTEND_READY_EVENT, move |_event| {
    let handle = handle_for_closure.clone();
    tauri::async_runtime::spawn(async move {
      run_updater(handle).await;
    });
  });
}

pub fn autoupdate(app: &App) {
  register_autoupdate(app.handle());
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
  #[error("failed to get system directory: {0}")]
  SystemDir(#[from] tauri::Error),

  #[error("failed to get schema file path: {0}")]
  SchemaFilePath(#[from] GetSchemaFilePathError),

  #[error("failed to create database connection pool: {0}")]
  ConnectionPool(#[from] CreateSqlitePoolError),
}

pub fn create_repository_pool(
  data_dir: &Path,
  resources_dir: &Path,
) -> Result<SqlitePool, RepositoryError> {
  let db_path = get_db_path(data_dir);
  let schema_path = get_schema_file_path(resources_dir)?;
  let pool = create_sqlite_pool(&db_path, &[schema_path])?;
  Ok(pool)
}

pub fn register_repositories<R: Runtime, M: Manager<R>>(
  manager: &M,
  pool: SqlitePool,
) {
  manager.manage(SqliteReleasesRepository::new(pool.clone()));
  manager.manage(SqliteBackupRepository::new(pool.clone()));
  manager.manage(SqliteManualBackupRepository::new(pool.clone()));
  manager.manage(SqliteActiveReleaseRepository::new(pool.clone()));
  manager.manage(SqlitePlayTimeRepository::new(pool.clone()));
  manager.manage(SqliteGameVariantOrderRepository::new(pool.clone()));
  manager.manage(SqliteThemePreferenceRepository::new(pool.clone()));
  manager.manage(SqliteInstalledModsRepository::new(pool.clone()));
  manager.manage(SqliteModsRepository::new(pool.clone()));
  manager
    .manage(SqliteInstalledTilesetsRepository::new(pool.clone()));
  manager
    .manage(SqliteInstalledSoundpacksRepository::new(pool.clone()));
  manager.manage(SqliteUsersRepository::new(pool));
}

pub fn manage_repositories(app: &App) -> Result<(), RepositoryError> {
  let data_dir = app.path().app_local_data_dir()?;
  let resources_dir = app.path().resource_dir()?;
  let pool = create_repository_pool(&data_dir, &resources_dir)?;
  register_repositories(app, pool);
  Ok(())
}

pub fn create_online_mod_repository_registry()
-> OnlineModRepositoryRegistry {
  let mut online_mod_repository_registry =
    OnlineModRepositoryRegistry::default();
  online_mod_repository_registry
    .register(Box::new(BrightNightsModRepository::new()));
  online_mod_repository_registry
}

pub fn register_online_mod_repository_registry<
  R: Runtime,
  M: Manager<R>,
>(
  manager: &M,
) {
  manager.manage(create_online_mod_repository_registry());
}

pub fn manage_online_mod_repository_registry(app: &App) {
  register_online_mod_repository_registry(app);
}

pub fn create_downloader(client: reqwest::Client) -> Downloader {
  Downloader::new(ReqwestHttpClient::new(client), PARALLEL_REQUESTS)
}

pub fn register_downloader<R: Runtime, M: Manager<R>>(
  manager: &M,
  client: reqwest::Client,
) {
  let downloader = create_downloader(client);
  manager.manage(downloader);
}

pub fn manage_downloader(app: &App) {
  let client: tauri::State<reqwest::Client> = app.state();
  register_downloader(app, client.inner().clone());
}

pub fn register_http_client<R: Runtime, M: Manager<R>>(
  manager: &M,
  client: reqwest::Client,
) {
  manager.manage(client.clone());
  manager.manage(ReqwestHttpClient::new(client));
}

pub fn manage_http_client(app: &App) -> Result<(), HttpClientError> {
  let client = create_http_client()?;
  register_http_client(app, client);
  Ok(())
}

pub fn register_quit_handler<R: Runtime>(
  window: &WebviewWindow<R>,
  app_handle: AppHandle<R>,
) {
  window.on_window_event(move |event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
      api.prevent_close();
      let _ = app_handle.emit(QUIT_REQUESTED_EVENT, ());
    }
  });
}

pub fn on_quit(app: &App) {
  let Some(window) = app.get_webview_window("main") else {
    eprintln!(
      "Failed to get main webview window when registering quit handler"
    );
    return;
  };

  register_quit_handler(&window, app.handle().clone());
}
