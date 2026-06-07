use crate::AppState;
use crate::api::{ApiError, ApiResponse};
use crate::asset::types::AssetContent;

pub async fn download_release_asset(
    owner: String,
    repo: String,
    tag: String,
    filename: String,
    state: AppState,
) -> ApiResponse<()> {
    let key = (owner.to_lowercase(), repo.to_lowercase(), tag, filename);
    let (content, content_type, filename) = {
        let assets_map = state.assets.read().await;
        let asset = match assets_map.get(&key) {
            Some(a) => a,
            None => {
                return ApiResponse::Error(ApiError::not_found(
                    "Not Found",
                    "https://docs.github.com/rest/releases/releases#get-a-release-asset",
                ));
            }
        };
        (
            asset.content.clone(),
            asset.content_type.clone(),
            asset.name.clone(),
        )
    };

    if axum::http::HeaderValue::from_str(&content_type).is_err() {
        return ApiResponse::Error(ApiError {
            status: 500,
            message: format!("Invalid content type: {}", content_type),
            documentation_url: "https://docs.github.com/rest".to_string(),
        });
    }

    let bytes = match content {
        AssetContent::Bytes(bytes) => bytes,
        AssetContent::File(path) => match tokio::fs::read(path).await {
            Ok(bytes) => bytes,
            Err(e) => {
                return ApiResponse::Error(ApiError {
                    status: 500,
                    message: format!("Failed to read asset file: {}", e),
                    documentation_url: "https://docs.github.com/rest".to_string(),
                });
            }
        },
    };

    ApiResponse::Raw {
        bytes,
        content_type,
        filename,
    }
}

#[cfg(test)]
mod tests {
    use crate::MockServer;
    use crate::asset::types::Asset;
    use axum::http::StatusCode;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_asset_bytes() -> crate::Result<()> {
        let server = MockServer::start().await?;
        let content = b"hello world".to_vec();
        let asset = Asset::from_bytes("test.txt", content.clone(), "text/plain");

        server.add_asset("owner", "repo", "v1.0.0", asset).await;

        let client = Client::new();
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0/test.txt")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers()["content-type"], "text/plain");
        assert_eq!(
            resp.headers()["content-disposition"],
            "attachment; filename=\"test.txt\""
        );
        let body = resp
            .bytes()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert_eq!(body.to_vec(), content);

        Ok(())
    }

    #[tokio::test]
    async fn test_download_asset_file() -> crate::Result<()> {
        let server = MockServer::start().await?;
        let temp_dir = tempfile::tempdir().map_err(crate::Error::Io)?;
        let file_path = temp_dir.path().join("test.bin");
        let content = vec![1, 2, 3, 4];
        std::fs::write(&file_path, &content).map_err(crate::Error::Io)?;

        let asset = Asset::from_path("test.bin", file_path, "application/octet-stream");
        server.add_asset("owner", "repo", "v1.0.0", asset).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0/test.bin")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers()["content-type"], "application/octet-stream");
        assert_eq!(
            resp.headers()["content-disposition"],
            "attachment; filename=\"test.bin\""
        );
        let body = resp
            .bytes()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert_eq!(body.to_vec(), content);

        Ok(())
    }

    #[tokio::test]
    async fn test_download_asset_not_found() -> crate::Result<()> {
        let server = MockServer::start().await?;
        let client = reqwest::Client::new();
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0/missing.txt")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn test_download_asset_invalid_content_type() -> crate::Result<()> {
        let server = MockServer::start().await?;
        let asset = Asset::from_bytes("test.txt", b"content".to_vec(), "invalid\nheader");

        server.add_asset("owner", "repo", "v1.0.0", asset).await;

        let client = Client::new();
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0/test.txt")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert!(body["message"].as_str().unwrap().contains("Invalid content type"));

        Ok(())
    }

    #[tokio::test]
    async fn test_download_asset_case_sensitivity() -> crate::Result<()> {
        let server = MockServer::start().await?;
        let content = b"content".to_vec();
        let asset = Asset::from_bytes("MyAsset.zip", content.clone(), "application/zip");

        // Registered with specific casing
        server.add_asset("Owner", "Repo", "v1.0.0-BETA", asset).await;

        let client = Client::new();

        // 1. Success with correct casing for tag and filename, different casing for owner/repo
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0-BETA/MyAsset.zip")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert_eq!(resp.status(), StatusCode::OK);

        // 2. Failure with wrong casing for tag
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0-beta/MyAsset.zip")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        // 3. Failure with wrong casing for filename
        let resp = client
            .get(server.uri() + "/owner/repo/releases/download/v1.0.0-BETA/myasset.zip")
            .send()
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::other(e)))?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        Ok(())
    }
}
