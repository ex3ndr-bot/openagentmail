//! Messages resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{
    ListMessagesParams, Message, PaginatedResponse, ReplyRequest, SendMessageRequest,
};

/// Operations on messages
pub struct MessagesResource<'a> {
    client: &'a ApiClient,
}

impl<'a> MessagesResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Send a new message
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, SendMessageRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let message = client.messages().send(
    ///         "inb_abc123",
    ///         SendMessageRequest::new("user@example.com", "Hello!")
    ///             .text("This is a test email from OpenAgentMail.")
    ///             .label("outbound")
    ///     ).await?;
    ///     
    ///     println!("Sent message: {}", message.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(&self, inbox_id: &str, request: SendMessageRequest) -> Result<Message> {
        self.client
            .post(&format!("inboxes/{}/messages", inbox_id), &request)
            .await
    }

    /// List messages in an inbox
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, ListMessagesParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     // List recent messages
    ///     let messages = client.messages().list(
    ///         "inb_abc123",
    ///         ListMessagesParams::new().limit(20)
    ///     ).await?;
    ///     
    ///     // Filter by thread
    ///     let thread_messages = client.messages().list(
    ///         "inb_abc123",
    ///         ListMessagesParams::new().thread_id("thr_xyz789")
    ///     ).await?;
    ///     
    ///     // Filter by label
    ///     let labeled_messages = client.messages().list(
    ///         "inb_abc123",
    ///         ListMessagesParams::new().label("important")
    ///     ).await?;
    ///     
    ///     for msg in messages.items {
    ///         println!("{}: {}", msg.from, msg.subject);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &str,
        params: ListMessagesParams,
    ) -> Result<PaginatedResponse<Message>> {
        let mut query = self.client.build_pagination_query(&params.pagination);
        if let Some(ref thread_id) = params.thread_id {
            query.push(("thread_id", thread_id.clone()));
        }
        if let Some(ref label) = params.label {
            query.push(("label", label.clone()));
        }
        self.client
            .get_with_query(&format!("inboxes/{}/messages", inbox_id), &query)
            .await
    }

    /// Get a message by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let message = client.messages().get("inb_abc123", "msg_def456").await?;
    ///     println!("Subject: {}", message.subject);
    ///     println!("Body: {}", message.text.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, inbox_id: &str, message_id: &str) -> Result<Message> {
        self.client
            .get(&format!("inboxes/{}/messages/{}", inbox_id, message_id))
            .await
    }

    /// Delete a message
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.messages().delete("inb_abc123", "msg_def456").await?;
    ///     println!("Message deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, inbox_id: &str, message_id: &str) -> Result<()> {
        self.client
            .delete(&format!("inboxes/{}/messages/{}", inbox_id, message_id))
            .await
    }

    /// Reply to a message
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, ReplyRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let reply = client.messages().reply(
    ///         "inb_abc123",
    ///         "msg_def456",
    ///         ReplyRequest::new()
    ///             .text("Thanks for reaching out! I'll look into this.")
    ///             .label("replied")
    ///     ).await?;
    ///     
    ///     println!("Reply sent: {}", reply.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn reply(
        &self,
        inbox_id: &str,
        message_id: &str,
        request: ReplyRequest,
    ) -> Result<Message> {
        self.client
            .post(
                &format!("inboxes/{}/messages/{}/reply", inbox_id, message_id),
                &request,
            )
            .await
    }

    /// Reply to all recipients of a message
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, ReplyRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let reply = client.messages().reply_all(
    ///         "inb_abc123",
    ///         "msg_def456",
    ///         ReplyRequest::new().text("Replying to everyone on this thread.")
    ///     ).await?;
    ///     
    ///     println!("Reply-all sent: {}", reply.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn reply_all(
        &self,
        inbox_id: &str,
        message_id: &str,
        request: ReplyRequest,
    ) -> Result<Message> {
        self.client
            .post(
                &format!("inboxes/{}/messages/{}/reply-all", inbox_id, message_id),
                &request,
            )
            .await
    }
}
