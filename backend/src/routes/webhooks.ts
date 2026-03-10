import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessPod } from '../middleware/auth.js';
import { validate, createWebhookSchema, updateWebhookSchema, paginationSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { webhookService } from '../services/webhook.js';
import { config } from '../config.js';

export async function webhooksRoutes(fastify: FastifyInstance) {
  // Create Webhook
  fastify.post('/webhooks', async (request, reply) => {
    const body = validate(createWebhookSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Check for client_id idempotency
    if (body.client_id) {
      const existing = await prisma.webhook.findUnique({
        where: { clientId: body.client_id },
      });
      if (existing && existing.organizationId === auth.organizationId) {
        return reply.code(200).send(formatWebhook(existing));
      }
      if (existing) {
        return reply.code(409).send(createApiError('conflict', 'client_id already exists'));
      }
    }

    // Verify pod access for pod_ids
    if (body.pod_ids && body.pod_ids.length > 0) {
      for (const podId of body.pod_ids) {
        const pod = await prisma.pod.findUnique({ where: { id: podId } });
        if (!pod || pod.organizationId !== auth.organizationId) {
          return reply.code(404).send(createApiError('not_found', `Pod ${podId} not found`));
        }
        if (!canAccessPod(auth, podId)) {
          return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
        }
      }
    }

    // Verify inbox access for inbox_ids
    if (body.inbox_ids && body.inbox_ids.length > 0) {
      for (const inboxId of body.inbox_ids) {
        const inbox = await prisma.inbox.findUnique({
          where: { id: inboxId },
          include: { pod: true },
        });
        if (!inbox || inbox.pod.organizationId !== auth.organizationId) {
          return reply.code(404).send(createApiError('not_found', `Inbox ${inboxId} not found`));
        }
        if (!canAccessPod(auth, inbox.podId)) {
          return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
        }
      }
    }

    const webhook = await prisma.webhook.create({
      data: {
        url: body.url,
        secret: webhookService.generateSecret(),
        eventTypes: body.event_types,
        inboxIds: body.inbox_ids || [],
        podIds: body.pod_ids || [],
        clientId: body.client_id,
        organizationId: auth.organizationId,
        podId: auth.podId,
      },
    });

    return reply.code(201).send(formatWebhook(webhook));
  });

  // List Webhooks
  fastify.get('/webhooks', async (request, reply) => {
    const query = validate(paginationSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;
    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit, config.pagination.maxLimit);

    const where: any = {
      organizationId: auth.organizationId,
      ...(cursor && { id: { lt: cursor.lastId } }),
    };

    if (auth.keyType === 'pod_key' && auth.podId) {
      where.podId = auth.podId;
    }

    const webhooks = await prisma.webhook.findMany({
      where,
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(webhooks, limit, formatWebhook);
  });

  // Get Webhook
  fastify.get('/webhooks/:webhook_id', async (request, reply) => {
    const { webhook_id } = request.params as { webhook_id: string };
    const { auth } = request;

    const webhook = await prisma.webhook.findUnique({
      where: { id: webhook_id },
    });

    if (!webhook || webhook.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Webhook not found'));
    }

    if (auth.keyType === 'pod_key' && webhook.podId !== auth.podId) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    return formatWebhook(webhook);
  });

  // Update Webhook
  fastify.patch('/webhooks/:webhook_id', async (request, reply) => {
    const { webhook_id } = request.params as { webhook_id: string };
    const body = validate(updateWebhookSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    const webhook = await prisma.webhook.findUnique({
      where: { id: webhook_id },
    });

    if (!webhook || webhook.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Webhook not found'));
    }

    if (auth.keyType === 'pod_key' && webhook.podId !== auth.podId) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    const updated = await prisma.webhook.update({
      where: { id: webhook_id },
      data: {
        ...(body.url && { url: body.url }),
        ...(body.event_types && { eventTypes: body.event_types }),
        ...(body.inbox_ids !== undefined && { inboxIds: body.inbox_ids }),
        ...(body.pod_ids !== undefined && { podIds: body.pod_ids }),
        ...(body.enabled !== undefined && { enabled: body.enabled }),
      },
    });

    return formatWebhook(updated);
  });

  // Delete Webhook
  fastify.delete('/webhooks/:webhook_id', async (request, reply) => {
    const { webhook_id } = request.params as { webhook_id: string };
    const { auth } = request;

    const webhook = await prisma.webhook.findUnique({
      where: { id: webhook_id },
    });

    if (!webhook || webhook.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Webhook not found'));
    }

    if (auth.keyType === 'pod_key' && webhook.podId !== auth.podId) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    await prisma.webhook.delete({
      where: { id: webhook_id },
    });

    return reply.code(204).send();
  });
}

function formatWebhook(webhook: {
  id: string;
  url: string;
  eventTypes: string[];
  inboxIds: string[];
  podIds: string[];
  clientId: string | null;
  secret: string;
  enabled: boolean;
  createdAt: Date;
  updatedAt: Date;
}) {
  return {
    webhook_id: webhook.id,
    url: webhook.url,
    event_types: webhook.eventTypes,
    inbox_ids: webhook.inboxIds,
    pod_ids: webhook.podIds,
    client_id: webhook.clientId,
    secret: webhook.secret,
    enabled: webhook.enabled,
    created_at: webhook.createdAt.toISOString(),
    updated_at: webhook.updatedAt.toISOString(),
  };
}
