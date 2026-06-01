use async_trait::async_trait;

use crate::infra::http_client::{HttpClient, HttpClientError};
use crate::mods::online::types::{
  FetchOnlineModsError, OnlineModRepository,
};
use crate::mods::types::ThirdPartyMod;
use crate::variants::GameVariant;

use super::manifest::BrightNightsOnlineMod;

#[derive(Default)]
pub struct BrightNightsModRepository;

impl BrightNightsModRepository {
  pub fn new() -> Self {
    Self
  }
}

#[async_trait]
impl OnlineModRepository for BrightNightsModRepository {
  async fn get_mods_for_variant(
    &self,
    variant: &GameVariant,
    client: &dyn HttpClient,
  ) -> Result<Vec<ThirdPartyMod>, FetchOnlineModsError> {
    if !matches!(variant, GameVariant::BrightNights) {
      return Ok(Vec::new());
    }

    let url = "https://mods.cataclysmbn.org/generated/mods.json";
    let response = client
      .get(url)
      .await?
      .error_for_status()
      .map_err(HttpClientError::from)?;
    let mods_values = response
      .json::<Vec<serde_json::Value>>()
      .await
      .map_err(HttpClientError::from)?;

    let mods: Vec<ThirdPartyMod> = mods_values
      .into_iter()
      .filter_map(|v| {
        match serde_json::from_value::<BrightNightsOnlineMod>(
          v.clone(),
        ) {
          Ok(m) => m.into_third_party_mod().ok(),
          Err(_) => None,
        }
      })
      .collect();

    Ok(mods)
  }
}
