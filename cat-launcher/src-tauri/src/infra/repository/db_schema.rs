use std::path::PathBuf;

use rusqlite::Connection;
use strum::IntoEnumIterator;

use crate::theme::theme::Theme;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InitializeSchemaError {
  #[error("failed to execute schema: {0}")]
  Execute(#[from] rusqlite::Error),

  #[error("failed to read schema file: {0}")]
  ReadFile(#[from] std::io::Error),
}

pub fn initialize_schema(
  conn: &Connection,
  schema_paths: &[PathBuf],
) -> Result<(), InitializeSchemaError> {
  for path in schema_paths {
    let schema = std::fs::read_to_string(path)?;
    conn.execute_batch(&schema)?;
  }

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
