import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessPod } from '../middleware/auth.js';
import { validate, createInboxSchema, updateInboxSchema, paginationSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { config } from '../config.js';

export async function inboxesRoutes(fastify: FastifyInstance) {
  // Create Inbox
  fastify.post('/inboxes', async (request, reply) => {
    const body = validate(createInboxSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Verify pod access
    const pod = await prisma.pod.findUnique({
      where: { id: body.pod_id },
    });

    if (!pod || pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Pod not found'));
    }

    if (!canAccessPod(auth, body.pod_id)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied to pod'));
    }

    // Check for client_id idempotency
    if (body.client_id) {
      const existing = await prisma.inbox.findUnique({
        where: { clientId: body.client_id },
      });
      if (existing && existing.podId === body.pod_id) {
        return reply.code(200).send(formatInbox(existing));
      }
      if (existing) {
        return reply.code(409).send(createApiError('conflict', 'client_id already exists'));
      }
    }

    const domain = body.domain || config.defaultDomain;
    const email = `${body.username}@${domain}`;

    // Check for email uniqueness
    const existingEmail = await prisma.inbox.findUnique({
      where: { email },
    });
    if (existingEmail) {
      return reply.code(409).send(createApiError('conflict', 'Email address already exists'));
    }

    const inbox = await prisma.inbox.create({
      data: {
        username: body.username,
        domain,
        email,
        displayName: body.display_name,
        clientId: body.client_id,
        podId: body.pod_id,
      },
    });

    return reply.code(201).send(formatInbox(inbox));
  });

  // List Inboxes
  fastify.get('/inboxes', async (request, reply) => {
    const query = validate(paginationSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;
    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit, config.pagination.maxLimit);

    // Build where clause based on key type
    const where: any = {
      pod: { organizationId: auth.organizationId },
      ...(cursor && { id: { lt: cursor.lastId } }),
    };

    if (auth.keyType === 'pod_key' && auth.podId) {
      where.podId = auth.podId;
    }

    const inboxes = await prisma.inbox.findMany({
      where,
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(inboxes, limit, formatInbox);
  });

  // Get Inbox
  fastify.get('/inboxes/:inbox_id', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const { auth } = request;

    const inbox = await prisma.inbox.findUnique({
      where: { id: inbox_id },
      include: { pod: true },
    });

    if (!inbox || inbox.pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    if (!canAccessPod(auth, inbox.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    return formatInbox(inbox);
  });

  // Update Inbox
  fastify.patch('/inboxes/:inbox_id', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const body = validate(updateInboxSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    const inbox = await prisma.inbox.findUnique({
      where: { id: inbox_id },
      include: { pod: true },
    });

    if (!inbox || inbox.pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    if (!canAccessPod(auth, inbox.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    const updated = await prisma.inbox.update({
      where: { id: inbox_id },
      data: {
        displayName: body.display_name,
      },
    });

    return formatInbox(updated);
  });

  // Delete Inbox
  fastify.delete('/inboxes/:inbox_id', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const { auth } = request;

    const inbox = await prisma.inbox.findUnique({
      where: { id: inbox_id },
      include: { pod: true },
    });

    if (!inbox || inbox.pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    if (!canAccessPod(auth, inbox.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    await prisma.inbox.delete({
      where: { id: inbox_id },
    });

    return reply.code(204).send();
  });
}

function formatInbox(inbox: {
  id: string;
  podId: string;
  username: string;
  domain: string;
  email: string;
  displayName: string | null;
  clientId: string | null;
  createdAt: Date;
  updatedAt: Date;
}) {
  return {
    inbox_id: inbox.id,
    pod_id: inbox.podId,
    username: inbox.username,
    domain: inbox.domain,
    email: inbox.email,
    display_name: inbox.displayName,
    client_id: inbox.clientId,
    created_at: inbox.createdAt.toISOString(),
    updated_at: inbox.updatedAt.toISOString(),
  };
}
