//! Inboxes resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{
    CreateInboxRequest, Inbox, ListInboxesParams, PaginatedResponse, UpdateInboxRequest,
};

/// Operations on inboxes
pub struct InboxesResource<'a> {
    client: &'a ApiClient,
}

impl<'a> InboxesResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Create a new inbox
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateInboxRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let inbox = client.inboxes().create(
    ///         "pod_xyz789",
    ///         CreateInboxRequest::new("support-agent")
    ///             .display_name("Support Agent")
    ///     ).await?;
    ///     println!("Created inbox: {}", inbox.email);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, pod_id: &str, request: CreateInboxRequest) -> Result<Inbox> {
        self.client
            .post(&format!("pods/{}/inboxes", pod_id), &request)
            .await
    }

    /// List inboxes
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, ListInboxesParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     // List all inboxes
    ///     let inboxes = client.inboxes().list(ListInboxesParams::new().limit(10)).await?;
    ///     
    ///     // List inboxes for a specific pod
    ///     let pod_inboxes = client.inboxes().list(
    ///         ListInboxesParams::new().pod_id("pod_xyz789")
    ///     ).await?;
    ///     
    ///     for inbox in inboxes.items {
    ///         println!("Inbox: {}", inbox.email);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, params: ListInboxesParams) -> Result<PaginatedResponse<Inbox>> {
        let mut query = self.client.build_pagination_query(&params.pagination);
        if let Some(ref pod_id) = params.pod_id {
            query.push(("pod_id", pod_id.clone()));
        }
        self.client.get_with_query("inboxes", &query).await
    }

    /// Get an inbox by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let inbox = client.inboxes().get("inb_abc123").await?;
    ///     println!("Inbox: {} ({})", inbox.email, inbox.display_name.unwrap_or_default());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, inbox_id: &str) -> Result<Inbox> {
        self.client.get(&format!("inboxes/{}", inbox_id)).await
    }

    /// Update an inbox
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, UpdateInboxRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let inbox = client.inboxes().update(
    ///         "inb_abc123",
    ///         UpdateInboxRequest::new().display_name("Updated Support Agent")
    ///     ).await?;
    ///     println!("Updated inbox: {}", inbox.email);
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(&self, inbox_id: &str, request: UpdateInboxRequest) -> Result<Inbox> {
        self.client
            .put(&format!("inboxes/{}", inbox_id), &request)
            .await
    }

    /// Delete an inbox
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.inboxes().delete("inb_abc123").await?;
    ///     println!("Inbox deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, inbox_id: &str) -> Result<()> {
        self.client.delete(&format!("inboxes/{}", inbox_id)).await
    }
}
