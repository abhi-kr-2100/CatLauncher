use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::filesystem::paths::get_releases_cache_filepath;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::{read_from_file, write_to_file};
use crate::repository::releases_repository::{ReleasesRepository, ReleasesRepositoryError};
use crate::variants::game_variant::GameVariant;

pub struct FileReleasesRepository {
    cache_dir: PathBuf,
}

impl FileReleasesRepository {
    pub fn new(cache_dir: &Path) -> Self {
        Self {
            cache_dir: cache_dir.to_path_buf(),
        }
    }
}

#[async_trait]
impl ReleasesRepository for FileReleasesRepository {
    async fn get_cached_releases(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<GitHubRelease>, ReleasesRepositoryError> {
        let cache_file = get_releases_cache_filepath(game_variant, &self.cache_dir);

        if !cache_file.exists() {
            return Ok(Vec::new());
        }

        let releases = read_from_file::<Vec<GitHubRelease>>(&cache_file)
            .await
            .map_err(|err| ReleasesRepositoryError::Get(Box::new(err)))?;

        Ok(releases)
    }

    async fn update_cached_releases(
        &self,
        game_variant: &GameVariant,
        releases: &[GitHubRelease],
    ) -> Result<(), ReleasesRepositoryError> {
        let cache_file = get_releases_cache_filepath(game_variant, &self.cache_dir);

        if let Some(parent) = cache_file.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|err| ReleasesRepositoryError::Update(Box::new(err)))?;
            }
        }

        let existing_releases = self.get_cached_releases(game_variant).await?;

        let mut releases_map: HashMap<u64, GitHubRelease> =
            existing_releases.into_iter().map(|r| (r.id, r)).collect();

        for release in releases {
            releases_map.insert(release.id, release.clone());
        }

        let updated_releases: Vec<GitHubRelease> = releases_map.into_values().collect();

        write_to_file(&cache_file, &updated_releases)
            .await
            .map_err(|err| ReleasesRepositoryError::Update(Box::new(err)))?;

        Ok(())
    }
}
