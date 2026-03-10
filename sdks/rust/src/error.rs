use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        code: String,
        message: String,
        details: Option<ErrorDetails>,
    },

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded. Reset at: {reset_at}")]
    RateLimited { reset_at: u64 },

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Request error: {0}")]
    Request(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub field: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<ErrorDetails>,
}

impl Error {
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

pub type Result<T> = std::result::Result<T, Error>;
