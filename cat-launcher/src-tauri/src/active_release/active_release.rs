use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::variants::GameVariant;

/// Errors that can occur when interacting with the active release.
#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseError {
  /// An error occurred in the active release repository.
  #[error("failed to access active release: {0}")]
  Repository(#[from] ActiveReleaseRepositoryError),
}

impl GameVariant {
  /// Retrieves the version of the currently active release for this variant.
  pub async fn get_active_release(
    &self,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<Option<String>, ActiveReleaseError> {
    Ok(repository.get_active_release(self).await?)
  }

  /// Sets the version of the active release for this variant.
  pub async fn set_active_release(
    &self,
    version: &str,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<(), ActiveReleaseError> {
    repository.set_active_release(self, version).await?;
    Ok(())
  }
}
