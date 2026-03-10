"""Webhook resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Webhook


class Webhooks:
    """Synchronous webhook operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def create(
        self,
        url: str,
        event_types: list[str],
        *,
        inbox_ids: Optional[list[str]] = None,
        pod_ids: Optional[list[str]] = None,
        client_id: Optional[str] = None,
    ) -> Webhook:
        """Create a new webhook.

        Args:
            url: URL to receive webhook events
            event_types: Events to subscribe to (e.g., ['message.received'])
            inbox_ids: Filter to specific inboxes
            pod_ids: Filter to specific pods
            client_id: Idempotency key

        Returns:
            Created Webhook object
        """
        payload: dict[str, Any] = {
            "url": url,
            "event_types": event_types,
        }
        if inbox_ids is not None:
            payload["inbox_ids"] = inbox_ids
        if pod_ids is not None:
            payload["pod_ids"] = pod_ids
        if client_id is not None:
            payload["client_id"] = client_id

        data = self._http.post("/webhooks", json=payload)
        return Webhook.model_validate(data)

    def get(self, webhook_id: str) -> Webhook:
        """Get a webhook by ID.

        Args:
            webhook_id: The webhook ID

        Returns:
            Webhook object
        """
        data = self._http.get(f"/webhooks/{webhook_id}")
        return Webhook.model_validate(data)

    def list(
        self,
        *,
        limit: int = 20,
    ) -> SyncPaginator[Webhook]:
        """List webhooks with pagination.

        Args:
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Webhook objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                "/webhooks",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Webhook, limit=limit)

    def update(
        self,
        webhook_id: str,
        *,
        url: Optional[str] = None,
        event_types: Optional[list[str]] = None,
        inbox_ids: Optional[list[str]] = None,
        pod_ids: Optional[list[str]] = None,
        enabled: Optional[bool] = None,
    ) -> Webhook:
        """Update a webhook.

        Args:
            webhook_id: The webhook ID
            url: New URL
            event_types: New event types
            inbox_ids: New inbox filter
            pod_ids: New pod filter
            enabled: Enable or disable webhook

        Returns:
            Updated Webhook object
        """
        payload: dict[str, Any] = {}
        if url is not None:
            payload["url"] = url
        if event_types is not None:
            payload["event_types"] = event_types
        if inbox_ids is not None:
            payload["inbox_ids"] = inbox_ids
        if pod_ids is not None:
            payload["pod_ids"] = pod_ids
        if enabled is not None:
            payload["enabled"] = enabled

        data = self._http.patch(f"/webhooks/{webhook_id}", json=payload)
        return Webhook.model_validate(data)

    def delete(self, webhook_id: str) -> None:
        """Delete a webhook.

        Args:
            webhook_id: The webhook ID
        """
        self._http.delete(f"/webhooks/{webhook_id}")

    def rotate_secret(self, webhook_id: str) -> Webhook:
        """Rotate the webhook secret.

        Args:
            webhook_id: The webhook ID

        Returns:
            Webhook with new secret
        """
        data = self._http.post(f"/webhooks/{webhook_id}/rotate-secret")
        return Webhook.model_validate(data)


class AsyncWebhooks:
    """Asynchronous webhook operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def create(
        self,
        url: str,
        event_types: list[str],
        *,
        inbox_ids: Optional[list[str]] = None,
        pod_ids: Optional[list[str]] = None,
        client_id: Optional[str] = None,
    ) -> Webhook:
        """Create a new webhook.

        Args:
            url: URL to receive webhook events
            event_types: Events to subscribe to (e.g., ['message.received'])
            inbox_ids: Filter to specific inboxes
            pod_ids: Filter to specific pods
            client_id: Idempotency key

        Returns:
            Created Webhook object
        """
        payload: dict[str, Any] = {
            "url": url,
            "event_types": event_types,
        }
        if inbox_ids is not None:
            payload["inbox_ids"] = inbox_ids
        if pod_ids is not None:
            payload["pod_ids"] = pod_ids
        if client_id is not None:
            payload["client_id"] = client_id

        data = await self._http.post("/webhooks", json=payload)
        return Webhook.model_validate(data)

    async def get(self, webhook_id: str) -> Webhook:
        """Get a webhook by ID.

        Args:
            webhook_id: The webhook ID

        Returns:
            Webhook object
        """
        data = await self._http.get(f"/webhooks/{webhook_id}")
        return Webhook.model_validate(data)

    def list(
        self,
        *,
        limit: int = 20,
    ) -> AsyncPaginator[Webhook]:
        """List webhooks with pagination.

        Args:
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Webhook objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                "/webhooks",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Webhook, limit=limit)

    async def update(
        self,
        webhook_id: str,
        *,
        url: Optional[str] = None,
        event_types: Optional[list[str]] = None,
        inbox_ids: Optional[list[str]] = None,
        pod_ids: Optional[list[str]] = None,
        enabled: Optional[bool] = None,
    ) -> Webhook:
        """Update a webhook.

        Args:
            webhook_id: The webhook ID
            url: New URL
            event_types: New event types
            inbox_ids: New inbox filter
            pod_ids: New pod filter
            enabled: Enable or disable webhook

        Returns:
            Updated Webhook object
        """
        payload: dict[str, Any] = {}
        if url is not None:
            payload["url"] = url
        if event_types is not None:
            payload["event_types"] = event_types
        if inbox_ids is not None:
            payload["inbox_ids"] = inbox_ids
        if pod_ids is not None:
            payload["pod_ids"] = pod_ids
        if enabled is not None:
            payload["enabled"] = enabled

        data = await self._http.patch(f"/webhooks/{webhook_id}", json=payload)
        return Webhook.model_validate(data)

    async def delete(self, webhook_id: str) -> None:
        """Delete a webhook.

        Args:
            webhook_id: The webhook ID
        """
        await self._http.delete(f"/webhooks/{webhook_id}")

    async def rotate_secret(self, webhook_id: str) -> Webhook:
        """Rotate the webhook secret.

        Args:
            webhook_id: The webhook ID

        Returns:
            Webhook with new secret
        """
        data = await self._http.post(f"/webhooks/{webhook_id}/rotate-secret")
        return Webhook.model_validate(data)
