use std::env;

use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

fn create_github_pat_headers() -> Option<HeaderMap> {
  if let Ok(github_pat) = env::var("GITHUB_PAT")
    && !github_pat.is_empty()
  {
    let authorization = format!("Bearer {}", github_pat);
    if let Ok(header_value) = HeaderValue::from_str(&authorization) {
      let mut headers = HeaderMap::new();
      headers.insert(AUTHORIZATION, header_value);
      return Some(headers);
    }
  }
  None
}

/// Creates and returns a `reqwest::Client` instance configured with a user-agent
/// and authorization headers if `GITHUB_PAT` is available in the environment.
pub fn create_http_client() -> Client {
  let mut builder = Client::builder().user_agent("cat-launcher");

  if let Some(headers) = create_github_pat_headers() {
    builder = builder.default_headers(headers);
  }

  builder.build().expect("Failed to build reqwest client")
}
