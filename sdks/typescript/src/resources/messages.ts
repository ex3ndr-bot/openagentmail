// Messages Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Message,
  SendMessageParams,
  ListMessagesParams,
  UpdateMessageParams,
  PaginatedResponse,
} from '../types.js';

/**
 * Client for managing messages
 */
export class MessagesClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Send a new message from an inbox
   * 
   * @example
   * ```typescript
   * const message = await client.messages.send('inbox_abc123', {
   *   to: ['user@example.com'],
   *   subject: 'Hello!',
   *   text: 'This is a test email.',
   *   html: '<p>This is a <strong>test</strong> email.</p>'
   * });
   * ```
   */
  async send(inboxId: string, params: SendMessageParams): Promise<Message> {
    return this.http.post<Message>(`/inboxes/${inboxId}/messages`, params);
  }

  /**
   * Get a message by ID
   * 
   * @example
   * ```typescript
   * const message = await client.messages.get('inbox_abc123', 'msg_xyz789');
   * ```
   */
  async get(inboxId: string, messageId: string): Promise<Message> {
    return this.http.get<Message>(`/inboxes/${inboxId}/messages/${messageId}`);
  }

  /**
   * Update a message (labels)
   * 
   * @example
   * ```typescript
   * const message = await client.messages.update('inbox_abc123', 'msg_xyz789', {
   *   addLabels: ['important'],
   *   removeLabels: ['unread']
   * });
   * ```
   */
  async update(inboxId: string, messageId: string, params: UpdateMessageParams): Promise<Message> {
    return this.http.patch<Message>(`/inboxes/${inboxId}/messages/${messageId}`, params);
  }

  /**
   * Delete a message
   * 
   * @example
   * ```typescript
   * await client.messages.delete('inbox_abc123', 'msg_xyz789');
   * ```
   */
  async delete(inboxId: string, messageId: string): Promise<void> {
    return this.http.delete(`/inboxes/${inboxId}/messages/${messageId}`);
  }

  /**
   * List messages in an inbox with auto-pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.messages.listPage('inbox_abc123', { limit: 10 });
   * 
   * // Auto-paginate through all messages
   * for await (const message of client.messages.list('inbox_abc123')) {
   *   console.log(message.subject);
   * }
   * 
   * // Filter by label
   * for await (const message of client.messages.list('inbox_abc123', { label: 'unread' })) {
   *   console.log(message.subject);
   * }
   * ```
   */
  list(inboxId: string, params: ListMessagesParams = {}): PageIterator<Message> {
    return paginate(
      (paginationParams) => this.listPage(inboxId, { ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of messages
   */
  async listPage(inboxId: string, params: ListMessagesParams = {}): Promise<PaginatedResponse<Message>> {
    return this.http.get<PaginatedResponse<Message>>(`/inboxes/${inboxId}/messages`, params);
  }

  /**
   * Download an attachment
   * 
   * @example
   * ```typescript
   * const content = await client.messages.downloadAttachment(
   *   'inbox_abc123',
   *   'msg_xyz789',
   *   'att_def456'
   * );
   * ```
   */
  async downloadAttachment(
    inboxId: string,
    messageId: string,
    attachmentId: string
  ): Promise<ArrayBuffer> {
    // Note: This endpoint returns binary data, not JSON
    // The HTTP client would need special handling for this
    return this.http.get<ArrayBuffer>(
      `/inboxes/${inboxId}/messages/${messageId}/attachments/${attachmentId}`
    );
  }
}
