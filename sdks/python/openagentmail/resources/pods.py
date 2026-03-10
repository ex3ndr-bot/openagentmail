"""Pod resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Pod


class Pods:
    """Synchronous pod operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def create(
        self,
        name: str,
        *,
        client_id: Optional[str] = None,
    ) -> Pod:
        """Create a new pod.

        Args:
            name: Pod display name
            client_id: Idempotency key

        Returns:
            Created Pod object
        """
        payload: dict[str, Any] = {"name": name}
        if client_id is not None:
            payload["client_id"] = client_id

        data = self._http.post("/pods", json=payload)
        return Pod.model_validate(data)

    def get(self, pod_id: str) -> Pod:
        """Get a pod by ID.

        Args:
            pod_id: The pod ID

        Returns:
            Pod object
        """
        data = self._http.get(f"/pods/{pod_id}")
        return Pod.model_validate(data)

    def list(
        self,
        *,
        limit: int = 20,
    ) -> SyncPaginator[Pod]:
        """List pods with pagination.

        Args:
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Pod objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                "/pods",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Pod, limit=limit)

    def update(
        self,
        pod_id: str,
        *,
        name: Optional[str] = None,
    ) -> Pod:
        """Update a pod.

        Args:
            pod_id: The pod ID
            name: New name

        Returns:
            Updated Pod object
        """
        payload: dict[str, Any] = {}
        if name is not None:
            payload["name"] = name

        data = self._http.patch(f"/pods/{pod_id}", json=payload)
        return Pod.model_validate(data)

    def delete(self, pod_id: str) -> None:
        """Delete a pod.

        Args:
            pod_id: The pod ID
        """
        self._http.delete(f"/pods/{pod_id}")


class AsyncPods:
    """Asynchronous pod operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def create(
        self,
        name: str,
        *,
        client_id: Optional[str] = None,
    ) -> Pod:
        """Create a new pod.

        Args:
            name: Pod display name
            client_id: Idempotency key

        Returns:
            Created Pod object
        """
        payload: dict[str, Any] = {"name": name}
        if client_id is not None:
            payload["client_id"] = client_id

        data = await self._http.post("/pods", json=payload)
        return Pod.model_validate(data)

    async def get(self, pod_id: str) -> Pod:
        """Get a pod by ID.

        Args:
            pod_id: The pod ID

        Returns:
            Pod object
        """
        data = await self._http.get(f"/pods/{pod_id}")
        return Pod.model_validate(data)

    def list(
        self,
        *,
        limit: int = 20,
    ) -> AsyncPaginator[Pod]:
        """List pods with pagination.

        Args:
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Pod objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                "/pods",
                params={
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Pod, limit=limit)

    async def update(
        self,
        pod_id: str,
        *,
        name: Optional[str] = None,
    ) -> Pod:
        """Update a pod.

        Args:
            pod_id: The pod ID
            name: New name

        Returns:
            Updated Pod object
        """
        payload: dict[str, Any] = {}
        if name is not None:
            payload["name"] = name

        data = await self._http.patch(f"/pods/{pod_id}", json=payload)
        return Pod.model_validate(data)

    async def delete(self, pod_id: str) -> None:
        """Delete a pod.

        Args:
            pod_id: The pod ID
        """
        await self._http.delete(f"/pods/{pod_id}")
