// OpenAgentMail Client

import type { OpenAgentMailConfig, Organization } from './types.js';
import { HttpClient } from './utils/http.js';
import { InboxesClient } from './resources/inboxes.js';
import { MessagesClient } from './resources/messages.js';
import { DraftsClient } from './resources/drafts.js';
import { WebhooksClient } from './resources/webhooks.js';
import { PodsClient } from './resources/pods.js';
import { DomainsClient } from './resources/domains.js';

/**
 * OpenAgentMail API client
 * 
 * @example
 * ```typescript
 * import { OpenAgentMail } from 'openagentmail';
 * 
 * const client = new OpenAgentMail({ apiKey: 'your_api_key' });
 * 
 * // Create an inbox
 * const inbox = await client.inboxes.create({
 *   username: 'support',
 *   domain: 'example.com'
 * });
 * 
 * // Send a message
 * await client.messages.send(inbox.inboxId, {
 *   to: ['user@example.com'],
 *   subject: 'Hello!',
 *   text: 'Hello from OpenAgentMail!'
 * });
 * 
 * // List messages with auto-pagination
 * for await (const message of client.messages.list(inbox.inboxId)) {
 *   console.log(message.subject);
 * }
 * ```
 */
export class OpenAgentMail {
  private readonly http: HttpClient;

  /** Client for managing inboxes */
  readonly inboxes: InboxesClient;

  /** Client for managing messages */
  readonly messages: MessagesClient;

  /** Client for managing drafts */
  readonly drafts: DraftsClient;

  /** Client for managing webhooks */
  readonly webhooks: WebhooksClient;

  /** Client for managing pods (multi-tenant isolation) */
  readonly pods: PodsClient;

  /** Client for managing domains */
  readonly domains: DomainsClient;

  /**
   * Create a new OpenAgentMail client
   * 
   * @param config - Configuration options
   * @param config.apiKey - Your API key (required)
   * @param config.baseUrl - Custom base URL (optional, defaults to https://api.openagentmail.com/v0)
   * @param config.timeout - Request timeout in milliseconds (optional, defaults to 30000)
   * @param config.fetch - Custom fetch implementation (optional)
   * 
   * @example
   * ```typescript
   * // Basic usage
   * const client = new OpenAgentMail({ apiKey: process.env.OAM_API_KEY });
   * 
   * // With custom options
   * const client = new OpenAgentMail({
   *   apiKey: process.env.OAM_API_KEY,
   *   baseUrl: 'https://api.custom.com/v0',
   *   timeout: 60000
   * });
   * ```
   */
  constructor(config: OpenAgentMailConfig) {
    if (!config.apiKey) {
      throw new Error('OpenAgentMail: apiKey is required');
    }

    this.http = new HttpClient(config);

    this.inboxes = new InboxesClient(this.http);
    this.messages = new MessagesClient(this.http);
    this.drafts = new DraftsClient(this.http);
    this.webhooks = new WebhooksClient(this.http);
    this.pods = new PodsClient(this.http);
    this.domains = new DomainsClient(this.http);
  }

  /**
   * Get organization details
   * 
   * @example
   * ```typescript
   * const org = await client.getOrganization();
   * console.log(org.name, org.plan);
   * ```
   */
  async getOrganization(): Promise<Organization> {
    return this.http.get<Organization>('/organization');
  }
}
