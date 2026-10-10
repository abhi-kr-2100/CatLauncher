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

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use std::collections::HashMap;
  use tokio::io::AsyncWriteExt;
  use tokio::net::TcpListener;

  use crate::infra::testing::http_client::TestHttpClient;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  async fn spawn_mock_server(
    status_line: &'static str,
    body: &'static str,
  ) -> TestResult<(u16, tokio::task::JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();

    let handle = tokio::spawn(async move {
      if let Ok((mut stream, _)) = listener.accept().await {
        let response = format!(
          "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
          status_line,
          body.len(),
          body
        );
        let _ = stream.write_all(response.as_bytes()).await;
      }
    });

    Ok((port, handle))
  }

  #[tokio::test]
  async fn test_get_mods_for_variant_ignores_other_variants()
  -> TestResult {
    let repo = BrightNightsModRepository::new();
    let client = TestHttpClient::new(HashMap::new())?;

    let dda_mods = repo
      .get_mods_for_variant(&GameVariant::DarkDaysAhead, &client)
      .await?;
    assert!(dda_mods.is_empty());

    let tlg_mods = repo
      .get_mods_for_variant(&GameVariant::TheLastGeneration, &client)
      .await?;
    assert!(tlg_mods.is_empty());

    assert_eq!(client.request_count(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_mods_for_variant_success() -> TestResult {
    let json_body = r#"[
      {
        "id": "bn_mod_1",
        "display_name": "BN Mod One",
        "description": "First BN Mod",
        "categories": ["content"],
        "source": {
          "type": "github_archive",
          "url": "https://github.com/cataclysmbnteam/mod-one/archive/refs/heads/master.zip",
          "extract_path": "ModOne"
        }
      },
      {
        "id": "bn_mod_invalid",
        "display_name": "Invalid Mod",
        "source": {
          "type": "git",
          "url": "https://github.com/cataclysmbnteam/invalid/archive/refs/heads/master.zip",
          "extract_path": "Invalid"
        }
      }
    ]"#;

    let (port, server_task) =
      spawn_mock_server("200 OK", json_body).await?;

    let mut host_mappings = HashMap::new();
    host_mappings.insert(
      "mods.cataclysmbn.org".to_string(),
      format!("127.0.0.1:{}", port),
    );
    let client = TestHttpClient::new(host_mappings)?;
    let repo = BrightNightsModRepository::new();

    let mods = repo
      .get_mods_for_variant(&GameVariant::BrightNights, &client)
      .await?;

    // Only valid mod is parsed, invalid one is safely filtered out
    assert_eq!(mods.len(), 1);
    assert_eq!(mods[0].id, "bn_mod_1");
    assert_eq!(mods[0].name, "BN Mod One");
    assert_eq!(mods[0].category, "content");
    assert_eq!(client.request_count(), 1);

    let _ = server_task.await;

    Ok(())
  }

  #[tokio::test]
  async fn test_get_mods_for_variant_http_error() -> TestResult {
    let (port, server_task) =
      spawn_mock_server("500 Internal Server Error", "error").await?;

    let mut host_mappings = HashMap::new();
    host_mappings.insert(
      "mods.cataclysmbn.org".to_string(),
      format!("127.0.0.1:{}", port),
    );
    let client = TestHttpClient::new(host_mappings)?;
    let repo = BrightNightsModRepository::new();

    let result = repo
      .get_mods_for_variant(&GameVariant::BrightNights, &client)
      .await;

    assert!(matches!(
      result,
      Err(FetchOnlineModsError::RequestFailed(_))
    ));
    assert_eq!(client.request_count(), 1);

    let _ = server_task.await;

    Ok(())
  }
}
