use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use strum::IntoEnumIterator;

use crate::filesystem::paths::get_releases_dir;
use crate::infra::github::release::GitHubRelease;
use crate::theme::theme::Theme;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InitializeSchemaError {
  #[error("failed to execute schema: {0}")]
  Execute(#[from] rusqlite::Error),

  #[error("failed to read schema file: {0}")]
  ReadFile(#[from] std::io::Error),

  #[error("failed to seed releases: {0}")]
  SeedReleases(rusqlite::Error),

  #[error("failed to parse releases: {0}")]
  ParseReleases(serde_json::Error),
}

pub fn initialize_schema(
  conn: &Connection,
  schema_path: &Path,
  resources_dir: &Path,
) -> Result<(), InitializeSchemaError> {
  let schema = std::fs::read_to_string(schema_path)?;
  conn.execute_batch(&schema)?;

  let mut stmt = conn.prepare("PRAGMA table_info(releases)")?;
  let column_exists = stmt
    .query_map([], |row| row.get::<_, String>(1))?
    .any(|column_name| {
      column_name.is_ok() && column_name.unwrap() == "release_notes"
    });

  if !column_exists {
    conn.execute(
      "ALTER TABLE releases ADD COLUMN release_notes TEXT",
      [],
    )?;
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

  let releases_dir = get_releases_dir(resources_dir);
  for variant in GameVariant::iter() {
    let path = releases_dir.join(format!("{}.json", variant.id()));
    let contents = fs::read_to_string(path)?;
    let releases: Vec<GitHubRelease> =
      serde_json::from_str(&contents).map_err(InitializeSchemaError::ParseReleases)?;

    for release in releases {
      conn.execute(
        "INSERT OR IGNORE INTO releases (id, tag_name, prerelease, created_at, game_variant, release_notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
          release.id,
          &release.tag_name,
          release.prerelease,
          release.created_at.to_rfc3339(),
          variant.to_string(),
          &release.body,
        ),
      ).map_err(InitializeSchemaError::SeedReleases)?;

      for asset in &release.assets {
        conn.execute(
          "INSERT OR IGNORE INTO assets (id, release_id, browser_download_url, name, digest) VALUES (?1, ?2, ?3, ?4, ?5)",
          (
            asset.id,
            release.id,
            &asset.browser_download_url,
            &asset.name,
            &asset.digest,
          ),
        ).map_err(InitializeSchemaError::SeedReleases)?;
      }
    }
  }

  Ok(())
}
