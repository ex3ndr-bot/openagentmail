// Domains Resource for OpenAgentMail

import type { HttpClient } from '../utils/http.js';
import { paginate, PageIterator } from '../utils/pagination.js';
import type {
  Domain,
  AddDomainParams,
  ListDomainsParams,
  PaginatedResponse,
} from '../types.js';

/**
 * Client for managing domains
 */
export class DomainsClient {
  constructor(private readonly http: HttpClient) {}

  /**
   * Add a new domain
   */
  async add(params: AddDomainParams): Promise<Domain> {
    return this.http.post<Domain>('/domains', params);
  }

  /**
   * Get a domain by ID
   */
  async get(domainId: string): Promise<Domain> {
    return this.http.get<Domain>(`/domains/${domainId}`);
  }

  /**
   * Delete a domain
   */
  async delete(domainId: string): Promise<void> {
    return this.http.delete(`/domains/${domainId}`);
  }

  /**
   * Verify a domain
   */
  async verify(domainId: string): Promise<Domain> {
    return this.http.post<Domain>(`/domains/${domainId}/verify`);
  }

  /**
   * List domains with auto-pagination
   */
  list(params: ListDomainsParams = {}): PageIterator<Domain> {
    return paginate(
      (paginationParams) => this.listPage({ ...params, ...paginationParams }),
      params
    );
  }

  /**
   * List a single page of domains
   */
  async listPage(params: ListDomainsParams = {}): Promise<PaginatedResponse<Domain>> {
    const { podId, limit, pageToken } = params;
    return this.http.get<PaginatedResponse<Domain>>('/domains', {
      podId,
      limit,
      pageToken,
    });
  }
}
