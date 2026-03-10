"""Domain resource for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional

from .._utils.http import AsyncHTTPClient, HTTPClient
from .._utils.pagination import AsyncPaginator, SyncPaginator
from ..types import Domain


class Domains:
    """Synchronous domain operations."""

    def __init__(self, http: HTTPClient) -> None:
        self._http = http

    def create(
        self,
        domain: str,
        *,
        pod_id: Optional[str] = None,
    ) -> Domain:
        """Add a custom domain.

        Args:
            domain: Domain name (e.g., 'example.com')
            pod_id: Associate domain with a specific pod

        Returns:
            Created Domain object with DNS records to configure
        """
        payload: dict[str, Any] = {"domain": domain}
        if pod_id is not None:
            payload["pod_id"] = pod_id

        data = self._http.post("/domains", json=payload)
        return Domain.model_validate(data)

    def get(self, domain_id: str) -> Domain:
        """Get a domain by ID.

        Args:
            domain_id: The domain ID

        Returns:
            Domain object
        """
        data = self._http.get(f"/domains/{domain_id}")
        return Domain.model_validate(data)

    def list(
        self,
        *,
        pod_id: Optional[str] = None,
        limit: int = 20,
    ) -> SyncPaginator[Domain]:
        """List domains with pagination.

        Args:
            pod_id: Filter by pod ID
            limit: Number of items per page (max 100)

        Returns:
            Paginator yielding Domain objects
        """

        def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return self._http.get(
                "/domains",
                params={
                    "pod_id": pod_id,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return SyncPaginator(fetch_page, Domain, limit=limit)

    def verify(self, domain_id: str) -> Domain:
        """Trigger domain verification.

        Args:
            domain_id: The domain ID

        Returns:
            Domain object with updated status
        """
        data = self._http.post(f"/domains/{domain_id}/verify")
        return Domain.model_validate(data)

    def delete(self, domain_id: str) -> None:
        """Delete a domain.

        Args:
            domain_id: The domain ID
        """
        self._http.delete(f"/domains/{domain_id}")


class AsyncDomains:
    """Asynchronous domain operations."""

    def __init__(self, http: AsyncHTTPClient) -> None:
        self._http = http

    async def create(
        self,
        domain: str,
        *,
        pod_id: Optional[str] = None,
    ) -> Domain:
        """Add a custom domain.

        Args:
            domain: Domain name (e.g., 'example.com')
            pod_id: Associate domain with a specific pod

        Returns:
            Created Domain object with DNS records to configure
        """
        payload: dict[str, Any] = {"domain": domain}
        if pod_id is not None:
            payload["pod_id"] = pod_id

        data = await self._http.post("/domains", json=payload)
        return Domain.model_validate(data)

    async def get(self, domain_id: str) -> Domain:
        """Get a domain by ID.

        Args:
            domain_id: The domain ID

        Returns:
            Domain object
        """
        data = await self._http.get(f"/domains/{domain_id}")
        return Domain.model_validate(data)

    def list(
        self,
        *,
        pod_id: Optional[str] = None,
        limit: int = 20,
    ) -> AsyncPaginator[Domain]:
        """List domains with pagination.

        Args:
            pod_id: Filter by pod ID
            limit: Number of items per page (max 100)

        Returns:
            Async paginator yielding Domain objects
        """

        async def fetch_page(page_token: Optional[str], lim: int) -> dict[str, Any]:
            return await self._http.get(
                "/domains",
                params={
                    "pod_id": pod_id,
                    "limit": lim,
                    "page_token": page_token,
                },
            )

        return AsyncPaginator(fetch_page, Domain, limit=limit)

    async def verify(self, domain_id: str) -> Domain:
        """Trigger domain verification.

        Args:
            domain_id: The domain ID

        Returns:
            Domain object with updated status
        """
        data = await self._http.post(f"/domains/{domain_id}/verify")
        return Domain.model_validate(data)

    async def delete(self, domain_id: str) -> None:
        """Delete a domain.

        Args:
            domain_id: The domain ID
        """
        await self._http.delete(f"/domains/{domain_id}")
