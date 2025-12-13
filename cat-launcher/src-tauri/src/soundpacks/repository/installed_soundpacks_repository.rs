use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledSoundpacksRepositoryError {
    #[error("database error: {0}")]
    Database(String),

    #[error("installed soundpack with id {0} not found for variant {1}")]
    NotFound(String, String),
}

#[async_trait]
pub trait InstalledSoundpacksRepository: Send + Sync {
    async fn add_installed_soundpack(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError>;

    async fn get_installed_soundpacks_for_variant(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<String>, InstalledSoundpacksRepositoryError>;

    async fn delete_installed_soundpack(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledSoundpacksRepositoryError>;

    async fn is_soundpack_installed(
        &self,
        soundpack_id: &str,
        game_variant: &GameVariant,
    ) -> Result<bool, InstalledSoundpacksRepositoryError>;
}