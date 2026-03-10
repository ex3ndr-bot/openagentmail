//! Data types for OpenAgentMail SDK

use serde::{Deserialize, Serialize};

// ============================================================================
// Common Types
// ============================================================================

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub next_page_token: Option<String>,
    pub has_more: bool,
}

/// Pagination parameters
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }
}

// ============================================================================
// Organization Types
// ============================================================================

/// Organization details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub organization_id: String,
    pub name: String,
    pub plan: String,
    pub created_at: String,
}

// ============================================================================
// Pod Types
// ============================================================================

/// Pod for multi-tenant isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub pod_id: String,
    pub name: String,
    pub client_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a pod
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePodRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl CreatePodRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            client_id: None,
        }
    }

    pub fn client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = Some(id.into());
        self
    }
}

/// Request to update a pod
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePodRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdatePodRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

// ============================================================================
// Inbox Types
// ============================================================================

/// Email inbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub inbox_id: String,
    pub pod_id: String,
    pub username: String,
    pub domain: String,
    pub email: String,
    pub display_name: Option<String>,
    pub client_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create an inbox
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInboxRequest {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl CreateInboxRequest {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            ..Default::default()
        }
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }

    pub fn client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = Some(id.into());
        self
    }
}

/// Request to update an inbox
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInboxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl UpdateInboxRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }
}

/// Parameters for listing inboxes
#[derive(Debug, Clone, Default)]
pub struct ListInboxesParams {
    pub pagination: PaginationParams,
    pub pod_id: Option<String>,
}

impl ListInboxesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.pagination.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.pagination.page_token = Some(token.into());
        self
    }

    pub fn pod_id(mut self, id: impl Into<String>) -> Self {
        self.pod_id = Some(id.into());
        self
    }
}

// ============================================================================
// Message Types
// ============================================================================

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub inbox_id: String,
    pub thread_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
    pub attachments: Vec<Attachment>,
    pub labels: Vec<String>,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    pub created_at: String,
}

/// Attachment metadata (returned from API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub attachment_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

/// Attachment for sending (includes content)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInput {
    pub filename: String,
    pub content: String, // Base64-encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl AttachmentInput {
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
            content_type: None,
        }
    }

    pub fn content_type(mut self, mime: impl Into<String>) -> Self {
        self.content_type = Some(mime.into());
        self
    }
}

/// Request to send a message
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageRequest {
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<AttachmentInput>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl SendMessageRequest {
    pub fn new(to: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            to: vec![to.into()],
            subject: subject.into(),
            ..Default::default()
        }
    }

    pub fn to(mut self, recipients: Vec<String>) -> Self {
        self.to = recipients;
        self
    }

    pub fn add_to(mut self, recipient: impl Into<String>) -> Self {
        self.to.push(recipient.into());
        self
    }

    pub fn cc(mut self, recipients: Vec<String>) -> Self {
        self.cc = recipients;
        self
    }

    pub fn add_cc(mut self, recipient: impl Into<String>) -> Self {
        self.cc.push(recipient.into());
        self
    }

    pub fn bcc(mut self, recipients: Vec<String>) -> Self {
        self.bcc = recipients;
        self
    }

    pub fn add_bcc(mut self, recipient: impl Into<String>) -> Self {
        self.bcc.push(recipient.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn attachment(mut self, attachment: AttachmentInput) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(std::collections::HashMap::new)
            .insert(key.into(), value.into());
        self
    }
}

/// Request to reply to a message
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<AttachmentInput>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
}

impl ReplyRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn attachment(mut self, attachment: AttachmentInput) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }
}

/// Parameters for listing messages
#[derive(Debug, Clone, Default)]
pub struct ListMessagesParams {
    pub pagination: PaginationParams,
    pub thread_id: Option<String>,
    pub label: Option<String>,
}

impl ListMessagesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.pagination.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.pagination.page_token = Some(token.into());
        self
    }

    pub fn thread_id(mut self, id: impl Into<String>) -> Self {
        self.thread_id = Some(id.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

// ============================================================================
// Draft Types
// ============================================================================

/// Email draft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub draft_id: String,
    pub inbox_id: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: Option<String>,
    pub text: Option<String>,
    pub html: Option<String>,
    pub attachments: Vec<Attachment>,
    pub send_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a draft
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDraftRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<AttachmentInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

impl CreateDraftRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(mut self, recipients: Vec<String>) -> Self {
        self.to = recipients;
        self
    }

    pub fn add_to(mut self, recipient: impl Into<String>) -> Self {
        self.to.push(recipient.into());
        self
    }

    pub fn cc(mut self, recipients: Vec<String>) -> Self {
        self.cc = recipients;
        self
    }

    pub fn bcc(mut self, recipients: Vec<String>) -> Self {
        self.bcc = recipients;
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn attachment(mut self, attachment: AttachmentInput) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn send_at(mut self, time: impl Into<String>) -> Self {
        self.send_at = Some(time.into());
        self
    }
}

/// Request to update a draft
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDraftRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

impl UpdateDraftRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(mut self, recipients: Vec<String>) -> Self {
        self.to = Some(recipients);
        self
    }

    pub fn cc(mut self, recipients: Vec<String>) -> Self {
        self.cc = Some(recipients);
        self
    }

    pub fn bcc(mut self, recipients: Vec<String>) -> Self {
        self.bcc = Some(recipients);
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn send_at(mut self, time: impl Into<String>) -> Self {
        self.send_at = Some(time.into());
        self
    }
}

// ============================================================================
// Webhook Types
// ============================================================================

/// Webhook event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "message.received")]
    MessageReceived,
    #[serde(rename = "message.sent")]
    MessageSent,
    #[serde(rename = "message.delivered")]
    MessageDelivered,
    #[serde(rename = "message.bounced")]
    MessageBounced,
    #[serde(rename = "message.complained")]
    MessageComplained,
    #[serde(rename = "message.rejected")]
    MessageRejected,
}

impl WebhookEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebhookEventType::MessageReceived => "message.received",
            WebhookEventType::MessageSent => "message.sent",
            WebhookEventType::MessageDelivered => "message.delivered",
            WebhookEventType::MessageBounced => "message.bounced",
            WebhookEventType::MessageComplained => "message.complained",
            WebhookEventType::MessageRejected => "message.rejected",
        }
    }
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub webhook_id: String,
    pub url: String,
    pub event_types: Vec<String>,
    pub inbox_ids: Vec<String>,
    pub pod_ids: Vec<String>,
    pub client_id: Option<String>,
    pub secret: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub event_types: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inbox_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pod_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl CreateWebhookRequest {
    pub fn new(url: impl Into<String>, event_types: Vec<WebhookEventType>) -> Self {
        Self {
            url: url.into(),
            event_types: event_types.iter().map(|e| e.as_str().to_string()).collect(),
            inbox_ids: Vec::new(),
            pod_ids: Vec::new(),
            client_id: None,
            secret: None,
        }
    }

    pub fn inbox_id(mut self, id: impl Into<String>) -> Self {
        self.inbox_ids.push(id.into());
        self
    }

    pub fn pod_id(mut self, id: impl Into<String>) -> Self {
        self.pod_ids.push(id.into());
        self
    }

    pub fn client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = Some(id.into());
        self
    }

    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }
}

/// Request to update a webhook
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl UpdateWebhookRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn event_types(mut self, types: Vec<WebhookEventType>) -> Self {
        self.event_types = Some(types.iter().map(|e| e.as_str().to_string()).collect());
        self
    }

    pub fn inbox_ids(mut self, ids: Vec<String>) -> Self {
        self.inbox_ids = Some(ids);
        self
    }

    pub fn pod_ids(mut self, ids: Vec<String>) -> Self {
        self.pod_ids = Some(ids);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

/// Webhook event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created_at: String,
    pub data: serde_json::Value,
}

// ============================================================================
// Domain Types
// ============================================================================

/// Domain status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DomainStatus {
    Pending,
    Verifying,
    Verified,
    Failed,
}

/// DNS record for domain verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub value: String,
}

/// Custom domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub domain_id: String,
    pub domain: String,
    pub pod_id: Option<String>,
    pub status: DomainStatus,
    pub dns_records: Vec<DnsRecord>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to add a domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
}

impl CreateDomainRequest {
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            pod_id: None,
        }
    }

    pub fn pod_id(mut self, id: impl Into<String>) -> Self {
        self.pod_id = Some(id.into());
        self
    }
}
