//! Error types and handling for the API
//!
//! This module provides a unified error type that can be converted
//! into appropriate HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Main error type for the API
///
/// This enum represents all possible errors that can occur during
/// request processing. Each variant maps to an appropriate HTTP status code.
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),
    /// Bad request - validation or input errors (400)
    BadRequest(String),
    /// Internal server error (500)
    Internal(String),
    /// Conflict - resource already exists (409)
    Conflict(String),
}

impl ApiError {
    /// Returns the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
        }
    }

    /// Returns a user-friendly error message
    pub fn message(&self) -> &str {
        match self {
            ApiError::NotFound(msg) => msg,
            ApiError::BadRequest(msg) => msg,
            ApiError::Internal(msg) => msg,
            ApiError::Conflict(msg) => msg,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": {
                "message": self.message(),
                "status": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ApiError {}
