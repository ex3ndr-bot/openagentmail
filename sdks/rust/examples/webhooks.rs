use openagentmail::{CreateWebhookRequest, OpenAgentMail, UpdateWebhookRequest, WebhookEventType};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OAM_API_KEY").expect("OAM_API_KEY not set");
    let webhook_url = env::var("OAM_WEBHOOK_URL").unwrap_or_else(|_| "https://example.com/webhook".to_string());

    let client = OpenAgentMail::new(api_key);

    let webhook = client.webhooks()
        .create(CreateWebhookRequest::builder()
            .url(&webhook_url)
            .event_type(WebhookEventType::MessageReceived)
            .event_type(WebhookEventType::MessageSent)
            .client_id("example-webhook-001")
            .build())
        .await?;

    println!("Created webhook: {}", webhook.webhook_id);
    println!("Secret: {}", webhook.secret);

    let updated = client.webhooks()
        .update(&webhook.webhook_id, UpdateWebhookRequest {
            enabled: Some(false),
            ..Default::default()
        })
        .await?;

    println!("Disabled: enabled={}", updated.enabled);

    let rotated = client.webhooks().rotate_secret(&webhook.webhook_id).await?;
    println!("New secret: {}", rotated.secret);

    Ok(())
}
