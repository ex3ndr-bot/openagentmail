"""Resource modules for OpenAgentMail SDK."""

from .domains import AsyncDomains, Domains
from .drafts import AsyncDrafts, Drafts
from .inboxes import AsyncInboxes, Inboxes
from .messages import AsyncMessages, Messages
from .pods import AsyncPods, Pods
from .webhooks import AsyncWebhooks, Webhooks

__all__ = [
    "Inboxes",
    "AsyncInboxes",
    "Messages",
    "AsyncMessages",
    "Drafts",
    "AsyncDrafts",
    "Webhooks",
    "AsyncWebhooks",
    "Pods",
    "AsyncPods",
    "Domains",
    "AsyncDomains",
]
