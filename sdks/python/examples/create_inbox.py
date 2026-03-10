"""Example: Creating an inbox with OpenAgentMail."""

import os

from openagentmail import OpenAgentMail

# Initialize client with API key from environment
client = OpenAgentMail(api_key=os.environ["OAM_API_KEY"])

# Create a new inbox
inbox = client.inboxes.create(
    username="support-agent",
    display_name="Support Agent",
    client_id="my-unique-inbox-id",  # For idempotency
)

print(f"Created inbox: {inbox.email}")
print(f"Inbox ID: {inbox.inbox_id}")
print(f"Created at: {inbox.created_at}")

# List all inboxes
print("\nAll inboxes:")
for inbox in client.inboxes.list():
    print(f"  - {inbox.email} ({inbox.inbox_id})")
