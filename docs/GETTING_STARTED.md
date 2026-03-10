# Getting Started with OpenAgentMail

Get your AI agent sending and receiving email in under 5 minutes.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Authentication](#authentication)
4. [Your First Inbox](#your-first-inbox)
5. [Sending Email](#sending-email)
6. [Receiving Email](#receiving-email)
7. [Webhooks](#webhooks)
8. [Next Steps](#next-steps)

---

## Prerequisites

- An OpenAgentMail account (self-hosted or cloud)
- An API key
- Node.js 18+ or Python 3.9+ (for SDKs)

---

## Installation

### TypeScript/JavaScript

```bash
npm install openagentmail
# or
yarn add openagentmail
# or
pnpm add openagentmail
```

### Python

```bash
pip install openagentmail
# or
poetry add openagentmail
```

### CLI

```bash
npm install -g openagentmail-cli
# or
brew install openagentmail-cli
```

---

## Authentication

### Get Your API Key

1. Log into your OpenAgentMail dashboard
2. Navigate to **Settings → API Keys**
3. Click **Create API Key**
4. Copy the key (it won't be shown again!)

### Configure the SDK

**TypeScript:**

```typescript
import { OpenAgentMail } from 'openagentmail';

const client = new OpenAgentMail({
  apiKey: process.env.OPENAGENTMAIL_API_KEY,
  // baseUrl: 'https://api.your-instance.com/v0' // for self-hosted
});
```

**Python:**

```python
from openagentmail import OpenAgentMail

client = OpenAgentMail(
    api_key=os.environ["OPENAGENTMAIL_API_KEY"],
    # base_url="https://api.your-instance.com/v0"  # for self-hosted
)
```

**Environment Variables:**

```bash
export OPENAGENTMAIL_API_KEY="oam_xxxxxxxxxxxx"
export OPENAGENTMAIL_BASE_URL="https://api.openagentmail.com/v0"  # optional
```

---

## Your First Inbox

Create an inbox for your agent to use:

### TypeScript

```typescript
const inbox = await client.inboxes.create({
  username: 'support-bot',
  displayName: 'Customer Support Agent'
});

console.log(`Created inbox: ${inbox.email}`);
// Created inbox: support-bot@openagentmail.com
```

### Python

```python
inbox = client.inboxes.create(
    username="support-bot",
    display_name="Customer Support Agent"
)

print(f"Created inbox: {inbox.email}")
# Created inbox: support-bot@openagentmail.com
```

### cURL

```bash
curl -X POST https://api.openagentmail.com/v0/inboxes \
  -H "Authorization: Bearer $OPENAGENTMAIL_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "support-bot",
    "display_name": "Customer Support Agent"
  }'
```

**Response:**

```json
{
  "inbox_id": "inb_abc123",
  "username": "support-bot",
  "domain": "openagentmail.com",
  "email": "support-bot@openagentmail.com",
  "display_name": "Customer Support Agent",
  "created_at": "2024-01-15T10:30:00Z"
}
```

---

## Sending Email

Send an email from your inbox:

### TypeScript

```typescript
const message = await client.messages.send(inbox.inbox_id, {
  to: ['customer@example.com'],
  subject: 'Your order has shipped!',
  text: 'Hi! Your order #12345 shipped today and will arrive in 3-5 days.',
  html: '<p>Hi! Your order <strong>#12345</strong> shipped today!</p>'
});

console.log(`Sent message: ${message.message_id}`);
```

### Python

```python
message = client.messages.send(
    inbox_id=inbox.inbox_id,
    to=["customer@example.com"],
    subject="Your order has shipped!",
    text="Hi! Your order #12345 shipped today and will arrive in 3-5 days.",
    html="<p>Hi! Your order <strong>#12345</strong> shipped today!</p>"
)

print(f"Sent message: {message.message_id}")
```

### cURL

```bash
curl -X POST "https://api.openagentmail.com/v0/inboxes/inb_abc123/messages" \
  -H "Authorization: Bearer $OPENAGENTMAIL_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "to": ["customer@example.com"],
    "subject": "Your order has shipped!",
    "text": "Hi! Your order #12345 shipped today.",
    "html": "<p>Hi! Your order <strong>#12345</strong> shipped!</p>"
  }'
```

### With Attachments

```typescript
const message = await client.messages.send(inbox.inbox_id, {
  to: ['customer@example.com'],
  subject: 'Invoice attached',
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

---

## Receiving Email

### Polling for New Messages

```typescript
// List recent messages
const messages = await client.messages.list(inbox.inbox_id, {
  limit: 10
});

for (const msg of messages.items) {
  console.log(`From: ${msg.from}`);
  console.log(`Subject: ${msg.subject}`);
  console.log(`Body: ${msg.text}`);
}
```

### Replying to Messages

```typescript
// Reply to a specific message
const reply = await client.messages.reply(inbox.inbox_id, message.message_id, {
  text: 'Thanks for reaching out! Let me look into this for you.'
});
```

### Filtering by Thread

```typescript
// Get all messages in a conversation
const thread = await client.messages.list(inbox.inbox_id, {
  thread_id: message.thread_id
});
```

---

## Webhooks

Get real-time notifications when emails arrive.

### Create a Webhook

```typescript
const webhook = await client.webhooks.create({
  url: 'https://api.myapp.com/webhooks/email',
  eventTypes: ['message.received', 'message.bounced'],
  inboxIds: [inbox.inbox_id]  // optional: filter to specific inboxes
});

console.log(`Webhook secret: ${webhook.secret}`);
// Store this secret for signature verification!
```

### Webhook Payload

When an email arrives, you'll receive:

```json
{
  "id": "evt_xyz789",
  "type": "message.received",
  "created_at": "2024-01-15T10:30:00Z",
  "data": {
    "message_id": "msg_def456",
    "inbox_id": "inb_abc123",
    "thread_id": "thr_ghi789",
    "from": "customer@example.com",
    "to": ["support-bot@openagentmail.com"],
    "subject": "Help with my order",
    "text": "I haven't received my package yet...",
    "received_at": "2024-01-15T10:30:00Z"
  }
}
```

### Verify Webhook Signature

Always verify webhook signatures to ensure authenticity:

**TypeScript:**

```typescript
import crypto from 'crypto';

function verifyWebhook(payload: string, headers: Record<string, string>, secret: string): boolean {
  const svixId = headers['svix-id'];
  const svixTimestamp = headers['svix-timestamp'];
  const svixSignature = headers['svix-signature'];
  
  const signedContent = `${svixId}.${svixTimestamp}.${payload}`;
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(signedContent)
    .digest('base64');
  
  return svixSignature.includes(`v1,${expectedSignature}`);
}
```

**Python:**

```python
import hmac
import hashlib
import base64

def verify_webhook(payload: str, headers: dict, secret: str) -> bool:
    svix_id = headers["svix-id"]
    svix_timestamp = headers["svix-timestamp"]
    svix_signature = headers["svix-signature"]
    
    signed_content = f"{svix_id}.{svix_timestamp}.{payload}"
    expected_signature = base64.b64encode(
        hmac.new(secret.encode(), signed_content.encode(), hashlib.sha256).digest()
    ).decode()
    
    return f"v1,{expected_signature}" in svix_signature
```

### Handle the Webhook

```typescript
// Express.js example
app.post('/webhooks/email', express.raw({ type: 'application/json' }), (req, res) => {
  const payload = req.body.toString();
  
  if (!verifyWebhook(payload, req.headers, process.env.WEBHOOK_SECRET)) {
    return res.status(401).send('Invalid signature');
  }
  
  const event = JSON.parse(payload);
  
  switch (event.type) {
    case 'message.received':
      // Process incoming email
      handleIncomingEmail(event.data);
      break;
    case 'message.bounced':
      // Handle bounce
      handleBounce(event.data);
      break;
  }
  
  res.status(200).send('OK');
});
```

---

## Complete Example: Customer Support Agent

Here's a full example of an AI agent that handles customer support emails:

```typescript
import { OpenAgentMail } from 'openagentmail';
import OpenAI from 'openai';

const mail = new OpenAgentMail({ apiKey: process.env.OAM_API_KEY });
const openai = new OpenAI();

// Create inbox on startup
const inbox = await mail.inboxes.create({
  username: 'support',
  displayName: 'AI Support Agent'
});

// Set up webhook handler
app.post('/webhooks/email', async (req, res) => {
  const event = JSON.parse(req.body);
  
  if (event.type === 'message.received') {
    const { message_id, from, subject, text } = event.data;
    
    // Generate AI response
    const completion = await openai.chat.completions.create({
      model: 'gpt-4',
      messages: [
        { role: 'system', content: 'You are a helpful customer support agent.' },
        { role: 'user', content: `Customer email:\nSubject: ${subject}\n\n${text}` }
      ]
    });
    
    const reply = completion.choices[0].message.content;
    
    // Send reply
    await mail.messages.reply(inbox.inbox_id, message_id, {
      text: reply
    });
  }
  
  res.status(200).send('OK');
});
```

---

## Next Steps

Now that you're up and running:

1. **[API Reference](./API.md)** - Complete endpoint documentation
2. **[Concepts](./CONCEPTS.md)** - Understand pods, threads, and more
3. **[Architecture](./ARCHITECTURE.md)** - System design for self-hosting
4. **Custom Domains** - Send from your own domain
5. **Semantic Search** - Search emails by meaning

### Best Practices

- ✅ Always verify webhook signatures
- ✅ Use idempotency keys (`client_id`) for critical operations
- ✅ Store webhook secrets securely
- ✅ Handle rate limits with exponential backoff
- ✅ Use labels to track email state in your workflows
- ✅ Set up bounce handling to maintain sender reputation

### Getting Help

- **Documentation**: docs.openagentmail.com
- **GitHub**: github.com/openagentmail
- **Discord**: discord.gg/openagentmail

---

*Last updated: 2024-01*
