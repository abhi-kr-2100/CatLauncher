use std::path::Path;

use crate::mods::types::ThirdPartyMod;
use crate::mods::utils::{
  get_third_party_mods_json, GetThirdPartyModsJsonError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModError {
  #[error("failed to get third party mods json: {0}")]
  GetThirdPartyModsJson(#[from] GetThirdPartyModsJsonError),

  #[error("failed to parse mod: {0}")]
  ParseMod(#[from] serde_json::Error),

  #[error("mod with id `{0}` not found")]
  ModNotFound(String),
}

pub async fn get_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<ThirdPartyMod, GetThirdPartyModError> {
  let mods_data = get_third_party_mods_json(resource_dir).await?;

  let variant_mods = match mods_data.get(game_variant) {
    Some(mods) => mods,
    None => {
      return Err(GetThirdPartyModError::ModNotFound(
        mod_id.to_string(),
      ))
    }
  };

  let mod_data = match variant_mods.get(mod_id) {
    Some(mod_data) => mod_data,
    None => {
      return Err(GetThirdPartyModError::ModNotFound(
        mod_id.to_string(),
      ))
    }
  };

  let third_party_mod: ThirdPartyMod =
    serde_json::from_value(mod_data.clone())?;

  Ok(third_party_mod)
}
