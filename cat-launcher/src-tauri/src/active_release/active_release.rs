use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseError {
  #[error("failed to access active release: {0}")]
  Repository(#[from] ActiveReleaseRepositoryError),
}

pub async fn get_active_release(
  variant: &GameVariant,
  repository: &dyn ActiveReleaseRepository,
) -> Result<Option<String>, ActiveReleaseError> {
  Ok(repository.get_active_release(variant).await?)
}

pub async fn set_active_release(
  variant: &GameVariant,
  version: &str,
  repository: &dyn ActiveReleaseRepository,
) -> Result<(), ActiveReleaseError> {
  repository.set_active_release(variant, version).await?;
  Ok(())
}
