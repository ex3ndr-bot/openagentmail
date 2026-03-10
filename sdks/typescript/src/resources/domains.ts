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
   * 
   * @example
   * ```typescript
   * const domain = await client.domains.add({
   *   domain: 'example.com',
   *   podId: 'pod_xyz789'
   * });
   * 
   * // Get DNS records to configure
   * for (const record of domain.dnsRecords) {
   *   console.log(`${record.type} ${record.name} -> ${record.value}`);
   * }
   * ```
   */
  async add(params: AddDomainParams): Promise<Domain> {
    return this.http.post<Domain>('/domains', params);
  }

  /**
   * Get a domain by ID
   * 
   * @example
   * ```typescript
   * const domain = await client.domains.get('dom_abc123');
   * ```
   */
  async get(domainId: string): Promise<Domain> {
    return this.http.get<Domain>(`/domains/${domainId}`);
  }

  /**
   * Delete a domain
   * 
   * @example
   * ```typescript
   * await client.domains.delete('dom_abc123');
   * ```
   */
  async delete(domainId: string): Promise<void> {
    return this.http.delete(`/domains/${domainId}`);
  }

  /**
   * Verify a domain
   * 
   * Triggers DNS verification. The domain status will change to 'verifying'
   * and then to 'verified' or 'failed' once complete.
   * 
   * @example
   * ```typescript
   * const domain = await client.domains.verify('dom_abc123');
   * console.log(domain.status); // 'verifying'
   * ```
   */
  async verify(domainId: string): Promise<Domain> {
    return this.http.post<Domain>(`/domains/${domainId}/verify`);
  }

  /**
   * List domains with auto-pagination
   * 
   * @example
   * ```typescript
   * // Get first page
   * const { items, hasMore } = await client.domains.listPage({ limit: 10 });
   * 
   * // Auto-paginate through all domains
   * for await (const domain of client.domains.list()) {
   *   console.log(`${domain.domain}: ${domain.status}`);
   * }
   * 
   * // Filter by pod
   * for await (const domain of client.domains.list({ podId: 'pod_xyz789' })) {
   *   console.log(domain.domain);
   * }
   * ```
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
    return this.http.get<PaginatedResponse<Domain>>('/domains', params);
  }
}
