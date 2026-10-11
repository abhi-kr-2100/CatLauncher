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

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use std::fs::File;

  use super::*;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[test]
  fn test_constants() {
    assert_eq!(FRONTEND_READY_EVENT, "frontend-ready");
    assert_eq!(QUIT_REQUESTED_EVENT, "quit-requested");
  }

  #[test]
  fn test_repository_error_system_dir() {
    let tauri_err = tauri::Error::AssetNotFound("missing.png".into());
    let repo_err = RepositoryError::from(tauri_err);
    let repo_err_msg = repo_err.to_string();
    assert!(
      repo_err_msg.starts_with("failed to get system directory: ")
    );
    assert!(repo_err_msg.contains("missing.png"));
    assert!(format!("{:?}", repo_err).contains("SystemDir"));
  }

  #[test]
  fn test_repository_error_schema_file_path() {
    let schema_err = GetSchemaFilePathError::ResourceDir(
      tauri::Error::AssetNotFound("schema.sql".into()),
    );
    let repo_err = RepositoryError::from(schema_err);
    let repo_err_msg = repo_err.to_string();
    assert!(
      repo_err_msg.starts_with("failed to get schema file path: ")
    );
    assert!(repo_err_msg.contains("schema.sql"));
    assert!(format!("{:?}", repo_err).contains("SchemaFilePath"));
  }

  #[test]
  fn test_repository_error_connection_pool_from_r2d2() -> TestResult {
    let pool = r2d2::Pool::builder()
      .max_size(1)
      .build(r2d2_sqlite::SqliteConnectionManager::memory())?;
    let _conn = pool.get()?;
    let r2d2_err = pool
      .get_timeout(std::time::Duration::from_millis(10))
      .unwrap_err();
    let pool_err = CreateSqlitePoolError::ConnectionPool(r2d2_err);
    let repo_err = RepositoryError::from(pool_err);
    let repo_err_msg = repo_err.to_string();
    assert!(
      repo_err_msg
        .starts_with("failed to create database connection pool: ")
    );
    assert!(repo_err_msg.contains("timed out"));
    assert!(format!("{:?}", repo_err).contains("ConnectionPool"));

    Ok(())
  }

  #[test]
  fn test_repository_error_connection_pool_from_schema() {
    let schema_init_err =
      crate::infra::repository::db_schema::InitializeDatabaseError::ReadFile(
        std::io::Error::new(std::io::ErrorKind::NotFound, "schema file missing"),
      );
    let pool_schema_err =
      CreateSqlitePoolError::Schema(schema_init_err);
    let repo_err = RepositoryError::from(pool_schema_err);
    let repo_err_msg = repo_err.to_string();
    assert!(
      repo_err_msg
        .starts_with("failed to create database connection pool: ")
    );
    assert!(repo_err_msg.contains("schema file missing"));
    assert!(format!("{:?}", repo_err).contains("ConnectionPool"));
  }

  #[test]
  fn test_create_repository_pool_success() -> TestResult {
    let temp_data_dir = tempfile::tempdir()?;
    let resources_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let pool =
      create_repository_pool(temp_data_dir.path(), resources_dir)?;
    let db_path = temp_data_dir.path().join("cat-launcher.db");
    assert!(db_path.exists());

    let conn = pool.get()?;
    let expected_tables = [
      "releases",
      "active_release",
      "backups",
      "manual_backups",
      "play_time",
      "game_variant_order",
      "theme_preferences",
      "installed_mods",
      "installed_tilesets",
      "installed_soundpacks",
      "users",
      "variants",
    ];
    for table in expected_tables {
      let count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
        [table],
        |row| row.get(0),
      )?;
      assert_eq!(count, 1, "expected table {} to exist", table);
    }

    Ok(())
  }

  #[test]
  fn test_create_repository_pool_missing_schema_error() -> TestResult
  {
    let temp_data_dir = tempfile::tempdir()?;
    let temp_resources_dir = tempfile::tempdir()?;

    let result = create_repository_pool(
      temp_data_dir.path(),
      temp_resources_dir.path(),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(matches!(err, RepositoryError::ConnectionPool(_)));

    Ok(())
  }

  #[test]
  fn test_create_repository_pool_invalid_data_dir_error() -> TestResult
  {
    let temp_dir = tempfile::tempdir()?;
    let blocking_file_path = temp_dir.path().join("not_a_dir");
    File::create(&blocking_file_path)?;
    let resources_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let result =
      create_repository_pool(&blocking_file_path, resources_dir);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(matches!(err, RepositoryError::ConnectionPool(_)));

    Ok(())
  }

  #[tokio::test]
  async fn test_create_online_mod_repository_registry() -> TestResult
  {
    let registry = create_online_mod_repository_registry();
    assert_eq!(registry.repositories().len(), 1);

    let client = ReqwestHttpClient::new(reqwest::Client::new());
    let mods = registry.repositories()[0]
      .get_mods_for_variant(
        &crate::variants::GameVariant::DarkDaysAhead,
        &client,
      )
      .await?;
    assert!(mods.is_empty());

    Ok(())
  }

  #[test]
  fn test_create_downloader() {
    let client = reqwest::Client::new();
    let _downloader = create_downloader(client);
  }

  #[test]
  fn test_register_repositories() -> TestResult {
    let app = tauri::test::mock_app();
    let pool = r2d2::Pool::new(
      r2d2_sqlite::SqliteConnectionManager::memory(),
    )?;

    register_repositories(&app, pool);

    assert!(app.try_state::<SqliteReleasesRepository>().is_some());
    assert!(app.try_state::<SqliteBackupRepository>().is_some());
    assert!(
      app.try_state::<SqliteManualBackupRepository>().is_some()
    );
    assert!(
      app.try_state::<SqliteActiveReleaseRepository>().is_some()
    );
    assert!(app.try_state::<SqlitePlayTimeRepository>().is_some());
    assert!(
      app
        .try_state::<SqliteGameVariantOrderRepository>()
        .is_some()
    );
    assert!(
      app.try_state::<SqliteThemePreferenceRepository>().is_some()
    );
    assert!(
      app.try_state::<SqliteInstalledModsRepository>().is_some()
    );
    assert!(app.try_state::<SqliteModsRepository>().is_some());
    assert!(
      app
        .try_state::<SqliteInstalledTilesetsRepository>()
        .is_some()
    );
    assert!(
      app
        .try_state::<SqliteInstalledSoundpacksRepository>()
        .is_some()
    );
    assert!(app.try_state::<SqliteUsersRepository>().is_some());

    Ok(())
  }

  #[test]
  fn test_register_online_mod_repository_registry() {
    let app = tauri::test::mock_app();

    register_online_mod_repository_registry(&app);

    let state = app.try_state::<OnlineModRepositoryRegistry>();
    assert!(state.is_some());
    let registry = state.unwrap();
    assert_eq!(registry.repositories().len(), 1);
  }

  #[test]
  fn test_register_downloader() {
    let app = tauri::test::mock_app();
    let client = reqwest::Client::new();

    register_downloader(&app, client);

    assert!(app.try_state::<Downloader>().is_some());
  }

  #[test]
  fn test_register_http_client() {
    let app = tauri::test::mock_app();
    let client = reqwest::Client::new();

    register_http_client(&app, client);

    assert!(app.try_state::<reqwest::Client>().is_some());
    assert!(app.try_state::<ReqwestHttpClient>().is_some());
  }
}
