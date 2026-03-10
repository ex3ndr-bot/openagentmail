"""Internal utilities for OpenAgentMail SDK."""

from .http import AsyncHTTPClient, HTTPClient
from .pagination import AsyncPaginator, SyncPaginator

__all__ = [
    "HTTPClient",
    "AsyncHTTPClient",
    "SyncPaginator",
    "AsyncPaginator",
]
