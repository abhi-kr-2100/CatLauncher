use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MockError {
    InternalServerError,
    RateLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockBehavior {
    pub(crate) error: Option<MockError>,
}

impl MockBehavior {
    pub fn builder() -> MockBehaviorBuilder {
        MockBehaviorBuilder::default()
    }
}

#[derive(Default)]
pub struct MockBehaviorBuilder {
    error: Option<MockError>,
}

impl MockBehaviorBuilder {
    pub fn error(mut self, error: MockError) -> Self {
        self.error = Some(error);
        self
    }

    pub fn build(self) -> MockBehavior {
        MockBehavior { error: self.error }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, MockBehavior, MockError, MockServer, Result};

    #[tokio::test]
    #[allow(clippy::panic_in_result_fn)]
    async fn test_internal_server_error_behavior() -> Result<()> {
        let server = MockServer::start().await?;
        let behavior = MockBehavior::builder()
            .error(MockError::InternalServerError)
            .build();
        server.add_mock_behavior(behavior).await?;

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/repos/owner/repo", server.uri()))
            .send()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        assert_eq!(
            body.get("message").and_then(|m| m.as_str()),
            Some("Internal Server Error")
        );
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic_in_result_fn)]
    async fn test_rate_limit_exceeded_behavior() -> Result<()> {
        let server = MockServer::start().await?;
        let behavior = MockBehavior::builder()
            .error(MockError::RateLimitExceeded)
            .build();
        server.add_mock_behavior(behavior).await?;

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/repos/owner/repo", server.uri()))
            .send()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        assert_eq!(
            body.get("message").and_then(|m| m.as_str()),
            Some("API rate limit exceeded")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_conflicting_behaviors() -> Result<()> {
        let server = MockServer::start().await?;
        let behavior1 = MockBehavior::builder()
            .error(MockError::InternalServerError)
            .build();
        let behavior2 = MockBehavior::builder()
            .error(MockError::RateLimitExceeded)
            .build();

        server.add_mock_behavior(behavior1).await?;
        let result = server.add_mock_behavior(behavior2).await;

        if !matches!(result, Err(Error::Conflict(_))) {
            return Err(Error::Conflict("Expected conflict error".to_string()));
        }
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic_in_result_fn)]
    async fn test_clear_mock_behaviors() -> Result<()> {
        let server = MockServer::start().await?;
        let behavior = MockBehavior::builder()
            .error(MockError::InternalServerError)
            .build();
        server.add_mock_behavior(behavior).await?;

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/repos/owner/repo", server.uri()))
            .send()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );

        server.clear_all_mock_behaviors().await;

        let response = client
            .get(format!("{}/repos/owner/repo", server.uri()))
            .send()
            .await
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        // Should be 404 since we haven't added the repo, but not 500
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }
}
