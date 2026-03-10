//! Message resource operations

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{
    ListMessagesParams, Message, PaginatedResponse, SendMessageRequest, UpdateMessageLabelsRequest,
};
use crate::utils::PaginatedStream;

/// Message resource operations
#[derive(Debug, Clone)]
pub struct Messages {
    inner: Arc<ClientInner>,
}

impl Messages {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Send a new message
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// # use openagentmail::{OpenAgentMail, SendMessageRequest};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OpenAgentMail::new("api_key");
    /// 
    /// let message = client.messages()
    ///     .send("inbox_123", SendMessageRequest::builder()
    ///         .to("user@example.com")
    ///         .subject("Hello!")
    ///         .text("This is a test email.")
    ///         .build())
    ///     .await?;
    /// 
    /// println!("Sent message: {}", message.message_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, inbox_id: &str, request: SendMessageRequest) -> Result<Message> {
        let response = self
            .inner
            .post(&format!("/inboxes/{}/messages", inbox_id))
            .json(&request)
            .send()
            .await?;
        self.inner.handle_response(response).await
    }

    /// List messages in an inbox
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// # use openagentmail::{OpenAgentMail, ListMessagesParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OpenAgentMail::new("api_key");
    /// 
    /// let messages = client.messages()
    ///     .list("inbox_123", ListMessagesParams::new().limit(10))
    ///     .await?;
    /// 
    /// for msg in messages.items {
    ///     println!("{}: {}", msg.from, msg.subject);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &str,
        params: ListMessagesParams,
    ) -> Result<PaginatedResponse<Message>> {
        let mut request = self.inner.get(&format!("/inboxes/{}/messages", inbox_id));

        for (key, value) in params.to_query_params() {
            request = request.query(&[(key, value)]);
        }

        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    /// Create a stream that automatically paginates through all messages
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// # use openagentmail::OpenAgentMail;
    /// # use futures::StreamExt;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OpenAgentMail::new("api_key");
    /// 
    /// let mut stream = client.messages().stream("inbox_123", 20);
    /// while let Some(result) = stream.next().await {
    ///     let message = result?;
    ///     println!("{}: {}", message.from, message.subject);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream(
        &self,
        inbox_id: &str,
        page_size: u32,
    ) -> PaginatedStream<
        Message,
        impl Fn(Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<PaginatedResponse<Message>>> + Send>> + '_,
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<PaginatedResponse<Message>>> + Send>>,
    > {
        let inner = self.inner.clone();
        let inbox_id = inbox_id.to_string();

        PaginatedStream::new(move |page_token| {
            let inner = inner.clone();
            let inbox_id = inbox_id.clone();
            Box::pin(async move {
                let mut request = inner.get(&format!("/inboxes/{}/messages", inbox_id));
                request = request.query(&[("limit", page_size.to_string())]);
                if let Some(token) = page_token {
                    request = request.query(&[("page_token", token)]);
                }
                let response = request.send().await?;
                inner.handle_response(response).await
            })
        })
    }

    /// Get a specific message by ID
    pub async fn get(&self, inbox_id: &str, message_id: &str) -> Result<Message> {
        let response = self
            .inner
            .get(&format!("/inboxes/{}/messages/{}", inbox_id, message_id))
            .send()
            .await?;
        self.inner.handle_response(response).await
    }

    /// Update message labels
    pub async fn update_labels(
        &self,
        inbox_id: &str,
        message_id: &str,
        request: UpdateMessageLabelsRequest,
    ) -> Result<Message> {
        let response = self
            .inner
            .patch(&format!(
                "/inboxes/{}/messages/{}/labels",
                inbox_id, message_id
            ))
            .json(&request)
            .send()
            .await?;
        self.inner.handle_response(response).await
    }

    /// Delete a message
    pub async fn delete(&self, inbox_id: &str, message_id: &str) -> Result<()> {
        let response = self
            .inner
            .delete(&format!("/inboxes/{}/messages/{}", inbox_id, message_id))
            .send()
            .await?;
        self.inner.handle_empty_response(response).await
    }
}
