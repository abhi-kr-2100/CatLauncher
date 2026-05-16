
use reqwest::Client;

/// Creates and returns a `reqwest::Client` instance configured with a user-agent
pub fn create_http_client() -> Client {
  let builder = Client::builder().user_agent("cat-launcher");
  builder.build().expect("Failed to build reqwest client")
}
