use crate::last_played::repository::{
    LastPlayedVersionRepository, LastPlayedVersionRepositoryError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum LastPlayedError {
    #[error("failed to access last played version: {0}")]
    Repository(#[from] LastPlayedVersionRepositoryError),
}

impl GameVariant {
    pub async fn get_last_played_version(
        &self,
        repository: &dyn LastPlayedVersionRepository,
    ) -> Result<Option<String>, LastPlayedError> {
        Ok(repository.get_last_played_version(self).await?)
    }

    pub async fn set_last_played_version(
        &self,
        version: &str,
        repository: &dyn LastPlayedVersionRepository,
    ) -> Result<(), LastPlayedError> {
        repository.set_last_played_version(self, version).await?;
        Ok(())
    }
}
