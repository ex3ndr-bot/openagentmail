import { z } from 'zod';
import { FastifyReply } from 'fastify';
import { createApiError } from './auth.js';

// Common schemas
export const paginationSchema = z.object({
  limit: z.coerce.number().min(1).max(100).default(20),
  page_token: z.string().optional(),
});

export const emailSchema = z.string().email();

export const emailArraySchema = z.array(emailSchema);

// Inbox schemas
export const createInboxSchema = z.object({
  pod_id: z.string().min(1),
  username: z.string().min(1).max(64).regex(/^[a-zA-Z0-9._-]+$/, 'Username must be alphanumeric with dots, underscores, or hyphens'),
  domain: z.string().optional(),
  display_name: z.string().max(128).optional(),
  client_id: z.string().optional(),
});

export const updateInboxSchema = z.object({
  display_name: z.string().max(128).optional(),
});

// Pod schemas
export const createPodSchema = z.object({
  name: z.string().min(1).max(128),
  client_id: z.string().optional(),
});

// Message schemas
export const sendMessageSchema = z.object({
  to: emailArraySchema.min(1),
  cc: emailArraySchema.optional().default([]),
  bcc: emailArraySchema.optional().default([]),
  subject: z.string().max(998), // RFC 5322 limit
  text: z.string().optional(),
  html: z.string().optional(),
  headers: z.record(z.string()).optional(),
  reply_to: z.string().optional(),
}).refine(data => data.text || data.html, {
  message: 'Either text or html must be provided',
});

export const listMessagesSchema = z.object({
  limit: z.coerce.number().min(1).max(100).default(20),
  page_token: z.string().optional(),
  thread_id: z.string().optional(),
  label: z.string().optional(),
  before: z.string().datetime().optional(),
  after: z.string().datetime().optional(),
});

// Draft schemas
export const createDraftSchema = z.object({
  to: emailArraySchema.optional().default([]),
  cc: emailArraySchema.optional().default([]),
  bcc: emailArraySchema.optional().default([]),
  subject: z.string().max(998).optional(),
  text: z.string().optional(),
  html: z.string().optional(),
  send_at: z.string().datetime().optional(),
});

export const updateDraftSchema = createDraftSchema.partial();

// Webhook schemas
export const webhookEventTypes = z.enum([
  'message.received',
  'message.sent',
  'message.delivered',
  'message.bounced',
]);

export const createWebhookSchema = z.object({
  url: z.string().url(),
  event_types: z.array(webhookEventTypes).min(1),
  inbox_ids: z.array(z.string()).optional().default([]),
  pod_ids: z.array(z.string()).optional().default([]),
  client_id: z.string().optional(),
});

export const updateWebhookSchema = z.object({
  url: z.string().url().optional(),
  event_types: z.array(webhookEventTypes).optional(),
  inbox_ids: z.array(z.string()).optional(),
  pod_ids: z.array(z.string()).optional(),
  enabled: z.boolean().optional(),
});

// Domain schemas
export const createDomainSchema = z.object({
  domain: z.string().min(1).max(253).regex(/^[a-zA-Z0-9][a-zA-Z0-9-]*\.([a-zA-Z]{2,})$/),
  pod_id: z.string().optional(),
});

// Validation helper
export function validate<T>(schema: z.Schema<T>, data: unknown, reply: FastifyReply): T | null {
  const result = schema.safeParse(data);
  if (!result.success) {
    const errors = result.error.flatten();
    reply.code(422).send(createApiError('validation_error', 'Request validation failed', {
      fieldErrors: errors.fieldErrors,
      formErrors: errors.formErrors,
    }));
    return null;
  }
  return result.data;
}
