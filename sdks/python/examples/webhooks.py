"""Example: Setting up webhooks with OpenAgentMail."""

import os

from openagentmail import OpenAgentMail

# Initialize client
client = OpenAgentMail(api_key=os.environ["OAM_API_KEY"])

# Create a webhook to receive message events
webhook = client.webhooks.create(
    url="https://your-server.com/webhooks/email",
    event_types=[
        "message.received",
        "message.sent",
        "message.bounced",
    ],
    client_id="main-webhook",  # For idempotency
)

print(f"Created webhook: {webhook.webhook_id}")
print(f"URL: {webhook.url}")
print(f"Events: {webhook.event_types}")
print(f"Secret: {webhook.secret}")  # Use this to verify webhook signatures

# Create a webhook for a specific inbox only
inbox = client.inboxes.create(username="monitored", client_id="monitored-inbox")

inbox_webhook = client.webhooks.create(
    url="https://your-server.com/webhooks/inbox-events",
    event_types=["message.received"],
    inbox_ids=[inbox.inbox_id],
)

print(f"\nCreated inbox-specific webhook: {inbox_webhook.webhook_id}")

# List all webhooks
print("\nAll webhooks:")
for wh in client.webhooks.list():
    status = "enabled" if wh.enabled else "disabled"
    print(f"  - {wh.url} ({status})")
    print(f"    Events: {wh.event_types}")

# Disable a webhook
disabled_webhook = client.webhooks.update(
    webhook.webhook_id,
    enabled=False,
)
print(f"\nDisabled webhook: {disabled_webhook.enabled}")

# Rotate webhook secret (in case of compromise)
rotated = client.webhooks.rotate_secret(webhook.webhook_id)
print(f"New secret: {rotated.secret}")
