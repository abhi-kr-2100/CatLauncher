use std::collections::HashMap;
use std::path::Path;

use tokio::fs;

use crate::variants::GameVariant;
use crate::world_options::types::WorldOptionMetadata;

#[derive(thiserror::Error, Debug)]
pub enum GetMetadataError {
  #[error("failed to read metadata file: {0}")]
  Io(#[from] std::io::Error),

  #[error("failed to parse metadata: {0}")]
  Serde(#[from] serde_json::Error),
}

pub async fn get_metadata_for_variant(
  variant: &GameVariant,
  resources_dir: &Path,
) -> Result<HashMap<String, WorldOptionMetadata>, GetMetadataError> {
  let filename = variant.world_options_filename();

  let path = resources_dir.join("content").join(filename);
  let content = fs::read_to_string(path).await?;
  let metadata: HashMap<String, WorldOptionMetadata> =
    serde_json::from_str(&content)?;

  Ok(metadata)
}
