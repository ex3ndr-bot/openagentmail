//! HTTP client for OpenAgentMail API

use reqwest::{Client as HttpClient, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{ApiErrorResponse, Error, Result};
use crate::types::PaginationParams;

const DEFAULT_BASE_URL: &str = "https://api.openagentmail.com";
const API_VERSION: &str = "v0";

/// Configuration for the OpenAgentMail client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API (defaults to https://api.openagentmail.com)
    pub base_url: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl ClientConfig {
    /// Create a new configuration with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_secs: 30,
        }
    }

    /// Set a custom base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the request timeout in seconds
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

/// Internal HTTP client for making API requests
#[derive(Debug, Clone)]
pub struct ApiClient {
    http: HttpClient,
    config: ClientConfig,
}

impl ApiClient {
    /// Create a new API client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(Error::Http)?;

        Ok(Self { http, config })
    }

    /// Build the full URL for an endpoint
    pub fn url(&self, path: &str) -> String {
        format!("{}/{}/{}", self.config.base_url, API_VERSION, path.trim_start_matches('/'))
    }

    /// Add authentication headers to a request
    fn auth(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("Authorization", format!("Bearer {}", self.config.api_key))
    }

    /// Build query parameters from pagination params
    pub fn build_pagination_query(&self, params: &PaginationParams) -> Vec<(&str, String)> {
        let mut query = Vec::new();
        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(ref token) = params.page_token {
            query.push(("page_token", token.clone()));
        }
        query
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .auth(self.http.get(self.url(path)))
            .send()
            .await
            .map_err(Error::Http)?;

        self.handle_response(response).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        let response = self
            .auth(self.http.get(self.url(path)))
            .query(query)
            .send()
            .await
            .map_err(Error::Http)?;

        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let response = self
            .auth(self.http.post(self.url(path)))
            .json(body)
            .send()
            .await
            .map_err(Error::Http)?;

        self.handle_response(response).await
    }

    /// Make a POST request without a body
    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .auth(self.http.post(self.url(path)))
            .send()
            .await
            .map_err(Error::Http)?;

        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let response = self
            .auth(self.http.put(self.url(path)))
            .json(body)
            .send()
            .await
            .map_err(Error::Http)?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let response = self
            .auth(self.http.delete(self.url(path)))
            .send()
            .await
            .map_err(Error::Http)?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            self.handle_error_response(response).await
        }
    }

    /// Handle API response
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(Error::Http)
        } else {
            self.handle_error_response(response).await
        }
    }

    /// Handle error response
    async fn handle_error_response<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let status_code = status.as_u16();

        // Try to parse error response
        match response.json::<ApiErrorResponse>().await {
            Ok(error_response) => Err(Error::from_api_response(status_code, error_response)),
            Err(_) => {
                // Couldn't parse error response, create generic error
                let message = match status {
                    StatusCode::BAD_REQUEST => "Bad request",
                    StatusCode::UNAUTHORIZED => "Unauthorized",
                    StatusCode::FORBIDDEN => "Forbidden",
                    StatusCode::NOT_FOUND => "Not found",
                    StatusCode::CONFLICT => "Conflict",
                    StatusCode::UNPROCESSABLE_ENTITY => "Validation error",
                    StatusCode::TOO_MANY_REQUESTS => "Rate limited",
                    _ => "Unknown error",
                };
                Err(Error::Api {
                    status: status_code,
                    code: "unknown".to_string(),
                    message: message.to_string(),
                    details: None,
                })
            }
        }
    }
}
