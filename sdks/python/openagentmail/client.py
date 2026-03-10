"""Main client for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional

from ._utils.http import AsyncHTTPClient, HTTPClient, DEFAULT_BASE_URL, DEFAULT_TIMEOUT
from .resources import (
    AsyncDomains,
    AsyncDrafts,
    AsyncInboxes,
    AsyncMessages,
    AsyncPods,
    AsyncWebhooks,
    Domains,
    Drafts,
    Inboxes,
    Messages,
    Pods,
    Webhooks,
)
from .types import Organization


class OpenAgentMail:
    """Synchronous OpenAgentMail API client.

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

        # List messages with pagination
        for message in client.messages.list(inbox.inbox_id):
            print(message.subject)
        ```
    """

    def __init__(
        self,
        api_key: str,
        *,
        base_url: str = DEFAULT_BASE_URL,
        timeout: float = DEFAULT_TIMEOUT,
    ) -> None:
        """Initialize the OpenAgentMail client.

        Args:
            api_key: Your API key (org_key or pod_key)
            base_url: API base URL (default: https://api.openagentmail.com/v0)
            timeout: Request timeout in seconds (default: 30)
        """
        self._http = HTTPClient(api_key, base_url=base_url, timeout=timeout)

        # Initialize resource clients
        self.inboxes = Inboxes(self._http)
        self.messages = Messages(self._http)
        self.drafts = Drafts(self._http)
        self.webhooks = Webhooks(self._http)
        self.pods = Pods(self._http)
        self.domains = Domains(self._http)

    def get_organization(self) -> Organization:
        """Get organization details.

        Returns:
            Organization object
        """
        data = self._http.get("/organization")
        return Organization.model_validate(data)

    def close(self) -> None:
        """Close the HTTP client."""
        self._http.close()

    def __enter__(self) -> "OpenAgentMail":
        return self

    def __exit__(self, *args: Any) -> None:
        self.close()


class AsyncOpenAgentMail:
    """Asynchronous OpenAgentMail API client.

    Example:
        ```python
        from openagentmail import AsyncOpenAgentMail

        async with AsyncOpenAgentMail(api_key="your_api_key") as client:
            # Create an inbox
            inbox = await client.inboxes.create(username="support")

            # Send a message
            await client.messages.send(
                inbox_id=inbox.inbox_id,
                to="user@example.com",
                subject="Hello",
                text="Hello from OpenAgentMail!"
            )

            # List messages with pagination
            async for message in client.messages.list(inbox.inbox_id):
                print(message.subject)
        ```
    """

    def __init__(
        self,
        api_key: str,
        *,
        base_url: str = DEFAULT_BASE_URL,
        timeout: float = DEFAULT_TIMEOUT,
    ) -> None:
        """Initialize the async OpenAgentMail client.

        Args:
            api_key: Your API key (org_key or pod_key)
            base_url: API base URL (default: https://api.openagentmail.com/v0)
            timeout: Request timeout in seconds (default: 30)
        """
        self._http = AsyncHTTPClient(api_key, base_url=base_url, timeout=timeout)

        # Initialize resource clients
        self.inboxes = AsyncInboxes(self._http)
        self.messages = AsyncMessages(self._http)
        self.drafts = AsyncDrafts(self._http)
        self.webhooks = AsyncWebhooks(self._http)
        self.pods = AsyncPods(self._http)
        self.domains = AsyncDomains(self._http)

    async def get_organization(self) -> Organization:
        """Get organization details.

        Returns:
            Organization object
        """
        data = await self._http.get("/organization")
        return Organization.model_validate(data)

    async def close(self) -> None:
        """Close the HTTP client."""
        await self._http.close()

    async def __aenter__(self) -> "AsyncOpenAgentMail":
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.close()
