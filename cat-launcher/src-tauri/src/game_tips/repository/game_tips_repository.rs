use async_trait::async_trait;
use thiserror::Error;

use crate::{
  game_tips::lib::GetAllTipsForVariantError, variants::GameVariant,
};

#[derive(Debug, Error)]
pub enum GameTipsRepositoryError {
  #[error("failed to get tips for variant: {0}")]
  GetTipsForVariant(#[from] GetAllTipsForVariantError),
}

#[async_trait]
pub trait GameTipsRepository {
  async fn get_all_tips_for_variant(
    &self,
    variant: &GameVariant,
  ) -> Result<Vec<String>, GameTipsRepositoryError>;
}
