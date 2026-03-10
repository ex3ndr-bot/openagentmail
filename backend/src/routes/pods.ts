import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessPod } from '../middleware/auth.js';
import { validate, createPodSchema, paginationSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { config } from '../config.js';

export async function podsRoutes(fastify: FastifyInstance) {
  // Create Pod
  fastify.post('/pods', async (request, reply) => {
    const body = validate(createPodSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Check for client_id idempotency
    if (body.client_id) {
      const existing = await prisma.pod.findUnique({
        where: { clientId: body.client_id },
      });
      if (existing && existing.organizationId === auth.organizationId) {
        return reply.code(200).send(formatPod(existing));
      }
      if (existing) {
        return reply.code(409).send(createApiError('conflict', 'client_id already exists'));
      }
    }

    const pod = await prisma.pod.create({
      data: {
        name: body.name,
        clientId: body.client_id,
        organizationId: auth.organizationId,
      },
    });

    return reply.code(201).send(formatPod(pod));
  });

  // List Pods
  fastify.get('/pods', async (request, reply) => {
    const query = validate(paginationSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;
    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit, config.pagination.maxLimit);

    const pods = await prisma.pod.findMany({
      where: {
        organizationId: auth.organizationId,
        ...(cursor && { id: { lt: cursor.lastId } }),
      },
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(pods, limit, formatPod);
  });

  // Get Pod
  fastify.get('/pods/:pod_id', async (request, reply) => {
    const { pod_id } = request.params as { pod_id: string };
    const { auth } = request;

    const pod = await prisma.pod.findUnique({
      where: { id: pod_id },
    });

    if (!pod || pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Pod not found'));
    }

    if (!canAccessPod(auth, pod_id)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    return formatPod(pod);
  });

  // Delete Pod
  fastify.delete('/pods/:pod_id', async (request, reply) => {
    const { pod_id } = request.params as { pod_id: string };
    const { auth } = request;

    // Only org keys can delete pods
    if (auth.keyType !== 'org_key') {
      return reply.code(403).send(createApiError('authorization_error', 'Only organization keys can delete pods'));
    }

    const pod = await prisma.pod.findUnique({
      where: { id: pod_id },
    });

    if (!pod || pod.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Pod not found'));
    }

    await prisma.pod.delete({
      where: { id: pod_id },
    });

    return reply.code(204).send();
  });
}

function formatPod(pod: {
  id: string;
  name: string;
  clientId: string | null;
  createdAt: Date;
  updatedAt: Date;
}) {
  return {
    pod_id: pod.id,
    name: pod.name,
    client_id: pod.clientId,
    created_at: pod.createdAt.toISOString(),
    updated_at: pod.updatedAt.toISOString(),
  };
}
