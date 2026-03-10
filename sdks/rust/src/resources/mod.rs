//! API resource modules

pub mod domains;
pub mod drafts;
pub mod inboxes;
pub mod messages;
pub mod organization;
pub mod pods;
pub mod webhooks;

pub use domains::DomainsResource;
pub use drafts::DraftsResource;
pub use inboxes::InboxesResource;
pub use messages::MessagesResource;
pub use organization::OrganizationResource;
pub use pods::PodsResource;
pub use webhooks::WebhooksResource;
