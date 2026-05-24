use std::collections::HashMap;

use reqwest::{Client, Method, RequestBuilder};
use url::Url;

use crate::infra::endpoint::Endpoint;
use crate::infra::http_client::{
  HttpClient, create_http_client,
};

/// An HTTP client that rewrites outgoing request URLs by replacing the
/// hostname according to domain redirect rules (e.g. github.com -> mock.github.com).
pub struct RewireClient {
  inner: Client,
  redirects: HashMap<Endpoint, Endpoint>,
}

impl RewireClient {
  pub fn new(redirects: HashMap<Endpoint, Endpoint>) -> Self {
    Self {
      inner: create_http_client(),
      redirects,
    }
  }

  pub fn from_reqwest_client(
    client: Client,
    redirects: HashMap<Endpoint, Endpoint>,
  ) -> Self {
    Self {
      inner: client,
      redirects,
    }
  }

  fn rewrite_url(&self, url: &str) -> String {
    let mut parsed =
      Url::parse(url).expect("rewire_client: failed to parse URL");

    let key = Endpoint::new(
      parsed.host_str().expect("rewire_client: URL has no host"),
      parsed.port(),
      parsed.scheme().to_string(),
    );

    let target = self.redirects.get(&key).unwrap_or_else(|| {
      panic!("no redirect entry for host: {}", key.host())
    });
    parsed
      .set_host(Some(target.host()))
      .expect("rewire_client: failed to set host");

    if let Some(port) = target.port() {
      parsed
        .set_port(Some(port))
        .expect("rewire_client: failed to set port");
    }

    parsed
      .set_scheme(target.scheme())
      .expect("rewire_client: failed to set scheme");

    parsed.to_string()
  }
}

impl HttpClient for RewireClient {
  fn get(&self, url: &str) -> RequestBuilder {
    self.inner.get(self.rewrite_url(url))
  }

  fn post(&self, url: &str) -> RequestBuilder {
    self.inner.post(self.rewrite_url(url))
  }

  fn put(&self, url: &str) -> RequestBuilder {
    self.inner.put(self.rewrite_url(url))
  }

  fn patch(&self, url: &str) -> RequestBuilder {
    self.inner.patch(self.rewrite_url(url))
  }

  fn delete(&self, url: &str) -> RequestBuilder {
    self.inner.delete(self.rewrite_url(url))
  }

  fn head(&self, url: &str) -> RequestBuilder {
    self.inner.head(self.rewrite_url(url))
  }

  fn request(&self, method: Method, url: &str) -> RequestBuilder {
    self.inner.request(method, self.rewrite_url(url))
  }
}
