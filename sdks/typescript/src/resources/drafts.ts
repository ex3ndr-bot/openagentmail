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
   */
  async create(inboxId: string, params: CreateDraftParams = {}): Promise<Draft> {
    return this.http.post<Draft>(`/inboxes/${inboxId}/drafts`, params);
  }

  /**
   * Get a draft by ID
   */
  async get(inboxId: string, draftId: string): Promise<Draft> {
    return this.http.get<Draft>(`/inboxes/${inboxId}/drafts/${draftId}`);
  }

  /**
   * Update a draft
   */
  async update(inboxId: string, draftId: string, params: UpdateDraftParams): Promise<Draft> {
    return this.http.patch<Draft>(`/inboxes/${inboxId}/drafts/${draftId}`, params);
  }

  /**
   * Delete a draft
   */
  async delete(inboxId: string, draftId: string): Promise<void> {
    return this.http.delete(`/inboxes/${inboxId}/drafts/${draftId}`);
  }

  /**
   * Send a draft immediately
   */
  async send(inboxId: string, draftId: string): Promise<Message> {
    return this.http.post<Message>(`/inboxes/${inboxId}/drafts/${draftId}/send`);
  }

  /**
   * List drafts in an inbox with auto-pagination
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
    const { limit, pageToken } = params;
    return this.http.get<PaginatedResponse<Draft>>(`/inboxes/${inboxId}/drafts`, {
      limit,
      pageToken,
    });
  }
}
