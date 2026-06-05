use std::path::PathBuf;

use rusqlite::Connection;
use tempfile::TempDir;

use crate::infra::repository::db_schema::{
  InitializeDatabaseError, apply_schema, initialize_database,
  seed_reference_data,
};
use crate::infra::repository::sqlite_pool::{
  SqlitePool, create_sqlite_pool_without_initialization,
};

type SchemaInitializer = Box<
  dyn FnOnce(&Connection, &[PathBuf]) -> Result<(), TestDatabaseError>
    + Send,
>;
type SeedInitializer = Box<
  dyn FnOnce(&Connection) -> Result<(), TestDatabaseError> + Send,
>;

#[derive(thiserror::Error, Debug)]
pub enum TestDatabaseError {
  #[error("failed to create temporary database directory: {0}")]
  TempDir(#[from] std::io::Error),

  #[error("failed to initialize test database: {0}")]
  Initialize(#[from] InitializeDatabaseError),

  #[error("failed to get a pooled sqlite connection: {0}")]
  Pool(#[from] r2d2::Error),

  #[error("failed to execute test database sql: {0}")]
  Sql(#[from] rusqlite::Error),
}

/// Builder for a temporary on-disk SQLite database used by tests.
pub struct TestDatabaseBuilder {
  schema_paths: Vec<PathBuf>,
  schema_initializer: Option<SchemaInitializer>,
  seed_initializer: Option<SeedInitializer>,
}

impl Default for TestDatabaseBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl TestDatabaseBuilder {
  pub fn new() -> Self {
    Self {
      schema_paths: vec![default_schema_path()],
      schema_initializer: None,
      seed_initializer: None,
    }
  }

  #[allow(dead_code)]
  pub fn with_schema_initializer<F>(mut self, initializer: F) -> Self
  where
    F: FnOnce(&Connection, &[PathBuf]) -> Result<(), TestDatabaseError>
      + Send
      + 'static,
  {
    self.schema_initializer = Some(Box::new(initializer));
    self
  }

  #[allow(dead_code)]
  pub fn with_seed_initializer<F>(mut self, initializer: F) -> Self
  where
    F: FnOnce(&Connection) -> Result<(), TestDatabaseError>
      + Send
      + 'static,
  {
    self.seed_initializer = Some(Box::new(initializer));
    self
  }

  pub fn build(self) -> Result<TestDatabase, TestDatabaseError> {
    let temp_dir = tempfile::tempdir()?;
    let db_path = temp_dir.path().join("cat-launcher.db");
    let schema_initializer = self.schema_initializer;
    let seed_initializer = self.seed_initializer;

    let pool = create_sqlite_pool_without_initialization(&db_path)?;

    {
      let conn = pool.get()?;
      match (schema_initializer, seed_initializer) {
        (None, None) => {
          initialize_database(&conn, &self.schema_paths)?;
        }
        (Some(schema_initializer), Some(seed_initializer)) => {
          schema_initializer(&conn, &self.schema_paths)?;
          seed_initializer(&conn)?;
        }
        (Some(schema_initializer), None) => {
          schema_initializer(&conn, &self.schema_paths)?;
          seed_reference_data(&conn)?;
        }
        (None, Some(seed_initializer)) => {
          apply_schema(&conn, &self.schema_paths)?;
          seed_initializer(&conn)?;
        }
      }
    }

    Ok(TestDatabase {
      _temp_dir: temp_dir,
      pool,
    })
  }
}

/// A reusable test database that keeps the temporary SQLite file alive.
pub struct TestDatabase {
  pool: SqlitePool,
  _temp_dir: TempDir,
}

impl TestDatabase {
  pub fn builder() -> TestDatabaseBuilder {
    TestDatabaseBuilder::new()
  }

  pub fn pool(&self) -> &SqlitePool {
    &self.pool
  }
}

fn default_schema_path() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas/schema.sql")
}
