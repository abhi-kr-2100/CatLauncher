use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{Client, Response};
use url::Url;

use crate::infra::http_client::{
  HttpClient, HttpClientError, create_http_client,
};

/// A test HTTP client that can rewrite URLs based on host mappings.
pub struct TestHttpClient {
  client: Client,
  host_mappings: HashMap<String, String>,
}

impl TestHttpClient {
  /// Creates a new `TestHttpClient` with the given host mappings.
  pub fn new(
    host_mappings: HashMap<String, String>,
  ) -> Result<Self, HttpClientError> {
    Ok(Self {
      client: create_http_client()?,
      host_mappings,
    })
  }

  fn rewrite_url(
    &self,
    url_str: &str,
  ) -> Result<String, HttpClientError> {
    let mut url =
      Url::parse(url_str).map_err(HttpClientError::from)?;

    let host = url.host_str().ok_or_else(|| {
      HttpClientError::NoMapping(url_str.to_string())
    })?;
    let mapped_host = self
      .host_mappings
      .get(host)
      .ok_or_else(|| HttpClientError::NoMapping(host.to_string()))?;

    // mapped_host can be "localhost:8080"
    let mapped_url = Url::parse(&format!("http://{}", mapped_host))
      .map_err(HttpClientError::from)?;

    url
      .set_host(mapped_url.host_str())
      .map_err(|_| HttpClientError::UrlComponentError)?;
    url
      .set_port(mapped_url.port())
      .map_err(|_| HttpClientError::UrlComponentError)?;
    url
      .set_scheme("http")
      .map_err(|_| HttpClientError::UrlComponentError)?;

    Ok(url.to_string())
  }
}

#[async_trait]
impl HttpClient for TestHttpClient {
  async fn get(
    &self,
    url: &str,
  ) -> Result<Response, HttpClientError> {
    let rewritten_url = self.rewrite_url(url)?;
    self
      .client
      .get(&rewritten_url)
      .send()
      .await
      .map_err(HttpClientError::from)
  }
}
