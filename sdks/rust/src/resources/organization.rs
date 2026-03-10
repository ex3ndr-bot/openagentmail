//! Organization resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::Organization;

/// Operations on the organization
pub struct OrganizationResource<'a> {
    client: &'a ApiClient,
}

impl<'a> OrganizationResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Get organization details
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let org = client.organization().get().await?;
    ///     println!("Organization: {}", org.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self) -> Result<Organization> {
        self.client.get("organization").await
    }
}
