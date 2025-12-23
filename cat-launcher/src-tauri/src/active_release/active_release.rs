use crate::active_release::repository::{
  ActiveReleaseRepository,
  GetActiveReleaseError as RepoGetActiveReleaseError,
  SetActiveReleaseError as RepoSetActiveReleaseError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetActiveReleaseError {
  #[error("failed to access active release: {0}")]
  Repository(#[from] RepoGetActiveReleaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum SetActiveReleaseError {
  #[error("failed to access active release: {0}")]
  Repository(#[from] RepoSetActiveReleaseError),
}

impl GameVariant {
  pub async fn get_active_release(
    &self,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<Option<String>, GetActiveReleaseError> {
    Ok(repository.get_active_release(self).await?)
  }

  pub async fn set_active_release(
    &self,
    version: &str,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<(), SetActiveReleaseError> {
    repository.set_active_release(self, version).await?;
    Ok(())
  }
}
