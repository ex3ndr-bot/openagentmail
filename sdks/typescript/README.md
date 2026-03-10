# OpenAgentMail TypeScript SDK

Official TypeScript/JavaScript SDK for [OpenAgentMail](https://openagentmail.com) - the open-source email API for AI agents.

## Installation

```bash
npm install openagentmail
# or
yarn add openagentmail
# or
pnpm add openagentmail
```

## Quick Start

```typescript
import { OpenAgentMail } from 'openagentmail';

const client = new OpenAgentMail({ apiKey: process.env.OAM_API_KEY });

// Create an inbox
const inbox = await client.inboxes.create({
  username: 'support',
  domain: 'example.com',
  displayName: 'Support Team'
});

// Send a message
const message = await client.messages.send(inbox.inboxId, {
  to: ['user@example.com'],
  subject: 'Hello!',
  text: 'Hello from OpenAgentMail!'
});

// List messages with auto-pagination
for await (const msg of client.messages.list(inbox.inboxId)) {
  console.log(msg.subject);
}
```

## Features

- **Full TypeScript support** with types for all requests and responses
- **Fluent API** design: `client.inboxes.create()`, `client.messages.send()`
- **Auto-pagination** for list endpoints using async iterators
- **Idempotency support** via `clientId` parameter
- **Custom error classes** for precise error handling
- **ESM and CommonJS** builds included

## API Reference

### Client Configuration

```typescript
const client = new OpenAgentMail({
  apiKey: 'your_api_key',         // Required
  baseUrl: 'https://...',          // Optional, custom base URL
  timeout: 30000,                  // Optional, request timeout in ms
  fetch: customFetch,              // Optional, custom fetch implementation
});
```

### Inboxes

```typescript
// Create inbox
const inbox = await client.inboxes.create({
  username: 'support',
  domain: 'example.com',
  displayName: 'Support Team',
  clientId: 'unique-id'  // Idempotency key
});

// Get inbox
const inbox = await client.inboxes.get('inbox_abc123');

// Update inbox
const inbox = await client.inboxes.update('inbox_abc123', {
  displayName: 'New Name'
});

// Delete inbox
await client.inboxes.delete('inbox_abc123');

// List inboxes (paginated)
for await (const inbox of client.inboxes.list()) {
  console.log(inbox.email);
}

// Get first page only
const { items, hasMore, nextPageToken } = await client.inboxes.listPage({ limit: 10 });
```

### Messages

```typescript
// Send message
const message = await client.messages.send('inbox_abc123', {
  to: ['user@example.com'],
  cc: ['cc@example.com'],
  bcc: ['bcc@example.com'],
  subject: 'Hello!',
  text: 'Plain text body',
  html: '<p>HTML body</p>',
  threadId: 'thread_xyz',  // Reply to thread
  clientId: 'unique-id'    // Idempotency key
});

// Get message
const message = await client.messages.get('inbox_abc123', 'msg_xyz789');

// Update labels
const message = await client.messages.update('inbox_abc123', 'msg_xyz789', {
  addLabels: ['important'],
  removeLabels: ['unread']
});

// Delete message
await client.messages.delete('inbox_abc123', 'msg_xyz789');

// List messages
for await (const msg of client.messages.list('inbox_abc123', { label: 'unread' })) {
  console.log(msg.subject);
}
```

### Drafts

```typescript
// Create draft
const draft = await client.drafts.create('inbox_abc123', {
  to: ['user@example.com'],
  subject: 'Draft subject',
  text: 'Draft content',
  sendAt: '2024-12-31T23:59:59Z'  // Schedule send
});

// Update draft
const draft = await client.drafts.update('inbox_abc123', 'draft_xyz', {
  subject: 'Updated subject'
});

// Send draft immediately
const message = await client.drafts.send('inbox_abc123', 'draft_xyz');

// Delete draft
await client.drafts.delete('inbox_abc123', 'draft_xyz');

// List drafts
for await (const draft of client.drafts.list('inbox_abc123')) {
  console.log(draft.subject);
}
```

### Webhooks

```typescript
// Create webhook
const webhook = await client.webhooks.create({
  url: 'https://example.com/webhook',
  eventTypes: ['message.received', 'message.sent'],
  inboxIds: ['inbox_abc123'],  // Optional filter
});
console.log(webhook.secret);  // Use this to verify webhook signatures

// Update webhook
const webhook = await client.webhooks.update('wh_xyz', {
  enabled: false
});

// Rotate secret
const webhook = await client.webhooks.rotateSecret('wh_xyz');

// Delete webhook
await client.webhooks.delete('wh_xyz');

// List webhooks
for await (const wh of client.webhooks.list()) {
  console.log(wh.url);
}
```

### Domains

```typescript
// Add domain
const domain = await client.domains.add({
  domain: 'example.com',
  podId: 'pod_xyz'  // Optional
});

// Get DNS records to configure
for (const record of domain.dnsRecords) {
  console.log(`${record.type} ${record.name} -> ${record.value}`);
}

// Verify domain
const domain = await client.domains.verify('dom_abc123');
console.log(domain.status);  // 'verifying' -> 'verified' or 'failed'

// Delete domain
await client.domains.delete('dom_abc123');

// List domains
for await (const d of client.domains.list({ podId: 'pod_xyz' })) {
  console.log(`${d.domain}: ${d.status}`);
}
```

### Pods (Multi-tenant)

```typescript
// Create pod
const pod = await client.pods.create({
  name: 'Production',
  clientId: 'prod-pod'
});

// Update pod
const pod = await client.pods.update('pod_xyz', {
  name: 'New Name'
});

// Delete pod
await client.pods.delete('pod_xyz');

// List pods
for await (const pod of client.pods.list()) {
  console.log(pod.name);
}
```

### Organization

```typescript
const org = await client.getOrganization();
console.log(org.name, org.plan);
```

## Pagination

All list methods return async iterators for automatic pagination:

```typescript
// Auto-paginate through all items
for await (const inbox of client.inboxes.list()) {
  console.log(inbox.email);
}

// Collect all items to array
const allInboxes = await client.inboxes.list().toArray();

// Take first N items
const firstTen = await client.inboxes.list().take(10);

// Find first matching item
const found = await client.inboxes.list().find(i => i.username === 'support');

// Get single page with pagination info
const { items, hasMore, nextPageToken } = await client.inboxes.listPage({
  limit: 20,
  pageToken: previousToken
});
```

## Error Handling

The SDK provides specific error classes for different scenarios:

```typescript
import { 
  OpenAgentMail,
  NotFoundError,
  AuthenticationError,
  RateLimitError,
  ValidationError
} from 'openagentmail';

try {
  await client.inboxes.get('nonexistent');
} catch (error) {
  if (error instanceof NotFoundError) {
    console.log('Inbox not found');
  } else if (error instanceof AuthenticationError) {
    console.log('Invalid API key');
  } else if (error instanceof RateLimitError) {
    console.log(`Rate limited. Retry after ${error.retryAfter}s`);
  } else if (error instanceof ValidationError) {
    console.log('Validation failed:', error.details);
  } else {
    throw error;
  }
}
```

### Error Classes

| Class | Status | Description |
|-------|--------|-------------|
| `BadRequestError` | 400 | Invalid request body or parameters |
| `AuthenticationError` | 401 | Invalid or missing API key |
| `AuthorizationError` | 403 | Insufficient permissions |
| `NotFoundError` | 404 | Resource doesn't exist |
| `ConflictError` | 409 | Duplicate resource (idempotency) |
| `ValidationError` | 422 | Request validation failed |
| `RateLimitError` | 429 | Too many requests |
| `InternalServerError` | 500+ | Server error |
| `TimeoutError` | - | Request timed out |
| `NetworkError` | - | Network connectivity error |

## Idempotency

Use `clientId` to make requests idempotent:

```typescript
// Safe to retry - will return same inbox
const inbox = await client.inboxes.create({
  username: 'support',
  clientId: 'unique-request-id'
});
```

## License

MIT
