use super::types::Repository;
use crate::api::{ApiError, ApiResponse};

pub async fn get_repository(
    owner: String,
    repo: String,
    state: crate::AppState,
) -> ApiResponse<Repository> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    match state.repositories.read().await.get(&key) {
        Some(r) => ApiResponse::Ok(r.clone()),
        None => ApiResponse::Error(ApiError::not_found(
            "Not Found",
            "https://docs.github.com/rest/repos/repos#get-a-repository",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::MockServer;
    use crate::Repository;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_get_repository_returns_repo() -> TestResult {
        let server = MockServer::start().await?;
        let repo = Repository::new("octocat", "hello-world")
            .description("A test repository")
            .stargazers_count(42);
        server.add_repository(repo).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["name"], "hello-world");
        assert_eq!(body["full_name"], "octocat/hello-world");
        assert_eq!(body["stargazers_count"], 42);
        assert_eq!(body["watchers_count"], 42);
        assert_eq!(body["description"], "A test repository");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_repository_not_found() -> TestResult {
        let server = MockServer::start().await?;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/nonexistent/user", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 404);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["message"], "Not Found");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_repository_case_insensitive() -> TestResult {
        let server = MockServer::start().await?;
        let repo = Repository::new("octocat", "hello-world").description("Case insensitive");
        server.add_repository(repo).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/Octocat/Hello-World", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body["name"], "hello-world");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_repository_has_hash_id() -> TestResult {
        let server = MockServer::start().await?;
        let repo = Repository::new("user", "my-repo");
        server.add_repository(repo).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/user/my-repo", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert!(body["id"].as_u64().is_some_and(|v| v > 0));
        assert!(
            body["node_id"]
                .as_str()
                .is_some_and(|v| v.starts_with("mock_node_id_"))
        );
        Ok(())
    }
}
