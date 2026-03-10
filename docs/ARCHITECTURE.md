# OpenAgentMail Architecture

This document describes the system architecture for OpenAgentMail, an open-source email API designed for AI agents.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Data Flow](#data-flow)
4. [Technology Stack](#technology-stack)
5. [Database Schema](#database-schema)
6. [Deployment Architecture](#deployment-architecture)
7. [Security Model](#security-model)
8. [Scaling Considerations](#scaling-considerations)

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
│ Email Sending │    │Email Receiving│    │  Webhook      │
│   (SMTP Out)  │    │  (MX/IMAP)    │    │  Delivery     │
└───────────────┘    └───────────────┘    └───────────────┘
```

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
Handles outbound email delivery via SMTP.

**Responsibilities:**
- Message queuing
- SMTP connection pooling
- Delivery retry logic
- Bounce handling
- SPF/DKIM signing

**Technology:** Node.js with Nodemailer or custom SMTP client

#### 3. Email Receiving Service
Handles inbound email via MX records.

**Responsibilities:**
- SMTP server for receiving
- Message parsing (MIME)
- Attachment extraction
- Spam/virus scanning
- Routing to correct inbox

**Technology:** Haraka, Postal, or custom SMTP server

#### 4. Webhook Service
Delivers real-time event notifications.

**Responsibilities:**
- Event queuing
- Webhook delivery with retry
- Signature generation
- Failure tracking

**Technology:** Node.js with Redis/BullMQ for queuing

#### 5. IMAP/SMTP Bridge (Optional)
Provides traditional email client access.

**Responsibilities:**
- IMAP server for mailbox access
- SMTP server for sending via clients
- Authentication against API keys

**Technology:** Dovecot for IMAP, custom auth plugin

### Supporting Components

#### 6. Database
Primary data store for all entities.

**Responsibilities:**
- Entity storage (inboxes, messages, etc.)
- Full-text search
- Transactional consistency

**Technology:** PostgreSQL with pgvector for semantic search

#### 7. Object Storage
Stores email attachments and large content.

**Responsibilities:**
- Attachment storage
- HTML email content
- Signed URL generation

**Technology:** S3, MinIO, or Cloudflare R2

#### 8. Message Queue
Async task processing.

**Responsibilities:**
- Email send queue
- Webhook delivery queue
- Retry management

**Technology:** Redis with BullMQ or RabbitMQ

#### 9. Cache
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
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  Client  │────▶│   API    │────▶│  Queue   │────▶│  SMTP    │
│          │     │  Server  │     │ (Send)   │     │ Sender   │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                      │                                  │
                      ▼                                  ▼
                 ┌──────────┐                      ┌──────────┐
                 │ Database │                      │ External │
                 │ (Draft)  │                      │   MTA    │
                 └──────────┘                      └──────────┘
                                                        │
                                                        ▼
                                                   ┌──────────┐
                                                   │ Webhook  │◀─ Bounce/Delivery
                                                   │ Service  │   notifications
                                                   └──────────┘
```

**Flow:**
1. Client calls `POST /v0/inboxes/{inbox_id}/messages`
2. API validates request, creates message record in DB
3. Message enqueued for sending
4. SMTP sender dequeues, signs (DKIM), sends via MX
5. Delivery status updates message record
6. Webhook fired for `message.sent` / `message.delivered` / `message.bounced`

### Receiving an Email

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ External │────▶│   MX     │────▶│  Parser  │────▶│ Database │
│  Sender  │     │ Receiver │     │ Service  │     │          │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                      │                │                 │
                      ▼                ▼                 │
                 ┌──────────┐    ┌──────────┐           │
                 │  Spam/   │    │  Object  │           │
                 │  Virus   │    │ Storage  │           │
                 └──────────┘    └──────────┘           │
                                                        ▼
                                                   ┌──────────┐
                                                   │ Webhook  │
                                                   │ Service  │
                                                   └──────────┘
                                                        │
                                                        ▼
                                                   ┌──────────┐
                                                   │  Client  │
                                                   │ Endpoint │
                                                   └──────────┘
```

**Flow:**
1. External server connects to MX receiver
2. SMTP handshake, recipient validation
3. Message received, parsed (headers, body, attachments)
4. Spam/virus scanning
5. Attachments stored in object storage
6. Message record created in database
7. Webhook fired for `message.received`

### Webhook Delivery

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  Event   │────▶│  Queue   │────▶│ Delivery │────▶│  Client  │
│ Trigger  │     │          │     │  Worker  │     │ Endpoint │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                                       │
                                       ▼
                                  ┌──────────┐
                                  │  Retry   │
                                  │  Logic   │
                                  └──────────┘
```

**Retry Strategy:**
- Exponential backoff: 1m, 5m, 30m, 2h, 8h
- Max 5 retries
- Failed webhooks logged for debugging

---

## Technology Stack

### Recommended Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| API Server | Node.js + TypeScript + Hono | Fast, type-safe, edge-compatible |
| Database | PostgreSQL 15+ | Reliable, full-text search, pgvector |
| Queue | Redis + BullMQ | Simple, battle-tested, good DX |
| Object Storage | S3/R2/MinIO | Commodity, cheap at scale |
| SMTP Inbound | Haraka | Node.js native, pluggable |
| SMTP Outbound | Nodemailer | Standard, well-maintained |
| IMAP | Dovecot | Industry standard |
| Caching | Redis | Multi-purpose (cache + queue) |
| Search | PostgreSQL FTS + pgvector | No extra infra, semantic search |

### Alternative Options

| Component | Alternative | Trade-off |
|-----------|-------------|-----------|
| API Server | Go + Chi | Faster, more memory efficient, less ecosystem |
| Database | CockroachDB | Distributed, higher complexity |
| Queue | RabbitMQ | More features, more ops overhead |
| SMTP Inbound | Postal | Full-featured, Ruby-based |
| Search | Elasticsearch | More powerful, significant ops burden |

---

## Database Schema

### Core Tables

```sql
-- Organizations (top-level accounts)
CREATE TABLE organizations (
    organization_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    plan TEXT NOT NULL DEFAULT 'free',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- API Keys
CREATE TABLE api_keys (
    key_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(organization_id),
    key_hash TEXT NOT NULL,  -- bcrypt hash of the key
    name TEXT,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Pods (multi-tenant isolation)
CREATE TABLE pods (
    pod_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(organization_id),
    name TEXT NOT NULL,
    client_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(organization_id, client_id)
);

-- Inboxes
CREATE TABLE inboxes (
    inbox_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pod_id UUID NOT NULL REFERENCES pods(pod_id),
    username TEXT NOT NULL,
    domain TEXT NOT NULL,
    display_name TEXT,
    client_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(username, domain),
    UNIQUE(pod_id, client_id)
);

-- Threads (email conversations)
CREATE TABLE threads (
    thread_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES inboxes(inbox_id) ON DELETE CASCADE,
    subject TEXT,
    last_message_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    message_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Messages
CREATE TABLE messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES inboxes(inbox_id) ON DELETE CASCADE,
    thread_id UUID NOT NULL REFERENCES threads(thread_id) ON DELETE CASCADE,
    direction TEXT NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    from_address TEXT NOT NULL,
    to_addresses TEXT[] NOT NULL,
    cc_addresses TEXT[] DEFAULT '{}',
    bcc_addresses TEXT[] DEFAULT '{}',
    subject TEXT,
    text_body TEXT,
    html_body TEXT,
    headers JSONB DEFAULT '{}',
    labels TEXT[] DEFAULT '{}',
    raw_message_id TEXT,  -- Original Message-ID header
    in_reply_to TEXT,     -- Message-ID being replied to
    references TEXT[],    -- Thread reference chain
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Full-text search index
CREATE INDEX messages_fts_idx ON messages 
    USING GIN (to_tsvector('english', coalesce(subject, '') || ' ' || coalesce(text_body, '')));

-- Attachments
CREATE TABLE attachments (
    attachment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    storage_key TEXT NOT NULL,  -- S3/R2 key
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Drafts
CREATE TABLE drafts (
    draft_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES inboxes(inbox_id) ON DELETE CASCADE,
    to_addresses TEXT[] DEFAULT '{}',
    cc_addresses TEXT[] DEFAULT '{}',
    bcc_addresses TEXT[] DEFAULT '{}',
    subject TEXT,
    text_body TEXT,
    html_body TEXT,
    send_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Webhooks
CREATE TABLE webhooks (
    webhook_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(organization_id),
    url TEXT NOT NULL,
    event_types TEXT[] NOT NULL,
    inbox_ids UUID[] DEFAULT '{}',
    pod_ids UUID[] DEFAULT '{}',
    client_id TEXT,
    secret TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(organization_id, client_id)
);

-- Domains
CREATE TABLE domains (
    domain_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(organization_id),
    pod_id UUID REFERENCES pods(pod_id),
    domain TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'pending',
    dns_records JSONB NOT NULL DEFAULT '[]',
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Indexes for common queries
CREATE INDEX inboxes_pod_id_idx ON inboxes(pod_id);
CREATE INDEX messages_inbox_id_created_idx ON messages(inbox_id, created_at DESC);
CREATE INDEX messages_thread_id_idx ON messages(thread_id);
CREATE INDEX webhooks_org_id_idx ON webhooks(organization_id);
```

---

## Deployment Architecture

### Single-Node (Development/Small Scale)

```
┌────────────────────────────────────────────┐
│              Single Server                  │
│  ┌──────────────────────────────────────┐  │
│  │           Docker Compose              │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │  │
│  │  │   API   │ │  SMTP   │ │  SMTP   │ │  │
│  │  │ Server  │ │  Send   │ │ Receive │ │  │
│  │  └─────────┘ └─────────┘ └─────────┘ │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │  │
│  │  │Postgres │ │  Redis  │ │  MinIO  │ │  │
│  │  └─────────┘ └─────────┘ └─────────┘ │  │
│  └──────────────────────────────────────┘  │
└────────────────────────────────────────────┘
```

### Production (Kubernetes)

```
                    ┌─────────────────┐
                    │   Load Balancer │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│  API Server   │   │  API Server   │   │  API Server   │
│  (Replica 1)  │   │  (Replica 2)  │   │  (Replica N)  │
└───────────────┘   └───────────────┘   └───────────────┘
        │                    │                    │
        └────────────────────┼────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│   PostgreSQL  │   │     Redis     │   │   S3 / R2     │
│   (Primary)   │   │   (Cluster)   │   │               │
└───────────────┘   └───────────────┘   └───────────────┘

┌─────────────────────────────────────────────────────────┐
│                    Worker Pods                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│  │ SMTP Sender │  │   Webhook   │  │   Email     │      │
│  │   Workers   │  │   Workers   │  │   Parser    │      │
│  └─────────────┘  └─────────────┘  └─────────────┘      │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                    MX Receivers                          │
│  ┌─────────────┐  ┌─────────────┐  (behind dedicated    │
│  │ SMTP Server │  │ SMTP Server │   IP with PTR record) │
│  └─────────────┘  └─────────────┘                       │
└─────────────────────────────────────────────────────────┘
```

### DNS Configuration

```
; MX record for receiving
@           MX    10  mx1.openagentmail.com.
@           MX    20  mx2.openagentmail.com.

; SPF for sending
@           TXT   "v=spf1 include:spf.openagentmail.com ~all"

; DKIM selector
oam._domainkey  CNAME  oam._domainkey.openagentmail.com.

; DMARC
_dmarc      TXT   "v=DMARC1; p=none; rua=mailto:dmarc@openagentmail.com"
```

---

## Security Model

### Authentication Layers

1. **API Keys** - Bearer tokens for API access
2. **Webhook Signatures** - HMAC-SHA256 for webhook verification
3. **IMAP/SMTP Auth** - Per-inbox credentials

### Data Isolation

- **Organization Level** - Complete isolation between orgs
- **Pod Level** - Logical isolation within org
- **Inbox Level** - Per-inbox access control

### Encryption

| Data | At Rest | In Transit |
|------|---------|------------|
| API Keys | bcrypt hashed | TLS 1.3 |
| Messages | AES-256 (optional) | TLS 1.3 |
| Attachments | Server-side encryption | TLS 1.3 |
| Webhooks | - | TLS 1.3 + HMAC |

### Email Security

- **SPF** - Sender Policy Framework validation
- **DKIM** - DomainKeys Identified Mail signing
- **DMARC** - Domain-based Message Authentication
- **TLS** - Opportunistic TLS for SMTP

### Rate Limiting

| Endpoint | Limit |
|----------|-------|
| API (authenticated) | 600/min |
| SMTP outbound | 100/hour per inbox |
| Webhook retries | 5 attempts max |

---

## Scaling Considerations

### Bottlenecks & Solutions

| Bottleneck | Solution |
|------------|----------|
| API requests | Horizontal scaling + caching |
| Database writes | Write-ahead logging, connection pooling |
| Email parsing | Worker pool with autoscaling |
| Attachment storage | CDN + pre-signed URLs |
| Webhook delivery | Queue partitioning |

### Capacity Planning

| Scale | API Servers | DB | Redis | Workers |
|-------|-------------|----|----|---------|
| 1K inboxes | 2 | 1 primary | 1 | 2 |
| 10K inboxes | 4 | 1 primary + read replica | 3 node cluster | 4 |
| 100K inboxes | 8+ | Primary + 2 replicas | 6 node cluster | 8+ |

### Performance Targets

| Metric | Target |
|--------|--------|
| API p99 latency | < 200ms |
| Email send time | < 5s (queued to sent) |
| Webhook delivery | < 30s from event |
| Message ingestion | < 2s from receipt |

---

## Monitoring & Observability

### Key Metrics

- API request rate, latency, error rate
- Email send/receive rates
- Queue depths (send, webhook)
- Bounce/complaint rates
- Database connection pool utilization

### Recommended Stack

- **Metrics**: Prometheus + Grafana
- **Logging**: Structured JSON logs → Loki or CloudWatch
- **Tracing**: OpenTelemetry → Jaeger or Honeycomb
- **Alerts**: Grafana Alerting or PagerDuty

---

*Last updated: 2024-01*
