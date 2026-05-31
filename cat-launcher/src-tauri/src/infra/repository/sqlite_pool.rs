use std::path::{Path, PathBuf};
use std::time::Duration;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_schema::{
  InitializeDatabaseError, initialize_database,
};

pub type SqlitePool = Pool<SqliteConnectionManager>;

#[derive(thiserror::Error, Debug)]
pub enum CreateSqlitePoolError {
  #[error("failed to create connection pool: {0}")]
  ConnectionPool(#[from] r2d2::Error),

  #[error("failed to initialize database: {0}")]
  Schema(#[from] InitializeDatabaseError),
}

pub fn create_sqlite_pool(
  db_path: &Path,
  schema_paths: &[PathBuf],
) -> Result<SqlitePool, CreateSqlitePoolError> {
  let pool = create_sqlite_pool_without_initialization(db_path)?;

  let conn = pool.get()?;
  initialize_database(&conn, schema_paths)?;

  Ok(pool)
}

pub fn create_sqlite_pool_without_initialization(
  db_path: &Path,
) -> Result<SqlitePool, r2d2::Error> {
  let manager =
    SqliteConnectionManager::file(db_path).with_init(|conn| {
      conn.pragma_update(None, "journal_mode", "WAL")?;
      conn.pragma_update(None, "foreign_keys", "ON")?;
      conn.busy_timeout(Duration::from_secs(5))
    });
  Pool::new(manager)
}
