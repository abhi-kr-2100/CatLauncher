use std::fs;
use std::io;
use std::time::Duration;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{App, Listener, Manager};

use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::filesystem::paths::{get_db_path, get_schema_file_path};
use crate::filesystem::paths::{get_settings_path, GetSchemaFilePathError};
use crate::infra::autoupdate::update::run_updater;
use crate::infra::repository::db_schema::initialize_schema;
use crate::infra::repository::db_schema::InitializeSchemaError;
use crate::last_played::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error("failed to get system directory: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to read settings file: {0}")]
    Read(#[from] io::Error),

    #[error("failed to parse settings file: {0}")]
    Parse(#[from] serde_json::Error),
}

pub fn manage_settings(app: &App) -> Result<(), SettingsError> {
    let resource_dir = app.path().resource_dir()?;
    let settings_path = get_settings_path(&resource_dir);

    let settings = match fs::read_to_string(&settings_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(settings) => settings,
            Err(_) => Settings::default(),
        },
        Err(_) => Settings::default(),
    };

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

    let manager = SqliteConnectionManager::file(&db_path).with_init(|conn| {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.busy_timeout(Duration::from_secs(5))
    });
    let pool = r2d2::Pool::new(manager)?;

    let conn = pool.get()?;
    initialize_schema(&conn, &[schema_path])?;

    app.manage(SqliteReleasesRepository::new(pool.clone()));
    app.manage(SqliteBackupRepository::new(pool.clone()));
    app.manage(SqliteLastPlayedVersionRepository::new(pool.clone()));
    app.manage(SqlitePlayTimeRepository::new(pool));

    Ok(())
}
