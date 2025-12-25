use crate::mods::repository::cached_mods_repository::{
  CachedModsRepository, CachedModsRepositoryError,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  #[error("failed to read cached mods: {0}")]
  Repository(#[from] CachedModsRepositoryError),

  #[error("mod not found: {0}")]
  ModNotFound(String),
}

pub async fn get_third_party_mod_by_id(
  mod_id: &str,
  variant: &GameVariant,
  cached_mods_repository: &dyn CachedModsRepository,
) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
  cached_mods_repository
    .get_cached_mod_by_id(variant, mod_id)
    .await?
    .ok_or_else(|| {
      GetThirdPartyModByIdError::ModNotFound(mod_id.to_string())
    })
}
