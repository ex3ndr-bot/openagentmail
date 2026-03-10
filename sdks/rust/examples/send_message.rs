//! Example: Sending an email
//!
//! This example demonstrates how to send an email message.
//!
//! Run with: cargo run --example send_message

use openagentmail::{AttachmentInput, ListInboxesParams, OpenAgentMail, SendMessageRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("OPENAGENTMAIL_API_KEY")
        .expect("OPENAGENTMAIL_API_KEY environment variable must be set");

    // Create client
    let client = OpenAgentMail::new(api_key)?;

    // Get an inbox to send from
    let inboxes = client
        .inboxes()
        .list(ListInboxesParams::new().limit(1))
        .await?;

    let inbox = inboxes
        .items
        .first()
        .expect("No inboxes found. Create one first.");

    println!("Sending from: {}", inbox.email);

    // Build the message
    let request = SendMessageRequest::new("recipient@example.com", "Hello from OpenAgentMail!")
        .text("This is a test email sent using the OpenAgentMail Rust SDK.")
        .html("<p>This is a test email sent using the <strong>OpenAgentMail Rust SDK</strong>.</p>")
        .add_cc("cc@example.com")
        .label("test")
        .label("outbound");

    // Optionally add an attachment
    // let request = request.attachment(
    //     AttachmentInput::new("hello.txt", base64::encode("Hello, World!"))
    //         .content_type("text/plain")
    // );

    // Send the message
    let message = client.messages().send(&inbox.inbox_id, request).await?;

    println!("\nMessage sent:");
    println!("  ID: {}", message.message_id);
    println!("  Thread: {}", message.thread_id);
    println!("  Subject: {}", message.subject);
    println!("  To: {:?}", message.to);

    Ok(())
}
