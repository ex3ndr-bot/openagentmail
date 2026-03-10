use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{CreateInboxRequest, Inbox, PaginatedResponse, PaginationParams, UpdateInboxRequest};

#[derive(Debug, Clone)]
pub struct Inboxes { inner: Arc<ClientInner> }

impl Inboxes {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn create(&self, request: CreateInboxRequest) -> Result<Inbox> {
        let response = self.inner.post("/inboxes").json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Inbox>> {
        let mut request = self.inner.get("/inboxes");
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list_by_pod(&self, pod_id: &str, params: PaginationParams) -> Result<PaginatedResponse<Inbox>> {
        let mut request = self.inner.get(&format!("/pods/{}/inboxes", pod_id));
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, inbox_id: &str) -> Result<Inbox> {
        let response = self.inner.get(&format!("/inboxes/{}", inbox_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn update(&self, inbox_id: &str, request: UpdateInboxRequest) -> Result<Inbox> {
        let response = self.inner.patch(&format!("/inboxes/{}", inbox_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, inbox_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/inboxes/{}", inbox_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }
}
