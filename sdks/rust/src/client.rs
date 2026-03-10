use reqwest::{Client, RequestBuilder, Response, StatusCode};
use std::sync::Arc;

use crate::error::{ApiErrorResponse, Error, Result};
use crate::resources::{Domains, Drafts, Inboxes, Messages, Pods, Webhooks};
use crate::types::Organization;

pub const DEFAULT_BASE_URL: &str = "https://api.openagentmail.com/v0";

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_secs: Option<u64>,
}

impl ClientConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_secs: None,
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }
}

#[derive(Debug)]
pub(crate) struct ClientInner {
    pub(crate) http: Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}

impl ClientInner {
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        self.http
            .get(format!("{}{}", self.base_url, path))
            .bearer_auth(&self.api_key)
    }

    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        self.http
            .post(format!("{}{}", self.base_url, path))
            .bearer_auth(&self.api_key)
    }

    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        self.http
            .patch(format!("{}{}", self.base_url, path))
            .bearer_auth(&self.api_key)
    }

    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        self.http
            .delete(format!("{}{}", self.base_url, path))
            .bearer_auth(&self.api_key)
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            let parsed: T = serde_json::from_str(&body)?;
            Ok(parsed)
        } else {
            self.handle_error(status, response).await
        }
    }

    pub(crate) async fn handle_empty_response(&self, response: Response) -> Result<()> {
        let status = response.status();

        if status == StatusCode::NO_CONTENT || status.is_success() {
            Ok(())
        } else {
            self.handle_error(status, response).await
        }
    }

    async fn handle_error<T>(&self, status: StatusCode, response: Response) -> Result<T> {
        let body = response.text().await.unwrap_or_default();

        if let Ok(error_response) = serde_json::from_str::<ApiErrorResponse>(&body) {
            Err(Error::from_api_response(status.as_u16(), error_response))
        } else {
            Err(Error::Api {
                status: status.as_u16(),
                code: "unknown".to_string(),
                message: body,
                details: None,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenAgentMail {
    pub(crate) inner: Arc<ClientInner>,
}

impl OpenAgentMail {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(ClientConfig::new(api_key))
    }

    pub fn with_config(config: ClientConfig) -> Self {
        let mut builder = Client::builder();

        if let Some(timeout) = config.timeout_secs {
            builder = builder.timeout(std::time::Duration::from_secs(timeout));
        }

        let http = builder.build().expect("Failed to build HTTP client");

        Self {
            inner: Arc::new(ClientInner {
                http,
                api_key: config.api_key,
                base_url: config.base_url,
            }),
        }
    }

    pub async fn organization(&self) -> Result<Organization> {
        let response = self.inner.get("/organization").send().await?;
        self.inner.handle_response(response).await
    }

    pub fn pods(&self) -> Pods {
        Pods::new(self.inner.clone())
    }

    pub fn inboxes(&self) -> Inboxes {
        Inboxes::new(self.inner.clone())
    }

    pub fn messages(&self) -> Messages {
        Messages::new(self.inner.clone())
    }

    pub fn drafts(&self) -> Drafts {
        Drafts::new(self.inner.clone())
    }

    pub fn webhooks(&self) -> Webhooks {
        Webhooks::new(self.inner.clone())
    }

    pub fn domains(&self) -> Domains {
        Domains::new(self.inner.clone())
    }
}
