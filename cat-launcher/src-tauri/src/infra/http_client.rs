use async_trait::async_trait;
use reqwest::{Client, Response};

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
  #[error(transparent)]
  Http(#[from] reqwest::Error),

  #[error("invalid URL: {0}")]
  InvalidUrl(#[from] url::ParseError),

  #[error("no host mapping found for: {0}")]
  NoMapping(String),

  #[error("failed to set URL component")]
  UrlComponentError,
}

#[async_trait]
pub trait HttpClient: Send + Sync {
  async fn get(&self, url: &str)
  -> Result<Response, HttpClientError>;
}

#[async_trait]
impl HttpClient for Client {
  async fn get(
    &self,
    url: &str,
  ) -> Result<Response, HttpClientError> {
    self.get(url).send().await.map_err(HttpClientError::from)
  }
}

/// Creates and returns a `reqwest::Client` instance configured with a user-agent
pub fn create_http_client() -> Client {
  let builder = Client::builder().user_agent("cat-launcher");
  builder.build().expect("Failed to build reqwest client")
}
