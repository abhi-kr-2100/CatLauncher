use async_trait::async_trait;
use std::io;

use crate::filesystem::paths::GetUserGameDataDirError;
use crate::variants::GameVariant;
use crate::world_options::types::{World, WorldOption};

#[derive(thiserror::Error, Debug)]
pub enum WorldOptionsError {
  #[error("failed to get user game data dir: {0}")]
  GetUserGameDataDir(#[from] GetUserGameDataDirError),

  #[error("io error: {0}")]
  Io(#[from] io::Error),

  #[error("serialization error: {0}")]
  Serialization(#[from] serde_json::Error),
}

#[async_trait]
pub trait WorldOptionsRepository: Send + Sync {
  async fn get_worlds(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<World>, WorldOptionsError>;

  async fn get_world_options(
    &self,
    variant: &GameVariant,
    world: &str,
  ) -> Result<Vec<WorldOption>, WorldOptionsError>;

  async fn update_world_options(
    &self,
    variant: &GameVariant,
    world: &str,
    options: &[WorldOption],
  ) -> Result<(), WorldOptionsError>;
}
