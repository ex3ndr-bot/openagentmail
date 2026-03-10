use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{CreateDraftRequest, Draft, Message, PaginatedResponse, PaginationParams, UpdateDraftRequest};

#[derive(Debug, Clone)]
pub struct Drafts { inner: Arc<ClientInner> }

impl Drafts {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn create(&self, inbox_id: &str, request: CreateDraftRequest) -> Result<Draft> {
        let response = self.inner.post(&format!("/inboxes/{}/drafts", inbox_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, inbox_id: &str, params: PaginationParams) -> Result<PaginatedResponse<Draft>> {
        let mut request = self.inner.get(&format!("/inboxes/{}/drafts", inbox_id));
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, inbox_id: &str, draft_id: &str) -> Result<Draft> {
        let response = self.inner.get(&format!("/inboxes/{}/drafts/{}", inbox_id, draft_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn update(&self, inbox_id: &str, draft_id: &str, request: UpdateDraftRequest) -> Result<Draft> {
        let response = self.inner.patch(&format!("/inboxes/{}/drafts/{}", inbox_id, draft_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn send(&self, inbox_id: &str, draft_id: &str) -> Result<Message> {
        let response = self.inner.post(&format!("/inboxes/{}/drafts/{}/send", inbox_id, draft_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, inbox_id: &str, draft_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/inboxes/{}/drafts/{}", inbox_id, draft_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }
}
