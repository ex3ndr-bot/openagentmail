use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{CreateWebhookRequest, PaginatedResponse, PaginationParams, UpdateWebhookRequest, Webhook};

#[derive(Debug, Clone)]
pub struct Webhooks { inner: Arc<ClientInner> }

impl Webhooks {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn create(&self, request: CreateWebhookRequest) -> Result<Webhook> {
        let response = self.inner.post("/webhooks").json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Webhook>> {
        let mut request = self.inner.get("/webhooks");
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, webhook_id: &str) -> Result<Webhook> {
        let response = self.inner.get(&format!("/webhooks/{}", webhook_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn update(&self, webhook_id: &str, request: UpdateWebhookRequest) -> Result<Webhook> {
        let response = self.inner.patch(&format!("/webhooks/{}", webhook_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, webhook_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/webhooks/{}", webhook_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }

    pub async fn rotate_secret(&self, webhook_id: &str) -> Result<Webhook> {
        let response = self.inner.post(&format!("/webhooks/{}/rotate-secret", webhook_id)).send().await?;
        self.inner.handle_response(response).await
    }
}
