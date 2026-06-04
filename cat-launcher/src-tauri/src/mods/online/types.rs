use async_trait::async_trait;

use crate::infra::http_client::{HttpClient, HttpClientError};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum FetchOnlineModsError {
  #[error("HTTP request failed: {0}")]
  RequestFailed(#[from] HttpClientError),
}

#[async_trait]
pub trait OnlineModRepository: Send + Sync {
  async fn get_mods_for_variant(
    &self,
    variant: &GameVariant,
    client: &dyn HttpClient,
  ) -> Result<Vec<ThirdPartyMod>, FetchOnlineModsError>;
}
