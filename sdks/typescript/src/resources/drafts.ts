// Drafts Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Draft,
  Message,
  CreateDraftParams,
  UpdateDraftParams,
  PaginatedResponse,
  PaginationParams,
} from '../types.js';

/**
 * Client for managing drafts
 */
export class DraftsClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Create a new draft
   * 
   * @example
   * ```typescript
   * const draft = await client.drafts.create('inbox_abc123', {
   *   to: ['user@example.com'],
   *   subject: 'Draft email',
   *   text: 'This is a draft.'
   * });
   * ```
   */
  async create(inboxId: string, params: CreateDraftParams = {}): Promise<Draft> {
    return this.http.post<Draft>(`/inboxes/${inboxId}/drafts`, params);
  }

  /**
   * Get a draft by ID
   * 
   * @example
   * ```typescript
   * const draft = await client.drafts.get('inbox_abc123', 'draft_xyz789');
   * ```
   */
  async get(inboxId: string, draftId: string): Promise<Draft> {
    return this.http.get<Draft>(`/inboxes/${inboxId}/drafts/${draftId}`);
  }

  /**
   * Update a draft
   * 
   * @example
   * ```typescript
   * const draft = await client.drafts.update('inbox_abc123', 'draft_xyz789', {
   *   subject: 'Updated subject',
   *   text: 'Updated content'
   * });
   * ```
   */
  async update(inboxId: string, draftId: string, params: UpdateDraftParams): Promise<Draft> {
    return this.http.patch<Draft>(`/inboxes/${inboxId}/drafts/${draftId}`, params);
  }

  /**
   * Delete a draft
   * 
   * @example
   * ```typescript
   * await client.drafts.delete('inbox_abc123', 'draft_xyz789');
   * ```
   */
  async delete(inboxId: string, draftId: string): Promise<void> {
    return this.http.delete(`/inboxes/${inboxId}/drafts/${draftId}`);
  }

  /**
   * Send a draft immediately
   * 
   * @example
   * ```typescript
   * const message = await client.drafts.send('inbox_abc123', 'draft_xyz789');
   * ```
   */
  async send(inboxId: string, draftId: string): Promise<Message> {
    return this.http.post<Message>(`/inboxes/${inboxId}/drafts/${draftId}/send`);
  }

  /**
   * List drafts in an inbox with auto-pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.drafts.listPage('inbox_abc123', { limit: 10 });
   * 
   * // Auto-paginate through all drafts
   * for await (const draft of client.drafts.list('inbox_abc123')) {
   *   console.log(draft.subject);
   * }
   * ```
   */
  list(inboxId: string, params: PaginationParams = {}): PageIterator<Draft> {
    return paginate(
      (paginationParams) => this.listPage(inboxId, { ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of drafts
   */
  async listPage(inboxId: string, params: PaginationParams = {}): Promise<PaginatedResponse<Draft>> {
    return this.http.get<PaginatedResponse<Draft>>(`/inboxes/${inboxId}/drafts`, params);
  }
}
