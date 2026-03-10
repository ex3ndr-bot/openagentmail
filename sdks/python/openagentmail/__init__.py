"""OpenAgentMail Python SDK - Email API for AI Agents.

Example:
    ```python
    from openagentmail import OpenAgentMail

    client = OpenAgentMail(api_key="your_api_key")

    # Create an inbox
    inbox = client.inboxes.create(username="support")

    # Send a message
    client.messages.send(
        inbox_id=inbox.inbox_id,
        to="user@example.com",
        subject="Hello",
        text="Hello from OpenAgentMail!"
    )
    ```
"""

__version__ = "0.1.0"

from .client import AsyncOpenAgentMail, OpenAgentMail
from .errors import (
    APIError,
    AuthenticationError,
    AuthorizationError,
    ConflictError,
    ConnectionError,
    InternalServerError,
    NotFoundError,
    OpenAgentMailError,
    RateLimitError,
    TimeoutError,
    ValidationError,
)
from .types import (
    Attachment,
    Domain,
    DnsRecord,
    Draft,
    Inbox,
    Message,
    Organization,
    Pod,
    Webhook,
    WebhookEvent,
)

__all__ = [
    # Version
    "__version__",
    # Clients
    "OpenAgentMail",
    "AsyncOpenAgentMail",
    # Types
    "Organization",
    "Pod",
    "Inbox",
    "Message",
    "Attachment",
    "Draft",
    "Domain",
    "DnsRecord",
    "Webhook",
    "WebhookEvent",
    # Errors
    "OpenAgentMailError",
    "APIError",
    "AuthenticationError",
    "AuthorizationError",
    "NotFoundError",
    "ConflictError",
    "ValidationError",
    "RateLimitError",
    "InternalServerError",
    "ConnectionError",
    "TimeoutError",
]
