use r2d2_sqlite::SqliteConnectionManager;
use std::time::Duration;
use tauri::{App, Manager};

use crate::{
  active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository,
  fetch_releases::repository::sqlite_releases_repository::SqliteReleasesRepository,
  filesystem::paths::{
    get_db_path, get_schema_file_path, GetSchemaFilePathError,
  },
  infra::repository::db_schema::{
    initialize_schema, InitializeSchemaError,
  },
  launch_game::repository::sqlite_backup_repository::SqliteBackupRepository,
  manual_backups::repository::sqlite_manual_backup_repository::SqliteManualBackupRepository,
  mods::repository::{
    sqlite_installed_mods_repository::SqliteInstalledModsRepository,
    sqlite_mods_repository::SqliteModsRepository,
  },
  play_time::sqlite_play_time_repository::SqlitePlayTimeRepository,
  settings::repository::sqlite_settings_repository::SqliteSettingsRepository,
  soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository,
  theme::sqlite_theme_preference_repository::SqliteThemePreferenceRepository,
  tilesets::repository::sqlite_installed_tilesets_repository::SqliteInstalledTilesetsRepository,
  users::repository::sqlite_users_repository::SqliteUsersRepository,
  variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository,
};

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
