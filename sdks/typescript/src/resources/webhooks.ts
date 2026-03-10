// Webhooks Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Webhook,
  CreateWebhookParams,
  UpdateWebhookParams,
  PaginatedResponse,
  PaginationParams,
} from '../types.js';

/**
 * Client for managing webhooks
 */
export class WebhooksClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Create a new webhook
   */
  async create(params: CreateWebhookParams): Promise<Webhook> {
    return this.http.post<Webhook>('/webhooks', params);
  }

  /**
   * Get a webhook by ID
   */
  async get(webhookId: string): Promise<Webhook> {
    return this.http.get<Webhook>(`/webhooks/${webhookId}`);
  }

  /**
   * Update a webhook
   */
  async update(webhookId: string, params: UpdateWebhookParams): Promise<Webhook> {
    return this.http.patch<Webhook>(`/webhooks/${webhookId}`, params);
  }

  /**
   * Delete a webhook
   */
  async delete(webhookId: string): Promise<void> {
    return this.http.delete(`/webhooks/${webhookId}`);
  }

  /**
   * List webhooks with auto-pagination
   */
  list(params: PaginationParams = {}): PageIterator<Webhook> {
    return paginate(
      (paginationParams) => this.listPage({ ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of webhooks
   */
  async listPage(params: PaginationParams = {}): Promise<PaginatedResponse<Webhook>> {
    const { limit, pageToken } = params;
    return this.http.get<PaginatedResponse<Webhook>>('/webhooks', {
      limit,
      pageToken,
    });
  }

  /**
   * Rotate webhook secret
   */
  async rotateSecret(webhookId: string): Promise<Webhook> {
    return this.http.post<Webhook>(`/webhooks/${webhookId}/rotate-secret`);
  }
}
