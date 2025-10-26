use std::time::Duration;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{App, Listener, Manager};

use crate::filesystem::paths::{get_db_path, get_schema_file_path};
use crate::infra::autoupdate::update::run_updater;
use crate::repository::db_schema;
use crate::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::repository::sqlite_releases_repository::SqliteReleasesRepository;

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
    Schema(#[from] db_schema::InitializeSchemaError),

    #[error("failed to get schema file path: {0}")]
    SchemaFilePath(#[from] crate::filesystem::paths::GetSchemaFilePathError),

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
    db_schema::initialize_schema(&conn, &[schema_path])?;

    app.manage(SqliteReleasesRepository::new(pool.clone()));
    app.manage(SqliteBackupRepository::new(pool.clone()));
    app.manage(SqliteLastPlayedVersionRepository::new(pool));

    Ok(())
}
