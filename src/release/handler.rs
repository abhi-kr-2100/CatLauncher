use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::types::Release;

impl crate::MockServer {
    /// Register a mocked release with the server.
    /// The release will be available at the read-only releases endpoints.
    pub async fn add_release(&self, release: Release) {
        let key = (
            release.owner_login.to_lowercase(),
            release.repo_name.to_lowercase(),
        );
        self.state
            .releases
            .write()
            .await
            .entry(key)
            .or_default()
            .push(release);
    }
}

pub async fn list_releases(
    Path((owner, repo)): Path<(String, String)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases = state.releases.read().await;
    let mut result = match releases.get(&key) {
        Some(r) => r.clone(),
        None => return Json::<Vec<Release>>(Vec::new()).into_response(),
    };
    result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Json(result).into_response()
}

pub async fn get_release(
    Path((owner, repo, release_id)): Path<(String, String, u64)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases = state.releases.read().await;
    match releases.get(&key) {
        Some(r) => match r.iter().find(|rel| rel.id == release_id) {
            Some(rel) => (StatusCode::OK, Json(rel.clone())).into_response(),
            None => not_found().into_response(),
        },
        None => not_found().into_response(),
    }
}

pub async fn get_release_by_tag(
    Path((owner, repo, tag)): Path<(String, String, String)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases = state.releases.read().await;
    match releases.get(&key) {
        Some(r) => match r.iter().find(|rel| rel.tag_name == tag) {
            Some(rel) => (StatusCode::OK, Json(rel.clone())).into_response(),
            None => not_found().into_response(),
        },
        None => not_found().into_response(),
    }
}

pub async fn get_latest_release(
    Path((owner, repo)): Path<(String, String)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases = state.releases.read().await;
    match releases.get(&key) {
        Some(r) => {
            let mut candidates: Vec<_> = r
                .iter()
                .filter(|rel| !rel.draft && !rel.prerelease)
                .collect();
            candidates.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            match candidates.into_iter().next() {
                Some(rel) => (StatusCode::OK, Json(rel.clone())).into_response(),
                None => not_found().into_response(),
            }
        }
        None => not_found().into_response(),
    }
}

fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "message": "Not Found",
            "documentation_url":
                "https://docs.github.com/rest/releases/releases#get-a-release",
        })),
    )
}

#[cfg(test)]
mod tests {
    use crate::MockServer;
    use crate::Release;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_list_releases_empty() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/releases", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert!(body.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_returns_releases() -> TestResult {
        let server = MockServer::start().await?;
        let release = Release::new("octocat", "hello-world", "v1.0.0")
            .name("v1.0.0")
            .body("First release");
        server.add_release(release).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/releases", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 1);
        assert_eq!(body[0]["tag_name"], "v1.0.0");
        assert_eq!(body[0]["name"], "v1.0.0");
        assert_eq!(body[0]["body"], "First release");
        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_multiple() -> TestResult {
        let server = MockServer::start().await?;
        server.add_release(Release::new("user", "repo", "v1.0.0")).await;
        server.add_release(Release::new("user", "repo", "v2.0.0")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/user/repo/releases", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_isolated_per_repo() -> TestResult {
        let server = MockServer::start().await?;
        server.add_release(Release::new("owner-a", "repo-a", "v1.0.0")).await;
        server.add_release(Release::new("owner-b", "repo-b", "v1.0.0")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/owner-a/repo-a/releases", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_id() -> TestResult {
        let server = MockServer::start().await?;
        let release = Release::new("octocat", "hello-world", "v1.0.0");
        let release_id = release.id;
        server.add_release(release).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/{}",
                server.uri(),
                release_id
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["id"], release_id as u64);
        assert_eq!(body["tag_name"], "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_id_not_found() -> TestResult {
        let server = MockServer::start().await?;
        server.add_release(Release::new("octocat", "hello-world", "v1.0.0")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/999999",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_tag() -> TestResult {
        let server = MockServer::start().await?;
        let release = Release::new("octocat", "hello-world", "v1.0.0")
            .name("v1.0.0");
        server.add_release(release).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/tags/v1.0.0",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["tag_name"], "v1.0.0");
        assert_eq!(body["name"], "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_tag_not_found() -> TestResult {
        let server = MockServer::start().await?;
        server.add_release(Release::new("octocat", "hello-world", "v1.0.0")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/tags/v2.0.0",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_tag_different_repo() -> TestResult {
        let server = MockServer::start().await?;
        server.add_release(Release::new("octocat", "repo-a", "v1.0.0")).await;
        server.add_release(Release::new("octocat", "repo-b", "v1.0.0")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/repo-a/releases/tags/v1.0.0",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["tag_name"], "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_release() -> TestResult {
        let server = MockServer::start().await?;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v1.0.0")
                    .created_at("2024-01-01T00:00:00Z"),
            )
            .await;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v2.0.0")
                    .created_at("2024-06-01T00:00:00Z"),
            )
            .await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/latest",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["tag_name"], "v2.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_release_skips_drafts() -> TestResult {
        let server = MockServer::start().await?;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v1.0.0")
                    .created_at("2024-06-01T00:00:00Z")
                    .draft(true),
            )
            .await;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v0.9.0")
                    .created_at("2024-01-01T00:00:00Z"),
            )
            .await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/latest",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["tag_name"], "v0.9.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_release_skips_prereleases() -> TestResult {
        let server = MockServer::start().await?;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v2.0.0-rc1")
                    .created_at("2024-06-01T00:00:00Z")
                    .prerelease(true),
            )
            .await;
        server
            .add_release(
                Release::new("octocat", "hello-world", "v1.0.0")
                    .created_at("2024-01-01T00:00:00Z"),
            )
            .await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/latest",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["tag_name"], "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_release_not_found() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/octocat/hello-world/releases/latest",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_not_found_by_id_for_unknown_repo() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/repos/unknown/repo/releases/42",
                server.uri()
            ))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        Ok(())
    }
}
