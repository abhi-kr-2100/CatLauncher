use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{App, Listener, Manager};
use tokio::fs as tokio_fs;

use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::backups::migration::migrate_older_automatic_backups;
use crate::fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository;
use crate::filesystem::paths::{get_db_path, get_schema_file_path};
use crate::filesystem::paths::{get_settings_path, GetSchemaFilePathError};
use crate::infra::autoupdate::update::run_updater;
use crate::infra::download::Downloader;
use crate::infra::http_client::create_http_client;
use crate::infra::repository::db_schema::initialize_schema;
use crate::infra::repository::db_schema::InitializeSchemaError;
use crate::launch_game::repository::sqlite_backup_repository::SqliteBackupRepository;
use crate::manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository;
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::settings::Settings;
use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;
use crate::theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository;
use crate::tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository;
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
    Ok(contents) => {
      serde_json::from_str(&contents).unwrap_or_default()
    }
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
  app.manage(SqliteInstalledModsRepository::new(pool.clone()));
  app.manage(SqliteInstalledTilesetsRepository::new(pool.clone()));
  app.manage(SqliteInstalledSoundpacksRepository::new(pool.clone()));
  app.manage(SqliteUsersRepository::new(pool));

  Ok(())
}

async fn migrate_dir_contents(
  from: &Path,
  to: &Path,
) -> Result<(), io::Error> {
  let mut entries = tokio_fs::read_dir(from).await?;
  while let Some(entry) = entries.next_entry().await? {
    let new_path = to.join(entry.file_name());
    if entry.file_type().await?.is_dir() {
      tokio_fs::create_dir_all(&new_path).await?;
      Box::pin(migrate_dir_contents(&entry.path(), &new_path))
        .await?;
      tokio_fs::remove_dir(&entry.path()).await?;
    } else if !tokio_fs::try_exists(&new_path).await.unwrap_or(true) {
      tokio_fs::rename(entry.path(), new_path).await?;
    }
  }

  Ok(())
}

pub fn migrate_to_local_dir(handle: &tauri::AppHandle) {
  let handle = handle.clone();
  tokio::spawn(async move {
    let app_data_dir = handle.path().app_data_dir();
    let app_local_data_dir = handle.path().app_local_data_dir();

    if let (Ok(app_data_dir), Ok(app_local_data_dir)) =
      (app_data_dir, app_local_data_dir)
    {
      if app_data_dir == app_local_data_dir {
        return;
      }

      if tokio_fs::try_exists(&app_data_dir).await.unwrap_or(false) {
        if let Err(e) =
          tokio_fs::create_dir_all(&app_local_data_dir).await
        {
          eprintln!("Failed to create local data directory: {}", e);
          return;
        }
        if let Err(e) =
          migrate_dir_contents(&app_data_dir, &app_local_data_dir)
            .await
        {
          eprintln!("Failed to migrate dir contents: {}", e);
          return;
        }
        if let Err(e) = tokio_fs::remove_dir_all(&app_data_dir).await
        {
          eprintln!("Failed to remove old data directory: {}", e);
        }
      }
    }
  });
}

pub fn migrate_backups(app: &App) {
  let handle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    let state: tauri::State<SqliteBackupRepository> = handle.state();
    match handle.path().app_local_data_dir() {
      Ok(data_dir) => {
        if let Err(e) =
          migrate_older_automatic_backups(&data_dir, state.inner())
            .await
        {
          eprintln!("Failed to migrate backups: {}", e);
        }
      }
      Err(e) => {
        eprintln!("Failed to get app local data directory: {}", e);
      }
    }
  });
}

pub fn manage_downloader(app: &App) {
  let settings: tauri::State<Settings> = app.state();
  let client: tauri::State<reqwest::Client> = app.state();
  let downloader = Downloader::new(
    client.inner().clone(),
    settings.parallel_requests,
  );
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

  let api_endpoint =
    format!("{}/capture/", host.trim_end_matches('/'));

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
