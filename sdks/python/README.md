# OpenAgentMail Python SDK

Official Python SDK for [OpenAgentMail](https://openagentmail.com) - the open-source email API for AI agents.

## Installation

```bash
pip install openagentmail
```

## Quick Start

```python
from openagentmail import OpenAgentMail

# Initialize the client
client = OpenAgentMail(api_key="your_api_key")

# Create an inbox
inbox = client.inboxes.create(
    username="support",
    display_name="Support Agent"
)
print(f"Created inbox: {inbox.email}")

# Send a message
message = client.messages.send(
    inbox_id=inbox.inbox_id,
    to="user@example.com",
    subject="Hello!",
    text="Hello from OpenAgentMail!"
)
print(f"Sent message: {message.message_id}")

# List messages with pagination
for msg in client.messages.list(inbox.inbox_id):
    print(f"- {msg.subject}")
```

## Async Support

```python
import asyncio
from openagentmail import AsyncOpenAgentMail

async def main():
    async with AsyncOpenAgentMail(api_key="your_api_key") as client:
        # Create an inbox
        inbox = await client.inboxes.create(username="async-support")

        # Send a message
        await client.messages.send(
            inbox_id=inbox.inbox_id,
            to="user@example.com",
            subject="Async Hello!",
            text="Hello from async OpenAgentMail!"
        )

        # List messages
        async for msg in client.messages.list(inbox.inbox_id):
            print(f"- {msg.subject}")

asyncio.run(main())
```

## Features

- ✅ **Full API coverage** - Inboxes, Messages, Drafts, Webhooks, Pods, Domains
- ✅ **Type hints** - Full type annotations with Pydantic models
- ✅ **Sync and Async** - Both synchronous and asynchronous clients
- ✅ **Pagination** - Iterator support for all list endpoints
- ✅ **Error handling** - Typed exceptions for all error cases
- ✅ **Idempotency** - Support for `client_id` on create operations

## Resources

### Inboxes

```python
# Create
inbox = client.inboxes.create(
    username="support",
    domain="example.com",  # optional
    display_name="Support Team",  # optional
    pod_id="pod_123",  # optional, for multi-tenant
    client_id="unique-id",  # optional, for idempotency
)

# Get
inbox = client.inboxes.get("inbox_123")

# List
for inbox in client.inboxes.list(pod_id="pod_123"):
    print(inbox.email)

# Update
inbox = client.inboxes.update("inbox_123", display_name="New Name")

# Delete
client.inboxes.delete("inbox_123")
```

### Messages

```python
# Send
message = client.messages.send(
    inbox_id="inbox_123",
    to=["user@example.com"],
    subject="Hello",
    text="Plain text body",
    html="<h1>HTML body</h1>",  # optional
    cc=["cc@example.com"],  # optional
    bcc=["bcc@example.com"],  # optional
)

# Get
message = client.messages.get("inbox_123", "msg_456")

# List with filters
for msg in client.messages.list(
    "inbox_123",
    thread_id="thread_789",  # optional
    label="important",  # optional
):
    print(msg.subject)

# Update labels
message = client.messages.update(
    "inbox_123",
    "msg_456",
    labels=["important", "read"]
)

# Delete
client.messages.delete("inbox_123", "msg_456")

# Download attachment
content = client.messages.get_attachment(
    "inbox_123",
    "msg_456",
    "attach_789"
)
```

### Drafts

```python
# Create
draft = client.drafts.create(
    inbox_id="inbox_123",
    to=["user@example.com"],
    subject="Draft subject",
    text="Draft body",
)

# Update
draft = client.drafts.update(
    "inbox_123",
    "draft_456",
    subject="Updated subject"
)

# Send draft
message = client.drafts.send("inbox_123", "draft_456")

# Schedule send
draft = client.drafts.create(
    inbox_id="inbox_123",
    to=["user@example.com"],
    subject="Scheduled email",
    text="This will be sent later",
    send_at="2024-12-01T10:00:00Z"
)

# Delete
client.drafts.delete("inbox_123", "draft_456")
```

### Webhooks

```python
# Create
webhook = client.webhooks.create(
    url="https://your-server.com/webhook",
    event_types=["message.received", "message.sent"],
    inbox_ids=["inbox_123"],  # optional filter
)

# List
for wh in client.webhooks.list():
    print(f"{wh.url}: {wh.event_types}")

# Update
webhook = client.webhooks.update(
    "webhook_123",
    enabled=False
)

# Rotate secret
webhook = client.webhooks.rotate_secret("webhook_123")

# Delete
client.webhooks.delete("webhook_123")
```

### Pods (Multi-tenant)

```python
# Create
pod = client.pods.create(
    name="Production",
    client_id="unique-pod-id"
)

# List
for pod in client.pods.list():
    print(pod.name)

# Delete
client.pods.delete("pod_123")
```

### Domains

```python
# Add domain
domain = client.domains.create(
    domain="example.com",
    pod_id="pod_123"  # optional
)

# Get DNS records to configure
for record in domain.dns_records:
    print(f"{record.type} {record.name} -> {record.value}")

# Trigger verification
domain = client.domains.verify("domain_123")

# Check status
print(domain.status)  # pending, verifying, verified, failed

# Delete
client.domains.delete("domain_123")
```

## Error Handling

```python
from openagentmail import (
    OpenAgentMail,
    AuthenticationError,
    NotFoundError,
    RateLimitError,
    ValidationError,
)

client = OpenAgentMail(api_key="your_api_key")

try:
    inbox = client.inboxes.get("invalid_id")
except AuthenticationError:
    print("Invalid API key")
except NotFoundError:
    print("Inbox not found")
except RateLimitError as e:
    print(f"Rate limited, retry after {e.retry_after} seconds")
except ValidationError as e:
    print(f"Validation error: {e.message}")
    print(f"Details: {e.details}")
```

## Configuration

```python
client = OpenAgentMail(
    api_key="your_api_key",
    base_url="https://api.openagentmail.com/v0",  # default
    timeout=30.0,  # seconds, default
)
```

## Requirements

- Python 3.9+
- httpx >= 0.25.0
- pydantic >= 2.0.0

## License

MIT License - see [LICENSE](LICENSE) for details.
