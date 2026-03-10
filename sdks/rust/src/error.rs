//! Error types for OpenAgentMail SDK

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for all OpenAgentMail SDK operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        code: String,
        message: String,
        details: Option<ErrorDetails>,
    },

    /// Authentication failed
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Reset at: {reset_at}")]
    RateLimited { reset_at: u64 },

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Request building error
    #[error("Request error: {0}")]
    Request(String),
}

/// API error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub field: Option<String>,
    pub reason: Option<String>,
}

/// API error response from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

/// API error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<ErrorDetails>,
}

impl Error {
    /// Create an API error from status code and response
    pub fn from_api_response(status: u16, response: ApiErrorResponse) -> Self {
        let ApiError { code, message, details } = response.error;
        
        match code.as_str() {
            "authentication_error" => Error::Authentication(message),
            "not_found" => Error::NotFound(message),
            "validation_error" => Error::Validation(message),
            "rate_limit_exceeded" => Error::RateLimited { reset_at: 0 },
            _ => Error::Api {
                status,
                code,
                message,
                details,
            },
        }
    }
}

/// Result type for OpenAgentMail operations
pub type Result<T> = std::result::Result<T, Error>;
