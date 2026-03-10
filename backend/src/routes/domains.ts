import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessPod } from '../middleware/auth.js';
import { validate, createDomainSchema, paginationSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { config } from '../config.js';
import crypto from 'crypto';

export async function domainsRoutes(fastify: FastifyInstance) {
  // Create Domain
  fastify.post('/domains', async (request, reply) => {
    const body = validate(createDomainSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Check if domain already exists
    const existing = await prisma.domain.findUnique({
      where: { domain: body.domain },
    });
    if (existing) {
      return reply.code(409).send(createApiError('conflict', 'Domain already exists'));
    }

    // Verify pod access if pod_id provided
    if (body.pod_id) {
      const pod = await prisma.pod.findUnique({ where: { id: body.pod_id } });
      if (!pod || pod.organizationId !== auth.organizationId) {
        return reply.code(404).send(createApiError('not_found', 'Pod not found'));
      }
      if (!canAccessPod(auth, body.pod_id)) {
        return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
      }
    }

    // Generate DNS records for verification
    const verificationToken = crypto.randomBytes(16).toString('hex');
    const dnsRecords = [
      {
        type: 'TXT',
        name: `_oam-verification.${body.domain}`,
        value: `oam-verify=${verificationToken}`,
      },
      {
        type: 'MX',
        name: body.domain,
        value: '10 mx.openagentmail.com',
      },
      {
        type: 'TXT',
        name: body.domain,
        value: 'v=spf1 include:openagentmail.com ~all',
      },
      {
        type: 'CNAME',
        name: `oam._domainkey.${body.domain}`,
        value: 'dkim.openagentmail.com',
      },
    ];

    const domain = await prisma.domain.create({
      data: {
        domain: body.domain,
        status: 'pending',
        organizationId: auth.organizationId,
        podId: body.pod_id,
        dnsRecords,
      },
    });

    return reply.code(201).send(formatDomain(domain));
  });

  // List Domains
  fastify.get('/domains', async (request, reply) => {
    const query = validate(paginationSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;
    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit ?? config.pagination.defaultLimit, config.pagination.maxLimit);

    const where: any = {
      organizationId: auth.organizationId,
      ...(cursor && { id: { lt: cursor.lastId } }),
    };

    if (auth.keyType === 'pod_key' && auth.podId) {
      where.podId = auth.podId;
    }

    const domains = await prisma.domain.findMany({
      where,
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(domains, query.limit, formatDomain);
  });

  // Get Domain
  fastify.get('/domains/:domain_id', async (request, reply) => {
    const { domain_id } = request.params as { domain_id: string };
    const { auth } = request;

    const domain = await prisma.domain.findUnique({
      where: { id: domain_id },
    });

    if (!domain || domain.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Domain not found'));
    }

    if (domain.podId && !canAccessPod(auth, domain.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    return formatDomain(domain);
  });

  // Verify Domain
  fastify.post('/domains/:domain_id/verify', async (request, reply) => {
    const { domain_id } = request.params as { domain_id: string };
    const { auth } = request;

    const domain = await prisma.domain.findUnique({
      where: { id: domain_id },
    });

    if (!domain || domain.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Domain not found'));
    }

    if (domain.podId && !canAccessPod(auth, domain.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    // Mark as verifying
    await prisma.domain.update({
      where: { id: domain_id },
      data: { status: 'verifying' },
    });

    // TODO: Implement actual DNS verification
    // For now, we'll just simulate verification
    // In production, you would check DNS records here
    setTimeout(async () => {
      try {
        await prisma.domain.update({
          where: { id: domain_id },
          data: {
            status: 'verified',
            verifiedAt: new Date(),
          },
        });
      } catch (error) {
        console.error('Domain verification update failed:', error);
      }
    }, 5000);

    const updated = await prisma.domain.findUnique({
      where: { id: domain_id },
    });

    return formatDomain(updated!);
  });

  // Delete Domain
  fastify.delete('/domains/:domain_id', async (request, reply) => {
    const { domain_id } = request.params as { domain_id: string };
    const { auth } = request;

    const domain = await prisma.domain.findUnique({
      where: { id: domain_id },
    });

    if (!domain || domain.organizationId !== auth.organizationId) {
      return reply.code(404).send(createApiError('not_found', 'Domain not found'));
    }

    if (domain.podId && !canAccessPod(auth, domain.podId)) {
      return reply.code(403).send(createApiError('authorization_error', 'Access denied'));
    }

    await prisma.domain.delete({
      where: { id: domain_id },
    });

    return reply.code(204).send();
  });
}

function formatDomain(domain: {
  id: string;
  domain: string;
  podId: string | null;
  status: string;
  dnsRecords: any;
  createdAt: Date;
  updatedAt: Date;
}) {
  return {
    domain_id: domain.id,
    domain: domain.domain,
    pod_id: domain.podId,
    status: domain.status as 'pending' | 'verifying' | 'verified' | 'failed',
    dns_records: domain.dnsRecords,
    created_at: domain.createdAt.toISOString(),
    updated_at: domain.updatedAt.toISOString(),
  };
}
