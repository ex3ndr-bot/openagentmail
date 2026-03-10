# OpenAgentMail Rust SDK

A strongly-typed, async-first Rust SDK for the [OpenAgentMail](https://github.com/openagentmail/openagentmail) API.

OpenAgentMail is an open-source email API designed for AI agents.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openagentmail = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use openagentmail::{OpenAgentMail, SendMessageRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with your API key
    let client = OpenAgentMail::new("your-api-key")?;

    // Send an email
    let message = client.messages().send(
        "inb_abc123",
        SendMessageRequest::new("user@example.com", "Hello!")
            .text("This is a test email from OpenAgentMail.")
    ).await?;

    println!("Sent message: {}", message.message_id);
    Ok(())
}
```

## Features

- **Async/await**: Built on tokio for high-performance async operations
- **Strong typing**: Full type safety with builder patterns for requests
- **Error handling**: Comprehensive error types with `thiserror`
- **Pagination**: Easy iteration over paginated results
- **Idempotency**: Support for `client_id` to prevent duplicate operations

## Resources

### Organization

```rust
let org = client.organization().get().await?;
println!("Organization: {} ({})", org.name, org.plan);
```

### Pods

Pods provide multi-tenant isolation.

```rust
use openagentmail::{CreatePodRequest, PaginationParams};

// Create a pod
let pod = client.pods().create(
    CreatePodRequest::new("Production").client_id("prod-001")
).await?;

// List pods
let pods = client.pods().list(PaginationParams::new().limit(10)).await?;
for pod in pods.items {
    println!("{}: {}", pod.pod_id, pod.name);
}
```

### Inboxes

```rust
use openagentmail::{CreateInboxRequest, ListInboxesParams};

// Create an inbox
let inbox = client.inboxes().create(
    "pod_xyz789",
    CreateInboxRequest::new("support-agent")
        .display_name("Support Agent")
).await?;

println!("Created: {}", inbox.email);

// List inboxes
let inboxes = client.inboxes().list(ListInboxesParams::new()).await?;
```

### Messages

```rust
use openagentmail::{SendMessageRequest, ListMessagesParams, ReplyRequest};

// Send a message
let message = client.messages().send(
    "inb_abc123",
    SendMessageRequest::new("user@example.com", "Hello!")
        .text("Plain text body")
        .html("<p>HTML body</p>")
        .add_cc("cc@example.com")
        .label("outbound")
).await?;

// List messages
let messages = client.messages().list(
    "inb_abc123",
    ListMessagesParams::new()
        .limit(20)
        .label("inbox")
).await?;

// Reply to a message
let reply = client.messages().reply(
    "inb_abc123",
    "msg_def456",
    ReplyRequest::new().text("Thanks for reaching out!")
).await?;
```

### Drafts

```rust
use openagentmail::{CreateDraftRequest, UpdateDraftRequest};

// Create a draft
let draft = client.drafts().create(
    "inb_abc123",
    CreateDraftRequest::new()
        .add_to("user@example.com")
        .subject("Draft: Weekly Report")
        .text("Work in progress...")
        .send_at("2024-01-20T09:00:00Z")  // Schedule send
).await?;

// Update the draft
let updated = client.drafts().update(
    "inb_abc123",
    &draft.draft_id,
    UpdateDraftRequest::new().text("Final version ready to send.")
).await?;

// Send the draft
let message = client.drafts().send("inb_abc123", &draft.draft_id).await?;
```

### Webhooks

```rust
use openagentmail::{CreateWebhookRequest, WebhookEventType, UpdateWebhookRequest};

// Create a webhook
let webhook = client.webhooks().create(
    CreateWebhookRequest::new(
        "https://api.myapp.com/webhooks/email",
        vec![
            WebhookEventType::MessageReceived,
            WebhookEventType::MessageBounced,
        ]
    )
    .inbox_id("inb_abc123")
).await?;

println!("Secret: {}", webhook.secret);

// Disable webhook
client.webhooks().update(
    &webhook.webhook_id,
    UpdateWebhookRequest::new().enabled(false)
).await?;
```

### Domains

```rust
use openagentmail::CreateDomainRequest;

// Add a custom domain
let domain = client.domains().create(
    CreateDomainRequest::new("mail.mycompany.com")
        .pod_id("pod_xyz789")
).await?;

// Print DNS records to configure
for record in &domain.dns_records {
    println!("{} {} -> {}", record.record_type, record.name, record.value);
}

// Verify after DNS configuration
let verified = client.domains().verify(&domain.domain_id).await?;
println!("Status: {:?}", verified.status);
```

## Error Handling

The SDK uses a comprehensive `Error` type:

```rust
use openagentmail::Error;

match client.inboxes().get("invalid-id").await {
    Ok(inbox) => println!("Found: {}", inbox.email),
    Err(Error::NotFound(msg)) => println!("Not found: {}", msg),
    Err(Error::Authentication(msg)) => println!("Auth failed: {}", msg),
    Err(Error::RateLimited { reset_at }) => println!("Rate limited until: {}", reset_at),
    Err(Error::Api { status, code, message, .. }) => {
        println!("API error {}: {} ({})", status, message, code);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Configuration

```rust
use openagentmail::{OpenAgentMail, ClientConfig};

let config = ClientConfig::new("your-api-key")
    .base_url("https://custom.api.com")  // Custom API endpoint
    .timeout(60);                         // Request timeout in seconds

let client = OpenAgentMail::with_config(config)?;
```

## Pagination

All list endpoints support cursor-based pagination:

```rust
use openagentmail::PaginationParams;

// First page
let page1 = client.messages().list(
    "inb_abc123",
    ListMessagesParams::new().limit(20)
).await?;

// Next page
if page1.has_more {
    if let Some(token) = page1.next_page_token {
        let page2 = client.messages().list(
            "inb_abc123",
            ListMessagesParams::new().page_token(token)
        ).await?;
    }
}
```

## Examples

See the [examples](./examples) directory:

- `create_inbox.rs` - Creating pods and inboxes
- `send_message.rs` - Sending emails with attachments
- `webhooks.rs` - Setting up webhook notifications

Run examples with:

```bash
OPENAGENTMAIL_API_KEY=your-key cargo run --example create_inbox
```

## License

MIT
