use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub enum ApiResponse<T> {
    Ok(T),
    Paginated(T, crate::util::PaginationMetadata),
    Error(ApiError),
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
    pub documentation_url: String,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            ApiResponse::Paginated(data, metadata) => {
                let mut response = (StatusCode::OK, Json(data)).into_response();
                response.extensions_mut().insert(metadata);
                response
            }
            ApiResponse::Error(err) => (
                StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                Json(serde_json::json!({
                    "message": err.message,
                    "documentation_url": err.documentation_url,
                })),
            )
                .into_response(),
        }
    }
}

impl ApiError {
    pub fn not_found(message: &str, documentation_url: &str) -> Self {
        Self {
            status: 404,
            message: message.to_string(),
            documentation_url: documentation_url.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_invalid_status() {
        let err = ApiError {
            status: 66, // Invalid HTTP status (must be 100-999)
            message: "Test".to_string(),
            documentation_url: "Test".to_string(),
        };
        let response = ApiResponse::<()>::Error(err).into_response();
        assert_eq!(response.status().as_u16(), 500);
    }

    #[test]
    fn test_api_response_ok() {
        let response = ApiResponse::Ok("data").into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
