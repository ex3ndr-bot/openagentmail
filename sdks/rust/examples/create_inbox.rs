//! Example: Creating an inbox
//!
//! This example demonstrates how to create an inbox in a pod.
//!
//! Run with: cargo run --example create_inbox

use openagentmail::{CreateInboxRequest, CreatePodRequest, OpenAgentMail, PaginationParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("OPENAGENTMAIL_API_KEY")
        .expect("OPENAGENTMAIL_API_KEY environment variable must be set");

    // Create client
    let client = OpenAgentMail::new(api_key)?;

    // Get organization info
    let org = client.organization().get().await?;
    println!("Organization: {} ({})", org.name, org.plan);

    // Create or get a pod
    let pods = client.pods().list(PaginationParams::new().limit(1)).await?;
    let pod_id = if let Some(pod) = pods.items.first() {
        println!("Using existing pod: {}", pod.name);
        pod.pod_id.clone()
    } else {
        let pod = client
            .pods()
            .create(CreatePodRequest::new("Default Pod"))
            .await?;
        println!("Created pod: {}", pod.name);
        pod.pod_id
    };

    // Create an inbox
    let inbox = client
        .inboxes()
        .create(
            &pod_id,
            CreateInboxRequest::new("support-agent")
                .display_name("Support Agent")
                .client_id("example-inbox-001"),
        )
        .await?;

    println!("\nCreated inbox:");
    println!("  ID: {}", inbox.inbox_id);
    println!("  Email: {}", inbox.email);
    println!("  Display Name: {}", inbox.display_name.unwrap_or_default());

    Ok(())
}
