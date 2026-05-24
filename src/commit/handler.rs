use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::types::Commit;

impl crate::MockServer {
    pub async fn add_commit(&self, commit: Commit) {
        let key = (commit.owner.to_lowercase(), commit.repo.to_lowercase());
        self.state.commits.write().await.entry(key).or_default().push(commit);
    }
}

pub async fn list_commits(
    Path((owner, repo)): Path<(String, String)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let commits = state.commits.read().await.get(&key).cloned().unwrap_or_default();
    (StatusCode::OK, Json(commits)).into_response()
}

pub async fn get_commit(
    Path((owner, repo, sha)): Path<(String, String, String)>,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let commits = state.commits.read().await;
    match commits.get(&key).and_then(|vec| vec.iter().find(|c| c.sha == sha)) {
        Some(commit) => (StatusCode::OK, Json(commit.clone())).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "message": "Not Found",
                "documentation_url":
                    "https://docs.github.com/rest/commits/commits#get-a-commit",
            })),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::MockServer;
    use crate::Commit;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_list_commits_returns_empty_array() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body, serde_json::json!([]));
        Ok(())
    }

    #[tokio::test]
    async fn test_list_commits_returns_commits() -> TestResult {
        let server = MockServer::start().await?;
        let commit = Commit::new("octocat", "hello-world")
            .message("Initial commit")
            .author_name("Mona Octocat");
        server.add_commit(commit).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["commit"]["message"], "Initial commit");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_by_sha() -> TestResult {
        let server = MockServer::start().await?;
        let commit = Commit::new("octocat", "hello-world")
            .sha("customsha")
            .message("Custom SHA commit");
        server.add_commit(commit).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits/customsha", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["sha"], "customsha");
        assert_eq!(body["commit"]["message"], "Custom SHA commit");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_not_found() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits/nonexistent", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["message"], "Not Found");
        Ok(())
    }

    #[tokio::test]
    async fn test_list_commits_multiple() -> TestResult {
        let server = MockServer::start().await?;
        let c1 = Commit::new("octocat", "hello-world").message("First");
        let c2 = Commit::new("octocat", "hello-world").message("Second");
        server.add_commit(c1).await;
        server.add_commit(c2).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().unwrap().len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_case_insensitive_owner_repo() -> TestResult {
        let server = MockServer::start().await?;
        let commit = Commit::new("octocat", "hello-world")
            .sha("abc123")
            .message("Case test");
        server.add_commit(commit).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/Octocat/Hello-World/commits/abc123", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["commit"]["message"], "Case test");
        Ok(())
    }

    #[tokio::test]
    async fn test_add_commit_increments_count() -> TestResult {
        let server = MockServer::start().await?;
        let c1 = Commit::new("u", "r").message("First");
        let c2 = Commit::new("u", "r").message("Second");
        server.add_commit(c1).await;
        server.add_commit(c2).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/u/r/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().unwrap().len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_commits_scoped_to_repo() -> TestResult {
        let server = MockServer::start().await?;
        server.add_commit(Commit::new("owner1", "repo1").message("Repo1 commit")).await;
        server.add_commit(Commit::new("owner2", "repo2").message("Repo2 commit")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/owner1/repo1/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["commit"]["message"], "Repo1 commit");
        Ok(())
    }
}
