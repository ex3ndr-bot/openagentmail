"""Custom exceptions for OpenAgentMail SDK."""

from __future__ import annotations

from typing import Any, Optional


class OpenAgentMailError(Exception):
    """Base exception for OpenAgentMail SDK."""

    def __init__(self, message: str) -> None:
        self.message = message
        super().__init__(message)


class APIError(OpenAgentMailError):
    """Error returned by the OpenAgentMail API."""

    def __init__(
        self,
        message: str,
        status_code: int,
        code: str,
        details: Optional[dict[str, Any]] = None,
    ) -> None:
        self.status_code = status_code
        self.code = code
        self.details = details
        super().__init__(message)

    def __str__(self) -> str:
        return f"[{self.status_code}] {self.code}: {self.message}"


class AuthenticationError(APIError):
    """Invalid or missing API key."""

    def __init__(self, message: str = "Invalid or missing API key") -> None:
        super().__init__(message, 401, "authentication_error")


class AuthorizationError(APIError):
    """Insufficient permissions for this operation."""

    def __init__(self, message: str = "Insufficient permissions") -> None:
        super().__init__(message, 403, "authorization_error")


class NotFoundError(APIError):
    """Resource not found."""

    def __init__(self, message: str = "Resource not found") -> None:
        super().__init__(message, 404, "not_found")


class ConflictError(APIError):
    """Resource already exists (idempotency conflict)."""

    def __init__(self, message: str = "Resource already exists") -> None:
        super().__init__(message, 409, "conflict")


class ValidationError(APIError):
    """Request validation failed."""

    def __init__(
        self,
        message: str = "Validation error",
        details: Optional[dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, 422, "validation_error", details)


class RateLimitError(APIError):
    """Rate limit exceeded."""

    def __init__(
        self,
        message: str = "Rate limit exceeded",
        retry_after: Optional[int] = None,
    ) -> None:
        self.retry_after = retry_after
        super().__init__(message, 429, "rate_limit_exceeded")


class InternalServerError(APIError):
    """Server-side error."""

    def __init__(self, message: str = "Internal server error") -> None:
        super().__init__(message, 500, "internal_error")


class ConnectionError(OpenAgentMailError):
    """Network connection error."""

    pass


class TimeoutError(OpenAgentMailError):
    """Request timed out."""

    pass
