// OpenAgentMail TypeScript Types
// Auto-generated from API specification

// ============================================================================
// Core Types
// ============================================================================

/**
 * Configuration options for the OpenAgentMail client
 */
export interface OpenAgentMailConfig {
  /** Your API key (org_key or pod_key) */
  apiKey: string;
  /** Base URL for the API (default: https://api.openagentmail.com/v0) */
  baseUrl?: string;
  /** Request timeout in milliseconds (default: 30000) */
  timeout?: number;
  /** Custom fetch implementation */
  fetch?: typeof fetch;
}

/**
 * Pagination parameters for list endpoints
 */
export interface PaginationParams {
  /** Number of items per page (default: 20, max: 100) */
  limit?: number;
  /** Cursor for the next page */
  pageToken?: string;
}

/**
 * Paginated response wrapper
 */
export interface PaginatedResponse<T> {
  /** Array of items */
  items: T[];
  /** Token for fetching the next page */
  nextPageToken: string | null;
  /** Whether there are more items to fetch */
  hasMore: boolean;
}

// ============================================================================
// Organization
// ============================================================================

export interface Organization {
  organizationId: string;
  name: string;
  plan: string;
  createdAt: string;
}

// ============================================================================
// Pods
// ============================================================================

export interface Pod {
  podId: string;
  name: string;
  clientId: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreatePodParams {
  /** Pod display name */
  name: string;
  /** Idempotency key */
  clientId?: string;
}

export interface UpdatePodParams {
  /** New pod display name */
  name: string;
}

// ============================================================================
// Inboxes
// ============================================================================

export interface Inbox {
  inboxId: string;
  podId: string;
  username: string;
  domain: string;
  /** Full email address (username@domain) */
  email: string;
  displayName: string | null;
  clientId: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateInboxParams {
  /** Local part of the email address */
  username: string;
  /** Domain for the inbox (must be verified) */
  domain?: string;
  /** Display name for the inbox */
  displayName?: string;
  /** Pod ID (optional, for multi-tenant setups) */
  podId?: string;
  /** Idempotency key */
  clientId?: string;
}

export interface UpdateInboxParams {
  /** New display name */
  displayName?: string;
}

export interface ListInboxesParams extends PaginationParams {
  /** Filter by pod ID */
  podId?: string;
}

// ============================================================================
// Messages
// ============================================================================

export interface Attachment {
  attachmentId: string;
  filename: string;
  contentType: string;
  /** Size in bytes */
  size: number;
}

export interface Message {
  messageId: string;
  inboxId: string;
  threadId: string;
  from: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  text: string | null;
  html: string | null;
  attachments: Attachment[];
  labels: string[];
  headers: Record<string, string>;
  createdAt: string;
}

export interface SendMessageParams {
  /** Recipient email addresses */
  to: string[];
  /** CC recipients */
  cc?: string[];
  /** BCC recipients */
  bcc?: string[];
  /** Email subject */
  subject: string;
  /** Plain text body */
  text?: string;
  /** HTML body */
  html?: string;
  /** Thread ID to reply to */
  threadId?: string;
  /** Custom headers */
  headers?: Record<string, string>;
  /** Idempotency key */
  clientId?: string;
}

export interface ListMessagesParams extends PaginationParams {
  /** Filter by label */
  label?: string;
  /** Filter by thread ID */
  threadId?: string;
}

export interface UpdateMessageParams {
  /** Labels to add */
  addLabels?: string[];
  /** Labels to remove */
  removeLabels?: string[];
}

// ============================================================================
// Drafts
// ============================================================================

export interface Draft {
  draftId: string;
  inboxId: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string | null;
  text: string | null;
  html: string | null;
  attachments: Attachment[];
  /** Scheduled send time (ISO 8601) */
  sendAt: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateDraftParams {
  /** Recipient email addresses */
  to?: string[];
  /** CC recipients */
  cc?: string[];
  /** BCC recipients */
  bcc?: string[];
  /** Email subject */
  subject?: string;
  /** Plain text body */
  text?: string;
  /** HTML body */
  html?: string;
  /** Scheduled send time (ISO 8601) */
  sendAt?: string;
  /** Idempotency key */
  clientId?: string;
}

export interface UpdateDraftParams {
  /** Recipient email addresses */
  to?: string[];
  /** CC recipients */
  cc?: string[];
  /** BCC recipients */
  bcc?: string[];
  /** Email subject */
  subject?: string;
  /** Plain text body */
  text?: string;
  /** HTML body */
  html?: string;
  /** Scheduled send time (ISO 8601) */
  sendAt?: string | null;
}

// ============================================================================
// Webhooks
// ============================================================================

export type WebhookEventType =
  | 'message.received'
  | 'message.sent'
  | 'message.bounced'
  | 'inbox.created'
  | 'inbox.deleted';

export interface Webhook {
  webhookId: string;
  url: string;
  eventTypes: WebhookEventType[];
  inboxIds: string[];
  podIds: string[];
  clientId: string | null;
  secret: string;
  enabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateWebhookParams {
  /** Webhook endpoint URL */
  url: string;
  /** Event types to subscribe to */
  eventTypes: WebhookEventType[];
  /** Filter by inbox IDs (empty = all inboxes) */
  inboxIds?: string[];
  /** Filter by pod IDs (empty = all pods) */
  podIds?: string[];
  /** Idempotency key */
  clientId?: string;
}

export interface UpdateWebhookParams {
  /** New webhook endpoint URL */
  url?: string;
  /** New event types to subscribe to */
  eventTypes?: WebhookEventType[];
  /** Filter by inbox IDs */
  inboxIds?: string[];
  /** Filter by pod IDs */
  podIds?: string[];
  /** Enable or disable the webhook */
  enabled?: boolean;
}

export interface WebhookEvent {
  id: string;
  type: WebhookEventType;
  createdAt: string;
  data: Record<string, unknown>;
}

// ============================================================================
// Domains
// ============================================================================

export type DomainStatus = 'pending' | 'verifying' | 'verified' | 'failed';

export interface DnsRecord {
  type: 'TXT' | 'CNAME' | 'MX';
  name: string;
  value: string;
}

export interface Domain {
  domainId: string;
  domain: string;
  podId: string | null;
  status: DomainStatus;
  dnsRecords: DnsRecord[];
  createdAt: string;
  updatedAt: string;
}

export interface AddDomainParams {
  /** Domain name to add */
  domain: string;
  /** Pod ID (optional, for multi-tenant setups) */
  podId?: string;
  /** Idempotency key */
  clientId?: string;
}

export interface ListDomainsParams extends PaginationParams {
  /** Filter by pod ID */
  podId?: string;
}

// ============================================================================
// API Response Types
// ============================================================================

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

export interface ApiErrorResponse {
  error: ApiError;
}

// ============================================================================
// Internal Types (for SDK implementation)
// ============================================================================

export interface RequestOptions {
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  path: string;
  body?: unknown;
  query?: Record<string, string | number | boolean | undefined>;
}

/** Raw API response types (snake_case) for internal transformation */
export interface RawPaginatedResponse<T> {
  items: T[];
  next_page_token: string | null;
  has_more: boolean;
}
