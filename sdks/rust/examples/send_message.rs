use openagentmail::{ListMessagesParams, OpenAgentMail, SendMessageRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OAM_API_KEY").expect("OAM_API_KEY not set");
    let inbox_id = env::var("OAM_INBOX_ID").expect("OAM_INBOX_ID not set");
    let recipient = env::var("OAM_RECIPIENT").unwrap_or_else(|_| "test@example.com".to_string());

    let client = OpenAgentMail::new(api_key);

    let message = client.messages()
        .send(&inbox_id, SendMessageRequest::builder()
            .to(&recipient)
            .subject("Hello from OpenAgentMail!")
            .text("This is a test email sent via the Rust SDK.")
            .html("<h1>Hello!</h1><p>Test from the <strong>Rust SDK</strong>.</p>")
            .build())
        .await?;

    println!("Sent: {} -> {:?}", message.message_id, message.to);

    let messages = client.messages()
        .list(&inbox_id, ListMessagesParams::new().limit(5))
        .await?;

    println!("Recent messages:");
    for msg in messages.items {
        println!("  - {}: {}", msg.from, msg.subject);
    }

    Ok(())
}
