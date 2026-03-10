"""Draft resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional, Union

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Draft, Message


class Drafts:
    """Synchronous draft operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def create(
        self,
        inbox_id: str,
        *,
        to: Optional[Union[str, list[str]]] = None,
        subject: Optional[str] = None,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        send_at: Optional[str] = None,
    ) -> Draft:
        """Create a new draft.

        Args:
            inbox_id: The inbox ID
            to: Recipient email address(es)
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            send_at: ISO 8601 datetime for scheduled send

        Returns:
            Created Draft object
        """
        payload: dict[str, Any] = {}
        if to is not None:
            payload["to"] = [to] if isinstance(to, str) else to
        if subject is not None:
            payload["subject"] = subject
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if send_at is not None:
            payload["send_at"] = send_at

        data = self._http.post(f"/inboxes/{inbox_id}/drafts", json=payload)
        return Draft.model_validate(data)

    def get(self, inbox_id: str, draft_id: str) -> Draft:
        """Get a draft by ID.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID

        Returns:
            Draft object
        """
        data = self._http.get(f"/inboxes/{inbox_id}/drafts/{draft_id}")
        return Draft.model_validate(data)

    def list(
        self,
        inbox_id: str,
        *,
        limit: int = 20,
    ) -> SyncPaginator[Draft]:
        """List drafts with pagination.

        Args:
            inbox_id: The inbox ID
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Draft objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                f"/inboxes/{inbox_id}/drafts",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Draft, limit=limit)

    def update(
        self,
        inbox_id: str,
        draft_id: str,
        *,
        to: Optional[list[str]] = None,
        subject: Optional[str] = None,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        send_at: Optional[str] = None,
    ) -> Draft:
        """Update a draft.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID
            to: Recipient email addresses
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            send_at: ISO 8601 datetime for scheduled send

        Returns:
            Updated Draft object
        """
        payload: dict[str, Any] = {}
        if to is not None:
            payload["to"] = to
        if subject is not None:
            payload["subject"] = subject
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if send_at is not None:
            payload["send_at"] = send_at

        data = self._http.patch(f"/inboxes/{inbox_id}/drafts/{draft_id}", json=payload)
        return Draft.model_validate(data)

    def delete(self, inbox_id: str, draft_id: str) -> None:
        """Delete a draft.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID
        """
        self._http.delete(f"/inboxes/{inbox_id}/drafts/{draft_id}")

    def send(self, inbox_id: str, draft_id: str) -> Message:
        """Send a draft immediately.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID

        Returns:
            Sent Message object
        """
        data = self._http.post(f"/inboxes/{inbox_id}/drafts/{draft_id}/send")
        return Message.model_validate(data)


class AsyncDrafts:
    """Asynchronous draft operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def create(
        self,
        inbox_id: str,
        *,
        to: Optional[Union[str, list[str]]] = None,
        subject: Optional[str] = None,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        send_at: Optional[str] = None,
    ) -> Draft:
        """Create a new draft.

        Args:
            inbox_id: The inbox ID
            to: Recipient email address(es)
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            send_at: ISO 8601 datetime for scheduled send

        Returns:
            Created Draft object
        """
        payload: dict[str, Any] = {}
        if to is not None:
            payload["to"] = [to] if isinstance(to, str) else to
        if subject is not None:
            payload["subject"] = subject
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if send_at is not None:
            payload["send_at"] = send_at

        data = await self._http.post(f"/inboxes/{inbox_id}/drafts", json=payload)
        return Draft.model_validate(data)

    async def get(self, inbox_id: str, draft_id: str) -> Draft:
        """Get a draft by ID.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID

        Returns:
            Draft object
        """
        data = await self._http.get(f"/inboxes/{inbox_id}/drafts/{draft_id}")
        return Draft.model_validate(data)

    def list(
        self,
        inbox_id: str,
        *,
        limit: int = 20,
    ) -> AsyncPaginator[Draft]:
        """List drafts with pagination.

        Args:
            inbox_id: The inbox ID
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Draft objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                f"/inboxes/{inbox_id}/drafts",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Draft, limit=limit)

    async def update(
        self,
        inbox_id: str,
        draft_id: str,
        *,
        to: Optional[list[str]] = None,
        subject: Optional[str] = None,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        send_at: Optional[str] = None,
    ) -> Draft:
        """Update a draft.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID
            to: Recipient email addresses
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            send_at: ISO 8601 datetime for scheduled send

        Returns:
            Updated Draft object
        """
        payload: dict[str, Any] = {}
        if to is not None:
            payload["to"] = to
        if subject is not None:
            payload["subject"] = subject
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if send_at is not None:
            payload["send_at"] = send_at

        data = await self._http.patch(
            f"/inboxes/{inbox_id}/drafts/{draft_id}", json=payload
        )
        return Draft.model_validate(data)

    async def delete(self, inbox_id: str, draft_id: str) -> None:
        """Delete a draft.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID
        """
        await self._http.delete(f"/inboxes/{inbox_id}/drafts/{draft_id}")

    async def send(self, inbox_id: str, draft_id: str) -> Message:
        """Send a draft immediately.

        Args:
            inbox_id: The inbox ID
            draft_id: The draft ID

        Returns:
            Sent Message object
        """
        data = await self._http.post(f"/inboxes/{inbox_id}/drafts/{draft_id}/send")
        return Message.model_validate(data)
