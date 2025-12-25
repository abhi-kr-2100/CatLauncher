use std::error::Error;

use async_trait::async_trait;

use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum CachedModsRepositoryError {
  #[error("failed to get cached mods: {0}")]
  Get(Box<dyn Error + Send + Sync>),

  #[error("failed to update cached mods: {0}")]
  Update(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait CachedModsRepository: Send + Sync {
  async fn get_cached_mods(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<ThirdPartyMod>, CachedModsRepositoryError>;

  async fn get_cached_mod_by_id(
    &self,
    variant: &GameVariant,
    mod_id: &str,
  ) -> Result<Option<ThirdPartyMod>, CachedModsRepositoryError>;

  async fn update_cached_mods(
    &self,
    variant: &GameVariant,
    mods: &[ThirdPartyMod],
  ) -> Result<(), CachedModsRepositoryError>;
}
