use crate::mods::repository::mods_repository::{
  GetThirdPartyModByIdError as RepoError, ModsRepository,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  #[error("failed to get mod from repository: {0}")]
  Repository(#[from] RepoError),
}

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
