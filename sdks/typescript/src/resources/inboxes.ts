// Inboxes Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Inbox,
  CreateInboxParams,
  UpdateInboxParams,
  ListInboxesParams,
  PaginatedResponse,
} from '../types.js';

/**
 * Client for managing inboxes
 */
export class InboxesClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Create a new inbox
   * 
   * @example
   * ```typescript
   * const inbox = await client.inboxes.create({
   *   username: 'support',
   *   domain: 'example.com',
   *   displayName: 'Support Team'
   * });
   * ```
   */
  async create(params: CreateInboxParams): Promise<Inbox> {
    return this.http.post<Inbox>('/inboxes', params);
  }

  /**
   * Get an inbox by ID
   * 
   * @example
   * ```typescript
   * const inbox = await client.inboxes.get('inbox_abc123');
   * ```
   */
  async get(inboxId: string): Promise<Inbox> {
    return this.http.get<Inbox>(`/inboxes/${inboxId}`);
  }

  /**
   * Update an inbox
   * 
   * @example
   * ```typescript
   * const inbox = await client.inboxes.update('inbox_abc123', {
   *   displayName: 'New Display Name'
   * });
   * ```
   */
  async update(inboxId: string, params: UpdateInboxParams): Promise<Inbox> {
    return this.http.patch<Inbox>(`/inboxes/${inboxId}`, params);
  }

  /**
   * Delete an inbox
   * 
   * @example
   * ```typescript
   * await client.inboxes.delete('inbox_abc123');
   * ```
   */
  async delete(inboxId: string): Promise<void> {
    return this.http.delete(`/inboxes/${inboxId}`);
  }

  /**
   * List inboxes with pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.inboxes.listPage({ limit: 10 });
   * 
   * // Auto-paginate through all inboxes
   * for await (const inbox of client.inboxes.list()) {
   *   console.log(inbox.email);
   * }
   * ```
   */
  list(params: ListInboxesParams = {}): PageIterator<Inbox> {
    return paginate(
      (paginationParams) => this.listPage({ ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of inboxes
   */
  async listPage(params: ListInboxesParams = {}): Promise<PaginatedResponse<Inbox>> {
    const { podId, limit, pageToken } = params;
    return this.http.get<PaginatedResponse<Inbox>>('/inboxes', {
      podId,
      limit,
      pageToken,
    });
  }
}
