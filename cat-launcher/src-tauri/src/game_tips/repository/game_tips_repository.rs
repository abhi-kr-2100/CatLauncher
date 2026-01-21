use std::error::Error;

use async_trait::async_trait;

use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum GameTipsRepositoryError {
    #[error("failed to get all tips: {0}")]
    GetAll(Box<dyn Error + Send + Sync>),
}

#[async_trait]
#[allow(dead_code)]
pub trait GameTipsRepository: Send + Sync {
    async fn get_all(&self, game_variant: &GameVariant) -> Result<Vec<String>, GameTipsRepositoryError>;
}
