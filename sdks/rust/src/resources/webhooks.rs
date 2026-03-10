//! Webhooks resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{
    CreateWebhookRequest, PaginatedResponse, PaginationParams, UpdateWebhookRequest, Webhook,
};

/// Operations on webhooks
pub struct WebhooksResource<'a> {
    client: &'a ApiClient,
}

impl<'a> WebhooksResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Create a new webhook
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateWebhookRequest, WebhookEventType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let webhook = client.webhooks().create(
    ///         CreateWebhookRequest::new(
    ///             "https://api.myapp.com/webhooks/email",
    ///             vec![WebhookEventType::MessageReceived, WebhookEventType::MessageBounced]
    ///         )
    ///         .inbox_id("inb_abc123")
    ///         .client_id("webhook-001")
    ///     ).await?;
    ///     
    ///     println!("Created webhook: {}", webhook.webhook_id);
    ///     println!("Secret: {}", webhook.secret);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, request: CreateWebhookRequest) -> Result<Webhook> {
        self.client.post("webhooks", &request).await
    }

    /// List all webhooks
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
    ///     let webhooks = client.webhooks().list(PaginationParams::new()).await?;
    ///     for wh in webhooks.items {
    ///         println!("Webhook: {} -> {}", wh.webhook_id, wh.url);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Webhook>> {
        let query = self.client.build_pagination_query(&params);
        self.client.get_with_query("webhooks", &query).await
    }

    /// Get a webhook by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let webhook = client.webhooks().get("whk_pqr678").await?;
    ///     println!("Webhook URL: {}", webhook.url);
    ///     println!("Events: {:?}", webhook.event_types);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, webhook_id: &str) -> Result<Webhook> {
        self.client.get(&format!("webhooks/{}", webhook_id)).await
    }

    /// Update a webhook
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, UpdateWebhookRequest, WebhookEventType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     // Update webhook URL
    ///     let webhook = client.webhooks().update(
    ///         "whk_pqr678",
    ///         UpdateWebhookRequest::new().url("https://api.myapp.com/webhooks/v2")
    ///     ).await?;
    ///     
    ///     // Disable webhook
    ///     let webhook = client.webhooks().update(
    ///         "whk_pqr678",
    ///         UpdateWebhookRequest::new().enabled(false)
    ///     ).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(&self, webhook_id: &str, request: UpdateWebhookRequest) -> Result<Webhook> {
        self.client
            .put(&format!("webhooks/{}", webhook_id), &request)
            .await
    }

    /// Delete a webhook
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.webhooks().delete("whk_pqr678").await?;
    ///     println!("Webhook deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, webhook_id: &str) -> Result<()> {
        self.client.delete(&format!("webhooks/{}", webhook_id)).await
    }
}
