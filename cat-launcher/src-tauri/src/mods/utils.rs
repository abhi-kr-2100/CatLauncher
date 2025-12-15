use std::collections::HashMap;
use std::io;
use std::path::Path;

use tokio::fs::read_to_string;

use crate::mods::paths::get_mods_resource_path;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModsJsonError {
  #[error("failed to read mods.json: {0}")]
  ReadModsJson(#[from] io::Error),

  #[error("failed to parse mods.json: {0}")]
  ParseModsJson(#[from] serde_json::Error),
}

pub async fn get_third_party_mods_json(
  resource_dir: &Path,
) -> Result<
  HashMap<GameVariant, HashMap<String, serde_json::Value>>,
  GetThirdPartyModsJsonError,
> {
  // Construct the path to mods.json
  let mods_json_path = get_mods_resource_path(resource_dir);

  // Try to read the mods.json file
  let content = match read_to_string(&mods_json_path).await {
    Ok(content) => content,
    Err(e) => {
      return Err(GetThirdPartyModsJsonError::ReadModsJson(e))
    }
  };

  let mods_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  Ok(mods_data)
}
