//! Example: Setting up webhooks
//!
//! This example demonstrates how to create and manage webhooks.
//!
//! Run with: cargo run --example webhooks

use openagentmail::{
    CreateWebhookRequest, OpenAgentMail, PaginationParams, UpdateWebhookRequest, WebhookEventType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("OPENAGENTMAIL_API_KEY")
        .expect("OPENAGENTMAIL_API_KEY environment variable must be set");

    // Create client
    let client = OpenAgentMail::new(api_key)?;

    // Create a webhook
    let webhook = client
        .webhooks()
        .create(
            CreateWebhookRequest::new(
                "https://api.example.com/webhooks/email",
                vec![
                    WebhookEventType::MessageReceived,
                    WebhookEventType::MessageSent,
                    WebhookEventType::MessageBounced,
                ],
            )
            .client_id("example-webhook-001"),
        )
        .await?;

    println!("Created webhook:");
    println!("  ID: {}", webhook.webhook_id);
    println!("  URL: {}", webhook.url);
    println!("  Events: {:?}", webhook.event_types);
    println!("  Secret: {}", webhook.secret);
    println!("  Enabled: {}", webhook.enabled);

    // List all webhooks
    println!("\nAll webhooks:");
    let webhooks = client.webhooks().list(PaginationParams::new()).await?;
    for wh in &webhooks.items {
        println!("  - {} -> {} ({})", wh.webhook_id, wh.url, if wh.enabled { "enabled" } else { "disabled" });
    }

    // Update the webhook (disable it)
    let updated = client
        .webhooks()
        .update(
            &webhook.webhook_id,
            UpdateWebhookRequest::new().enabled(false),
        )
        .await?;
    println!("\nWebhook disabled: {}", !updated.enabled);

    // Delete the webhook
    client.webhooks().delete(&webhook.webhook_id).await?;
    println!("Webhook deleted");

    Ok(())
}
