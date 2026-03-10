"""Message resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional, Union

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Message


class Messages:
    """Synchronous message operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def send(
        self,
        inbox_id: str,
        *,
        to: Union[str, list[str]],
        subject: str,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        headers: Optional[dict[str, str]] = None,
        reply_to_message_id: Optional[str] = None,
    ) -> Message:
        """Send an email message.

        Args:
            inbox_id: The inbox to send from
            to: Recipient email address(es)
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            headers: Custom headers
            reply_to_message_id: Message ID to reply to

        Returns:
            Sent Message object
        """
        to_list = [to] if isinstance(to, str) else to

        payload: dict[str, Any] = {
            "to": to_list,
            "subject": subject,
        }
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if headers is not None:
            payload["headers"] = headers
        if reply_to_message_id is not None:
            payload["reply_to_message_id"] = reply_to_message_id

        data = self._http.post(f"/inboxes/{inbox_id}/messages", json=payload)
        return Message.model_validate(data)

    def get(self, inbox_id: str, message_id: str) -> Message:
        """Get a message by ID.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID

        Returns:
            Message object
        """
        data = self._http.get(f"/inboxes/{inbox_id}/messages/{message_id}")
        return Message.model_validate(data)

    def list(
        self,
        inbox_id: str,
        *,
        thread_id: Optional[str] = None,
        label: Optional[str] = None,
        limit: int = 20,
    ) -> SyncPaginator[Message]:
        """List messages with pagination.

        Args:
            inbox_id: The inbox ID
            thread_id: Filter by thread ID
            label: Filter by label
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Message objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                f"/inboxes/{inbox_id}/messages",
                params={
                    "thread_id": thread_id,
                    "label": label,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Message, limit=limit)

    def update(
        self,
        inbox_id: str,
        message_id: str,
        *,
        labels: Optional[list[str]] = None,
    ) -> Message:
        """Update message labels.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
            labels: New labels

        Returns:
            Updated Message object
        """
        payload: dict[str, Any] = {}
        if labels is not None:
            payload["labels"] = labels

        data = self._http.patch(
            f"/inboxes/{inbox_id}/messages/{message_id}", json=payload
        )
        return Message.model_validate(data)

    def delete(self, inbox_id: str, message_id: str) -> None:
        """Delete a message.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
        """
        self._http.delete(f"/inboxes/{inbox_id}/messages/{message_id}")

    def get_attachment(
        self, inbox_id: str, message_id: str, attachment_id: str
    ) -> bytes:
        """Download an attachment.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
            attachment_id: The attachment ID

        Returns:
            Attachment content as bytes
        """
        # This endpoint returns binary data, not JSON
        response = self._http.client.get(
            f"/inboxes/{inbox_id}/messages/{message_id}/attachments/{attachment_id}"
        )
        response.raise_for_status()
        return response.content


class AsyncMessages:
    """Asynchronous message operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def send(
        self,
        inbox_id: str,
        *,
        to: Union[str, list[str]],
        subject: str,
        text: Optional[str] = None,
        html: Optional[str] = None,
        cc: Optional[list[str]] = None,
        bcc: Optional[list[str]] = None,
        headers: Optional[dict[str, str]] = None,
        reply_to_message_id: Optional[str] = None,
    ) -> Message:
        """Send an email message.

        Args:
            inbox_id: The inbox to send from
            to: Recipient email address(es)
            subject: Email subject
            text: Plain text body
            html: HTML body
            cc: CC recipients
            bcc: BCC recipients
            headers: Custom headers
            reply_to_message_id: Message ID to reply to

        Returns:
            Sent Message object
        """
        to_list = [to] if isinstance(to, str) else to

        payload: dict[str, Any] = {
            "to": to_list,
            "subject": subject,
        }
        if text is not None:
            payload["text"] = text
        if html is not None:
            payload["html"] = html
        if cc is not None:
            payload["cc"] = cc
        if bcc is not None:
            payload["bcc"] = bcc
        if headers is not None:
            payload["headers"] = headers
        if reply_to_message_id is not None:
            payload["reply_to_message_id"] = reply_to_message_id

        data = await self._http.post(f"/inboxes/{inbox_id}/messages", json=payload)
        return Message.model_validate(data)

    async def get(self, inbox_id: str, message_id: str) -> Message:
        """Get a message by ID.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID

        Returns:
            Message object
        """
        data = await self._http.get(f"/inboxes/{inbox_id}/messages/{message_id}")
        return Message.model_validate(data)

    def list(
        self,
        inbox_id: str,
        *,
        thread_id: Optional[str] = None,
        label: Optional[str] = None,
        limit: int = 20,
    ) -> AsyncPaginator[Message]:
        """List messages with pagination.

        Args:
            inbox_id: The inbox ID
            thread_id: Filter by thread ID
            label: Filter by label
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Message objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                f"/inboxes/{inbox_id}/messages",
                params={
                    "thread_id": thread_id,
                    "label": label,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Message, limit=limit)

    async def update(
        self,
        inbox_id: str,
        message_id: str,
        *,
        labels: Optional[list[str]] = None,
    ) -> Message:
        """Update message labels.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
            labels: New labels

        Returns:
            Updated Message object
        """
        payload: dict[str, Any] = {}
        if labels is not None:
            payload["labels"] = labels

        data = await self._http.patch(
            f"/inboxes/{inbox_id}/messages/{message_id}", json=payload
        )
        return Message.model_validate(data)

    async def delete(self, inbox_id: str, message_id: str) -> None:
        """Delete a message.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
        """
        await self._http.delete(f"/inboxes/{inbox_id}/messages/{message_id}")

    async def get_attachment(
        self, inbox_id: str, message_id: str, attachment_id: str
    ) -> bytes:
        """Download an attachment.

        Args:
            inbox_id: The inbox ID
            message_id: The message ID
            attachment_id: The attachment ID

        Returns:
            Attachment content as bytes
        """
        response = await self._http.client.get(
            f"/inboxes/{inbox_id}/messages/{message_id}/attachments/{attachment_id}"
        )
        response.raise_for_status()
        return response.content
