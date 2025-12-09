use crate::active_release::repository::{
    ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseError {
    #[error("failed to access active release: {0}")]
    Repository(#[from] ActiveReleaseRepositoryError),
}

impl GameVariant {
    pub async fn get_active_release(
        &self,
        repository: &dyn ActiveReleaseRepository,
    ) -> Result<Option<String>, ActiveReleaseError> {
        Ok(repository.get_active_release(self).await?)
    }

    pub async fn set_active_release(
        &self,
        version: &str,
        repository: &dyn ActiveReleaseRepository,
    ) -> Result<(), ActiveReleaseError> {
        repository.set_active_release(self, version).await?;
        Ok(())
    }
}
