use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

/// Errors that can occur when interacting with the installed soundpacks repository.
#[derive(thiserror::Error, Debug)]
pub enum InstalledSoundpacksRepositoryError {
  /// An error occurred while adding an installed soundpack.
  #[error("failed to add installed soundpack: {0}")]
  Add(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while deleting an installed soundpack.
  #[error("failed to delete installed soundpack: {0}")]
  Delete(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while deleting all installed soundpacks.
  #[error("failed to delete all installed soundpacks: {0}")]
  DeleteAll(#[source] Box<dyn Error + Send + Sync>),

  /// An error occurred while checking if a soundpack is installed.
  #[error("failed to check if soundpack is installed: {0}")]
  IsInstalled(#[source] Box<dyn Error + Send + Sync>),

  /// The specified installed soundpack was not found.
  #[error(
    "installed soundpack with id {0} not found for variant {1}"
  )]
  NotFound(String, String),
}

/// A repository for managing records of installed third-party soundpacks.
#[async_trait]
pub trait InstalledSoundpacksRepository: Send + Sync {
  /// Adds a record indicating that a soundpack has been installed for a specific game variant.
  async fn add_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  /// Deletes the record of an installed soundpack for a specific game variant.
  async fn delete_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  /// Deletes all records of installed soundpacks for a specific game variant.
  async fn delete_all_installed_soundpacks(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  /// Checks if a record exists for an installed soundpack for a specific game variant.
  async fn is_soundpack_installed(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledSoundpacksRepositoryError>;
}
