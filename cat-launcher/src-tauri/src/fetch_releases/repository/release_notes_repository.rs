use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;

use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum ReleaseNotesRepositoryError {
  #[error("failed to get cached release notes: {0}")]
  Get(Box<dyn Error + Send + Sync>),
}

#[async_trait]
pub trait ReleaseNotesRepository: Send + Sync {
  async fn get_release_notes_by_tag_names(
    &self,
    game_variant: &GameVariant,
    tag_names: &[String],
  ) -> Result<
    HashMap<String, Option<String>>,
    ReleaseNotesRepositoryError,
  >;
}
