use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::read_to_string;

use crate::mods::paths::get_mods_resource_path;
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ListThirdPartyModsFromResourceError {
  #[error("failed to read mods.json: {0}")]
  ReadModsJson(#[from] io::Error),

  #[error("failed to parse mods.json: {0}")]
  ParseModsJson(#[from] serde_json::Error),
}

pub async fn list_third_party_mods_from_resource(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<ThirdPartyMod>, ListThirdPartyModsFromResourceError> {
  let mods_json_path = get_mods_resource_path(resource_dir);
  let content = read_to_string(&mods_json_path).await?;

  let mods_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_mods = match mods_data.get(game_variant) {
    Some(mods) => mods,
    None => return Ok(Vec::new()),
  };

  let mut mods = Vec::new();
  for mod_data in variant_mods.values() {
    let third_party_mod =
      serde_json::from_value::<ThirdPartyMod>(mod_data.clone())?;
    mods.push(third_party_mod);
  }

  Ok(mods)
}
