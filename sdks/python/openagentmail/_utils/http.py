"""HTTP client utilities for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional, Type, TypeVar

import httpx

from ..errors import (
    APIError,
    AuthenticationError,
    AuthorizationError,
    ConflictError,
    ConnectionError,
    InternalServerError,
    NotFoundError,
    RateLimitError,
    TimeoutError,
    ValidationError,
)

T = TypeVar("T")

DEFAULT_BASE_URL = "https://api.openagentmail.com/v0"
DEFAULT_TIMEOUT = 30.0


class HTTPClient:
    """Synchronous HTTP client for API requests."""

    def __init__(
        self,
        api_key: str,
        base_url: str = DEFAULT_BASE_URL,
        timeout: float = DEFAULT_TIMEOUT,
    ) -> None:
        self.api_key = api_key
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self._client: Optional[httpx.Client] = None

    @property
    def client(self) -> httpx.Client:
        if self._client is None:
            self._client = httpx.Client(
                base_url=self.base_url,
                headers={
                    "Authorization": f"Bearer {self.api_key}",
                    "Content-Type": "application/json",
                    "User-Agent": "openagentmail-python/0.1.0",
                },
                timeout=self.timeout,
            )
        return self._client

    def close(self) -> None:
        if self._client is not None:
            self._client.close()
            self._client = None

    def __enter__(self) -> "HTTPClient":
        return self

    def __exit__(self, *args: Any) -> None:
        self.close()

    def request(
        self,
        method: str,
        path: str,
        json: Optional[dict[str, Any]] = None,
        params: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        """Make an HTTP request and return JSON response."""
        # Filter out None values from params
        if params:
            params = {k: v for k, v in params.items() if v is not None}

        try:
            response = self.client.request(
                method=method,
                url=path,
                json=json,
                params=params,
            )
        except httpx.ConnectError as e:
            raise ConnectionError(f"Failed to connect: {e}") from e
        except httpx.TimeoutException as e:
            raise TimeoutError(f"Request timed out: {e}") from e

        return self._handle_response(response)

    def _handle_response(self, response: httpx.Response) -> dict[str, Any]:
        """Handle response and raise appropriate errors."""
        if response.status_code == 204:
            return {}

        try:
            data = response.json()
        except Exception:
            if response.is_success:
                return {}
            raise InternalServerError(f"Invalid JSON response: {response.text}")

        if response.is_success:
            return data

        # Extract error info
        error_data = data.get("error", {})
        message = error_data.get("message", "Unknown error")
        code = error_data.get("code", "unknown_error")
        details = error_data.get("details")

        # Map to specific exception types
        status = response.status_code
        if status == 401:
            raise AuthenticationError(message)
        elif status == 403:
            raise AuthorizationError(message)
        elif status == 404:
            raise NotFoundError(message)
        elif status == 409:
            raise ConflictError(message)
        elif status == 422:
            raise ValidationError(message, details)
        elif status == 429:
            retry_after = response.headers.get("Retry-After")
            raise RateLimitError(
                message,
                retry_after=int(retry_after) if retry_after else None,
            )
        elif status >= 500:
            raise InternalServerError(message)
        else:
            raise APIError(message, status, code, details)

    def get(
        self,
        path: str,
        params: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return self.request("GET", path, params=params)

    def post(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return self.request("POST", path, json=json)

    def put(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return self.request("PUT", path, json=json)

    def patch(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return self.request("PATCH", path, json=json)

    def delete(
        self,
        path: str,
    ) -> dict[str, Any]:
        return self.request("DELETE", path)


class AsyncHTTPClient:
    """Asynchronous HTTP client for API requests."""

    def __init__(
        self,
        api_key: str,
        base_url: str = DEFAULT_BASE_URL,
        timeout: float = DEFAULT_TIMEOUT,
    ) -> None:
        self.api_key = api_key
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self._client: Optional[httpx.AsyncClient] = None

    @property
    def client(self) -> httpx.AsyncClient:
        if self._client is None:
            self._client = httpx.AsyncClient(
                base_url=self.base_url,
                headers={
                    "Authorization": f"Bearer {self.api_key}",
                    "Content-Type": "application/json",
                    "User-Agent": "openagentmail-python/0.1.0",
                },
                timeout=self.timeout,
            )
        return self._client

    async def close(self) -> None:
        if self._client is not None:
            await self._client.aclose()
            self._client = None

    async def __aenter__(self) -> "AsyncHTTPClient":
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.close()

    async def request(
        self,
        method: str,
        path: str,
        json: Optional[dict[str, Any]] = None,
        params: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        """Make an async HTTP request and return JSON response."""
        # Filter out None values from params
        if params:
            params = {k: v for k, v in params.items() if v is not None}

        try:
            response = await self.client.request(
                method=method,
                url=path,
                json=json,
                params=params,
            )
        except httpx.ConnectError as e:
            raise ConnectionError(f"Failed to connect: {e}") from e
        except httpx.TimeoutException as e:
            raise TimeoutError(f"Request timed out: {e}") from e

        return self._handle_response(response)

    def _handle_response(self, response: httpx.Response) -> dict[str, Any]:
        """Handle response and raise appropriate errors."""
        if response.status_code == 204:
            return {}

        try:
            data = response.json()
        except Exception:
            if response.is_success:
                return {}
            raise InternalServerError(f"Invalid JSON response: {response.text}")

        if response.is_success:
            return data

        # Extract error info
        error_data = data.get("error", {})
        message = error_data.get("message", "Unknown error")
        code = error_data.get("code", "unknown_error")
        details = error_data.get("details")

        # Map to specific exception types
        status = response.status_code
        if status == 401:
            raise AuthenticationError(message)
        elif status == 403:
            raise AuthorizationError(message)
        elif status == 404:
            raise NotFoundError(message)
        elif status == 409:
            raise ConflictError(message)
        elif status == 422:
            raise ValidationError(message, details)
        elif status == 429:
            retry_after = response.headers.get("Retry-After")
            raise RateLimitError(
                message,
                retry_after=int(retry_after) if retry_after else None,
            )
        elif status >= 500:
            raise InternalServerError(message)
        else:
            raise APIError(message, status, code, details)

    async def get(
        self,
        path: str,
        params: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return await self.request("GET", path, params=params)

    async def post(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return await self.request("POST", path, json=json)

    async def put(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return await self.request("PUT", path, json=json)

    async def patch(
        self,
        path: str,
        json: Optional[dict[str, Any]] = None,
    ) -> dict[str, Any]:
        return await self.request("PATCH", path, json=json)

    async def delete(
        self,
        path: str,
    ) -> dict[str, Any]:
        return await self.request("DELETE", path)
