use async_trait::async_trait;

use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum SaveThirdPartyModsError {
  #[error("failed to save third-party mods: {0}")]
  Save(Box<dyn std::error::Error + Send + Sync>),

  #[error("unsupported activity type: {0}")]
  UnsupportedActivityType(String),

  #[error("invalid data: {0}")]
  InvalidData(String),
}

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  #[error("failed to get third-party mod by id: {0}")]
  Get(Box<dyn std::error::Error + Send + Sync>),

  #[error("mod with id {0} not found for variant {1}")]
  NotFound(String, String),

  #[error("inconsistent data: {0}")]
  InconsistentData(String),

  #[error("unsupported activity type: {0}")]
  UnsupportedActivityType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ListCachedThirdPartyModsError {
  #[error("failed to list cached third-party mods: {0}")]
  Get(Box<dyn std::error::Error + Send + Sync>),

  #[error("inconsistent data: {0}")]
  InconsistentData(String),

  #[error("unsupported activity type: {0}")]
  UnsupportedActivityType(String),
}

#[async_trait]
pub trait ModsRepository: Send + Sync {
  async fn save_third_party_mods(
    &self,
    variant: &GameVariant,
    mods: Vec<ThirdPartyMod>,
  ) -> Result<(), SaveThirdPartyModsError>;

  async fn get_third_party_mod_by_id(
    &self,
    mod_id: &str,
    variant: &GameVariant,
  ) -> Result<ThirdPartyMod, GetThirdPartyModByIdError>;

  async fn get_third_party_mods(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<ThirdPartyMod>, ListCachedThirdPartyModsError>;
}
