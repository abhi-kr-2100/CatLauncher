use std::path::{Path, PathBuf};
use std::time::Duration;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::infra::repository::db_schema::{
  InitializeSchemaError, initialize_schema,
};

pub type SqlitePool = Pool<SqliteConnectionManager>;

#[derive(thiserror::Error, Debug)]
pub enum CreateSqlitePoolError {
  #[error("failed to create connection pool: {0}")]
  ConnectionPool(#[from] r2d2::Error),

  #[error("failed to initialize schema: {0}")]
  Schema(#[from] InitializeSchemaError),
}

pub fn create_sqlite_pool(
  db_path: &Path,
  schema_paths: &[PathBuf],
) -> Result<SqlitePool, CreateSqlitePoolError> {
  let manager =
    SqliteConnectionManager::file(db_path).with_init(|conn| {
      conn.pragma_update(None, "journal_mode", "WAL")?;
      conn.pragma_update(None, "foreign_keys", "ON")?;
      conn.busy_timeout(Duration::from_secs(5))
    });
  let pool = Pool::new(manager)?;

  let conn = pool.get()?;
  initialize_schema(&conn, schema_paths)?;

  Ok(pool)
}
