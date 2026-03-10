# AgentMail.to Research Summary

## Overview
AgentMail is an email inbox API designed specifically for AI agents. It provides programmatic email inbox creation, two-way communication (send and receive), and usage-based pricing that scales with agent throughput.

## Core Concepts

### Pods
- Multi-tenant isolation units
- Each pod contains its own inboxes, domains, and data
- Used for isolating different tenants/environments

### Inboxes
- Each inbox is an API-first email account for agents
- Can have custom usernames and domains
- Properties: inbox_id, pod_id, username, domain, display_name, client_id, created_at, updated_at

### Messages
- Full email messages with headers, body (text/html), and attachments
- Support for threads and replies
- Labels for tracking email state/workflow

### Drafts
- Draft messages that can be edited before sending
- Support for scheduled sending (send_at)

### Webhooks
- Real-time event notifications
- Events: message.received, message.sent, message.delivered, message.bounced, message.complained, message.rejected
- Can be filtered by inbox_ids or pod_ids
- Signature verification with Svix

### Domains
- Custom domain support
- SPF, DKIM, DMARC authentication
- DNS record management

## API Structure

Base URL: https://api.agentmail.to/v0

### Authentication
- Bearer token in Authorization header
- API keys managed per organization/pod

### Endpoints (v0)

#### Inboxes
- POST /v0/inboxes - Create inbox
- GET /v0/inboxes - List inboxes
- GET /v0/inboxes/{inbox_id} - Get inbox
- DELETE /v0/inboxes/{inbox_id} - Delete inbox

#### Inboxes (Pod-scoped)
- POST /v0/pods/{pod_id}/inboxes - Create inbox in pod
- GET /v0/pods/{pod_id}/inboxes - List inboxes in pod
- GET /v0/pods/{pod_id}/inboxes/{inbox_id} - Get inbox in pod

#### Messages
- POST /v0/inboxes/{inbox_id}/messages - Send message
- GET /v0/inboxes/{inbox_id}/messages - List messages
- GET /v0/inboxes/{inbox_id}/messages/{message_id} - Get message
- DELETE /v0/inboxes/{inbox_id}/messages/{message_id} - Delete message
- POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply - Reply to message
- POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply-all - Reply all

#### Drafts
- POST /v0/inboxes/{inbox_id}/drafts - Create draft
- GET /v0/inboxes/{inbox_id}/drafts - List drafts
- GET /v0/inboxes/{inbox_id}/drafts/{draft_id} - Get draft
- PUT /v0/inboxes/{inbox_id}/drafts/{draft_id} - Update draft
- DELETE /v0/inboxes/{inbox_id}/drafts/{draft_id} - Delete draft
- POST /v0/inboxes/{inbox_id}/drafts/{draft_id}/send - Send draft

#### Webhooks
- POST /v0/webhooks - Create webhook
- GET /v0/webhooks - List webhooks
- GET /v0/webhooks/{webhook_id} - Get webhook
- PUT /v0/webhooks/{webhook_id} - Update webhook
- DELETE /v0/webhooks/{webhook_id} - Delete webhook

#### Pods
- POST /v0/pods - Create pod
- GET /v0/pods - List pods
- GET /v0/pods/{pod_id} - Get pod

#### Domains
- POST /v0/domains - Add domain
- GET /v0/domains - List domains
- GET /v0/domains/{domain_id} - Get domain
- DELETE /v0/domains/{domain_id} - Remove domain

#### Organizations
- GET /v0/organization - Get organization

### Request/Response Formats

#### CreateInboxRequest
```json
{
  "username": "string (optional, random if not specified)",
  "domain": "string (optional, defaults to agentmail.to)",
  "display_name": "string (optional)",
  "client_id": "string (optional, for idempotency)"
}
```

#### Inbox
```json
{
  "inbox_id": "string",
  "pod_id": "string",
  "username": "string",
  "domain": "string",
  "email": "username@domain",
  "display_name": "string",
  "client_id": "string",
  "created_at": "ISO 8601 timestamp",
  "updated_at": "ISO 8601 timestamp"
}
```

#### SendMessageRequest
```json
{
  "to": "string | string[]",
  "cc": "string | string[] (optional)",
  "bcc": "string | string[] (optional)",
  "subject": "string",
  "text": "string (optional)",
  "html": "string (optional)",
  "reply_to": "string (optional)",
  "attachments": [{"filename": "string", "content": "base64"}],
  "labels": ["string"],
  "headers": {"key": "value"}
}
```

#### Message
```json
{
  "message_id": "string",
  "inbox_id": "string",
  "thread_id": "string",
  "from": "string",
  "to": ["string"],
  "cc": ["string"],
  "bcc": ["string"],
  "subject": "string",
  "text": "string",
  "html": "string",
  "attachments": [...],
  "labels": ["string"],
  "headers": {...},
  "created_at": "ISO 8601 timestamp"
}
```

#### WebhookRequest
```json
{
  "url": "string",
  "event_types": ["message.received", "message.sent", ...],
  "inbox_ids": ["string"] (optional),
  "pod_ids": ["string"] (optional),
  "client_id": "string (optional)"
}
```

## Features

1. **Instant Inbox Creation** - Spin up new inboxes in milliseconds
2. **Threads + Replies** - Full conversation threading support
3. **Attachments** - Send and receive file attachments
4. **Real-time Events** - Webhooks for instant notifications
5. **Custom Domains** - Use your own domain for sending
6. **SDKs** - TypeScript and Python SDKs available
7. **MCP Integration** - Model Context Protocol support
8. **CLI** - Command-line interface available
9. **Semantic Search** - Search emails by meaning
10. **Data Extraction** - Extract structured data from emails
11. **Spam & Virus Detection** - Automatic scanning of incoming emails
12. **Labels** - Track email state in agent workflows
13. **Idempotent Requests** - Use client_id to prevent duplicates
14. **IMAP & SMTP** - Access inboxes via standard protocols

## Pagination
All list endpoints support pagination with:
- `limit` - Number of items per page
- `page_token` - Token for next page

## Error Handling
Standard HTTP status codes:
- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 404: Not Found
- 422: Validation Error
- 429: Rate Limited
- 500: Internal Server Error

Error response format:
```json
{
  "error": {
    "code": "string",
    "message": "string"
  }
}
```
