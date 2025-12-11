use std::fs;
use std::io;
use std::time::Duration;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{App, Listener, Manager};

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::backups::migration::migrate_older_automatic_backups;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::filesystem::paths::{get_db_path, get_schema_file_path};
use crate::filesystem::paths::{get_settings_path, GetSchemaFilePathError};
use crate::infra::autoupdate::update::run_updater;
use crate::infra::repository::db_schema::initialize_schema;
use crate::infra::repository::db_schema::InitializeSchemaError;
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::settings::Settings;
use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::users::service::get_or_create_user_id;
use crate::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;

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
    app.manage(SqliteManualBackupRepository::new(pool.clone()));
    app.manage(SqliteActiveReleaseRepository::new(pool.clone()));
    app.manage(SqlitePlayTimeRepository::new(pool.clone()));
    app.manage(SqliteGameVariantOrderRepository::new(pool.clone()));
    app.manage(SqliteInstalledModsRepository::new(pool.clone()));
    app.manage(SqliteUsersRepository::new(pool));

    Ok(())
}

pub fn migrate_backups(app: &App) {
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<SqliteBackupRepository> = handle.state();
        let data_dir = handle.path().app_local_data_dir().unwrap();
        if let Err(e) = migrate_older_automatic_backups(&data_dir, state.inner()).await {
            eprintln!("Failed to migrate backups: {}", e);
        }
    });
}

pub fn manage_posthog(app: &App) {
    let api_key = option_env!("VITE_PUBLIC_POSTHOG_KEY").unwrap_or_default();
    let host = option_env!("VITE_PUBLIC_POSTHOG_HOST").unwrap_or_default();

    if api_key.is_empty() || host.is_empty() {
        eprintln!("PostHog key or host not found, skipping initialization");
        return;
    }

    let api_endpoint = format!("{}/capture/", host.trim_end_matches('/'));

    let options = posthog_rs::ClientOptionsBuilder::default()
        .api_key(api_key.to_string())
        .api_endpoint(api_endpoint)
        .build();

    match options {
        Ok(options) => {
            let client = posthog_rs::client(options);
            let handle = app.handle().clone();

            app.manage(client);

            tauri::async_runtime::spawn(async move {
                let user_repo: tauri::State<SqliteUsersRepository> = handle.state();
                let user_id = match get_or_create_user_id(user_repo.inner()).await {
                    Ok(id) => id,
                    Err(e) => {
                        eprintln!(
                            "Failed to get or create user for PostHog identification: {}",
                            e
                        );
                        return;
                    }
                };

                if let Some(posthog) = handle.try_state::<posthog_rs::Client>() {
                    let mut event = posthog_rs::Event::new("$identify", &user_id);
                    let _ = event.insert_prop(
                        "$set",
                        std::collections::HashMap::from([("is_user_identified", true)]),
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
