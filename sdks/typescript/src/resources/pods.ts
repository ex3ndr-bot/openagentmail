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
   */
  async create(params: CreatePodParams): Promise<Pod> {
    return this.http.post<Pod>('/pods', params);
  }

  /**
   * Get a pod by ID
   */
  async get(podId: string): Promise<Pod> {
    return this.http.get<Pod>(`/pods/${podId}`);
  }

  /**
   * Update a pod
   */
  async update(podId: string, params: UpdatePodParams): Promise<Pod> {
    return this.http.patch<Pod>(`/pods/${podId}`, params);
  }

  /**
   * Delete a pod
   */
  async delete(podId: string): Promise<void> {
    return this.http.delete(`/pods/${podId}`);
  }

  /**
   * List pods with auto-pagination
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
    const { limit, pageToken } = params;
    return this.http.get<PaginatedResponse<Pod>>('/pods', {
      limit,
      pageToken,
    });
  }
}
