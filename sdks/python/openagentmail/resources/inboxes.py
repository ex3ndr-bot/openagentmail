"""Inbox resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional, Union

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Inbox


class Inboxes:
    """Synchronous inbox operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def create(
        self,
        username: str,
        *,
        domain: Optional[str] = None,
        display_name: Optional[str] = None,
        pod_id: Optional[str] = None,
        client_id: Optional[str] = None,
    ) -> Inbox:
        """Create a new inbox.

        Args:
            username: Local part of email address
            domain: Domain for the inbox (uses default if not specified)
            display_name: Human-readable display name
            pod_id: Pod to create inbox in (for multi-tenant setups)
            client_id: Idempotency key

        Returns:
            Created Inbox object
        """
        payload: dict[str, Any] = {"username": username}
        if domain is not None:
            payload["domain"] = domain
        if display_name is not None:
            payload["display_name"] = display_name
        if pod_id is not None:
            payload["pod_id"] = pod_id
        if client_id is not None:
            payload["client_id"] = client_id

        data = self._http.post("/inboxes", json=payload)
        return Inbox.model_validate(data)

    def get(self, inbox_id: str) -> Inbox:
        """Get an inbox by ID.

        Args:
            inbox_id: The inbox ID

        Returns:
            Inbox object
        """
        data = self._http.get(f"/inboxes/{inbox_id}")
        return Inbox.model_validate(data)

    def list(
        self,
        *,
        pod_id: Optional[str] = None,
        limit: int = 20,
    ) -> SyncPaginator[Inbox]:
        """List inboxes with pagination.

        Args:
            pod_id: Filter by pod ID
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Inbox objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                "/inboxes",
                params={
                    "pod_id": pod_id,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Inbox, limit=limit)

    def update(
        self,
        inbox_id: str,
        *,
        display_name: Optional[str] = None,
    ) -> Inbox:
        """Update an inbox.

        Args:
            inbox_id: The inbox ID
            display_name: New display name

        Returns:
            Updated Inbox object
        """
        payload: dict[str, Any] = {}
        if display_name is not None:
            payload["display_name"] = display_name

        data = self._http.patch(f"/inboxes/{inbox_id}", json=payload)
        return Inbox.model_validate(data)

    def delete(self, inbox_id: str) -> None:
        """Delete an inbox.

        Args:
            inbox_id: The inbox ID to delete
        """
        self._http.delete(f"/inboxes/{inbox_id}")


class AsyncInboxes:
    """Asynchronous inbox operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def create(
        self,
        username: str,
        *,
        domain: Optional[str] = None,
        display_name: Optional[str] = None,
        pod_id: Optional[str] = None,
        client_id: Optional[str] = None,
    ) -> Inbox:
        """Create a new inbox.

        Args:
            username: Local part of email address
            domain: Domain for the inbox (uses default if not specified)
            display_name: Human-readable display name
            pod_id: Pod to create inbox in (for multi-tenant setups)
            client_id: Idempotency key

        Returns:
            Created Inbox object
        """
        payload: dict[str, Any] = {"username": username}
        if domain is not None:
            payload["domain"] = domain
        if display_name is not None:
            payload["display_name"] = display_name
        if pod_id is not None:
            payload["pod_id"] = pod_id
        if client_id is not None:
            payload["client_id"] = client_id

        data = await self._http.post("/inboxes", json=payload)
        return Inbox.model_validate(data)

    async def get(self, inbox_id: str) -> Inbox:
        """Get an inbox by ID.

        Args:
            inbox_id: The inbox ID

        Returns:
            Inbox object
        """
        data = await self._http.get(f"/inboxes/{inbox_id}")
        return Inbox.model_validate(data)

    def list(
        self,
        *,
        pod_id: Optional[str] = None,
        limit: int = 20,
    ) -> AsyncPaginator[Inbox]:
        """List inboxes with pagination.

        Args:
            pod_id: Filter by pod ID
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Inbox objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                "/inboxes",
                params={
                    "pod_id": pod_id,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Inbox, limit=limit)

    async def update(
        self,
        inbox_id: str,
        *,
        display_name: Optional[str] = None,
    ) -> Inbox:
        """Update an inbox.

        Args:
            inbox_id: The inbox ID
            display_name: New display name

        Returns:
            Updated Inbox object
        """
        payload: dict[str, Any] = {}
        if display_name is not None:
            payload["display_name"] = display_name

        data = await self._http.patch(f"/inboxes/{inbox_id}", json=payload)
        return Inbox.model_validate(data)

    async def delete(self, inbox_id: str) -> None:
        """Delete an inbox.

        Args:
            inbox_id: The inbox ID to delete
        """
        await self._http.delete(f"/inboxes/{inbox_id}")
