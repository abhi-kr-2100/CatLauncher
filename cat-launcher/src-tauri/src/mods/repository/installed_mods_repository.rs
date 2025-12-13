use async_trait::async_trait;

use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum InstalledModsRepositoryError {
  #[error("database error: {0}")]
  Database(String),

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

  async fn is_mod_installed(
    &self,
    mod_id: &str,
    game_variant: &GameVariant,
  ) -> Result<bool, InstalledModsRepositoryError>;
}
