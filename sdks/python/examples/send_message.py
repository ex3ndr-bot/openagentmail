"""Example: Sending messages with OpenAgentMail."""

import os

from openagentmail import OpenAgentMail

# Initialize client
client = OpenAgentMail(api_key=os.environ["OAM_API_KEY"])

# First, get or create an inbox
inbox = client.inboxes.create(
    username="sender",
    client_id="sender-inbox",
)

# Send a simple text message
message = client.messages.send(
    inbox_id=inbox.inbox_id,
    to="recipient@example.com",
    subject="Hello from OpenAgentMail!",
    text="This is a test message sent via the Python SDK.",
)

print(f"Sent message: {message.message_id}")
print(f"Subject: {message.subject}")

# Send an HTML message with CC
html_message = client.messages.send(
    inbox_id=inbox.inbox_id,
    to=["recipient@example.com"],
    cc=["cc@example.com"],
    subject="HTML Newsletter",
    text="Plain text fallback",
    html="""
    <html>
    <body>
        <h1>Welcome!</h1>
        <p>This is an <strong>HTML</strong> email.</p>
    </body>
    </html>
    """,
)

print(f"\nSent HTML message: {html_message.message_id}")

# List recent messages
print("\nRecent messages:")
for msg in client.messages.list(inbox.inbox_id, limit=5):
    print(f"  - {msg.subject} (from: {msg.from_})")
