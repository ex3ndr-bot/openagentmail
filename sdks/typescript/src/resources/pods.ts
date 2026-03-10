// Pods Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Pod,
  CreatePodParams,
  UpdatePodParams,
  PaginatedResponse,
  PaginationParams,
} from '../types.js';

/**
 * Client for managing pods (multi-tenant isolation)
 */
export class PodsClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Create a new pod
   * 
   * @example
   * ```typescript
   * const pod = await client.pods.create({
   *   name: 'Production',
   *   clientId: 'my-unique-id'
   * });
   * ```
   */
  async create(params: CreatePodParams): Promise<Pod> {
    return this.http.post<Pod>('/pods', params);
  }

  /**
   * Get a pod by ID
   * 
   * @example
   * ```typescript
   * const pod = await client.pods.get('pod_xyz789');
   * ```
   */
  async get(podId: string): Promise<Pod> {
    return this.http.get<Pod>(`/pods/${podId}`);
  }

  /**
   * Update a pod
   * 
   * @example
   * ```typescript
   * const pod = await client.pods.update('pod_xyz789', {
   *   name: 'New Pod Name'
   * });
   * ```
   */
  async update(podId: string, params: UpdatePodParams): Promise<Pod> {
    return this.http.patch<Pod>(`/pods/${podId}`, params);
  }

  /**
   * Delete a pod
   * 
   * @example
   * ```typescript
   * await client.pods.delete('pod_xyz789');
   * ```
   */
  async delete(podId: string): Promise<void> {
    return this.http.delete(`/pods/${podId}`);
  }

  /**
   * List pods with auto-pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.pods.listPage({ limit: 10 });
   * 
   * // Auto-paginate through all pods
   * for await (const pod of client.pods.list()) {
   *   console.log(pod.name);
   * }
   * ```
   */
  list(params: PaginationParams = {}): PageIterator<Pod> {
    return paginate(
      (paginationParams) => this.listPage({ ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of pods
   */
  async listPage(params: PaginationParams = {}): Promise<PaginatedResponse<Pod>> {
    return this.http.get<PaginatedResponse<Pod>>('/pods', params);
  }
}
