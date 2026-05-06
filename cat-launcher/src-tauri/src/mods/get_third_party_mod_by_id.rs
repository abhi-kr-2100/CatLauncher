use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError as RepoError, ModsRepository,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

/// Errors that can occur when retrieving a third-party mod by its ID.
#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  /// Failed to retrieve the mod from the repository.
  #[error("failed to get mod from repository: {0}")]
  Repository(#[from] RepoError),
}

/// Retrieves a third-party mod by its ID for a specific game variant.
pub async fn get_third_party_mod_by_id(
  mod_id: &str,
  variant: &GameVariant,
  mods_repository: &impl ModsRepository,
) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
  let m = mods_repository
    .get_third_party_mod_by_id(mod_id, variant)
    .await?;

  Ok(m)
}
