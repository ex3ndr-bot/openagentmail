"""Tests for OpenAgentMail SDK."""

import pytest
import respx
from httpx import Response

from openagentmail import (
    AsyncOpenAgentMail,
    OpenAgentMail,
    AuthenticationError,
    NotFoundError,
    ValidationError,
)


@pytest.fixture
def api_key() -> str:
    return "test_api_key"


@pytest.fixture
def base_url() -> str:
    return "https://api.openagentmail.com/v0"


class TestOpenAgentMail:
    """Tests for synchronous client."""

    @respx.mock
    def test_get_organization(self, api_key: str, base_url: str) -> None:
        """Test getting organization details."""
        respx.get(f"{base_url}/organization").mock(
            return_value=Response(
                200,
                json={
                    "organization_id": "org_123",
                    "name": "Test Org",
                    "plan": "pro",
                    "created_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        org = client.get_organization()

        assert org.organization_id == "org_123"
        assert org.name == "Test Org"
        assert org.plan == "pro"

    @respx.mock
    def test_create_inbox(self, api_key: str, base_url: str) -> None:
        """Test creating an inbox."""
        respx.post(f"{base_url}/inboxes").mock(
            return_value=Response(
                201,
                json={
                    "inbox_id": "inbox_456",
                    "pod_id": "pod_123",
                    "username": "support",
                    "domain": "example.com",
                    "email": "support@example.com",
                    "display_name": "Support",
                    "client_id": None,
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        inbox = client.inboxes.create(username="support", display_name="Support")

        assert inbox.inbox_id == "inbox_456"
        assert inbox.username == "support"
        assert inbox.email == "support@example.com"

    @respx.mock
    def test_send_message(self, api_key: str, base_url: str) -> None:
        """Test sending a message."""
        respx.post(f"{base_url}/inboxes/inbox_123/messages").mock(
            return_value=Response(
                201,
                json={
                    "message_id": "msg_789",
                    "inbox_id": "inbox_123",
                    "thread_id": "thread_001",
                    "from": "sender@example.com",
                    "to": ["recipient@example.com"],
                    "cc": [],
                    "bcc": [],
                    "subject": "Test",
                    "text": "Hello",
                    "html": None,
                    "attachments": [],
                    "labels": [],
                    "headers": {},
                    "created_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        message = client.messages.send(
            inbox_id="inbox_123",
            to="recipient@example.com",
            subject="Test",
            text="Hello",
        )

        assert message.message_id == "msg_789"
        assert message.subject == "Test"
        assert message.to == ["recipient@example.com"]

    @respx.mock
    def test_list_inboxes_pagination(self, api_key: str, base_url: str) -> None:
        """Test paginated inbox listing."""
        # First page
        respx.get(f"{base_url}/inboxes").mock(
            return_value=Response(
                200,
                json={
                    "items": [
                        {
                            "inbox_id": "inbox_1",
                            "pod_id": "pod_123",
                            "username": "inbox1",
                            "domain": "example.com",
                            "email": "inbox1@example.com",
                            "display_name": None,
                            "client_id": None,
                            "created_at": "2024-01-15T10:30:00Z",
                            "updated_at": "2024-01-15T10:30:00Z",
                        }
                    ],
                    "next_page_token": "page2",
                    "has_more": True,
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        paginator = client.inboxes.list(limit=1)
        
        # Get first item
        first = paginator.first()
        assert first is not None
        assert first.inbox_id == "inbox_1"

    @respx.mock
    def test_authentication_error(self, api_key: str, base_url: str) -> None:
        """Test authentication error handling."""
        respx.get(f"{base_url}/organization").mock(
            return_value=Response(
                401,
                json={
                    "error": {
                        "code": "authentication_error",
                        "message": "Invalid API key",
                    }
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        with pytest.raises(AuthenticationError):
            client.get_organization()

    @respx.mock
    def test_not_found_error(self, api_key: str, base_url: str) -> None:
        """Test not found error handling."""
        respx.get(f"{base_url}/inboxes/invalid").mock(
            return_value=Response(
                404,
                json={
                    "error": {
                        "code": "not_found",
                        "message": "Inbox not found",
                    }
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        with pytest.raises(NotFoundError):
            client.inboxes.get("invalid")

    @respx.mock
    def test_validation_error(self, api_key: str, base_url: str) -> None:
        """Test validation error handling."""
        respx.post(f"{base_url}/inboxes/inbox_123/messages").mock(
            return_value=Response(
                422,
                json={
                    "error": {
                        "code": "validation_error",
                        "message": "Invalid email format",
                        "details": {"field": "to", "reason": "invalid email"},
                    }
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        with pytest.raises(ValidationError) as exc_info:
            client.messages.send(
                inbox_id="inbox_123",
                to="invalid-email",
                subject="Test",
            )
        
        assert exc_info.value.details is not None
        assert exc_info.value.details["field"] == "to"


class TestAsyncOpenAgentMail:
    """Tests for asynchronous client."""

    @respx.mock
    @pytest.mark.asyncio
    async def test_async_get_organization(self, api_key: str, base_url: str) -> None:
        """Test async organization retrieval."""
        respx.get(f"{base_url}/organization").mock(
            return_value=Response(
                200,
                json={
                    "organization_id": "org_123",
                    "name": "Test Org",
                    "plan": "pro",
                    "created_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        async with AsyncOpenAgentMail(api_key=api_key) as client:
            org = await client.get_organization()

        assert org.organization_id == "org_123"
        assert org.name == "Test Org"

    @respx.mock
    @pytest.mark.asyncio
    async def test_async_create_inbox(self, api_key: str, base_url: str) -> None:
        """Test async inbox creation."""
        respx.post(f"{base_url}/inboxes").mock(
            return_value=Response(
                201,
                json={
                    "inbox_id": "inbox_456",
                    "pod_id": "pod_123",
                    "username": "async-support",
                    "domain": "example.com",
                    "email": "async-support@example.com",
                    "display_name": None,
                    "client_id": None,
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        async with AsyncOpenAgentMail(api_key=api_key) as client:
            inbox = await client.inboxes.create(username="async-support")

        assert inbox.inbox_id == "inbox_456"
        assert inbox.email == "async-support@example.com"

    @respx.mock
    @pytest.mark.asyncio
    async def test_async_send_message(self, api_key: str, base_url: str) -> None:
        """Test async message sending."""
        respx.post(f"{base_url}/inboxes/inbox_123/messages").mock(
            return_value=Response(
                201,
                json={
                    "message_id": "msg_async",
                    "inbox_id": "inbox_123",
                    "thread_id": "thread_001",
                    "from": "sender@example.com",
                    "to": ["recipient@example.com"],
                    "cc": [],
                    "bcc": [],
                    "subject": "Async Test",
                    "text": "Hello async",
                    "html": None,
                    "attachments": [],
                    "labels": [],
                    "headers": {},
                    "created_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        async with AsyncOpenAgentMail(api_key=api_key) as client:
            message = await client.messages.send(
                inbox_id="inbox_123",
                to="recipient@example.com",
                subject="Async Test",
                text="Hello async",
            )

        assert message.message_id == "msg_async"
        assert message.subject == "Async Test"


class TestWebhooks:
    """Tests for webhook operations."""

    @respx.mock
    def test_create_webhook(self, api_key: str, base_url: str) -> None:
        """Test creating a webhook."""
        respx.post(f"{base_url}/webhooks").mock(
            return_value=Response(
                201,
                json={
                    "webhook_id": "wh_123",
                    "url": "https://example.com/webhook",
                    "event_types": ["message.received"],
                    "inbox_ids": [],
                    "pod_ids": [],
                    "client_id": None,
                    "secret": "whsec_test123",
                    "enabled": True,
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        webhook = client.webhooks.create(
            url="https://example.com/webhook",
            event_types=["message.received"],
        )

        assert webhook.webhook_id == "wh_123"
        assert webhook.secret == "whsec_test123"
        assert webhook.enabled is True

    @respx.mock
    def test_rotate_webhook_secret(self, api_key: str, base_url: str) -> None:
        """Test rotating webhook secret."""
        respx.post(f"{base_url}/webhooks/wh_123/rotate-secret").mock(
            return_value=Response(
                200,
                json={
                    "webhook_id": "wh_123",
                    "url": "https://example.com/webhook",
                    "event_types": ["message.received"],
                    "inbox_ids": [],
                    "pod_ids": [],
                    "client_id": None,
                    "secret": "whsec_new_secret",
                    "enabled": True,
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T10:30:00Z",
                },
            )
        )

        client = OpenAgentMail(api_key=api_key)
        webhook = client.webhooks.rotate_secret("wh_123")

        assert webhook.secret == "whsec_new_secret"
