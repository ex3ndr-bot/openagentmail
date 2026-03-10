//! # OpenAgentMail Rust SDK
//!
//! A strongly-typed, async-first Rust SDK for the OpenAgentMail API.
//!
//! OpenAgentMail is an open-source email API designed for AI agents.
//!
//! ## Quick Start
//!
//! ```no_run
//! use openagentmail::{OpenAgentMail, SendMessageRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client
//!     let client = OpenAgentMail::new("your-api-key")?;
//!
//!     // Send an email
//!     let message = client.messages().send(
//!         "inb_abc123",
//!         SendMessageRequest::new("user@example.com", "Hello!")
//!             .text("This is a test email.")
//!     ).await?;
//!
//!     println!("Sent: {}", message.message_id);
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Async/await**: Built on tokio for high-performance async operations
//! - **Strong typing**: Full type safety with builder patterns
//! - **Error handling**: Comprehensive error types with `thiserror`
//! - **Pagination**: Easy iteration over paginated results
//!
//! ## Resources
//!
//! The SDK provides access to the following API resources:
//!
//! - [`OpenAgentMail::organization`] - Organization details
//! - [`OpenAgentMail::pods`] - Multi-tenant isolation
//! - [`OpenAgentMail::inboxes`] - Email inboxes
//! - [`OpenAgentMail::messages`] - Send and receive emails
//! - [`OpenAgentMail::drafts`] - Draft management
//! - [`OpenAgentMail::webhooks`] - Real-time notifications
//! - [`OpenAgentMail::domains`] - Custom domains

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod client;
pub mod error;
pub mod resources;
pub mod types;

pub use client::ClientConfig;
pub use error::{Error, Result};
pub use types::*;

use client::ApiClient;
use resources::{
    DomainsResource, DraftsResource, InboxesResource, MessagesResource, OrganizationResource,
    PodsResource, WebhooksResource,
};

/// OpenAgentMail API client
///
/// This is the main entry point for interacting with the OpenAgentMail API.
/// Create an instance with your API key and use the resource methods to
/// access different parts of the API.
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
///     // Access resources
///     let org = client.organization().get().await?;
///     println!("Organization: {}", org.name);
///     
///     Ok(())
/// }
/// ```
pub struct OpenAgentMail {
    client: ApiClient,
}

impl OpenAgentMail {
    /// Create a new OpenAgentMail client with the default configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAgentMail API key
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// let client = OpenAgentMail::new("your-api-key")?;
    /// # Ok::<(), openagentmail::Error>(())
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_config(ClientConfig::new(api_key))
    }

    /// Create a new OpenAgentMail client with custom configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, ClientConfig};
    ///
    /// let config = ClientConfig::new("your-api-key")
    ///     .base_url("https://custom.api.com")
    ///     .timeout(60);
    ///
    /// let client = OpenAgentMail::with_config(config)?;
    /// # Ok::<(), openagentmail::Error>(())
    /// ```
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            client: ApiClient::new(config)?,
        })
    }

    /// Access organization operations
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    /// let org = client.organization().get().await?;
    /// println!("Plan: {}", org.plan);
    /// # Ok(())
    /// # }
    /// ```
    pub fn organization(&self) -> OrganizationResource<'_> {
        OrganizationResource::new(&self.client)
    }

    /// Access pod operations
    ///
    /// Pods provide multi-tenant isolation. Each pod has its own
    /// inboxes, domains, and data.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreatePodRequest, PaginationParams};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// // Create a pod
    /// let pod = client.pods().create(CreatePodRequest::new("Production")).await?;
    ///
    /// // List pods
    /// let pods = client.pods().list(PaginationParams::new()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pods(&self) -> PodsResource<'_> {
        PodsResource::new(&self.client)
    }

    /// Access inbox operations
    ///
    /// Inboxes are email addresses that can send and receive messages.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateInboxRequest};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// let inbox = client.inboxes().create(
    ///     "pod_xyz789",
    ///     CreateInboxRequest::new("support-agent").display_name("Support Agent")
    /// ).await?;
    ///
    /// println!("Created: {}", inbox.email);
    /// # Ok(())
    /// # }
    /// ```
    pub fn inboxes(&self) -> InboxesResource<'_> {
        InboxesResource::new(&self.client)
    }

    /// Access message operations
    ///
    /// Send emails and manage received messages.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, SendMessageRequest, ListMessagesParams};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// // Send a message
    /// let msg = client.messages().send(
    ///     "inb_abc123",
    ///     SendMessageRequest::new("user@example.com", "Hello!")
    ///         .text("Hello from OpenAgentMail!")
    /// ).await?;
    ///
    /// // List messages
    /// let messages = client.messages().list(
    ///     "inb_abc123",
    ///     ListMessagesParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn messages(&self) -> MessagesResource<'_> {
        MessagesResource::new(&self.client)
    }

    /// Access draft operations
    ///
    /// Drafts are unsent messages that can be edited before sending.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateDraftRequest};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// // Create a draft
    /// let draft = client.drafts().create(
    ///     "inb_abc123",
    ///     CreateDraftRequest::new()
    ///         .add_to("user@example.com")
    ///         .subject("Draft email")
    ///         .text("Work in progress...")
    /// ).await?;
    ///
    /// // Send the draft
    /// let message = client.drafts().send("inb_abc123", &draft.draft_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn drafts(&self) -> DraftsResource<'_> {
        DraftsResource::new(&self.client)
    }

    /// Access webhook operations
    ///
    /// Webhooks enable real-time notifications for email events.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateWebhookRequest, WebhookEventType};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// let webhook = client.webhooks().create(
    ///     CreateWebhookRequest::new(
    ///         "https://api.myapp.com/webhooks/email",
    ///         vec![WebhookEventType::MessageReceived]
    ///     )
    /// ).await?;
    ///
    /// println!("Webhook secret: {}", webhook.secret);
    /// # Ok(())
    /// # }
    /// ```
    pub fn webhooks(&self) -> WebhooksResource<'_> {
        WebhooksResource::new(&self.client)
    }

    /// Access domain operations
    ///
    /// Add and manage custom email domains.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateDomainRequest};
    ///
    /// # async fn example() -> Result<(), openagentmail::Error> {
    /// let client = OpenAgentMail::new("your-api-key")?;
    ///
    /// let domain = client.domains().create(
    ///     CreateDomainRequest::new("mail.mycompany.com")
    /// ).await?;
    ///
    /// // Configure DNS records
    /// for record in &domain.dns_records {
    ///     println!("Add {} record: {} -> {}", record.record_type, record.name, record.value);
    /// }
    ///
    /// // Verify after DNS is configured
    /// let verified = client.domains().verify(&domain.domain_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn domains(&self) -> DomainsResource<'_> {
        DomainsResource::new(&self.client)
    }
}
