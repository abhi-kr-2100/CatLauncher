use crate::api::{ApiError, ApiResponse};
use crate::release::types::Release;
use crate::AppState;
use crate::util::Pagination;

pub async fn list_releases(
    owner: String,
    repo: String,
    pagination: Pagination,
    state: AppState,
) -> ApiResponse<Vec<Release>> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases_map = state.releases.read().await;

    let releases = match releases_map.get(&key) {
        Some(r) => r,
        None => return ApiResponse::Ok(Vec::new()),
    };

    let mut releases = releases.clone();
    // Sort by created_at descending (latest first)
    releases.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let (paginated_releases, metadata) = crate::util::paginate(&releases, pagination);
    ApiResponse::Paginated(paginated_releases, metadata)
}

pub async fn get_release(owner: String, repo: String, id: u64, state: AppState) -> ApiResponse<Release> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases_map = state.releases.read().await;
    let releases = match releases_map.get(&key) {
        Some(r) => r,
        None => {
            return ApiResponse::Error(ApiError::not_found(
                "Not Found",
                "https://docs.github.com/rest/releases/releases#get-a-release",
            ))
        }
    };

    match releases.iter().find(|r| r.id == id) {
        Some(release) => ApiResponse::Ok(release.clone()),
        None => ApiResponse::Error(ApiError::not_found(
            "Not Found",
            "https://docs.github.com/rest/releases/releases#get-a-release",
        )),
    }
}

pub async fn get_release_by_tag(
    owner: String,
    repo: String,
    tag: String,
    state: AppState,
) -> ApiResponse<Release> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases_map = state.releases.read().await;
    let releases = match releases_map.get(&key) {
        Some(r) => r,
        None => {
            return ApiResponse::Error(ApiError::not_found(
                "Not Found",
                "https://docs.github.com/rest/releases/releases#get-a-release-by-tag-name",
            ))
        }
    };

    match releases.iter().find(|r| r.tag_name == tag) {
        Some(release) => ApiResponse::Ok(release.clone()),
        None => ApiResponse::Error(ApiError::not_found(
            "Not Found",
            "https://docs.github.com/rest/releases/releases#get-a-release-by-tag-name",
        )),
    }
}

pub async fn get_latest_release(owner: String, repo: String, state: AppState) -> ApiResponse<Release> {
    let key = (owner.to_lowercase(), repo.to_lowercase());
    let releases_map = state.releases.read().await;
    let releases = match releases_map.get(&key) {
        Some(r) => r,
        None => {
            return ApiResponse::Error(ApiError::not_found(
                "Not Found",
                "https://docs.github.com/rest/releases/releases#get-the-latest-release",
            ))
        }
    };

    // Latest release is the newest non-draft, non-prerelease
    let mut candidates: Vec<_> = releases
        .iter()
        .filter(|r| !r.draft && !r.prerelease)
        .cloned()
        .collect();
    candidates.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    match candidates.into_iter().next() {
        Some(release) => ApiResponse::Ok(release),
        None => ApiResponse::Error(ApiError::not_found(
            "Not Found",
            "https://docs.github.com/rest/releases/releases#get-the-latest-release",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockServer;
    use axum::http::StatusCode;
    use reqwest::Client;

    #[tokio::test]
    async fn test_list_releases_returns_releases() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;

        let release = Release::new("owner", "repo", "v1.0.0");
        server.add_release("owner", "repo", release).await;

        let client = Client::new();
        let response = client
            .get(server.uri() + "/repos/owner/repo/releases")
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let releases: Vec<Release> = response.json().await?;
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].tag_name, "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;
        let release = Release::new("owner", "repo", "v1.0.0");
        let id = release.id;
        server.add_release("owner", "repo", release).await;

        let client = Client::new();
        let response = client
            .get(server.uri() + &format!("/repos/owner/repo/releases/{}", id))
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let release: Release = response.json().await?;
        assert_eq!(release.id, id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_by_tag() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;
        let release = Release::new("owner", "repo", "v1.0.0");
        server.add_release("owner", "repo", release).await;

        let client = Client::new();
        let response = client
            .get(server.uri() + "/repos/owner/repo/releases/tags/v1.0.0")
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let release: Release = response.json().await?;
        assert_eq!(release.tag_name, "v1.0.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;
        let client = Client::new();
        let response = client
            .get(server.uri() + "/repos/owner/repo/releases/123")
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_empty() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;
        let client = Client::new();
        let response = client
            .get(server.uri() + "/repos/owner/repo/releases")
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let releases: Vec<Release> = response.json().await?;
        assert!(releases.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_release() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start().await?;

        server
            .add_release(
                "owner",
                "repo",
                Release::new("owner", "repo", "v1.0.0").created_at("2024-01-01T00:00:00Z"),
            )
            .await;
        server
            .add_release(
                "owner",
                "repo",
                Release::new("owner", "repo", "v1.1.0").created_at("2024-02-01T00:00:00Z"),
            )
            .await;

        let client = Client::new();
        let response = client
            .get(server.uri() + "/repos/owner/repo/releases/latest")
            .send()
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let release: Release = response.json().await?;
        assert_eq!(release.tag_name, "v1.1.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_pagination() -> crate::Result<()> {
        let server = MockServer::start().await?;

        for i in 1..=35 {
            server
                .add_release(
                    "owner",
                    "repo",
                    Release::new("owner", "repo", &format!("v{:02}", i)).created_at(format!(
                        "2024-01-{:02}T00:00:00Z",
                        i
                    )),
                )
                .await;
        }

        let client = reqwest::Client::new();

        // Default (30 items)
        let resp = client
            .get(server.uri() + "/repos/owner/repo/releases")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let body: Vec<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        assert_eq!(body.len(), 30);

        // Verify descending order by created_at (v35 should be first)
        assert_eq!(body[0]["tag_name"], "v35");
        assert_eq!(body[29]["tag_name"], "v06");

        // Page 2 (Remaining 5 items)
        let resp = client
            .get(server.uri() + "/repos/owner/repo/releases?page=2")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let body: Vec<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        assert_eq!(body.len(), 5);
        assert_eq!(body[0]["tag_name"], "v05");
        assert_eq!(body[4]["tag_name"], "v01");

        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_link_header_has_next_page() -> crate::Result<()> {
        let server = MockServer::start().await?;

        // Add 35 releases (30 is default per_page)
        for i in 1..=35 {
            server.add_release("owner", "repo", Release::new("owner", "repo", &format!("v{}", i))).await;
        }

        let client = reqwest::Client::new();

        // Page 1: Should have link header for page 2
        let resp = client
            .get(server.uri() + "/repos/owner/repo/releases")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let link = resp.headers().get("link").and_then(|h| h.to_str().ok());
        assert!(link.is_some());
        let link_str = link.unwrap();
        assert!(link_str.contains("rel=\"next\""));
        assert!(link_str.contains("page=2"));
        assert!(link_str.contains("per_page=30"));
        assert!(link_str.contains(&server.uri()));

        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_link_header_last_page_has_no_link() -> crate::Result<()> {
        let server = MockServer::start().await?;

        // Add 35 releases (30 is default per_page)
        for i in 1..=35 {
            server.add_release("owner", "repo", Release::new("owner", "repo", &format!("v{}", i))).await;
        }

        let client = reqwest::Client::new();

        // Page 2: Should NOT have link header (it's the last page)
        let resp = client
            .get(server.uri() + "/repos/owner/repo/releases?page=2")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let link = resp.headers().get("link");
        assert!(link.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_list_releases_link_header_preserves_query_params() -> crate::Result<()> {
        let server = MockServer::start().await?;

        // Add 35 releases (30 is default per_page)
        for i in 1..=35 {
            server.add_release("owner", "repo", Release::new("owner", "repo", &format!("v{}", i))).await;
        }

        let client = reqwest::Client::new();

        // Test preserving query parameters
        let resp = client
            .get(server.uri() + "/repos/owner/repo/releases?foo=bar")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let link = resp.headers().get("link").and_then(|h| h.to_str().ok());
        assert!(link.is_some());
        let link_str = link.unwrap();
        assert!(link_str.contains("foo=bar"));
        assert!(link_str.contains("page=2"));

        Ok(())
    }
}
