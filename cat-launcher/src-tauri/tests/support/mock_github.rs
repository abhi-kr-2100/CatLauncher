use std::collections::HashMap;

use wiremock::matchers::{method, path as path_matcher};
use wiremock::{Mock, MockServer, ResponseTemplate};

use cat_launcher_lib::infra::endpoint::Endpoint;

use super::rewire_client::RewireClient;

/// A mock GitHub API server built on wiremock. Provides helpers for setting up
/// common GitHub API response patterns and creating a `RewireClient` that
/// redirects `https://api.github.com` requests to this mock server.
pub struct MockGitHubApi {
  server: MockServer,
}

impl MockGitHubApi {
  pub async fn start() -> Self {
    Self {
      server: MockServer::start().await,
    }
  }

  pub fn uri(&self) -> String {
    self.server.uri()
  }

  /// Creates a `RewireClient` that redirects all `https://api.github.com`
  /// requests to this mock server.
  pub fn create_rewire_client(&self) -> RewireClient {
    let mut redirects = HashMap::new();
    let mock_uri = self.uri();
    let mock_url = url::Url::parse(&mock_uri).unwrap();
    let mock_host = mock_url.host_str().unwrap().to_string();
    let mock_port = mock_url.port();
    redirects.insert(
      Endpoint::new("api.github.com", None, "https"),
      Endpoint::new(mock_host, mock_port, "http"),
    );
    RewireClient::new(redirects)
  }

  /// Mock the `GET /repos/{owner}/{repo}/releases/tags/{tag}` endpoint.
  pub fn mock_release_by_tag(
    &self,
    owner_repo: &str,
    tag: &str,
    response: ResponseTemplate,
  ) -> Mock {
    let path_str = format!("/repos/{owner_repo}/releases/tags/{tag}");
    Mock::given(method("GET"))
      .and(path_matcher(path_str))
      .respond_with(response)
  }

  /// Mock the `GET /repos/{owner}/{repo}/releases` endpoint.
  pub fn mock_releases_list(
    &self,
    owner_repo: &str,
    response: ResponseTemplate,
  ) -> Mock {
    let path_str = format!("/repos/{owner_repo}/releases");
    Mock::given(method("GET"))
      .and(path_matcher(path_str))
      .respond_with(response)
  }

  /// Register a mock on the server.
  pub async fn register(&self, mock: Mock) {
    self.server.register(mock).await;
  }
}

/// Helper to create a standard GitHub release JSON response.
pub fn github_release_response(
  id: u64,
  tag_name: &str,
  body: Option<&str>,
  prerelease: bool,
) -> ResponseTemplate {
  let json = serde_json::json!({
    "id": id,
    "tag_name": tag_name,
    "name": format!("Release {tag_name}"),
    "prerelease": prerelease,
    "body": body,
    "created_at": "2024-01-01T00:00:00Z",
    "assets": [],
  });
  ResponseTemplate::new(200).set_body_json(&json)
}

/// Helper to create a GitHub 404 response.
pub fn github_not_found_response() -> ResponseTemplate {
  ResponseTemplate::new(404)
    .set_body_json(&serde_json::json!({ "message": "Not Found" }))
}
