use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::read_to_string;

use crate::mods::paths::get_mods_resource_path;
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModError {
  #[error("failed to read mods.json: {0}")]
  ReadModsJson(#[from] io::Error),

  #[error("failed to parse mods.json: {0}")]
  ParseModsJson(#[from] serde_json::Error),

  #[error("mod with id `{0}` not found")]
  ModNotFound(String),
}

pub async fn get_third_party_mod(
  mod_id: &str,
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<ThirdPartyMod, GetThirdPartyModError> {
  // Construct the path to mods.json
  let mods_json_path = get_mods_resource_path(resource_dir);

  // Try to read the mods.json file
  let content = match read_to_string(&mods_json_path).await {
    Ok(content) => content,
    Err(e) => return Err(GetThirdPartyModError::ReadModsJson(e)),
  };

  let mods_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

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
