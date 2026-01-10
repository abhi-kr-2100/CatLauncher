use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledSoundpacksRepositoryError {
  #[error("failed to add installed soundpack: {0}")]
  Add(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete installed soundpack: {0}")]
  Delete(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete all installed soundpacks: {0}")]
  DeleteAll(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to check if soundpack is installed: {0}")]
  IsInstalled(#[source] Box<dyn Error + Send + Sync>),

  #[error(
    "installed soundpack with id {0} not found for variant {1}"
  )]
  NotFound(String, String),
}

#[async_trait]
pub trait InstalledSoundpacksRepository: Send + Sync {
  async fn add_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  async fn delete_installed_soundpack(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  async fn delete_all_installed_soundpacks(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledSoundpacksRepositoryError>;

  async fn is_soundpack_installed(
    &self,
    soundpack_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledSoundpacksRepositoryError>;
}
