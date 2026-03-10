import { FastifyInstance } from 'fastify';
import { prisma, createApiError, canAccessInbox } from '../middleware/auth.js';
import { validate, sendMessageSchema, listMessagesSchema } from '../middleware/validation.js';
import { formatPaginatedResponse, decodeCursor } from '../services/pagination.js';
import { emailService } from '../services/email.js';
import { webhookService } from '../services/webhook.js';
import { config } from '../config.js';
import { v4 as uuidv4 } from 'uuid';

export async function messagesRoutes(fastify: FastifyInstance) {
  // Send Message
  fastify.post('/inboxes/:inbox_id/messages', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const body = validate(sendMessageSchema, request.body, reply);
    if (!body) return;

    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
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
        to: body.to,
        cc: body.cc,
        bcc: body.bcc,
        subject: body.subject,
        text: body.text,
        html: body.html,
        headers: body.headers,
        replyTo: body.reply_to,
      });
    } catch (error) {
      console.error('Email send failed:', error);
      return reply.code(500).send(createApiError('internal_error', 'Failed to send email'));
    }

    // Create message record
    const threadId = uuidv4(); // New thread for sent messages (unless replying)
    const message = await prisma.message.create({
      data: {
        threadId,
        inboxId: inbox_id,
        fromAddress: inbox.email,
        toAddresses: body.to,
        ccAddresses: body.cc || [],
        bccAddresses: body.bcc || [],
        subject: body.subject,
        textBody: body.text,
        htmlBody: body.html,
        headers: body.headers || {},
        direction: 'outbound',
        status: 'sent',
        externalId: emailResult.messageId,
      },
    });

    // Dispatch webhook
    await webhookService.dispatchEvent(
      auth.organizationId,
      'message.sent',
      formatMessage(message),
      inbox_id,
      inbox.podId
    );

    return reply.code(201).send(formatMessage(message));
  });

  // List Messages
  fastify.get('/inboxes/:inbox_id/messages', async (request, reply) => {
    const { inbox_id } = request.params as { inbox_id: string };
    const query = validate(listMessagesSchema, request.query, reply);
    if (!query) return;

    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const cursor = query.page_token ? decodeCursor(query.page_token) : null;
    const limit = Math.min(query.limit, config.pagination.maxLimit);

    const where: any = {
      inboxId: inbox_id,
      ...(cursor && { id: { lt: cursor.lastId } }),
      ...(query.thread_id && { threadId: query.thread_id }),
      ...(query.label && { labels: { has: query.label } }),
      ...(query.before && { createdAt: { lt: new Date(query.before) } }),
      ...(query.after && { createdAt: { gt: new Date(query.after) } }),
    };

    const messages = await prisma.message.findMany({
      where,
      include: { attachments: true },
      orderBy: { createdAt: 'desc' },
      take: limit + 1,
    });

    return formatPaginatedResponse(messages, limit, formatMessage);
  });

  // Get Message
  fastify.get('/inboxes/:inbox_id/messages/:message_id', async (request, reply) => {
    const { inbox_id, message_id } = request.params as { inbox_id: string; message_id: string };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const message = await prisma.message.findUnique({
      where: { id: message_id },
      include: { attachments: true },
    });

    if (!message || message.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Message not found'));
    }

    return formatMessage(message);
  });

  // Delete Message
  fastify.delete('/inboxes/:inbox_id/messages/:message_id', async (request, reply) => {
    const { inbox_id, message_id } = request.params as { inbox_id: string; message_id: string };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const message = await prisma.message.findUnique({
      where: { id: message_id },
    });

    if (!message || message.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Message not found'));
    }

    await prisma.message.delete({
      where: { id: message_id },
    });

    return reply.code(204).send();
  });

  // Get Attachment
  fastify.get('/inboxes/:inbox_id/messages/:message_id/attachments/:attachment_id', async (request, reply) => {
    const { inbox_id, message_id, attachment_id } = request.params as {
      inbox_id: string;
      message_id: string;
      attachment_id: string;
    };
    const { auth } = request;

    // Verify inbox access
    if (!await canAccessInbox(auth, inbox_id)) {
      return reply.code(404).send(createApiError('not_found', 'Inbox not found'));
    }

    const message = await prisma.message.findUnique({
      where: { id: message_id },
    });

    if (!message || message.inboxId !== inbox_id) {
      return reply.code(404).send(createApiError('not_found', 'Message not found'));
    }

    const attachment = await prisma.attachment.findUnique({
      where: { id: attachment_id },
    });

    if (!attachment || attachment.messageId !== message_id) {
      return reply.code(404).send(createApiError('not_found', 'Attachment not found'));
    }

    // TODO: Implement actual file download from storage
    return reply.code(501).send(createApiError('internal_error', 'Attachment download not implemented'));
  });
}

function formatMessage(message: {
  id: string;
  inboxId: string;
  threadId: string;
  fromAddress: string;
  toAddresses: string[];
  ccAddresses: string[];
  bccAddresses: string[];
  subject: string | null;
  textBody: string | null;
  htmlBody: string | null;
  labels: string[];
  headers: any;
  createdAt: Date;
  attachments?: Array<{
    id: string;
    filename: string;
    contentType: string;
    size: number;
  }>;
}) {
  return {
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
    attachments: (message.attachments || []).map(att => ({
      attachment_id: att.id,
      filename: att.filename,
      content_type: att.contentType,
      size: att.size,
    })),
    labels: message.labels,
    headers: message.headers,
    created_at: message.createdAt.toISOString(),
  };
}
