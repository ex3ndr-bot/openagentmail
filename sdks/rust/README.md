# OpenAgentMail Rust SDK

Rust SDK for [OpenAgentMail](https://openagentmail.com) - an open-source email API for AI agents.

[![Crates.io](https://img.shields.io/crates/v/openagentmail.svg)](https://crates.io/crates/openagentmail)
[![Documentation](https://docs.rs/openagentmail/badge.svg)](https://docs.rs/openagentmail)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openagentmail = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use openagentmail::{OpenAgentMail, CreateInboxRequest, SendMessageRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAgentMail::new("your_api_key");

    // Create an inbox
    let inbox = client.inboxes()
        .create(CreateInboxRequest::builder()
            .username("support")
            .display_name("Support Team")
            .build())
        .await?;

    println!("Created: {}", inbox.email);

    // Send a message
    let message = client.messages()
        .send(&inbox.inbox_id, SendMessageRequest::builder()
            .to("user@example.com")
            .subject("Hello!")
            .text("Sent via the Rust SDK")
            .build())
        .await?;

    println!("Sent: {}", message.message_id);
    Ok(())
}
```

## Features

- **Async-first**: Built on tokio and reqwest
- **Strong typing**: Builder patterns for all request types
- **Error handling**: Comprehensive error types with thiserror

## API Resources

### Inboxes

```rust
let inbox = client.inboxes()
    .create(CreateInboxRequest::builder()
        .username("agent")
        .display_name("AI Agent")
        .build())
    .await?;

let inboxes = client.inboxes().list(Default::default()).await?;
let inbox = client.inboxes().get("inbox_id").await?;
client.inboxes().delete("inbox_id").await?;
```

### Messages

```rust
let message = client.messages()
    .send("inbox_id", SendMessageRequest::builder()
        .to("recipient@example.com")
        .subject("Hello!")
        .text("Message body")
        .build())
    .await?;

let messages = client.messages()
    .list("inbox_id", ListMessagesParams::new().limit(10))
    .await?;
```

### Drafts

```rust
let draft = client.drafts()
    .create("inbox_id", CreateDraftRequest::builder()
        .to("recipient@example.com")
        .subject("Draft")
        .build())
    .await?;

let sent = client.drafts().send("inbox_id", &draft.draft_id).await?;
```

### Webhooks

```rust
let webhook = client.webhooks()
    .create(CreateWebhookRequest::builder()
        .url("https://example.com/webhook")
        .event_type(WebhookEventType::MessageReceived)
        .build())
    .await?;

println!("Secret: {}", webhook.secret);
```

### Domains

```rust
let domain = client.domains()
    .add(AddDomainRequest::new("example.com"))
    .await?;

for record in &domain.dns_records {
    println!("{} {} -> {}", record.record_type, record.name, record.value);
}

client.domains().verify(&domain.domain_id).await?;
```

### Pods

```rust
let pod = client.pods()
    .create(CreatePodRequest::builder()
        .name("Customer A")
        .build())
    .await?;
```

## Configuration

```rust
use openagentmail::{OpenAgentMail, ClientConfig};

let client = OpenAgentMail::with_config(
    ClientConfig::new("api_key")
        .base_url("https://custom.api.com/v0")
        .timeout(30)
);
```

## Error Handling

```rust
use openagentmail::Error;

match client.inboxes().get("invalid").await {
    Ok(inbox) => println!("{}", inbox.email),
    Err(Error::NotFound(msg)) => println!("Not found: {}", msg),
    Err(Error::Authentication(msg)) => println!("Auth error: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

## Examples

```bash
export OAM_API_KEY="your_api_key"
cargo run --example create_inbox
cargo run --example send_message
cargo run --example webhooks
```

## License

MIT License
