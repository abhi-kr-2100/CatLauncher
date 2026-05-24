use std::path::PathBuf;

use crate::database::{SqlitePool, create_sqlite_pool};
use tempfile::TempDir;

pub struct TestDatabase {
  pub pool: SqlitePool,
  _temp_dir: TempDir,
}

impl TestDatabase {
  pub fn new() -> Self {
    let temp_dir =
      tempfile::tempdir().expect("create temp database dir");
    let db_path = temp_dir.path().join("cat-launcher-test.sqlite3");
    let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("schemas")
      .join("schema.sql");
    let pool = create_sqlite_pool(&db_path, &[schema_path])
      .expect("create test database pool");

    Self {
      pool,
      _temp_dir: temp_dir,
    }
  }
}

impl Default for TestDatabase {
  fn default() -> Self {
    Self::new()
  }
}
