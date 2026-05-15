use reqwest::{Client, Method, RequestBuilder};

/// A trait abstracting over HTTP clients. Implemented by `reqwest::Client` in
/// production and by `RewireClient` in tests. This allows test code to redirect
/// requests to a mock server without changing production code.
pub trait HttpClient: Send + Sync {
  fn get(&self, url: &str) -> RequestBuilder;
  fn post(&self, url: &str) -> RequestBuilder;
  fn put(&self, url: &str) -> RequestBuilder;
  fn patch(&self, url: &str) -> RequestBuilder;
  fn delete(&self, url: &str) -> RequestBuilder;
  fn head(&self, url: &str) -> RequestBuilder;
  fn request(&self, method: Method, url: &str) -> RequestBuilder;
}

impl HttpClient for Client {
  fn get(&self, url: &str) -> RequestBuilder {
    Client::get(self, url)
  }

  fn post(&self, url: &str) -> RequestBuilder {
    Client::post(self, url)
  }

  fn put(&self, url: &str) -> RequestBuilder {
    Client::put(self, url)
  }

  fn patch(&self, url: &str) -> RequestBuilder {
    Client::patch(self, url)
  }

  fn delete(&self, url: &str) -> RequestBuilder {
    Client::delete(self, url)
  }

  fn head(&self, url: &str) -> RequestBuilder {
    Client::head(self, url)
  }

  fn request(&self, method: Method, url: &str) -> RequestBuilder {
    Client::request(self, method, url)
  }
}

/// Creates and returns a `reqwest::Client` instance configured with a user-agent
pub fn create_http_client() -> Client {
  let builder = Client::builder().user_agent("cat-launcher");
  builder.build().expect("Failed to build reqwest client")
}
