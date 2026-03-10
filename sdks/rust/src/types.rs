//! Type definitions for OpenAgentMail API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Common Types
// ============================================================================

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
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

    pub(crate) fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(ref token) = self.page_token {
            params.push(("page_token", token.clone()));
        }
        params
    }
}

// ============================================================================
// Organization
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
// Pods
// ============================================================================

/// Pod for multi-tenant isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub pod_id: String,
    pub name: String,
    #[serde(default)]
    pub client_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a new pod
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

    pub fn builder() -> CreatePodRequestBuilder {
        CreatePodRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreatePodRequestBuilder {
    name: Option<String>,
    client_id: Option<String>,
}

impl CreatePodRequestBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn build(self) -> CreatePodRequest {
        CreatePodRequest {
            name: self.name.unwrap_or_default(),
            client_id: self.client_id,
        }
    }
}

/// Request to update a pod
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePodRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ============================================================================
// Inboxes
// ============================================================================

/// Email inbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub inbox_id: String,
    pub pod_id: String,
    pub username: String,
    pub domain: String,
    pub email: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a new inbox
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

    pub fn builder() -> CreateInboxRequestBuilder {
        CreateInboxRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateInboxRequestBuilder {
    username: Option<String>,
    domain: Option<String>,
    display_name: Option<String>,
    client_id: Option<String>,
}

impl CreateInboxRequestBuilder {
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn build(self) -> CreateInboxRequest {
        CreateInboxRequest {
            username: self.username.unwrap_or_default(),
            domain: self.domain,
            display_name: self.display_name,
            client_id: self.client_id,
        }
    }
}

/// Request to update an inbox
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInboxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

// ============================================================================
// Messages
// ============================================================================

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub inbox_id: String,
    pub thread_id: String,
    pub from: String,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub subject: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub html: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub created_at: String,
}

/// Email attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub attachment_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

/// Request to send a message
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageRequest {
    pub to: Vec<String>,
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
    pub reply_to_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

impl SendMessageRequest {
    pub fn builder() -> SendMessageRequestBuilder {
        SendMessageRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SendMessageRequestBuilder {
    to: Vec<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    subject: Option<String>,
    text: Option<String>,
    html: Option<String>,
    reply_to_message_id: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl SendMessageRequestBuilder {
    pub fn to(mut self, recipient: impl Into<String>) -> Self {
        self.to.push(recipient.into());
        self
    }

    pub fn to_many(mut self, recipients: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.to.extend(recipients.into_iter().map(|r| r.into()));
        self
    }

    pub fn cc(mut self, recipient: impl Into<String>) -> Self {
        self.cc.get_or_insert_with(Vec::new).push(recipient.into());
        self
    }

    pub fn bcc(mut self, recipient: impl Into<String>) -> Self {
        self.bcc.get_or_insert_with(Vec::new).push(recipient.into());
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

    pub fn reply_to(mut self, message_id: impl Into<String>) -> Self {
        self.reply_to_message_id = Some(message_id.into());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.get_or_insert_with(HashMap::new).insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> SendMessageRequest {
        SendMessageRequest {
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            text: self.text,
            html: self.html,
            reply_to_message_id: self.reply_to_message_id,
            headers: self.headers,
        }
    }
}

/// Message labels update request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateMessageLabelsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

/// Message list parameters
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

    pub fn thread_id(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        if let Some(ref thread_id) = self.thread_id {
            params.push(("thread_id", thread_id.clone()));
        }
        if let Some(ref label) = self.label {
            params.push(("label", label.clone()));
        }
        params
    }
}

// ============================================================================
// Drafts
// ============================================================================

/// Email draft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub draft_id: String,
    pub inbox_id: String,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub html: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub send_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a draft
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDraftRequest {
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
    pub send_at: Option<String>,
}

impl CreateDraftRequest {
    pub fn builder() -> CreateDraftRequestBuilder {
        CreateDraftRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateDraftRequestBuilder {
    to: Option<Vec<String>>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    subject: Option<String>,
    text: Option<String>,
    html: Option<String>,
    send_at: Option<String>,
}

impl CreateDraftRequestBuilder {
    pub fn to(mut self, recipient: impl Into<String>) -> Self {
        self.to.get_or_insert_with(Vec::new).push(recipient.into());
        self
    }

    pub fn cc(mut self, recipient: impl Into<String>) -> Self {
        self.cc.get_or_insert_with(Vec::new).push(recipient.into());
        self
    }

    pub fn bcc(mut self, recipient: impl Into<String>) -> Self {
        self.bcc.get_or_insert_with(Vec::new).push(recipient.into());
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

    pub fn send_at(mut self, send_at: impl Into<String>) -> Self {
        self.send_at = Some(send_at.into());
        self
    }

    pub fn build(self) -> CreateDraftRequest {
        CreateDraftRequest {
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            text: self.text,
            html: self.html,
            send_at: self.send_at,
        }
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
    pub send_at: Option<String>,
}

// ============================================================================
// Webhooks
// ============================================================================

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub webhook_id: String,
    pub url: String,
    pub event_types: Vec<String>,
    #[serde(default)]
    pub inbox_ids: Vec<String>,
    #[serde(default)]
    pub pod_ids: Vec<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    pub secret: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Available webhook event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookEventType {
    MessageReceived,
    MessageSent,
    MessageBounced,
    InboxCreated,
    InboxDeleted,
}

impl WebhookEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebhookEventType::MessageReceived => "message.received",
            WebhookEventType::MessageSent => "message.sent",
            WebhookEventType::MessageBounced => "message.bounced",
            WebhookEventType::InboxCreated => "inbox.created",
            WebhookEventType::InboxDeleted => "inbox.deleted",
        }
    }
}

impl std::fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Request to create a webhook
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub event_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl CreateWebhookRequest {
    pub fn builder() -> CreateWebhookRequestBuilder {
        CreateWebhookRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateWebhookRequestBuilder {
    url: Option<String>,
    event_types: Vec<String>,
    inbox_ids: Option<Vec<String>>,
    pod_ids: Option<Vec<String>>,
    client_id: Option<String>,
}

impl CreateWebhookRequestBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn event_type(mut self, event_type: WebhookEventType) -> Self {
        self.event_types.push(event_type.as_str().to_string());
        self
    }

    pub fn event_type_str(mut self, event_type: impl Into<String>) -> Self {
        self.event_types.push(event_type.into());
        self
    }

    pub fn inbox_id(mut self, inbox_id: impl Into<String>) -> Self {
        self.inbox_ids.get_or_insert_with(Vec::new).push(inbox_id.into());
        self
    }

    pub fn pod_id(mut self, pod_id: impl Into<String>) -> Self {
        self.pod_ids.get_or_insert_with(Vec::new).push(pod_id.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn build(self) -> CreateWebhookRequest {
        CreateWebhookRequest {
            url: self.url.unwrap_or_default(),
            event_types: self.event_types,
            inbox_ids: self.inbox_ids,
            pod_ids: self.pod_ids,
            client_id: self.client_id,
        }
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
    pub enabled: Option<bool>,
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
// Domains
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

impl std::fmt::Display for DomainStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainStatus::Pending => write!(f, "pending"),
            DomainStatus::Verifying => write!(f, "verifying"),
            DomainStatus::Verified => write!(f, "verified"),
            DomainStatus::Failed => write!(f, "failed"),
        }
    }
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
    #[serde(default)]
    pub pod_id: Option<String>,
    pub status: DomainStatus,
    #[serde(default)]
    pub dns_records: Vec<DnsRecord>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to add a domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDomainRequest {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
}

impl AddDomainRequest {
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            pod_id: None,
        }
    }

    pub fn with_pod(domain: impl Into<String>, pod_id: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            pod_id: Some(pod_id.into()),
        }
    }
}
