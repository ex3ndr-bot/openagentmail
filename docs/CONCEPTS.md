# OpenAgentMail Concepts

Understanding the core concepts behind OpenAgentMail.

---

## Table of Contents

1. [Overview](#overview)
2. [Organizations](#organizations)
3. [Pods](#pods)
4. [Inboxes](#inboxes)
5. [Messages & Threads](#messages--threads)
6. [Drafts](#drafts)
7. [Webhooks](#webhooks)
8. [Domains](#domains)
9. [Labels](#labels)
10. [Attachments](#attachments)
11. [Idempotency](#idempotency)

---

## Overview

OpenAgentMail is built around a simple hierarchy:

```
Organization
└── Pod (isolation boundary)
    └── Inbox (email account)
        ├── Message (sent/received email)
        │   └── Attachment
        ├── Draft (unsent email)
        └── Thread (conversation)
```

---

## Organizations

An **Organization** is the top-level account in OpenAgentMail.

### What it represents
- A company, team, or individual user
- The billing and authentication boundary
- Container for all other resources

### Key properties
- **API Keys**: All keys belong to an organization
- **Billing**: Usage is tracked at the org level
- **Isolation**: Complete data separation between orgs

### Example

```json
{
  "organization_id": "org_abc123",
  "name": "Acme Corp",
  "plan": "pro",
  "created_at": "2024-01-15T10:30:00Z"
}
```

---

## Pods

A **Pod** is a logical isolation unit within an organization.

### What it represents
- A tenant in a multi-tenant application
- An environment (production, staging, development)
- A project or team boundary

### Use cases

| Use Case | Pod Structure |
|----------|---------------|
| Multi-tenant SaaS | One pod per customer |
| Environments | `production`, `staging`, `development` pods |
| Projects | One pod per project |
| Single-tenant | Use the default pod |

### Key properties
- Pods contain inboxes
- Pods can have their own domains
- Webhooks can filter by pod
- API keys can be scoped to a pod

### Example

```json
{
  "pod_id": "pod_xyz789",
  "name": "Customer: Acme Inc",
  "client_id": "tenant_acme",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Creating pods

```typescript
// Create a pod for each tenant
const pod = await client.pods.create({
  name: `Customer: ${tenantName}`,
  clientId: tenantId  // your internal identifier
});
```

---

## Inboxes

An **Inbox** is an API-first email account for your AI agent.

### What it represents
- A unique email address
- A container for messages
- The identity your agent uses to send/receive email

### Key properties

| Property | Description |
|----------|-------------|
| `username` | The local part of the email (before @) |
| `domain` | The domain (after @) |
| `email` | Full email address (`username@domain`) |
| `display_name` | Human-readable name shown in email clients |
| `client_id` | Your identifier for idempotency |

### Example

```json
{
  "inbox_id": "inb_abc123",
  "pod_id": "pod_default",
  "username": "support-agent",
  "domain": "openagentmail.com",
  "email": "support-agent@openagentmail.com",
  "display_name": "Customer Support",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Creating inboxes

```typescript
// Random username
const inbox1 = await client.inboxes.create({
  displayName: 'Sales Agent'
});
// Result: xk7f9m2p@openagentmail.com

// Specific username
const inbox2 = await client.inboxes.create({
  username: 'billing',
  displayName: 'Billing Department'
});
// Result: billing@openagentmail.com

// In a specific pod
const inbox3 = await client.inboxes.create({
  username: 'support',
  displayName: 'Support'
}, { podId: 'pod_tenant123' });
```

### One inbox per agent?

Common patterns:

| Pattern | Description |
|---------|-------------|
| **One inbox per agent** | Each AI agent has its own email address |
| **Shared inbox** | Multiple agents share one inbox, using labels to route |
| **Functional inboxes** | `support@`, `sales@`, `billing@` for different functions |

---

## Messages & Threads

A **Message** is a single email (sent or received).

A **Thread** is a conversation containing related messages.

### Message properties

| Property | Description |
|----------|-------------|
| `message_id` | Unique identifier |
| `thread_id` | Groups related messages |
| `from` | Sender email address |
| `to`, `cc`, `bcc` | Recipients |
| `subject` | Email subject line |
| `text` | Plain text body |
| `html` | HTML body |
| `attachments` | File attachments |
| `labels` | Custom labels for workflow tracking |
| `headers` | Email headers |

### Threading behavior

Messages are automatically threaded based on:
1. `In-Reply-To` header
2. `References` header
3. Subject line matching (fallback)

### Example

```json
{
  "message_id": "msg_def456",
  "thread_id": "thr_ghi789",
  "from": "customer@example.com",
  "to": ["support@openagentmail.com"],
  "subject": "Re: Help with my order",
  "text": "Thanks for the quick response!",
  "created_at": "2024-01-15T10:35:00Z"
}
```

### Working with threads

```typescript
// Get all messages in a thread
const thread = await client.messages.list(inboxId, {
  threadId: 'thr_ghi789'
});

// Reply (automatically threads)
await client.messages.reply(inboxId, messageId, {
  text: 'Happy to help!'
});
```

---

## Drafts

A **Draft** is an unsent email that can be edited before sending.

### Use cases
- Prepare emails for human review
- Schedule emails for later
- Build complex emails incrementally

### Key properties

| Property | Description |
|----------|-------------|
| `draft_id` | Unique identifier |
| `send_at` | Scheduled send time (ISO 8601) |
| `to`, `cc`, `bcc` | Recipients (can be empty) |
| `subject`, `text`, `html` | Content (can be partial) |

### Example workflow

```typescript
// 1. Create draft
const draft = await client.drafts.create(inboxId, {
  to: ['customer@example.com'],
  subject: 'Your weekly report'
});

// 2. Update with content
await client.drafts.update(inboxId, draft.draft_id, {
  text: 'Here is your weekly report...',
  attachments: [{ filename: 'report.pdf', content: reportPdf }]
});

// 3a. Send immediately
await client.drafts.send(inboxId, draft.draft_id);

// 3b. Or schedule for later
await client.drafts.update(inboxId, draft.draft_id, {
  sendAt: '2024-01-20T09:00:00Z'
});
```

---

## Webhooks

**Webhooks** deliver real-time notifications when email events occur.

### Event types

| Event | Description |
|-------|-------------|
| `message.received` | New inbound email |
| `message.sent` | Email successfully sent |
| `message.delivered` | Email delivered to recipient |
| `message.bounced` | Email bounced (hard or soft) |
| `message.complained` | Recipient marked as spam |
| `message.rejected` | Email rejected by server |

### Filtering

Webhooks can be filtered to specific:
- **Inboxes**: Only events from certain inboxes
- **Pods**: Only events from certain pods

### Reliability

Webhooks use:
- **Exponential backoff**: Retries at 1m, 5m, 30m, 2h, 8h
- **Signature verification**: HMAC-SHA256 signatures
- **Idempotency**: Use event ID to deduplicate

### Example

```typescript
const webhook = await client.webhooks.create({
  url: 'https://api.myapp.com/webhooks/email',
  eventTypes: ['message.received'],
  inboxIds: ['inb_abc123']  // optional filter
});
```

### Webhook payload

```json
{
  "id": "evt_xyz789",
  "type": "message.received",
  "created_at": "2024-01-15T10:30:00Z",
  "data": {
    "message_id": "msg_def456",
    "inbox_id": "inb_abc123",
    "from": "customer@example.com",
    "subject": "Help needed"
  }
}
```

---

## Domains

**Domains** let you send email from your own domain instead of openagentmail.com.

### Why use custom domains?
- **Branding**: `support@yourcompany.com` looks professional
- **Deliverability**: Build sender reputation on your domain
- **Trust**: Customers recognize your domain

### Setup process

1. **Add domain** via API
2. **Configure DNS** (SPF, DKIM, DMARC records)
3. **Verify** — system checks DNS automatically
4. **Use** in inbox creation

### DNS records required

| Record | Purpose |
|--------|---------|
| SPF (TXT) | Authorizes servers to send for your domain |
| DKIM (CNAME) | Signs emails cryptographically |
| DMARC (TXT) | Tells receivers how to handle failures |

### Example

```typescript
// Add domain
const domain = await client.domains.create({
  domain: 'mail.mycompany.com'
});

// DNS records are returned — configure these in your DNS provider
console.log(domain.dnsRecords);

// Check verification status
const status = await client.domains.get(domain.domain_id);
console.log(status.status);  // 'pending' | 'verifying' | 'verified' | 'failed'

// Once verified, create inboxes on your domain
const inbox = await client.inboxes.create({
  username: 'support',
  domain: 'mail.mycompany.com'
});
```

---

## Labels

**Labels** are custom tags for organizing and tracking emails.

### Use cases
- Track processing state: `unprocessed`, `in-progress`, `resolved`
- Categorize: `billing`, `technical`, `general`
- Route to handlers: `urgent`, `needs-human`
- Mark outcomes: `replied`, `escalated`, `spam`

### Applying labels

```typescript
// When sending
await client.messages.send(inboxId, {
  to: ['customer@example.com'],
  subject: 'Response',
  text: '...',
  labels: ['outbound', 'auto-reply']
});

// Filter by label
const urgentMessages = await client.messages.list(inboxId, {
  label: 'urgent'
});
```

### Best practices

- Use consistent naming: `snake_case` or `kebab-case`
- Keep labels domain-specific to your workflow
- Document your label taxonomy

---

## Attachments

**Attachments** are files sent with emails.

### Sending attachments

```typescript
await client.messages.send(inboxId, {
  to: ['customer@example.com'],
  subject: 'Your invoice',
  text: 'Please find your invoice attached.',
  attachments: [
    {
      filename: 'invoice.pdf',
      content: Buffer.from(pdfBytes).toString('base64'),
      contentType: 'application/pdf'
    }
  ]
});
```

### Receiving attachments

Attachments on received messages include metadata:

```json
{
  "attachment_id": "att_jkl012",
  "filename": "document.pdf",
  "content_type": "application/pdf",
  "size": 12345
}
```

To download attachment content, use the attachments endpoint (if implemented) or access via presigned URL.

### Limits

| Limit | Value |
|-------|-------|
| Max attachment size | 25 MB |
| Max total attachments | 100 MB |
| Max attachment count | 100 |

---

## Idempotency

**Idempotency** prevents duplicate resources when retrying requests.

### How it works

1. Include a `client_id` in your request
2. If the same `client_id` is sent again, the original resource is returned
3. No duplicate is created

### Example

```typescript
// First request — creates the inbox
const inbox = await client.inboxes.create({
  username: 'support',
  clientId: 'my-unique-id-12345'
});

// Retry (same client_id) — returns the same inbox, no duplicate
const sameInbox = await client.inboxes.create({
  username: 'support',
  clientId: 'my-unique-id-12345'
});

console.log(inbox.inbox_id === sameInbox.inbox_id);  // true
```

### Best practices

- Generate unique `client_id` values (UUIDs work well)
- Store `client_id` before making the request
- Use for any operation you might retry

### Resources supporting idempotency

- Inboxes
- Pods
- Webhooks
- Messages (optional, for deduplication)

---

## Summary

| Concept | Purpose |
|---------|---------|
| Organization | Top-level account, billing boundary |
| Pod | Tenant/environment isolation |
| Inbox | Email account for your agent |
| Message | Sent or received email |
| Thread | Conversation grouping |
| Draft | Unsent email for editing/scheduling |
| Webhook | Real-time event notification |
| Domain | Custom sending domain |
| Label | Custom tags for organization |
| Attachment | Files in emails |
| Idempotency | Duplicate prevention |

---

*Last updated: 2024-01*
