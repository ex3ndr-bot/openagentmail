# OpenAgentMail Architecture

This document describes the system architecture for OpenAgentMail, an open-source email API designed for AI agents.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Email Infrastructure (Amazon SES)](#email-infrastructure-amazon-ses)
3. [Why Amazon SES?](#why-amazon-ses)
4. [Component Architecture](#component-architecture)
5. [Data Flow](#data-flow)
6. [Technology Stack](#technology-stack)
7. [Database Schema](#database-schema)
8. [Deployment Architecture](#deployment-architecture)
9. [Security Model](#security-model)
10. [Scaling Considerations](#scaling-considerations)

---

## System Overview

OpenAgentMail provides a complete email infrastructure as an API, enabling AI agents to create inboxes, send/receive email, and react to email events in real-time.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         AI Agent Applications                        │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        OpenAgentMail API                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Inboxes   │  │  Messages   │  │   Drafts    │  │  Webhooks   │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│  Amazon SES   │    │  Amazon SES   │    │  Webhook      │
│   (Outbound)  │    │   (Inbound)   │    │  Delivery     │
└───────────────┘    └───────────────┘    └───────────────┘
```

---

## Email Infrastructure (Amazon SES)

OpenAgentMail uses **Amazon Simple Email Service (SES)** as the backbone for both inbound and outbound email. This provides enterprise-grade email delivery and receiving at scale with minimal operational overhead.

### DNS Configuration

For domains using OpenAgentMail, the following DNS records are required:

```
; MX record - Route inbound email to SES
@           MX    10  inbound-smtp.us-east-1.amazonaws.com.

; SPF record - Authorize SES to send on behalf of domain
@           TXT   "v=spf1 include:amazonses.com -all"

; DKIM records - Added automatically via SES domain verification
; (3 CNAME records provided by SES)

; DMARC - Policy for email authentication
_dmarc      TXT   "v=DMARC1; p=quarantine; rua=mailto:dmarc@yourdomain.com"
```

### Inbound Email Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Incoming   │     │  Amazon     │     │  Amazon     │     │  Lambda /   │
│   Email     │────▶│    SES      │────▶│    SNS      │────▶│  Webhook    │
│             │     │  (Receipt)  │     │  (Topic)    │     │  Endpoint   │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                                   │
                                                                   ▼
                                                           ┌─────────────┐
                                                           │ OpenAgentMail│
                                                           │   Backend   │
                                                           └─────────────┘
```

**Step-by-step flow:**

1. **External sender** sends email to `user@yourdomain.com`
2. **DNS MX lookup** resolves to `inbound-smtp.us-east-1.amazonaws.com`
3. **Amazon SES** receives the email and applies receipt rules
4. **SES receipt rule** publishes the email content to an **SNS topic**
5. **SNS** delivers the message to an HTTP endpoint (Lambda or webhook URL)
6. **OpenAgentMail backend** processes, parses, and stores the message
7. **Webhook** notifies the AI agent of the new message (if configured)

### Outbound Email Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ OpenAgentMail│     │  Amazon     │     │  Recipient  │     │  Recipient  │
│   Backend   │────▶│    SES      │────▶│    MX       │────▶│   Mailbox   │
│             │     │  (SMTP)     │     │  Server     │     │             │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

**Outbound uses SES SMTP or API:**
- Emails are signed with DKIM automatically
- SPF passes because SES IPs are authorized via `include:amazonses.com`
- Bounce and complaint notifications come back via SNS

### SES Region

By default, OpenAgentMail uses **us-east-1** for SES. This can be configured per deployment. Ensure MX records match the region:

| Region | MX Record |
|--------|-----------|
| us-east-1 | `inbound-smtp.us-east-1.amazonaws.com` |
| us-west-2 | `inbound-smtp.us-west-2.amazonaws.com` |
| eu-west-1 | `inbound-smtp.eu-west-1.amazonaws.com` |

---

## Why Amazon SES?

When building email infrastructure for AI agents at scale, Amazon SES is the only practical choice. Here's why:

### The Competition

| Provider | Inbound | Outbound | Programmatic Routing | Cost (per 1K emails) | At Scale |
|----------|---------|----------|---------------------|----------------------|----------|
| **Amazon SES** | ✅ Full MX support | ✅ SMTP + API | ✅ SNS → Lambda | $0.10 | ✅ Millions |
| Google Workspace | ⚠️ Limited API | ✅ Gmail API | ❌ No receipt rules | $6/user/month | ❌ Per-seat |
| Azure Comm Services | ⚠️ Basic | ✅ API | ❌ No routing | $0.25 | ⚠️ Limited |
| Mailgun | ⚠️ Webhook parsing | ✅ API | ⚠️ Routes only | $0.80 | ⚠️ Expensive |
| SendGrid | ⚠️ Inbound Parse | ✅ API | ⚠️ Basic | $0.50 | ⚠️ Limited |
| Postmark | ⚠️ Inbound webhook | ✅ API | ❌ No | $1.25 | ❌ Transactional |

### Why Each Alternative Falls Short

**Google (Gmail/Workspace)**
- Gmail API has severe rate limits (250 quota units/second)
- No programmatic inbound routing—emails go to a mailbox, not a webhook
- Per-user pricing ($6-18/user/month) makes it cost-prohibitive at scale
- No MX-level control; you're locked into Google's infrastructure
- Designed for humans, not programmatic access

**Microsoft Azure Communication Services**
- Email is a secondary feature, not the core product
- Inbound email support is minimal—no receipt rules, no SNS equivalent
- Limited routing options; no way to programmatically process incoming mail
- Higher cost ($0.25/1K) and less mature than SES

**Mailgun**
- Decent outbound, but inbound is webhook-only (no full MX control)
- "Routes" are basic pattern matching, not programmable logic
- Costs 8x more than SES ($0.80/1K vs $0.10/1K)
- Less control over deliverability and IP reputation

**SendGrid (Twilio)**
- Inbound Parse is a webhook that receives parsed emails—limited flexibility
- No native integration with cloud functions or queuing
- You're parsing emails, not controlling the MX layer
- $0.50/1K is 5x the cost of SES

**Postmark**
- Excellent for transactional email, but that's all it does
- Inbound is basic webhook parsing
- No receipt rules, no SNS, no Lambda integration
- $1.25/1K and primarily designed for one-off transactional messages

### Why SES Wins

1. **True MX-Level Control**: SES is an actual MX server. You point your DNS at it, and it handles everything—SPF, DKIM, receiving, routing.

2. **Programmable Routing**: Receipt rules + SNS + Lambda means you can run arbitrary code on every incoming email. Route to different backends, filter spam, extract attachments—all before it hits your database.

3. **Cost at Scale**: $0.10 per 1,000 emails (both send and receive). No per-mailbox fees. An AI agent platform with 100,000 inboxes sending 10 emails/day each costs ~$3,000/month with SES. With Google Workspace at $6/user, that's $600,000/month.

4. **Native AWS Integration**: IAM for permissions, CloudWatch for monitoring, S3 for attachment storage, Lambda for processing. No glue code needed.

5. **Deliverability**: SES has excellent deliverability out of the box. Shared IPs are well-maintained, or you can use dedicated IPs for full control.

6. **No Vendor Lock-in on Logic**: Your email processing logic runs in Lambda or your own servers. SES is just the transport layer—you own the code.

### The Bottom Line

For an email platform serving AI agents:
- You need **full inbound control** (MX + routing) → Only SES has this
- You need **programmatic processing** (not just parsing) → SNS + Lambda
- You need **scale without per-seat costs** → $0.10/1K vs $6/user
- You need **enterprise deliverability** → SES reputation management

Amazon SES is not just the best option—it's the only option that makes technical and economic sense at scale.

---

## Component Architecture

### Core Components

#### 1. API Server
The main REST API handling all client requests.

**Responsibilities:**
- Request authentication & authorization
- Input validation
- Business logic orchestration
- Response formatting
- Rate limiting

**Technology:** Node.js/TypeScript with Express or Hono

#### 2. Email Sending Service
Handles outbound email delivery via Amazon SES SMTP/API.

**Responsibilities:**
- Message queuing
- SES API/SMTP integration
- Delivery tracking
- Bounce/complaint handling via SNS
- DKIM signing (automatic via SES)

**Technology:** Node.js with AWS SDK or Nodemailer (SES transport)

#### 3. Email Receiving Service
Handles inbound email via SES receipt rules.

**Responsibilities:**
- SNS notification processing
- Message parsing (MIME)
- Attachment extraction to S3
- Spam filtering (SES built-in)
- Routing to correct inbox

**Technology:** Lambda function or HTTP webhook endpoint

#### 4. Webhook Service
Delivers real-time event notifications to AI agents.

**Responsibilities:**
- Event queuing
- Webhook delivery with retry
- Signature generation
- Failure tracking

**Technology:** Node.js with Redis/BullMQ for queuing

### Supporting Components

#### 5. Database
Primary data store for all entities.

**Responsibilities:**
- Entity storage (inboxes, messages, etc.)
- Full-text search
- Transactional consistency

**Technology:** PostgreSQL with pgvector for semantic search

#### 6. Object Storage
Stores email attachments and large content.

**Responsibilities:**
- Attachment storage
- HTML email content
- Signed URL generation

**Technology:** Amazon S3 or Cloudflare R2

#### 7. Message Queue
Async task processing.

**Responsibilities:**
- Webhook delivery queue
- Retry management
- SNS message buffering

**Technology:** Redis with BullMQ or SQS

#### 8. Cache
Performance optimization.

**Responsibilities:**
- API response caching
- Session/rate limit data
- Hot path optimization

**Technology:** Redis

---

## Data Flow

### Sending an Email

```
┌────────────┐     ┌────────────┐     ┌────────────┐     ┌────────────┐
│  AI Agent  │     │  API       │     │  Queue     │     │  SES       │
│  (Client)  │────▶│  Server    │────▶│  (BullMQ)  │────▶│  SMTP/API  │
└────────────┘     └────────────┘     └────────────┘     └────────────┘
      │                  │                   │                  │
      │  POST /send      │  Validate &       │  Pick up job     │  Send via
      │                  │  enqueue          │                  │  SES
      ▼                  ▼                   ▼                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Agent calls POST /messages/send with recipient, subject, body   │
│  2. API validates, creates message record, enqueues send job        │
│  3. Worker picks up job, calls SES API or SMTP                      │
│  4. SES handles DKIM signing, delivery, bounces                     │
│  5. SNS notifications update message status (delivered/bounced)     │
└─────────────────────────────────────────────────────────────────────┘
```

### Receiving an Email

```
┌────────────┐     ┌────────────┐     ┌────────────┐     ┌────────────┐
│  External  │     │  Amazon    │     │  SNS       │     │  Backend   │
│  Sender    │────▶│  SES       │────▶│  Topic     │────▶│  Webhook   │
└────────────┘     └────────────┘     └────────────┘     └────────────┘
      │                  │                   │                  │
      │  SMTP to MX      │  Receipt rule     │  HTTP POST       │  Process &
      │                  │  matches          │                  │  store
      ▼                  ▼                   ▼                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Sender's mail server looks up MX → inbound-smtp.us-east-1...   │
│  2. SES receives email, applies receipt rules                       │
│  3. Receipt rule publishes to SNS topic (raw email or notification) │
│  4. SNS POSTs to backend webhook endpoint                           │
│  5. Backend parses MIME, extracts attachments to S3, stores message │
│  6. Backend triggers webhook to AI agent if configured              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Technology Stack

| Layer | Technology |
|-------|------------|
| API Framework | Node.js + Hono/Express |
| Database | PostgreSQL |
| Cache/Queue | Redis + BullMQ |
| Email Transport | Amazon SES |
| Inbound Processing | AWS Lambda or webhook |
| Event Bus | Amazon SNS |
| Object Storage | Amazon S3 |
| Authentication | JWT + API Keys |

---

## Database Schema

```sql
-- Organizations
CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Inboxes
CREATE TABLE inboxes (
    id UUID PRIMARY KEY,
    organization_id UUID REFERENCES organizations(id),
    email_address TEXT UNIQUE NOT NULL,
    display_name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Messages
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    inbox_id UUID REFERENCES inboxes(id),
    message_id TEXT,  -- RFC 5322 Message-ID
    from_address TEXT NOT NULL,
    to_addresses TEXT[] NOT NULL,
    cc_addresses TEXT[],
    subject TEXT,
    body_text TEXT,
    body_html TEXT,
    direction TEXT CHECK (direction IN ('inbound', 'outbound')),
    status TEXT DEFAULT 'received',
    ses_message_id TEXT,  -- SES tracking ID
    received_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Attachments
CREATE TABLE attachments (
    id UUID PRIMARY KEY,
    message_id UUID REFERENCES messages(id),
    filename TEXT NOT NULL,
    content_type TEXT,
    size_bytes INTEGER,
    s3_key TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Webhooks
CREATE TABLE webhooks (
    id UUID PRIMARY KEY,
    inbox_id UUID REFERENCES inboxes(id),
    url TEXT NOT NULL,
    secret TEXT NOT NULL,
    events TEXT[] DEFAULT ARRAY['message.received'],
    active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## Deployment Architecture

### AWS Deployment (Recommended)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Route 53 / CloudFlare                       │
│                    (DNS: MX → SES, A → ALB)                         │
└─────────────────────────────────────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
┌───────────────┐          ┌───────────────┐          ┌───────────────┐
│  Application  │          │  Amazon SES   │          │  Amazon SNS   │
│  Load Balancer│          │  (Email I/O)  │          │  (Events)     │
└───────────────┘          └───────────────┘          └───────────────┘
        │                           │                           │
        ▼                           │                           ▼
┌───────────────┐                   │                  ┌───────────────┐
│   ECS/EKS     │◀──────────────────┴─────────────────▶│  Lambda       │
│  (API + Workers)│                                    │  (Inbound)    │
└───────────────┘                                      └───────────────┘
        │                    │                    │
        └────────────────────┼────────────────────┘
                             ▼
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│   RDS         │   │  ElastiCache  │   │   S3          │
│  (PostgreSQL) │   │   (Redis)     │   │ (Attachments) │
└───────────────┘   └───────────────┘   └───────────────┘
```

### DNS Configuration (SES-based)

```
; MX record for receiving (points to SES)
@           MX    10  inbound-smtp.us-east-1.amazonaws.com.

; SPF for sending (authorizes SES)
@           TXT   "v=spf1 include:amazonses.com -all"

; DKIM (CNAME records from SES console)
abcdef._domainkey  CNAME  abcdef.dkim.amazonses.com.
ghijkl._domainkey  CNAME  ghijkl.dkim.amazonses.com.
mnopqr._domainkey  CNAME  mnopqr.dkim.amazonses.com.

; DMARC
_dmarc      TXT   "v=DMARC1; p=quarantine; rua=mailto:dmarc@yourdomain.com"
```

---

## Security Model

### Authentication Layers

1. **API Keys** - Bearer tokens for API access
2. **Webhook Signatures** - HMAC-SHA256 for webhook verification
3. **IAM Roles** - AWS IAM for SES/S3/SNS access

### Data Isolation

- **Organization Level** - Complete isolation between orgs
- **Inbox Level** - Per-inbox access control
- **IAM Policies** - Least-privilege access to AWS resources

### Encryption

| Data | At Rest | In Transit |
|------|---------|------------|
| API Keys | bcrypt hashed | TLS 1.3 |
| Messages | RDS encryption | TLS 1.3 |
| Attachments | S3 SSE | TLS 1.3 |
| SES Traffic | - | TLS (STARTTLS) |

### Email Security

- **SPF** - `include:amazonses.com` authorizes SES IPs
- **DKIM** - Automatic signing by SES
- **DMARC** - Policy enforcement for domain
- **TLS** - SES uses opportunistic TLS for delivery

### Rate Limiting

| Endpoint | Limit |
|----------|-------|
| API (authenticated) | 600/min |
| SES sending | Per-account quota (starts at 200/day in sandbox) |
| Webhook retries | 5 attempts max |

---

## Scaling Considerations

### SES Limits

| Limit | Sandbox | Production |
|-------|---------|------------|
| Daily send quota | 200 emails | 50,000+ (request increase) |
| Max send rate | 1 email/second | 14+ emails/second |
| Inbound email size | 30 MB | 30 MB |
| Receipt rules | 200 per rule set | 200 per rule set |

### Capacity Planning

| Scale | API Servers | DB | Redis | SES Quota |
|-------|-------------|----|----|-----------|
| 1K inboxes | 2 | 1 primary | 1 | 50K/day |
| 10K inboxes | 4 | 1 primary + read replica | 3 node | 500K/day |
| 100K inboxes | 8+ | Primary + 2 replicas | 6 node | 5M/day |

### Performance Targets

| Metric | Target |
|--------|--------|
| API p99 latency | < 200ms |
| Email send time | < 5s (queued to SES accepted) |
| Inbound processing | < 2s from SNS to stored |
| Webhook delivery | < 30s from event |

---

## Monitoring & Observability

### Key Metrics

- API request rate, latency, error rate
- SES send/delivery/bounce rates
- SNS message throughput
- Queue depths (webhook delivery)
- SES reputation metrics (bounce %, complaint %)

### SES-Specific Monitoring

- **CloudWatch Metrics**: Sends, deliveries, bounces, complaints
- **SNS Notifications**: Bounce/complaint feedback loop
- **SES Reputation Dashboard**: Account health score

### Recommended Stack

- **Metrics**: CloudWatch + Prometheus + Grafana
- **Logging**: Structured JSON logs → CloudWatch Logs
- **Tracing**: AWS X-Ray or OpenTelemetry
- **Alerts**: CloudWatch Alarms → SNS → PagerDuty

---

*Last updated: 2026-03*
