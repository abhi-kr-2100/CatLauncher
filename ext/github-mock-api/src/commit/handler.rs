use crate::api::{ApiResponse, ApiError};
use super::types::Commit;

pub async fn list_commits(
    owner: String,
    repo: String,
    pagination: crate::util::Pagination,
    state: crate::AppState,
) -> ApiResponse<Vec<Commit>> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let commits = state.commits.read().await.get(&key).cloned().unwrap_or_default();
    let paginated_commits = crate::util::paginate(&commits, pagination);
    ApiResponse::Ok(paginated_commits)
}

pub async fn get_commit(
    owner: String,
    repo: String,
    sha: String,
    state: crate::AppState,
) -> ApiResponse<Commit> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let commits_map = state.commits.read().await;
    let commits = match commits_map.get(&key) {
        Some(v) => v,
        None => {
            return ApiResponse::Error(ApiError::not_found(
                "Not Found",
                "https://docs.github.com/rest/commits/commits#get-a-commit",
            ))
        }
    };

    match commits.iter().find(|c| c.sha == sha) {
        Some(commit) => ApiResponse::Ok(commit.clone()),
        None => ApiResponse::Error(ApiError::not_found(
            "Not Found",
            "https://docs.github.com/rest/commits/commits#get-a-commit",
        )),
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
        server.add_commit("octocat", "hello-world", commit).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert!(body.is_array());
        assert_eq!(body.as_array().map(|a| a.len()), Some(1));
        assert_eq!(body[0]["commit"]["message"], "Initial commit");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_by_sha() -> TestResult {
        let server = MockServer::start().await?;
        let commit = Commit::new("octocat", "hello-world")
            .sha("customsha")
            .message("Custom SHA commit");
        server.add_commit("octocat", "hello-world", commit).await;

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
        server.add_commit("octocat", "hello-world", c1).await;
        server.add_commit("octocat", "hello-world", c2).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/octocat/hello-world/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().map(|a| a.len()), Some(2));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_case_insensitive_owner_repo() -> TestResult {
        let server = MockServer::start().await?;
        let commit = Commit::new("octocat", "hello-world")
            .sha("abc123")
            .message("Case test");
        server.add_commit("octocat", "hello-world", commit).await;

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
        server.add_commit("u", "r", c1).await;
        server.add_commit("u", "r", c2).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/u/r/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().map(|a| a.len()), Some(2));
        Ok(())
    }

    #[tokio::test]
    async fn test_commits_scoped_to_repo() -> TestResult {
        let server = MockServer::start().await?;
        server
            .add_commit(
                "owner1",
                "repo1",
                Commit::new("owner1", "repo1").message("Repo1 commit"),
            )
            .await;
        server
            .add_commit(
                "owner2",
                "repo2",
                Commit::new("owner2", "repo2").message("Repo2 commit"),
            )
            .await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/owner1/repo1/commits", server.uri()))
            .send()
            .await?;

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await?;
        assert_eq!(body.as_array().map(|a| a.len()), Some(1));
        assert_eq!(body[0]["commit"]["message"], "Repo1 commit");
        Ok(())
    }

    #[tokio::test]
    async fn test_list_commits_pagination() -> TestResult {
        let server = MockServer::start().await?;
        for i in 1..=35 {
            server
                .add_commit(
                    "u",
                    "r",
                    Commit::new("u", "r").message(&format!("Commit {}", i)),
                )
                .await;
        }

        let client = reqwest::Client::new();

        // Default (30 items)
        let resp = client
            .get(format!("{}/repos/u/r/commits", server.uri()))
            .send()
            .await?;
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 30);

        // Page 2 (Remaining 5 items)
        let resp = client
            .get(format!("{}/repos/u/r/commits?page=2", server.uri()))
            .send()
            .await?;
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 5);

        // Custom per_page
        let resp = client
            .get(format!(
                "{}/repos/u/r/commits?per_page=10",
                server.uri()
            ))
            .send()
            .await?;
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 10);

        // per_page capping
        let resp = client
            .get(format!(
                "{}/repos/u/r/commits?per_page=200",
                server.uri()
            ))
            .send()
            .await?;
        let body: Vec<serde_json::Value> = resp.json().await?;
        // We only have 35 commits total, so it should return 35 even if capped at 100
        assert_eq!(body.len(), 35);

        // Add more commits to test 100 cap
        for i in 36..=110 {
            server
                .add_commit(
                    "u",
                    "r",
                    Commit::new("u", "r").message(&format!("Commit {}", i)),
                )
                .await;
        }
        let resp = client
            .get(format!(
                "{}/repos/u/r/commits?per_page=200",
                server.uri()
            ))
            .send()
            .await?;
        let body: Vec<serde_json::Value> = resp.json().await?;
        assert_eq!(body.len(), 100);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_not_found_in_existing_repo() -> TestResult {
        let server = MockServer::start().await?;
        server.add_commit("u", "r", Commit::new("u", "r").sha("sha1")).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/u/r/commits/sha2", server.uri()))
            .send()
            .await?;
        assert_eq!(resp.status(), 404);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_commit_missing_repo() -> TestResult {
        let server = MockServer::start().await?;
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}/repos/no/repo/commits/abc", server.uri()))
            .send()
            .await?;
        assert_eq!(resp.status(), 404);
        Ok(())
    }
}
