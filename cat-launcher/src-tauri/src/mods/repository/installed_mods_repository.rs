use std::error::Error;

use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledModsRepositoryError {
  #[error("failed to add installed mod: {0}")]
  Add(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete installed mod: {0}")]
  Delete(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to delete all installed mods: {0}")]
  DeleteAll(#[source] Box<dyn Error + Send + Sync>),

  #[error("failed to check if mod is installed: {0}")]
  IsInstalled(#[source] Box<dyn Error + Send + Sync>),

  #[error("installed mod with id {0} not found for variant {1}")]
  NotFound(String, String),
}

#[async_trait]
pub trait InstalledModsRepository: Send + Sync {
  async fn add_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError>;

  async fn delete_installed_mod(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError>;

  async fn delete_all_installed_mods(
    &self,
    game_variant: &GameVariant,
  ) -> Result<(), InstalledModsRepositoryError>;

  async fn is_mod_installed(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledModsRepositoryError>;
}
