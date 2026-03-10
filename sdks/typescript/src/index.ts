// OpenAgentMail TypeScript SDK
// https://openagentmail.com

export { OpenAgentMail } from './client.js';

// Types
export type {
  OpenAgentMailConfig,
  PaginationParams,
  PaginatedResponse,
  Organization,
  Pod,
  CreatePodParams,
  UpdatePodParams,
  Inbox,
  CreateInboxParams,
  UpdateInboxParams,
  ListInboxesParams,
  Attachment,
  Message,
  SendMessageParams,
  ListMessagesParams,
  UpdateMessageParams,
  Draft,
  CreateDraftParams,
  UpdateDraftParams,
  Webhook,
  WebhookEventType,
  CreateWebhookParams,
  UpdateWebhookParams,
  WebhookEvent,
  Domain,
  DomainStatus,
  DnsRecord,
  AddDomainParams,
  ListDomainsParams,
  ApiError,
  ApiErrorResponse,
} from './types.js';

// Errors
export {
  OpenAgentMailError,
  APIError,
  BadRequestError,
  AuthenticationError,
  AuthorizationError,
  NotFoundError,
  ConflictError,
  ValidationError,
  RateLimitError,
  InternalServerError,
  TimeoutError,
  NetworkError,
} from './errors.js';

// Pagination utilities
export { PageIterator, paginate } from './utils/pagination.js';

// Resource clients (for advanced usage)
export { InboxesClient } from './resources/inboxes.js';
export { MessagesClient } from './resources/messages.js';
export { DraftsClient } from './resources/drafts.js';
export { WebhooksClient } from './resources/webhooks.js';
export { PodsClient } from './resources/pods.js';
export { DomainsClient } from './resources/domains.js';
