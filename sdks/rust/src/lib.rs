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

#![allow(missing_docs)]

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
    pub fn organization(&self) -> OrganizationResource<'_> {
        OrganizationResource::new(&self.client)
    }

    /// Access pod operations
    ///
    /// Pods provide multi-tenant isolation. Each pod has its own
    /// inboxes, domains, and data.
    pub fn pods(&self) -> PodsResource<'_> {
        PodsResource::new(&self.client)
    }

    /// Access inbox operations
    ///
    /// Inboxes are email addresses that can send and receive messages.
    pub fn inboxes(&self) -> InboxesResource<'_> {
        InboxesResource::new(&self.client)
    }

    /// Access message operations
    ///
    /// Send emails and manage received messages.
    pub fn messages(&self) -> MessagesResource<'_> {
        MessagesResource::new(&self.client)
    }

    /// Access draft operations
    ///
    /// Drafts are unsent messages that can be edited before sending.
    pub fn drafts(&self) -> DraftsResource<'_> {
        DraftsResource::new(&self.client)
    }

    /// Access webhook operations
    ///
    /// Webhooks enable real-time notifications for email events.
    pub fn webhooks(&self) -> WebhooksResource<'_> {
        WebhooksResource::new(&self.client)
    }

    /// Access domain operations
    ///
    /// Add and manage custom email domains.
    pub fn domains(&self) -> DomainsResource<'_> {
        DomainsResource::new(&self.client)
    }
}
