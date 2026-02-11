use std::fs;
use std::io;
use std::time::Duration;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{App, Emitter, Listener, Manager, WindowEvent};

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::constants::PARALLEL_REQUESTS;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::filesystem::paths::{get_db_path, get_schema_file_path};
use crate::filesystem::paths::GetSchemaFilePathError;
use crate::filesystem::utils::{copy_dir_all, CopyDirError};
use crate::infra::autoupdate::update::run_updater;
use crate::infra::download::Downloader;
use crate::infra::http_client::create_http_client;
use crate::infra::repository::db_schema::initialize_schema;
use crate::infra::repository::db_schema::InitializeSchemaError;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::mods::lib::OnlineModRepositoryRegistry;
use crate::mods::online::bright_nights::BrightNightsModRepository;
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::settings::repository::settings_repository::SettingsRepository;
use crate::settings::repository::settings_repository::GetSettingsError;
use crate::settings::repository::sqlite_settings_repository::SqliteSettingsRepository;
use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
use crate::theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository;
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::users::service::get_or_create_user_id;
use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;

#[derive(thiserror::Error, Debug)]
pub enum ManageSettingsError {
  #[error("failed to get settings: {0}")]
  Get(#[from] GetSettingsError),
}

pub fn manage_settings(app: &App) -> Result<(), ManageSettingsError> {
  let settings_repo = app.state::<SqliteSettingsRepository>();
  let settings =
    tauri::async_runtime::block_on(settings_repo.get_settings())?;

  app.manage(settings);

  Ok(())
}

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

  #[error("failed to initialize database: {0}")]
  Database(#[from] rusqlite::Error),

  #[error("failed to initialize schema: {0}")]
  Schema(#[from] InitializeSchemaError),

  #[error("failed to get schema file path: {0}")]
  SchemaFilePath(#[from] GetSchemaFilePathError),

  #[error("failed to create connection pool: {0}")]
  ConnectionPool(#[from] r2d2::Error),
}

pub fn manage_repositories(app: &App) -> Result<(), RepositoryError> {
  let data_dir = app.path().app_local_data_dir()?;
  let db_path = get_db_path(&data_dir);

  let resources_dir = app.path().resource_dir()?;
  let schema_path = get_schema_file_path(&resources_dir)?;

  let manager =
    SqliteConnectionManager::file(&db_path).with_init(|conn| {
      conn.pragma_update(None, "journal_mode", "WAL")?;
      conn.pragma_update(None, "foreign_keys", "ON")?;
      conn.busy_timeout(Duration::from_secs(5))
    });
  let pool = r2d2::Pool::new(manager)?;

  let conn = pool.get()?;
  initialize_schema(&conn, &[schema_path])?;

  app.manage(SqliteReleasesRepository::new(pool.clone()));
  app.manage(SqliteBackupRepository::new(pool.clone()));
  app.manage(SqliteManualBackupRepository::new(pool.clone()));
  app.manage(SqliteActiveReleaseRepository::new(pool.clone()));
  app.manage(SqlitePlayTimeRepository::new(pool.clone()));
  app.manage(SqliteGameVariantOrderRepository::new(pool.clone()));
  app.manage(SqliteThemePreferenceRepository::new(pool.clone()));
  app.manage(SqliteSettingsRepository::new(pool.clone()));
  app.manage(SqliteInstalledModsRepository::new(pool.clone()));
  app.manage(SqliteModsRepository::new(pool.clone()));
  app.manage(SqliteInstalledTilesetsRepository::new(pool.clone()));
  app.manage(SqliteInstalledSoundpacksRepository::new(pool.clone()));
  app.manage(SqliteUsersRepository::new(pool));

  Ok(())
}

pub fn manage_online_mod_repository_registry(app: &App) {
  let mut online_mod_repository_registry =
    OnlineModRepositoryRegistry::default();
  online_mod_repository_registry
    .register(Box::new(BrightNightsModRepository::new()));
  app.manage(online_mod_repository_registry);
}

#[derive(thiserror::Error, Debug)]
pub enum MigrateToLocalDataDirError {
  #[error("failed to get app directory: {0}")]
  GetAppDir(#[from] tauri::Error),

  #[error("failed to canonicalize path: {0}")]
  CanonicalizePath(#[from] io::Error),

  #[error("failed to get OS: {0}")]
  GetOs(#[from] OSNotSupportedError),

  #[error("failed to copy directory: {0}")]
  CopyDir(#[from] CopyDirError),

  #[error("failed to remove directory: {0}")]
  RemoveDir(io::Error),
}

pub fn migrate_to_local_data_dir(app: &App) {
  let handle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    if let Err(e) = migrate_to_local_data_dir_impl(&handle).await {
      eprintln!("Migration to local data directory failed: {}", e);
    }
  });
}

async fn migrate_to_local_data_dir_impl(
  handle: &tauri::AppHandle,
) -> Result<(), MigrateToLocalDataDirError> {
  let app_data_dir = handle.path().app_data_dir()?;
  let app_local_data_dir = handle.path().app_local_data_dir()?;

  let app_data_dir_canonical = fs::canonicalize(&app_data_dir)?;
  let app_local_data_dir_canonical =
    fs::canonicalize(&app_local_data_dir)?;

  if app_data_dir_canonical == app_local_data_dir_canonical {
    return Ok(());
  }

  if !app_data_dir.exists() {
    return Ok(());
  }

  let os = get_os_enum(std::env::consts::OS)?;
  copy_dir_all(&app_data_dir, &app_local_data_dir, &os).await?;
  tokio::fs::remove_dir_all(&app_data_dir)
    .await
    .map_err(MigrateToLocalDataDirError::RemoveDir)?;

  Ok(())
}

pub fn manage_downloader(app: &App) {
  let client: tauri::State<reqwest::Client> = app.state();
  let downloader =
    Downloader::new(client.inner().clone(), PARALLEL_REQUESTS);
  app.manage(downloader);
}

pub fn manage_http_client(app: &App) {
  let client = create_http_client();
  app.manage(client);
}

pub fn manage_posthog(app: &App) {
  let api_key =
    option_env!("VITE_PUBLIC_POSTHOG_KEY").unwrap_or_default();
  let host =
    option_env!("VITE_PUBLIC_POSTHOG_HOST").unwrap_or_default();

  if api_key.is_empty() || host.is_empty() {
    eprintln!(
      "PostHog key or host not found, skipping initialization"
    );
    return;
  }

  let options = posthog_rs::ClientOptionsBuilder::default()
    .api_key(api_key.to_string())
    .host(host.to_string())
    .build();

  match options {
    Ok(options) => {
      let client =
        tauri::async_runtime::block_on(posthog_rs::client(options));
      let handle = app.handle().clone();

      app.manage(client);

      tauri::async_runtime::spawn(async move {
        let user_repo: tauri::State<SqliteUsersRepository> =
          handle.state();
        let user_id = match get_or_create_user_id(user_repo.inner())
          .await
        {
          Ok(id) => id,
          Err(e) => {
            eprintln!(
                            "Failed to get or create user for PostHog identification: {}",
                            e
                        );
            return;
          }
        };

        if let Some(posthog) =
          handle.try_state::<posthog_rs::Client>()
        {
          let mut event =
            posthog_rs::Event::new("$identify", &user_id);
          let _ = event.insert_prop(
            "$set",
            std::collections::HashMap::from([(
              "is_user_identified",
              true,
            )]),
          );
          if let Err(e) = posthog.capture(event).await {
            eprintln!("Failed to capture identify event: {}", e);
          }
        }
      });
    }
    Err(e) => {
      eprintln!("Failed to build PostHog options: {}", e);
    }
  }
}

pub fn on_quit(app: &App) {
  let app_handle = app.handle().clone();

  // Let the app crash and quit if webview window could not be gotten when quitting
  let window = app.get_webview_window("main").unwrap();

  window.on_window_event(move |event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
      api.prevent_close();
      let _ = app_handle.emit("quit-requested", ());
    }
  });
}
