import { FastifyRequest, FastifyReply } from 'fastify';
import { PrismaClient } from '@prisma/client';
import type { AuthContext, ApiError, ErrorCode } from '../types/index.js';

const prisma = new PrismaClient();

// Extend FastifyRequest with auth context
declare module 'fastify' {
  interface FastifyRequest {
    auth: AuthContext;
  }
}

export function createApiError(code: ErrorCode, message: string, details?: Record<string, unknown>): ApiError {
  return {
    error: {
      code,
      message,
      details,
    },
  };
}

export async function authMiddleware(
  request: FastifyRequest,
  reply: FastifyReply
): Promise<void> {
  const authHeader = request.headers.authorization;

  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    reply.code(401).send(createApiError('authentication_error', 'Missing or invalid Authorization header'));
    return;
  }

  const token = authHeader.slice(7); // Remove 'Bearer '

  try {
    // Look up API key
    const apiKey = await prisma.apiKey.findUnique({
      where: { key: token },
      include: {
        organization: true,
        pod: true,
      },
    });

    if (!apiKey) {
      reply.code(401).send(createApiError('authentication_error', 'Invalid API key'));
      return;
    }

    // Check expiration
    if (apiKey.expiresAt && apiKey.expiresAt < new Date()) {
      reply.code(401).send(createApiError('authentication_error', 'API key has expired'));
      return;
    }

    // Update last used timestamp (fire and forget)
    prisma.apiKey.update({
      where: { id: apiKey.id },
      data: { lastUsedAt: new Date() },
    }).catch(() => {});

    // Set auth context
    request.auth = {
      organizationId: apiKey.organizationId,
      podId: apiKey.podId ?? undefined,
      keyType: apiKey.keyType as 'org_key' | 'pod_key',
    };
  } catch (error) {
    console.error('Auth middleware error:', error);
    reply.code(500).send(createApiError('internal_error', 'Authentication failed'));
  }
}

// Helper to check pod access
export function canAccessPod(auth: AuthContext, podId: string): boolean {
  // Org keys can access all pods
  if (auth.keyType === 'org_key') {
    return true;
  }
  // Pod keys can only access their specific pod
  return auth.podId === podId;
}

// Helper to check inbox access
export async function canAccessInbox(auth: AuthContext, inboxId: string): Promise<boolean> {
  const inbox = await prisma.inbox.findUnique({
    where: { id: inboxId },
    select: { podId: true, pod: { select: { organizationId: true } } },
  });

  if (!inbox) {
    return false;
  }

  // Check organization ownership
  if (inbox.pod.organizationId !== auth.organizationId) {
    return false;
  }

  // Check pod access
  return canAccessPod(auth, inbox.podId);
}

export { prisma };
