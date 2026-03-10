# OpenAgentMail Backend

Email API backend for AI agents. Built with Fastify, Prisma, and PostgreSQL.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start all services
docker-compose up -d

# The API will be available at http://localhost:3000/v0
# MailHog UI at http://localhost:8025
```

### Local Development

```bash
# Install dependencies
npm install

# Set up environment
cp .env.example .env
# Edit .env with your settings

# Generate Prisma client
npx prisma generate

# Push schema to database
npx prisma db push

# Start development server
npm run dev
```

## API Endpoints

Base URL: `http://localhost:3000/v0`

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /health | Health check (no auth) |
| GET | /organization | Get organization details |
| POST | /pods | Create a pod |
| GET | /pods | List pods |
| GET | /pods/:id | Get a pod |
| DELETE | /pods/:id | Delete a pod |
| POST | /inboxes | Create an inbox |
| GET | /inboxes | List inboxes |
| GET | /inboxes/:id | Get an inbox |
| PATCH | /inboxes/:id | Update an inbox |
| DELETE | /inboxes/:id | Delete an inbox |
| POST | /inboxes/:id/messages | Send a message |
| GET | /inboxes/:id/messages | List messages |
| GET | /inboxes/:id/messages/:id | Get a message |
| DELETE | /inboxes/:id/messages/:id | Delete a message |
| POST | /inboxes/:id/drafts | Create a draft |
| GET | /inboxes/:id/drafts | List drafts |
| GET | /inboxes/:id/drafts/:id | Get a draft |
| PATCH | /inboxes/:id/drafts/:id | Update a draft |
| DELETE | /inboxes/:id/drafts/:id | Delete a draft |
| POST | /inboxes/:id/drafts/:id/send | Send a draft |
| POST | /webhooks | Create a webhook |
| GET | /webhooks | List webhooks |
| GET | /webhooks/:id | Get a webhook |
| PATCH | /webhooks/:id | Update a webhook |
| DELETE | /webhooks/:id | Delete a webhook |
| POST | /domains | Create a domain |
| GET | /domains | List domains |
| GET | /domains/:id | Get a domain |
| POST | /domains/:id/verify | Verify a domain |
| DELETE | /domains/:id | Delete a domain |

## Authentication

All API requests require a Bearer token:

```bash
curl -H "Authorization: Bearer <your_api_key>" \
  http://localhost:3000/v0/organization
```

## Setting Up Initial Data

After starting the server, you'll need to create an organization and API key to use the API:

```bash
# Connect to database
docker-compose exec db psql -U postgres -d openagentmail

# Create an organization
INSERT INTO organizations (id, name, plan) VALUES 
  ('org_demo', 'Demo Organization', 'pro');

# Create an API key (use a secure random key in production)
INSERT INTO api_keys (id, key, key_hash, key_type, organization_id) VALUES 
  ('key_demo', 'oam_demo_key_12345', 'demo_hash', 'org_key', 'org_demo');
```

Then use `oam_demo_key_12345` as your API key.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| PORT | Server port | 3000 |
| HOST | Server host | 0.0.0.0 |
| NODE_ENV | Environment | development |
| DATABASE_URL | PostgreSQL connection URL | - |
| SMTP_HOST | SMTP server host | localhost |
| SMTP_PORT | SMTP server port | 587 |
| SMTP_SECURE | Use TLS | false |
| SMTP_USER | SMTP username | - |
| SMTP_PASS | SMTP password | - |
| DEFAULT_DOMAIN | Default inbox domain | mail.openagentmail.com |

## Project Structure

```
backend/
├── src/
│   ├── index.ts          # Entry point
│   ├── config.ts         # Configuration
│   ├── routes/           # API routes
│   │   ├── organization.ts
│   │   ├── pods.ts
│   │   ├── inboxes.ts
│   │   ├── messages.ts
│   │   ├── drafts.ts
│   │   ├── webhooks.ts
│   │   └── domains.ts
│   ├── services/         # Business logic
│   │   ├── email.ts
│   │   ├── webhook.ts
│   │   └── pagination.ts
│   ├── middleware/       # Request processing
│   │   ├── auth.ts
│   │   └── validation.ts
│   └── types/            # TypeScript types
├── prisma/
│   └── schema.prisma     # Database schema
├── Dockerfile
├── docker-compose.yml
└── package.json
```

## Development

```bash
# Run database migrations
npx prisma migrate dev

# Open Prisma Studio
npx prisma studio

# Build for production
npm run build

# Run production build
npm start
```

## License

MIT
