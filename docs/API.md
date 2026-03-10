# OpenAgentMail API Reference

> **Version**: v0  
> **Base URL**: `https://api.yourdomain.com/v0`  
> **Content-Type**: `application/json`

This is the canonical API specification for OpenAgentMail. All backend implementations and SDKs MUST conform to this spec.

---

## Table of Contents

1. [Authentication](#authentication)
2. [Pagination](#pagination)
3. [Error Handling](#error-handling)
4. [Rate Limiting](#rate-limiting)
5. [Endpoints](#endpoints)
   - [Organization](#organization)
   - [Pods](#pods)
   - [Inboxes](#inboxes)
   - [Messages](#messages)
   - [Drafts](#drafts)
   - [Webhooks](#webhooks)
   - [Domains](#domains)

---

## Authentication

All API requests require a Bearer token in the Authorization header:

```http
Authorization: Bearer <your_api_key>
```

API keys are scoped to an organization and can be managed via the dashboard or API.

### Key Types

| Type | Scope | Use Case |
|------|-------|----------|
| `org_key` | Full organization access | Server-side integrations |
| `pod_key` | Single pod access | Multi-tenant deployments |

---

## Pagination

All list endpoints return paginated results using cursor-based pagination.

### Request Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | integer | 20 | Number of items per page (max: 100) |
| `page_token` | string | - | Cursor for the next page |

### Response Format

```json
{
  "items": [...],
  "next_page_token": "eyJsYXN0X2lkIjoiYWJjMTIzIn0",
  "has_more": true
}
```

To fetch the next page, pass `next_page_token` as the `page_token` parameter.

---

## Error Handling

### HTTP Status Codes

| Code | Description |
|------|-------------|
| `200` | Success |
| `201` | Created |
| `204` | No Content (successful deletion) |
| `400` | Bad Request - Invalid request body or parameters |
| `401` | Unauthorized - Invalid or missing API key |
| `403` | Forbidden - Insufficient permissions |
| `404` | Not Found - Resource doesn't exist |
| `409` | Conflict - Resource already exists (idempotency) |
| `422` | Validation Error - Request validation failed |
| `429` | Rate Limited - Too many requests |
| `500` | Internal Server Error |

### Error Response Format

```json
{
  "error": {
    "code": "validation_error",
    "message": "Invalid email address format",
    "details": {
      "field": "to",
      "reason": "must be a valid email address"
    }
  }
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `invalid_request` | Malformed request body |
| `validation_error` | Field validation failed |
| `authentication_error` | Invalid API key |
| `authorization_error` | Insufficient permissions |
| `not_found` | Resource not found |
| `conflict` | Duplicate resource (client_id collision) |
| `rate_limit_exceeded` | Too many requests |
| `internal_error` | Server error |

---

## Rate Limiting

Rate limits are applied per API key:

| Plan | Requests/minute | Burst |
|------|-----------------|-------|
| Free | 60 | 10 |
| Pro | 600 | 100 |
| Enterprise | Custom | Custom |

Rate limit headers are included in every response:

```http
X-RateLimit-Limit: 600
X-RateLimit-Remaining: 599
X-RateLimit-Reset: 1609459200
```

---

## Endpoints

---

### Organization

#### Get Organization

Retrieve details about your organization.

```http
GET /v0/organization
```

**Response: `200 OK`**

```json
{
  "organization_id": "org_abc123",
  "name": "Acme Corp",
  "plan": "pro",
  "created_at": "2024-01-15T10:30:00Z"
}
```

---

### Pods

Pods provide multi-tenant isolation. Each pod has its own inboxes, domains, and data.

#### Create Pod

```http
POST /v0/pods
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Pod display name |
| `client_id` | string | No | Idempotency key |

```json
{
  "name": "Production",
  "client_id": "my-unique-id"
}
```

**Response: `201 Created`**

```json
{
  "pod_id": "pod_xyz789",
  "name": "Production",
  "client_id": "my-unique-id",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### List Pods

```http
GET /v0/pods
```

**Query Parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | integer | 20 | Items per page (max 100) |
| `page_token` | string | - | Pagination cursor |

**Response: `200 OK`**

```json
{
  "items": [
    {
      "pod_id": "pod_xyz789",
      "name": "Production",
      "client_id": "my-unique-id",
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    }
  ],
  "next_page_token": null,
  "has_more": false
}
```

#### Get Pod

```http
GET /v0/pods/{pod_id}
```

**Path Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `pod_id` | string | Pod identifier |

**Response: `200 OK`**

```json
{
  "pod_id": "pod_xyz789",
  "name": "Production",
  "client_id": "my-unique-id",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

---

### Inboxes

Inboxes are API-first email accounts for AI agents.

#### Create Inbox

Create an inbox at the organization level (default pod).

```http
POST /v0/inboxes
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | No | Email username (random if omitted) |
| `domain` | string | No | Email domain (default: openagentmail.com) |
| `display_name` | string | No | Friendly name for the inbox |
| `client_id` | string | No | Idempotency key |

```json
{
  "username": "support-agent",
  "domain": "openagentmail.com",
  "display_name": "Support Agent",
  "client_id": "inbox-001"
}
```

**Response: `201 Created`**

```json
{
  "inbox_id": "inb_abc123",
  "pod_id": "pod_default",
  "username": "support-agent",
  "domain": "openagentmail.com",
  "email": "support-agent@openagentmail.com",
  "display_name": "Support Agent",
  "client_id": "inbox-001",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### Create Inbox in Pod

```http
POST /v0/pods/{pod_id}/inboxes
```

Same request/response as above, but creates the inbox in the specified pod.

#### List Inboxes

```http
GET /v0/inboxes
```

**Query Parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | integer | 20 | Items per page (max 100) |
| `page_token` | string | - | Pagination cursor |

**Response: `200 OK`**

```json
{
  "items": [
    {
      "inbox_id": "inb_abc123",
      "pod_id": "pod_default",
      "username": "support-agent",
      "domain": "openagentmail.com",
      "email": "support-agent@openagentmail.com",
      "display_name": "Support Agent",
      "client_id": "inbox-001",
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    }
  ],
  "next_page_token": null,
  "has_more": false
}
```

#### List Inboxes in Pod

```http
GET /v0/pods/{pod_id}/inboxes
```

Same response format, filtered to the specified pod.

#### Get Inbox

```http
GET /v0/inboxes/{inbox_id}
```

**Response: `200 OK`**

Returns a single Inbox object.

#### Get Inbox in Pod

```http
GET /v0/pods/{pod_id}/inboxes/{inbox_id}
```

#### Delete Inbox

```http
DELETE /v0/inboxes/{inbox_id}
```

**Response: `204 No Content`**

⚠️ This permanently deletes the inbox and all associated messages.

---

### Messages

#### Send Message

```http
POST /v0/inboxes/{inbox_id}/messages
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `to` | string \| string[] | Yes | Recipient(s) |
| `cc` | string \| string[] | No | CC recipient(s) |
| `bcc` | string \| string[] | No | BCC recipient(s) |
| `subject` | string | Yes | Email subject |
| `text` | string | No* | Plain text body |
| `html` | string | No* | HTML body |
| `reply_to` | string | No | Reply-to address |
| `attachments` | Attachment[] | No | File attachments |
| `labels` | string[] | No | Labels for tracking |
| `headers` | object | No | Custom email headers |

*At least one of `text` or `html` is required.

**Attachment Object**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filename` | string | Yes | File name |
| `content` | string | Yes | Base64-encoded content |
| `content_type` | string | No | MIME type |

```json
{
  "to": ["user@example.com"],
  "cc": ["manager@example.com"],
  "subject": "Your order has shipped",
  "text": "Your order #12345 has shipped and will arrive in 3-5 days.",
  "html": "<p>Your order <strong>#12345</strong> has shipped!</p>",
  "attachments": [
    {
      "filename": "receipt.pdf",
      "content": "JVBERi0xLjQK...",
      "content_type": "application/pdf"
    }
  ],
  "labels": ["outbound", "transactional"]
}
```

**Response: `201 Created`**

```json
{
  "message_id": "msg_def456",
  "inbox_id": "inb_abc123",
  "thread_id": "thr_ghi789",
  "from": "support-agent@openagentmail.com",
  "to": ["user@example.com"],
  "cc": ["manager@example.com"],
  "bcc": [],
  "subject": "Your order has shipped",
  "text": "Your order #12345 has shipped...",
  "html": "<p>Your order <strong>#12345</strong> has shipped!</p>",
  "attachments": [
    {
      "attachment_id": "att_jkl012",
      "filename": "receipt.pdf",
      "content_type": "application/pdf",
      "size": 12345
    }
  ],
  "labels": ["outbound", "transactional"],
  "headers": {},
  "created_at": "2024-01-15T10:30:00Z"
}
```

#### List Messages

```http
GET /v0/inboxes/{inbox_id}/messages
```

**Query Parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | integer | 20 | Items per page (max 100) |
| `page_token` | string | - | Pagination cursor |
| `thread_id` | string | - | Filter by thread |
| `label` | string | - | Filter by label |

**Response: `200 OK`**

```json
{
  "items": [...],
  "next_page_token": "eyJsYXN0X2lkIjoibXNnXzEyMyJ9",
  "has_more": true
}
```

#### Get Message

```http
GET /v0/inboxes/{inbox_id}/messages/{message_id}
```

**Response: `200 OK`**

Returns a single Message object.

#### Delete Message

```http
DELETE /v0/inboxes/{inbox_id}/messages/{message_id}
```

**Response: `204 No Content`**

#### Reply to Message

```http
POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | No* | Plain text body |
| `html` | string | No* | HTML body |
| `attachments` | Attachment[] | No | File attachments |
| `labels` | string[] | No | Labels |

*At least one of `text` or `html` is required.

```json
{
  "text": "Thanks for reaching out! Let me look into this.",
  "labels": ["replied"]
}
```

**Response: `201 Created`**

Returns the new Message object with the same `thread_id` as the original.

#### Reply All

```http
POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply-all
```

Same as reply, but includes all original recipients.

---

### Drafts

Drafts are unsent messages that can be edited before sending.

#### Create Draft

```http
POST /v0/inboxes/{inbox_id}/drafts
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `to` | string \| string[] | No | Recipient(s) |
| `cc` | string \| string[] | No | CC recipient(s) |
| `bcc` | string \| string[] | No | BCC recipient(s) |
| `subject` | string | No | Email subject |
| `text` | string | No | Plain text body |
| `html` | string | No | HTML body |
| `reply_to` | string | No | Reply-to address |
| `attachments` | Attachment[] | No | File attachments |
| `send_at` | string | No | ISO 8601 scheduled send time |

```json
{
  "to": ["user@example.com"],
  "subject": "Draft: Weekly Report",
  "text": "Work in progress...",
  "send_at": "2024-01-20T09:00:00Z"
}
```

**Response: `201 Created`**

```json
{
  "draft_id": "drf_mno345",
  "inbox_id": "inb_abc123",
  "to": ["user@example.com"],
  "cc": [],
  "bcc": [],
  "subject": "Draft: Weekly Report",
  "text": "Work in progress...",
  "html": null,
  "attachments": [],
  "send_at": "2024-01-20T09:00:00Z",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### List Drafts

```http
GET /v0/inboxes/{inbox_id}/drafts
```

**Response: `200 OK`**

```json
{
  "items": [...],
  "next_page_token": null,
  "has_more": false
}
```

#### Get Draft

```http
GET /v0/inboxes/{inbox_id}/drafts/{draft_id}
```

**Response: `200 OK`**

#### Update Draft

```http
PUT /v0/inboxes/{inbox_id}/drafts/{draft_id}
```

Same request body as Create Draft. All fields are optional; only provided fields are updated.

**Response: `200 OK`**

Returns the updated Draft object.

#### Delete Draft

```http
DELETE /v0/inboxes/{inbox_id}/drafts/{draft_id}
```

**Response: `204 No Content`**

#### Send Draft

```http
POST /v0/inboxes/{inbox_id}/drafts/{draft_id}/send
```

**Request Body**: Empty or optional override fields.

**Response: `200 OK`**

Returns the sent Message object. The draft is deleted after sending.

---

### Webhooks

Webhooks enable real-time notifications for email events.

#### Event Types

| Event | Description |
|-------|-------------|
| `message.received` | New message received in inbox |
| `message.sent` | Message sent successfully |
| `message.delivered` | Message delivered to recipient |
| `message.bounced` | Message bounced (hard/soft) |
| `message.complained` | Recipient marked as spam |
| `message.rejected` | Message rejected by server |

#### Create Webhook

```http
POST /v0/webhooks
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | string | Yes | Webhook endpoint URL |
| `event_types` | string[] | Yes | Events to subscribe to |
| `inbox_ids` | string[] | No | Filter to specific inboxes |
| `pod_ids` | string[] | No | Filter to specific pods |
| `client_id` | string | No | Idempotency key |
| `secret` | string | No | Signing secret (auto-generated if omitted) |

```json
{
  "url": "https://api.myapp.com/webhooks/email",
  "event_types": ["message.received", "message.bounced"],
  "inbox_ids": ["inb_abc123"],
  "client_id": "webhook-001"
}
```

**Response: `201 Created`**

```json
{
  "webhook_id": "whk_pqr678",
  "url": "https://api.myapp.com/webhooks/email",
  "event_types": ["message.received", "message.bounced"],
  "inbox_ids": ["inb_abc123"],
  "pod_ids": [],
  "client_id": "webhook-001",
  "secret": "whsec_xxxxxxxxxxxxx",
  "enabled": true,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### List Webhooks

```http
GET /v0/webhooks
```

**Response: `200 OK`**

#### Get Webhook

```http
GET /v0/webhooks/{webhook_id}
```

**Response: `200 OK`**

#### Update Webhook

```http
PUT /v0/webhooks/{webhook_id}
```

**Request Body**: Same as Create. All fields optional.

**Response: `200 OK`**

#### Delete Webhook

```http
DELETE /v0/webhooks/{webhook_id}
```

**Response: `204 No Content`**

#### Webhook Payload Format

```json
{
  "id": "evt_stu901",
  "type": "message.received",
  "created_at": "2024-01-15T10:30:00Z",
  "data": {
    "message_id": "msg_def456",
    "inbox_id": "inb_abc123",
    "from": "customer@example.com",
    "to": ["support-agent@openagentmail.com"],
    "subject": "Help with my order",
    "received_at": "2024-01-15T10:30:00Z"
  }
}
```

#### Webhook Signature Verification

Webhooks include a signature header for verification:

```http
Svix-Id: evt_stu901
Svix-Timestamp: 1705314600
Svix-Signature: v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=
```

Verify using HMAC-SHA256:
```
payload = svix_id + "." + svix_timestamp + "." + body
signature = base64(hmac_sha256(webhook_secret, payload))
```

---

### Domains

Custom domains for sending email with your brand.

#### Add Domain

```http
POST /v0/domains
```

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | string | Yes | Domain name |
| `pod_id` | string | No | Associate with a pod |

```json
{
  "domain": "mail.mycompany.com"
}
```

**Response: `201 Created`**

```json
{
  "domain_id": "dom_vwx234",
  "domain": "mail.mycompany.com",
  "pod_id": null,
  "status": "pending",
  "dns_records": [
    {
      "type": "TXT",
      "name": "_dmarc.mail.mycompany.com",
      "value": "v=DMARC1; p=none; rua=mailto:dmarc@openagentmail.com"
    },
    {
      "type": "TXT",
      "name": "mail.mycompany.com",
      "value": "v=spf1 include:spf.openagentmail.com ~all"
    },
    {
      "type": "CNAME",
      "name": "oam._domainkey.mail.mycompany.com",
      "value": "oam._domainkey.openagentmail.com"
    }
  ],
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### List Domains

```http
GET /v0/domains
```

**Response: `200 OK`**

#### Get Domain

```http
GET /v0/domains/{domain_id}
```

**Response: `200 OK`**

Includes current `status`:
- `pending` - DNS records not yet verified
- `verifying` - Verification in progress
- `verified` - Domain ready for use
- `failed` - Verification failed

#### Delete Domain

```http
DELETE /v0/domains/{domain_id}
```

**Response: `204 No Content`**

---

## Data Types Reference

### Inbox

```typescript
interface Inbox {
  inbox_id: string;
  pod_id: string;
  username: string;
  domain: string;
  email: string;  // username@domain
  display_name: string | null;
  client_id: string | null;
  created_at: string;  // ISO 8601
  updated_at: string;  // ISO 8601
}
```

### Message

```typescript
interface Message {
  message_id: string;
  inbox_id: string;
  thread_id: string;
  from: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  text: string | null;
  html: string | null;
  attachments: Attachment[];
  labels: string[];
  headers: Record<string, string>;
  created_at: string;  // ISO 8601
}
```

### Attachment

```typescript
interface Attachment {
  attachment_id: string;
  filename: string;
  content_type: string;
  size: number;  // bytes
}
```

### Draft

```typescript
interface Draft {
  draft_id: string;
  inbox_id: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string | null;
  text: string | null;
  html: string | null;
  attachments: Attachment[];
  send_at: string | null;  // ISO 8601
  created_at: string;
  updated_at: string;
}
```

### Webhook

```typescript
interface Webhook {
  webhook_id: string;
  url: string;
  event_types: string[];
  inbox_ids: string[];
  pod_ids: string[];
  client_id: string | null;
  secret: string;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}
```

### Domain

```typescript
interface Domain {
  domain_id: string;
  domain: string;
  pod_id: string | null;
  status: "pending" | "verifying" | "verified" | "failed";
  dns_records: DnsRecord[];
  created_at: string;
  updated_at: string;
}

interface DnsRecord {
  type: "TXT" | "CNAME" | "MX";
  name: string;
  value: string;
}
```

### Pod

```typescript
interface Pod {
  pod_id: string;
  name: string;
  client_id: string | null;
  created_at: string;
  updated_at: string;
}
```

### WebhookEvent

```typescript
interface WebhookEvent {
  id: string;
  type: string;
  created_at: string;
  data: Record<string, any>;
}
```

---

## Idempotency

Use `client_id` to make requests idempotent. If you send the same `client_id` twice:
- For `POST` requests: Returns the original resource with `200 OK` (not `201`)
- Prevents duplicate inboxes, webhooks, etc.

```json
{
  "username": "support",
  "client_id": "unique-request-id-12345"
}
```

---

## IMAP & SMTP Access

Inboxes can also be accessed via standard email protocols:

**IMAP**
- Host: `imap.openagentmail.com`
- Port: `993` (SSL)
- Username: Full email address
- Password: Inbox API key

**SMTP**
- Host: `smtp.openagentmail.com`
- Port: `587` (TLS)
- Username: Full email address
- Password: Inbox API key

---

## SDK Examples

### TypeScript

```typescript
import { OpenAgentMail } from 'openagentmail';

const client = new OpenAgentMail({ apiKey: process.env.OAM_API_KEY });

// Create inbox
const inbox = await client.inboxes.create({
  username: 'support-agent',
  displayName: 'Support Agent'
});

// Send email
const message = await client.messages.send(inbox.inbox_id, {
  to: ['user@example.com'],
  subject: 'Hello!',
  text: 'This is a test email.'
});

// List messages
const messages = await client.messages.list(inbox.inbox_id, {
  limit: 10
});
```

### Python

```python
from openagentmail import OpenAgentMail

client = OpenAgentMail(api_key=os.environ["OAM_API_KEY"])

# Create inbox
inbox = client.inboxes.create(
    username="support-agent",
    display_name="Support Agent"
)

# Send email
message = client.messages.send(
    inbox_id=inbox.inbox_id,
    to=["user@example.com"],
    subject="Hello!",
    text="This is a test email."
)

# List messages
messages = client.messages.list(inbox_id=inbox.inbox_id, limit=10)
```

---

*Last updated: 2024-01*
