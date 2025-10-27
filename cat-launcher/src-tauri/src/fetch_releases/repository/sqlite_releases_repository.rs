use std::collections::HashMap;

use async_trait::async_trait;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::fetch_releases::repository::{ReleasesRepository, ReleasesRepositoryError};
use crate::variants::game_variant::GameVariant;

type Pool = r2d2::Pool<SqliteConnectionManager>;

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
        let game_variant = game_variant.clone();

        task::spawn_blocking(move || {
            let conn = pool.get().map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

            let mut stmt = conn
                .prepare(
                    "SELECT r.id, r.tag_name, r.prerelease, r.created_at, a.id, a.browser_download_url, a.name, a.digest
                     FROM releases r
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
                    let asset_id: Option<u64> = row.get(4)?;
                    let browser_download_url: Option<String> = row.get(5)?;
                    let name: Option<String> = row.get(6)?;
                    let digest: Option<String> = row.get(7)?;

                    let asset = asset_id
                        .zip(browser_download_url)
                        .zip(name)
                        .map(|((id, url), name)| GitHubAsset {
                            id,
                            browser_download_url: url,
                            name,
                            digest,
                        });

                    Ok((release_id, tag_name, prerelease, created_at, asset))
                })
                .map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

            let mut releases_map: HashMap<u64, GitHubRelease> = HashMap::new();
            for row in rows {
                let (release_id, tag_name, prerelease, created_at_str, asset) =
                    row.map_err(|e| ReleasesRepositoryError::Get(Box::new(e)))?;

                if !releases_map.contains_key(&release_id) {
                    let created_at = created_at_str.parse().map_err(|e| {
                        ReleasesRepositoryError::Get(Box::new(e))
                    })?;
                    releases_map.insert(
                        release_id,
                        GitHubRelease {
                            id: release_id,
                            tag_name,
                            prerelease,
                            assets: Vec::new(),
                            created_at,
                        },
                    );
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
        let game_variant = game_variant.clone();
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
