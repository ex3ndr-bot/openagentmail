import { FastifyInstance } from 'fastify';
import { prisma } from '../middleware/auth.js';

export async function organizationRoutes(fastify: FastifyInstance) {
  // Get Organization
  fastify.get('/organization', async (request, reply) => {
    const { auth } = request;

    const organization = await prisma.organization.findUnique({
      where: { id: auth.organizationId },
    });

    if (!organization) {
      return reply.code(404).send({
        error: {
          code: 'not_found',
          message: 'Organization not found',
        },
      });
    }

    return {
      organization_id: organization.id,
      name: organization.name,
      plan: organization.plan,
      created_at: organization.createdAt.toISOString(),
    };
  });
}
