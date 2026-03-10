use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{ListMessagesParams, Message, PaginatedResponse, SendMessageRequest, UpdateMessageLabelsRequest};

#[derive(Debug, Clone)]
pub struct Messages { inner: Arc<ClientInner> }

impl Messages {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn send(&self, inbox_id: &str, request: SendMessageRequest) -> Result<Message> {
        let response = self.inner.post(&format!("/inboxes/{}/messages", inbox_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, inbox_id: &str, params: ListMessagesParams) -> Result<PaginatedResponse<Message>> {
        let mut request = self.inner.get(&format!("/inboxes/{}/messages", inbox_id));
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, inbox_id: &str, message_id: &str) -> Result<Message> {
        let response = self.inner.get(&format!("/inboxes/{}/messages/{}", inbox_id, message_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn update_labels(&self, inbox_id: &str, message_id: &str, request: UpdateMessageLabelsRequest) -> Result<Message> {
        let response = self.inner.patch(&format!("/inboxes/{}/messages/{}/labels", inbox_id, message_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, inbox_id: &str, message_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/inboxes/{}/messages/{}", inbox_id, message_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }
}
