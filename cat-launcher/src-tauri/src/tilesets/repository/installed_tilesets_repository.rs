use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledTilesetsRepositoryError {
    #[error("database error: {0}")]
    Database(String),

    #[error("installed tileset with id {0} not found for variant {1}")]
    NotFound(String, String),
}

#[async_trait]
pub trait InstalledTilesetsRepository: Send + Sync {
    async fn add_installed_tileset(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError>;

    async fn get_installed_tilesets_for_variant(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Vec<String>, InstalledTilesetsRepositoryError>;

    async fn delete_installed_tileset(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<(), InstalledTilesetsRepositoryError>;

    async fn is_tileset_installed(
        &self,
        tileset_id: &str,
        game_variant: &GameVariant,
    ) -> Result<bool, InstalledTilesetsRepositoryError>;
}
