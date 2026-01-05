use async_trait::async_trait;
use reqwest::Client;

use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchOnlineModsError {
  #[error("HTTP request failed: {0}")]
  RequestFailed(#[from] reqwest::Error),

  #[error("failed to fetch from repository: {0}")]
  Repository(Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait OnlineModRepository: Send + Sync {
  async fn get_mods_for_variant(
    &self,
    variant: &GameVariant,
    client: &Client,
  ) -> Result<Vec<ThirdPartyMod>, FetchOnlineModsError>;
}
