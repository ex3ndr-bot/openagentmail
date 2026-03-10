"""Pydantic models for OpenAgentMail API types."""

from __future__ import annotations

from datetime import datetime
from typing import Any, Optional

from pydantic import BaseModel, ConfigDict, Field


class Organization(BaseModel):
    """Organization details."""

    organization_id: str
    name: str
    plan: str
    created_at: datetime


class Pod(BaseModel):
    """Pod for multi-tenant isolation."""

    pod_id: str
    name: str
    client_id: Optional[str] = None
    created_at: datetime
    updated_at: datetime


class Inbox(BaseModel):
    """Email inbox."""

    inbox_id: str
    pod_id: str
    username: str
    domain: str
    email: str
    display_name: Optional[str] = None
    client_id: Optional[str] = None
    created_at: datetime
    updated_at: datetime


class Attachment(BaseModel):
    """Email attachment."""

    attachment_id: str
    filename: str
    content_type: str
    size: int  # bytes


class Message(BaseModel):
    """Email message."""

    model_config = ConfigDict(populate_by_name=True)

    message_id: str
    inbox_id: str
    thread_id: str
    from_: str = Field(alias="from")
    to: list[str]
    cc: list[str] = []
    bcc: list[str] = []
    subject: str
    text: Optional[str] = None
    html: Optional[str] = None
    attachments: list[Attachment] = []
    labels: list[str] = []
    headers: dict[str, str] = {}
    created_at: datetime


class Draft(BaseModel):
    """Email draft."""

    draft_id: str
    inbox_id: str
    to: list[str] = []
    cc: list[str] = []
    bcc: list[str] = []
    subject: Optional[str] = None
    text: Optional[str] = None
    html: Optional[str] = None
    attachments: list[Attachment] = []
    send_at: Optional[datetime] = None
    created_at: datetime
    updated_at: datetime


class DnsRecord(BaseModel):
    """DNS record for domain verification."""

    type: str  # TXT, CNAME, MX
    name: str
    value: str


class Domain(BaseModel):
    """Custom email domain."""

    domain_id: str
    domain: str
    pod_id: Optional[str] = None
    status: str  # pending, verifying, verified, failed
    dns_records: list[DnsRecord] = []
    created_at: datetime
    updated_at: datetime


class Webhook(BaseModel):
    """Webhook configuration."""

    webhook_id: str
    url: str
    event_types: list[str]
    inbox_ids: list[str] = []
    pod_ids: list[str] = []
    client_id: Optional[str] = None
    secret: str
    enabled: bool
    created_at: datetime
    updated_at: datetime


class WebhookEvent(BaseModel):
    """Webhook event payload."""

    id: str
    type: str
    created_at: datetime
    data: dict[str, Any]


class PaginatedResponse(BaseModel):
    """Paginated response wrapper."""

    items: list[Any]
    next_page_token: Optional[str] = None
    has_more: bool = False


class ErrorDetail(BaseModel):
    """Error detail information."""

    field: Optional[str] = None
    reason: Optional[str] = None


class APIError(BaseModel):
    """API error response."""

    code: str
    message: str
    details: Optional[ErrorDetail] = None
