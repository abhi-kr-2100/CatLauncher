use std::path::Path;

use crate::mods::list_all_mods::{
  list_all_third_party_mods, ListThirdPartyModsError,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
  #[error("failed to list third-party mods: {0}")]
  ListThirdPartyMods(#[from] ListThirdPartyModsError),

  #[error("mod not found")]
  ModNotFound,
}

pub async fn get_third_party_mod_by_id(
  mod_id: &str,
  variant: &GameVariant,
  resource_dir: &Path,
) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
  let mods = list_all_third_party_mods(variant, resource_dir).await?;

  mods
    .iter()
    .find_map(|m| match m {
      crate::mods::types::Mod::ThirdParty(third_party_mod)
        if third_party_mod.id == mod_id =>
      {
        Some(third_party_mod.clone())
      }
      _ => None,
    })
    .ok_or(GetThirdPartyModByIdError::ModNotFound)
}
