use std::collections::HashMap;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::fetch_releases::repository::{
  ReleaseNotesRepository, ReleaseNotesRepositoryError,
  ReleasesRepository, ReleasesRepositoryError,
};
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::variants::game_variant::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteReleasesRepository {
  pool: Pool,
}

impl SqliteReleasesRepository {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl ReleasesRepository for SqliteReleasesRepository {
  async fn get_cached_releases(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<GitHubRelease>, ReleasesRepositoryError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;

    task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

            let mut stmt = conn
                .prepare(
                    "SELECT r.id, r.tag_name, r.prerelease, r.created_at, rn.body, a.id, a.browser_download_url, a.name, a.digest
                     FROM releases r
                     LEFT JOIN release_notes rn ON r.id = rn.release_id
                     LEFT JOIN assets a ON r.id = a.release_id
                     WHERE r.game_variant = ?1",
                )
                .map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

            let rows = stmt
                .query_map([game_variant.to_string()], |row| {
                    let release_id: u64 = row.get(0)?;
                    let tag_name: String = row.get(1)?;
                    let prerelease: bool = row.get(2)?;
                    let created_at: String = row.get(3)?;
                    let body: Option<String> = row.get(4)?;
                    let asset_id: Option<u64> = row.get(5)?;
                    let browser_download_url: Option<String> = row.get(6)?;
                    let name: Option<String> = row.get(7)?;
                    let digest: Option<String> = row.get(8)?;

                    let asset = asset_id
                        .zip(browser_download_url)
                        .zip(name)
                        .map(|((id, url), name)| GitHubAsset {
                            id,
                            browser_download_url: url,
                            name,
                            digest,
                        });

                    Ok((release_id, tag_name, prerelease, created_at, body, asset))
                })
                .map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

            let mut releases_map: HashMap<u64, GitHubRelease> = HashMap::new();
            for row in rows {
                let (release_id, tag_name, prerelease, created_at_str, body, asset) =
                    row.map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

                match releases_map.entry(release_id) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        let created_at = created_at_str.parse().map_err(|e| {
                            ReleasesRepositoryError::Get(Box::new(e))
                        })?;
                        e.insert(GitHubRelease {
                            id: release_id,
                            tag_name,
                            prerelease,
                            assets: Vec::new(),
                            created_at,
                            body,
                        });
                    }
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        if e.get().body.is_none() {
                            e.get_mut().body = body;
                        }
                    }
                }

                if let Some(asset) = asset {
                    if let Some(release) = releases_map.get_mut(&release_id) {
                        release.assets.push(asset);
                    }
                }
            }

            Ok(releases_map.into_values().collect())
        })
        .await
        .map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?
  }

  async fn update_cached_releases(
    &self,
    game_variant: &GameVariant,
    releases: &[GitHubRelease],
  ) -> Result<(), ReleasesRepositoryError> {
    let pool = self.pool.clone();
    let game_variant = *game_variant;
    let releases = releases.to_vec();

    task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;
            let tx = conn
                .transaction()
                .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;

            for release in releases {
                tx.execute(
                    "INSERT OR REPLACE INTO releases (id, tag_name, prerelease, created_at, game_variant) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                        release.id,
                        &release.tag_name,
                        release.prerelease,
                        release.created_at.to_rfc3339(),
                        game_variant.to_string(),
                    ),
                )
                .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;

                if let Some(body) = &release.body {
                    tx.execute(
                        "INSERT OR REPLACE INTO release_notes (release_id, body) VALUES (?1, ?2)",
                        (release.id, body),
                    )
                    .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;
                }

                for asset in &release.assets {
                    tx.execute(
                        "INSERT OR REPLACE INTO assets (id, release_id, browser_download_url, name, digest) VALUES (?1, ?2, ?3, ?4, ?5)",
                        (
                            asset.id,
                            release.id,
                            &asset.browser_download_url,
                            &asset.name,
                            &asset.digest,
                        ),
                    )
                    .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;
                }
            }

            tx.commit()
                .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?;
            Ok(())
        })
        .await
        .map_err(|e| ReleasesRepositoryError::Update(Box::new(e)))?
  }
}

#[async_trait]
impl ReleaseNotesRepository for SqliteReleasesRepository {
  async fn get_release_notes_by_tag_names(
    &self,
    game_variant: &GameVariant,
    tag_names: &[String],
  ) -> Result<
    HashMap<String, Option<String>>,
    ReleaseNotesRepositoryError,
  > {
    if tag_names.is_empty() {
      return Ok(HashMap::new());
    }

    let pool = self.pool.clone();
    let game_variant = *game_variant;
    let tag_names = tag_names.to_vec();

    task::spawn_blocking(move || {
      let conn = pool
        .get()
        .map_err(|e| ReleaseNotesRepositoryError::Get(Box::new(e)))?;

      let placeholders = std::iter::repeat_n("?", tag_names.len())
        .collect::<Vec<_>>()
        .join(", ");
      let sql = format!(
        "SELECT r.tag_name, rn.body\n         FROM releases r\n         LEFT JOIN release_notes rn ON r.id = rn.release_id\n         WHERE r.game_variant = ?\n           AND r.tag_name IN ({})",
        placeholders
      );

      let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| ReleaseNotesRepositoryError::Get(Box::new(e)))?;

      let params = std::iter::once(game_variant.to_string())
        .chain(tag_names.into_iter());

      let rows = stmt
        .query_map(rusqlite::params_from_iter(params), |row| {
          let tag_name: String = row.get(0)?;
          let body: Option<String> = row.get(1)?;
          Ok((tag_name, body))
        })
        .map_err(|e| ReleaseNotesRepositoryError::Get(Box::new(e)))?;

      let mut map = HashMap::new();
      for row in rows {
        let (tag_name, body) =
          row.map_err(|e| ReleaseNotesRepositoryError::Get(Box::new(e)))?;
        map.insert(tag_name, body);
      }

      Ok(map)
    })
    .await
    .map_err(|e| ReleaseNotesRepositoryError::Get(Box::new(e)))?
  }
}
