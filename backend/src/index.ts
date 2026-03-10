import Fastify from 'fastify';
import cors from '@fastify/cors';
import helmet from '@fastify/helmet';
import rateLimit from '@fastify/rate-limit';
import { config } from './config.js';
import { authMiddleware, createApiError, prisma } from './middleware/auth.js';
import { organizationRoutes } from './routes/organization.js';
import { podsRoutes } from './routes/pods.js';
import { inboxesRoutes } from './routes/inboxes.js';
import { messagesRoutes } from './routes/messages.js';
import { draftsRoutes } from './routes/drafts.js';
import { webhooksRoutes } from './routes/webhooks.js';
import { domainsRoutes } from './routes/domains.js';

const fastify = Fastify({
  logger: true,
});

// Register plugins
await fastify.register(cors, {
  origin: true,
  credentials: true,
});

await fastify.register(helmet);

await fastify.register(rateLimit, {
  max: config.rateLimit.free.max,
  timeWindow: config.rateLimit.free.timeWindow,
  keyGenerator: (request) => {
    // Use API key for rate limiting, fallback to IP
    const authHeader = request.headers.authorization;
    if (authHeader?.startsWith('Bearer ')) {
      return authHeader.slice(7);
    }
    return request.ip;
  },
  addHeadersOnExceeding: {
    'x-ratelimit-limit': true,
    'x-ratelimit-remaining': true,
    'x-ratelimit-reset': true,
  },
  addHeaders: {
    'x-ratelimit-limit': true,
    'x-ratelimit-remaining': true,
    'x-ratelimit-reset': true,
    'retry-after': true,
  },
  errorResponseBuilder: () => {
    return createApiError('rate_limit_exceeded', 'Too many requests');
  },
});

// Health check (no auth required)
fastify.get('/health', async () => {
  return { status: 'ok', timestamp: new Date().toISOString() };
});

// API v0 routes
const v0Routes = async (fastify: typeof import('fastify').FastifyInstance) => {
  // Add auth middleware to all v0 routes
  fastify.addHook('preHandler', authMiddleware);

  // Register route modules
  await fastify.register(organizationRoutes);
  await fastify.register(podsRoutes);
  await fastify.register(inboxesRoutes);
  await fastify.register(messagesRoutes);
  await fastify.register(draftsRoutes);
  await fastify.register(webhooksRoutes);
  await fastify.register(domainsRoutes);
};

await fastify.register(v0Routes, { prefix: '/v0' });

// Global error handler
fastify.setErrorHandler((error, request, reply) => {
  fastify.log.error(error);

  // Handle Prisma errors
  if (error.code === 'P2002') {
    return reply.code(409).send(createApiError('conflict', 'Resource already exists'));
  }
  if (error.code === 'P2025') {
    return reply.code(404).send(createApiError('not_found', 'Resource not found'));
  }

  // Handle validation errors
  if (error.validation) {
    return reply.code(422).send(createApiError('validation_error', error.message));
  }

  // Default internal error
  return reply.code(500).send(createApiError('internal_error', 'An unexpected error occurred'));
});

// Graceful shutdown
const signals = ['SIGINT', 'SIGTERM'] as const;
for (const signal of signals) {
  process.on(signal, async () => {
    fastify.log.info(`Received ${signal}, closing server...`);
    await fastify.close();
    await prisma.$disconnect();
    process.exit(0);
  });
}

// Start server
const start = async () => {
  try {
    await fastify.listen({
      port: config.port,
      host: config.host,
    });
    fastify.log.info(`Server listening on http://${config.host}:${config.port}`);
    fastify.log.info(`API available at http://${config.host}:${config.port}/v0`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
