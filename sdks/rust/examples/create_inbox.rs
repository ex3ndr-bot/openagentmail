use openagentmail::{CreateInboxRequest, OpenAgentMail};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OAM_API_KEY").expect("OAM_API_KEY not set");
    let client = OpenAgentMail::new(api_key);

    let org = client.organization().await?;
    println!("Organization: {} ({})", org.name, org.plan);

    let inbox = client.inboxes()
        .create(CreateInboxRequest::builder()
            .username("support-agent")
            .display_name("Support Agent")
            .client_id("example-inbox-001")
            .build())
        .await?;

    println!("Created inbox: {} ({})", inbox.email, inbox.inbox_id);

    let inboxes = client.inboxes().list(Default::default()).await?;
    println!("Total inboxes: {}", inboxes.items.len());

    Ok(())
}
