"""Pagination utilities for OpenAgentMail SDK."""

from __future__ import annotations

from typing import (
    Any,
    AsyncIterator,
    Callable,
    Coroutine,
    Generic,
    Iterator,
    Optional,
    Type,
    TypeVar,
)

from pydantic import BaseModel

T = TypeVar("T", bound=BaseModel)


class SyncPaginator(Generic[T]):
    """Synchronous paginator for list endpoints."""

    def __init__(
        self,
        fetch_page: Callable[[Optional[str], int], dict[str, Any]],
        model_class: Type[T],
        limit: int = 20,
    ) -> None:
        self.fetch_page = fetch_page
        self.model_class = model_class
        self.limit = limit

    def __iter__(self) -> Iterator[T]:
        page_token: Optional[str] = None
        while True:
            data = self.fetch_page(page_token, self.limit)
            items = data.get("items", [])
            for item in items:
                yield self.model_class.model_validate(item)

            if not data.get("has_more", False):
                break
            page_token = data.get("next_page_token")
            if page_token is None:
                break

    def to_list(self) -> list[T]:
        """Convert paginator to a list of all items."""
        return list(self)

    def first(self) -> Optional[T]:
        """Get the first item or None."""
        for item in self:
            return item
        return None


class AsyncPaginator(Generic[T]):
    """Asynchronous paginator for list endpoints."""

    def __init__(
        self,
        fetch_page: Callable[[Optional[str], int], Coroutine[Any, Any, dict[str, Any]]],
        model_class: Type[T],
        limit: int = 20,
    ) -> None:
        self.fetch_page = fetch_page
        self.model_class = model_class
        self.limit = limit

    def __aiter__(self) -> AsyncIterator[T]:
        return self._iterate()

    async def _iterate(self) -> AsyncIterator[T]:
        page_token: Optional[str] = None
        while True:
            data = await self.fetch_page(page_token, self.limit)
            items = data.get("items", [])
            for item in items:
                yield self.model_class.model_validate(item)

            if not data.get("has_more", False):
                break
            page_token = data.get("next_page_token")
            if page_token is None:
                break

    async def to_list(self) -> list[T]:
        """Convert paginator to a list of all items."""
        return [item async for item in self]

    async def first(self) -> Optional[T]:
        """Get the first item or None."""
        async for item in self:
            return item
        return None
