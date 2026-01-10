use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledTilesetsRepositoryError {
  #[error("failed to add installed tileset: {0}")]
  Add(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete installed tileset: {0}")]
  Delete(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete all installed tilesets: {0}")]
  DeleteAll(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to check if tileset is installed: {0}")]
  IsInstalled(#[source] Box<dyn Error + Send + Sync>),

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

  async fn delete_installed_tileset(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError>;

  async fn delete_all_installed_tilesets(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError>;

  async fn is_tileset_installed(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledTilesetsRepositoryError>;
}
