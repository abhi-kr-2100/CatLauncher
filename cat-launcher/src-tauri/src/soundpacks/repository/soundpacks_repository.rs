use std::error::Error;

use async_trait::async_trait;

use crate::soundpacks::models::InstalledSoundpackMetadata;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum SoundpacksRepositoryError {
    #[error("failed to get installed soundpack: {0}")]
    Get(Box<dyn Error + Send + Sync>),

    #[error("failed to save installed soundpack: {0}")]
    Save(Box<dyn Error + Send + Sync>),

    #[error("failed to delete installed soundpack: {0}")]
    Delete(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait SoundpacksRepository: Send + Sync {
    async fn get_installed_soundpack(
        &self,
        variant: &GameVariant,
        soundpack_id: &str,
    ) -> Result<Option<InstalledSoundpackMetadata>, SoundpacksRepositoryError>;

    async fn save_installed_soundpack(
        &self,
        metadata: &InstalledSoundpackMetadata,
    ) -> Result<(), SoundpacksRepositoryError>;

    async fn delete_installed_soundpack(
        &self,
        variant: &GameVariant,
        soundpack_id: &str,
    ) -> Result<(), SoundpacksRepositoryError>;
}
