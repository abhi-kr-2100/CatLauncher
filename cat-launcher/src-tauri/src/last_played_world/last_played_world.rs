use std::io;
use std::path::Path;

use tokio::fs;

use crate::filesystem::paths::GetUserGameDataDirError;
use crate::last_played_world::paths::get_last_world_path;
use crate::last_played_world::types::LastWorld;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetLastPlayedWorldError {
  #[error("failed to get user game data directory: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to read lastworld.json: {0}")]
  Read(#[from] io::Error),

  #[error("failed to parse lastworld.json: {0}")]
  Parse(#[from] serde_json::Error),
}

pub async fn get_last_played_world(
  data_dir: &Path,
  variant: &GameVariant,
) -> Result<Option<String>, GetLastPlayedWorldError> {
  let last_world_path =
    get_last_world_path(data_dir, variant).await?;

  if !last_world_path.exists() {
    return Ok(None);
  }

  let content = fs::read_to_string(last_world_path).await?;
  let last_world: LastWorld = serde_json::from_str(&content)?;

  Ok(Some(last_world.world_name))
}
