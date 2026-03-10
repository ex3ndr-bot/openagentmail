// OpenAgentMail API Types

// Pagination
export interface PaginationParams {
  limit?: number;
  page_token?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  next_page_token: string | null;
  has_more: boolean;
}

// API Error
export interface ApiError {
  error: {
    code: string;
    message: string;
    details?: Record<string, unknown>;
  };
}

export type ErrorCode = 
  | 'invalid_request'
  | 'validation_error'
  | 'authentication_error'
  | 'authorization_error'
  | 'not_found'
  | 'conflict'
  | 'rate_limit_exceeded'
  | 'internal_error';

// Organization
export interface Organization {
  organization_id: string;
  name: string;
  plan: string;
  created_at: string;
}

// Pod
export interface Pod {
  pod_id: string;
  name: string;
  client_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreatePodRequest {
  name: string;
  client_id?: string;
}

// Inbox
export interface Inbox {
  inbox_id: string;
  pod_id: string;
  username: string;
  domain: string;
  email: string;
  display_name: string | null;
  client_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateInboxRequest {
  pod_id: string;
  username: string;
  domain?: string;
  display_name?: string;
  client_id?: string;
}

export interface UpdateInboxRequest {
  display_name?: string;
}

// Attachment
export interface Attachment {
  attachment_id: string;
  filename: string;
  content_type: string;
  size: number;
}

// Message
export interface Message {
  message_id: string;
  inbox_id: string;
  thread_id: string;
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
  created_at: string;
}

export interface SendMessageRequest {
  to: string[];
  cc?: string[];
  bcc?: string[];
  subject: string;
  text?: string;
  html?: string;
  headers?: Record<string, string>;
  reply_to?: string;
}

export interface ListMessagesParams extends PaginationParams {
  thread_id?: string;
  label?: string;
  before?: string;
  after?: string;
}

// Draft
export interface Draft {
  draft_id: string;
  inbox_id: string;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string | null;
  text: string | null;
  html: string | null;
  attachments: Attachment[];
  send_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateDraftRequest {
  to?: string[];
  cc?: string[];
  bcc?: string[];
  subject?: string;
  text?: string;
  html?: string;
  send_at?: string;
}

export interface UpdateDraftRequest {
  to?: string[];
  cc?: string[];
  bcc?: string[];
  subject?: string;
  text?: string;
  html?: string;
  send_at?: string;
}

// Webhook
export interface Webhook {
  webhook_id: string;
  url: string;
  event_types: string[];
  inbox_ids: string[];
  pod_ids: string[];
  client_id: string | null;
  secret: string;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateWebhookRequest {
  url: string;
  event_types: string[];
  inbox_ids?: string[];
  pod_ids?: string[];
  client_id?: string;
}

export interface UpdateWebhookRequest {
  url?: string;
  event_types?: string[];
  inbox_ids?: string[];
  pod_ids?: string[];
  enabled?: boolean;
}

export type WebhookEventType = 
  | 'message.received'
  | 'message.sent'
  | 'message.delivered'
  | 'message.bounced';

export interface WebhookEvent {
  id: string;
  type: string;
  created_at: string;
  data: Record<string, unknown>;
}

// DNS Record
export interface DnsRecord {
  type: 'TXT' | 'CNAME' | 'MX';
  name: string;
  value: string;
}

// Domain
export interface Domain {
  domain_id: string;
  domain: string;
  pod_id: string | null;
  status: 'pending' | 'verifying' | 'verified' | 'failed';
  dns_records: DnsRecord[];
  created_at: string;
  updated_at: string;
}

export interface CreateDomainRequest {
  domain: string;
  pod_id?: string;
}

// Auth context
export interface AuthContext {
  organizationId: string;
  podId?: string;
  keyType: 'org_key' | 'pod_key';
}
