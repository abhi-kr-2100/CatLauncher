use std::path::PathBuf;

use rusqlite::Connection;
use strum::IntoEnumIterator;

use crate::theme::theme::Theme;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
/// Errors that can occur during database initialization.
pub enum InitializeDatabaseError {
  #[error("failed to execute schema: {0}")]
  Execute(#[from] rusqlite::Error),

  #[error("failed to read schema file: {0}")]
  ReadFile(#[from] std::io::Error),
}

/// Executes the SQL schema files for the database.
pub fn apply_schema(
  conn: &Connection,
  schema_paths: &[PathBuf],
) -> Result<(), InitializeDatabaseError> {
  for path in schema_paths {
    let schema = std::fs::read_to_string(path)?;
    conn.execute_batch(&schema)?;
  }

  Ok(())
}

/// Seeds the database with the stable reference data used by the app.
pub fn seed_reference_data(
  conn: &Connection,
) -> Result<(), InitializeDatabaseError> {
  for variant in GameVariant::iter() {
    conn.execute(
      "INSERT OR IGNORE INTO variants (name) VALUES (?1)",
      [variant.to_string()],
    )?;
  }

  for theme in Theme::iter() {
    conn.execute(
      "INSERT OR IGNORE INTO themes (name) VALUES (?1)",
      [theme.to_string()],
    )?;
  }

  Ok(())
}

/// Initializes the database schema and seeds the reference tables.
pub fn initialize_database(
  conn: &Connection,
  schema_paths: &[PathBuf],
) -> Result<(), InitializeDatabaseError> {
  apply_schema(conn, schema_paths)?;
  seed_reference_data(conn)?;

  Ok(())
}
