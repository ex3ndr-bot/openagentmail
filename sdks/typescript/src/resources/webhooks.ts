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
   * 
   * @example
   * ```typescript
   * const webhook = await client.webhooks.create({
   *   url: 'https://example.com/webhook',
   *   eventTypes: ['message.received', 'message.sent'],
   *   inboxIds: ['inbox_abc123']
   * });
   * ```
   */
  async create(params: CreateWebhookParams): Promise<Webhook> {
    return this.http.post<Webhook>('/webhooks', params);
  }

  /**
   * Get a webhook by ID
   * 
   * @example
   * ```typescript
   * const webhook = await client.webhooks.get('wh_xyz789');
   * ```
   */
  async get(webhookId: string): Promise<Webhook> {
    return this.http.get<Webhook>(`/webhooks/${webhookId}`);
  }

  /**
   * Update a webhook
   * 
   * @example
   * ```typescript
   * const webhook = await client.webhooks.update('wh_xyz789', {
   *   enabled: false
   * });
   * ```
   */
  async update(webhookId: string, params: UpdateWebhookParams): Promise<Webhook> {
    return this.http.patch<Webhook>(`/webhooks/${webhookId}`, params);
  }

  /**
   * Delete a webhook
   * 
   * @example
   * ```typescript
   * await client.webhooks.delete('wh_xyz789');
   * ```
   */
  async delete(webhookId: string): Promise<void> {
    return this.http.delete(`/webhooks/${webhookId}`);
  }

  /**
   * List webhooks with auto-pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.webhooks.listPage({ limit: 10 });
   * 
   * // Auto-paginate through all webhooks
   * for await (const webhook of client.webhooks.list()) {
   *   console.log(webhook.url);
   * }
   * ```
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
    return this.http.get<PaginatedResponse<Webhook>>('/webhooks', params);
  }

  /**
   * Rotate webhook secret
   * 
   * @example
   * ```typescript
   * const webhook = await client.webhooks.rotateSecret('wh_xyz789');
   * console.log(webhook.secret); // New secret
   * ```
   */
  async rotateSecret(webhookId: string): Promise<Webhook> {
    return this.http.post<Webhook>(`/webhooks/${webhookId}/rotate-secret`);
  }
}
