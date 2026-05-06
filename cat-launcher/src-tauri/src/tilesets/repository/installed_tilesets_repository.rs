use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

/// Errors that can occur when interacting with the installed tilesets repository.
#[derive(thiserror::Error, Debug)]
pub enum InstalledTilesetsRepositoryError {
  /// An error occurred while adding an installed tileset.
  #[error("failed to add installed tileset: {0}")]
  Add(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while deleting an installed tileset.
  #[error("failed to delete installed tileset: {0}")]
  Delete(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while deleting all installed tilesets.
  #[error("failed to delete all installed tilesets: {0}")]
  DeleteAll(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while checking if a tileset is installed.
  #[error("failed to check if tileset is installed: {0}")]
  IsInstalled(#[source] Box<dyn Error + Send + Sync>),

  /// The specified installed tileset was not found.
  #[error("installed tileset with id {0} not found for variant {1}")]
  NotFound(String, String),
}

/// A repository for managing records of installed third-party tilesets.
#[async_trait]
pub trait InstalledTilesetsRepository: Send + Sync {
  /// Adds a record indicating that a tileset has been installed for a specific game variant.
  async fn add_installed_tileset(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError>;

  /// Deletes the record of an installed tileset for a specific game variant.
  async fn delete_installed_tileset(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError>;

  /// Deletes all records of installed tilesets for a specific game variant.
  async fn delete_all_installed_tilesets(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledTilesetsRepositoryError>;

  /// Checks if a record exists for an installed tileset for a specific game variant.
  async fn is_tileset_installed(
    &self,
    tileset_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledTilesetsRepositoryError>;
}
