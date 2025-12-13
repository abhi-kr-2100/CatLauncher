use std::error::Error;

use async_trait::async_trait;

use crate::infra::github::release::GitHubRelease;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ReleasesRepositoryError {
  #[error("failed to get cached releases: {0}")]
  Get(Box<dyn Error + Send + Sync>),

  #[error("failed to update cached releases: {0}")]
  Update(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait ReleasesRepository: Send + Sync {
  async fn get_cached_releases(
    &self,
    game_variant: &GameVariant,
  ) -> Result<Vec<GitHubRelease>, ReleasesRepositoryError>;

  async fn update_cached_releases(
    &self,
    game_variant: &GameVariant,
    releases: &[GitHubRelease],
  ) -> Result<(), ReleasesRepositoryError>;
}
