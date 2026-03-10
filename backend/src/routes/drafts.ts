import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessInbox } from '../middleware/auth.js';
import { validate, createDraftSchema, updateDraftSchema, paginationSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { emailService } from '../services/email.js';
import { webhookService } from '../services/webhook.js';
import { config } from '../config.js';
import { v4 as uuidv4 } from 'uuid';

export async function draftsRoutes(fastify: FastifyInstance) {
  // Create Draft
  fastify.post('/inboxes/:inbox_id/drafts', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const body = validate(createDraftSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const draft = await prisma.draft.create({
      data: {
        inboxId: inbox_id,
        toAddresses: body.to || [],
        ccAddresses: body.cc || [],
        bccAddresses: body.bcc || [],
        subject: body.subject,
        textBody: body.text,
        htmlBody: body.html,
        sendAt: body.send_at ? new Date(body.send_at) : null,
      },
    });

    return reply.code(201).send(formatDraft(draft));
  });

  // List Drafts
  fastify.get('/inboxes/:inbox_id/drafts', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const query = validate(paginationSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit, config.pagination.maxLimit);

    const drafts = await prisma.draft.findMany({
      where: {
        inboxId: inbox_id,
        ...(cursor && { id: { lt: cursor.lastId } }),
      },
      include: { attachments: true },
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(drafts, limit, formatDraft);
  });

  // Get Draft
  fastify.get('/inboxes/:inbox_id/drafts/:draft_id', async (request, reply) => {
    const { inbox_id, draft_id } = request.params as { inbox_id: string; draft_id: string };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const draft = await prisma.draft.findUnique({
      where: { id: draft_id },
      include: { attachments: true },
    });

    if (!draft || draft.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Draft not found'));
    }

    return formatDraft(draft);
  });

  // Update Draft
  fastify.patch('/inboxes/:inbox_id/drafts/:draft_id', async (request, reply) => {
    const { inbox_id, draft_id } = request.params as { inbox_id: string; draft_id: string };
    const body = validate(updateDraftSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const draft = await prisma.draft.findUnique({
      where: { id: draft_id },
    });

    if (!draft || draft.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Draft not found'));
    }

    const updated = await prisma.draft.update({
      where: { id: draft_id },
      data: {
        ...(body.to !== undefined && { toAddresses: body.to }),
        ...(body.cc !== undefined && { ccAddresses: body.cc }),
        ...(body.bcc !== undefined && { bccAddresses: body.bcc }),
        ...(body.subject !== undefined && { subject: body.subject }),
        ...(body.text !== undefined && { textBody: body.text }),
        ...(body.html !== undefined && { htmlBody: body.html }),
        ...(body.send_at !== undefined && { sendAt: body.send_at ? new Date(body.send_at) : null }),
      },
      include: { attachments: true },
    });

    return formatDraft(updated);
  });

  // Delete Draft
  fastify.delete('/inboxes/:inbox_id/drafts/:draft_id', async (request, reply) => {
    const { inbox_id, draft_id } = request.params as { inbox_id: string; draft_id: string };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const draft = await prisma.draft.findUnique({
      where: { id: draft_id },
    });

    if (!draft || draft.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Draft not found'));
    }

    await prisma.draft.delete({
      where: { id: draft_id },
    });

    return reply.code(204).send();
  });

  // Send Draft
  fastify.post('/inboxes/:inbox_id/drafts/:draft_id/send', async (request, reply) => {
    const { inbox_id, draft_id } = request.params as { inbox_id: string; draft_id: string };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const draft = await prisma.draft.findUnique({
      where: { id: draft_id },
      include: { attachments: true },
    });

    if (!draft || draft.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Draft not found'));
    }

    // Validate draft has required fields
    if (draft.toAddresses.length === 0) {
      return reply.code(422).send(createApiError('validation_error', 'Draft must have at least one recipient'));
    }
    if (!draft.subject && !draft.textBody && !draft.htmlBody) {
      return reply.code(422).send(createApiError('validation_error', 'Draft must have a subject or body'));
    }

    const inbox = await prisma.inbox.findUnique({
      where: { id: inbox_id },
      include: { pod: true },
    });

    if (!inbox) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    // Send email
    let emailResult;
    try {
      emailResult = await emailService.sendEmail({
        from: inbox.displayName ? `${inbox.displayName} <${inbox.email}>` : inbox.email,
        to: draft.toAddresses,
        cc: draft.ccAddresses,
        bcc: draft.bccAddresses,
        subject: draft.subject || '',
        text: draft.textBody || undefined,
        html: draft.htmlBody || undefined,
      });
    } catch (error) {
      console.error('Email send failed:', error);
      return reply.code(500).send(createApiError('internal_error', 'Failed to send email'));
    }

    // Create message record
    const threadId = uuidv4();
    const message = await prisma.message.create({
      data: {
        threadId,
        inboxId: inbox_id,
        fromAddress: inbox.email,
        toAddresses: draft.toAddresses,
        ccAddresses: draft.ccAddresses,
        bccAddresses: draft.bccAddresses,
        subject: draft.subject,
        textBody: draft.textBody,
        htmlBody: draft.htmlBody,
        direction: 'outbound',
        status: 'sent',
        externalId: emailResult.messageId,
      },
    });

    // Delete draft after sending
    await prisma.draft.delete({
      where: { id: draft_id },
    });

    // Dispatch webhook
    await webhookService.dispatchEvent(
      auth.organizationId,
      'message.sent',
      {
        message_id: message.id,
        inbox_id: message.inboxId,
        thread_id: message.threadId,
        from: message.fromAddress,
        to: message.toAddresses,
        subject: message.subject,
        created_at: message.createdAt.toISOString(),
      },
      inbox_id,
      inbox.podId
    );

    return reply.code(200).send({
      message_id: message.id,
      inbox_id: message.inboxId,
      thread_id: message.threadId,
      from: message.fromAddress,
      to: message.toAddresses,
      cc: message.ccAddresses,
      bcc: message.bccAddresses,
      subject: message.subject || '',
      text: message.textBody,
      html: message.htmlBody,
      attachments: [],
      labels: message.labels,
      headers: message.headers,
      created_at: message.createdAt.toISOString(),
    });
  });
}

function formatDraft(draft: {
  id: string;
  inboxId: string;
  toAddresses: string[];
  ccAddresses: string[];
  bccAddresses: string[];
  subject: string | null;
  textBody: string | null;
  htmlBody: string | null;
  sendAt: Date | null;
  createdAt: Date;
  updatedAt: Date;
  attachments?: Array<{
    id: string;
    filename: string;
    contentType: string;
    size: number;
  }>;
}) {
  return {
    draft_id: draft.id,
    inbox_id: draft.inboxId,
    to: draft.toAddresses,
    cc: draft.ccAddresses,
    bcc: draft.bccAddresses,
    subject: draft.subject,
    text: draft.textBody,
    html: draft.htmlBody,
    attachments: (draft.attachments || []).map(att => ({
      attachment_id: att.id,
      filename: att.filename,
      content_type: att.contentType,
      size: att.size,
    })),
    send_at: draft.sendAt?.toISOString() || null,
    created_at: draft.createdAt.toISOString(),
    updated_at: draft.updatedAt.toISOString(),
  };
}
