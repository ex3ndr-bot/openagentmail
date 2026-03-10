//! Drafts resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{
    CreateDraftRequest, Draft, Message, PaginatedResponse, PaginationParams, UpdateDraftRequest,
};

/// Operations on drafts
pub struct DraftsResource<'a> {
    client: &'a ApiClient,
}

impl<'a> DraftsResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Create a new draft
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateDraftRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let draft = client.drafts().create(
    ///         "inb_abc123",
    ///         CreateDraftRequest::new()
    ///             .add_to("user@example.com")
    ///             .subject("Draft: Weekly Report")
    ///             .text("Work in progress...")
    ///     ).await?;
    ///     
    ///     println!("Created draft: {}", draft.draft_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, inbox_id: &str, request: CreateDraftRequest) -> Result<Draft> {
        self.client
            .post(&format!("inboxes/{}/drafts", inbox_id), &request)
            .await
    }

    /// List drafts in an inbox
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, PaginationParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let drafts = client.drafts().list(
    ///         "inb_abc123",
    ///         PaginationParams::new().limit(10)
    ///     ).await?;
    ///     
    ///     for draft in drafts.items {
    ///         println!("Draft: {}", draft.subject.unwrap_or_default());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &str,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<Draft>> {
        let query = self.client.build_pagination_query(&params);
        self.client
            .get_with_query(&format!("inboxes/{}/drafts", inbox_id), &query)
            .await
    }

    /// Get a draft by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let draft = client.drafts().get("inb_abc123", "drf_mno345").await?;
    ///     println!("Draft subject: {}", draft.subject.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, inbox_id: &str, draft_id: &str) -> Result<Draft> {
        self.client
            .get(&format!("inboxes/{}/drafts/{}", inbox_id, draft_id))
            .await
    }

    /// Update a draft
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, UpdateDraftRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let draft = client.drafts().update(
    ///         "inb_abc123",
    ///         "drf_mno345",
    ///         UpdateDraftRequest::new()
    ///             .subject("Updated: Weekly Report")
    ///             .text("Final version ready to send.")
    ///     ).await?;
    ///     
    ///     println!("Updated draft: {}", draft.draft_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(
        &self,
        inbox_id: &str,
        draft_id: &str,
        request: UpdateDraftRequest,
    ) -> Result<Draft> {
        self.client
            .put(&format!("inboxes/{}/drafts/{}", inbox_id, draft_id), &request)
            .await
    }

    /// Delete a draft
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.drafts().delete("inb_abc123", "drf_mno345").await?;
    ///     println!("Draft deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, inbox_id: &str, draft_id: &str) -> Result<()> {
        self.client
            .delete(&format!("inboxes/{}/drafts/{}", inbox_id, draft_id))
            .await
    }

    /// Send a draft
    ///
    /// Converts the draft to a sent message. The draft is deleted after sending.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let message = client.drafts().send("inb_abc123", "drf_mno345").await?;
    ///     println!("Sent message: {}", message.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(&self, inbox_id: &str, draft_id: &str) -> Result<Message> {
        self.client
            .post_empty(&format!("inboxes/{}/drafts/{}/send", inbox_id, draft_id))
            .await
    }
}
