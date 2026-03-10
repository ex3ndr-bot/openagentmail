//! Domains resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{CreateDomainRequest, Domain, PaginatedResponse, PaginationParams};

/// Operations on custom domains
pub struct DomainsResource<'a> {
    client: &'a ApiClient,
}

impl<'a> DomainsResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Add a custom domain
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreateDomainRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let domain = client.domains().create(
    ///         CreateDomainRequest::new("mail.mycompany.com")
    ///             .pod_id("pod_xyz789")
    ///     ).await?;
    ///     
    ///     println!("Domain added: {}", domain.domain);
    ///     println!("Status: {:?}", domain.status);
    ///     
    ///     // DNS records to configure
    ///     for record in &domain.dns_records {
    ///         println!("{} {} -> {}", record.record_type, record.name, record.value);
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, request: CreateDomainRequest) -> Result<Domain> {
        self.client.post("domains", &request).await
    }

    /// List all custom domains
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, PaginationParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let domains = client.domains().list(PaginationParams::new()).await?;
    ///     for domain in domains.items {
    ///         println!("{} - {:?}", domain.domain, domain.status);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Domain>> {
        let query = self.client.build_pagination_query(&params);
        self.client.get_with_query("domains", &query).await
    }

    /// Get a domain by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let domain = client.domains().get("dom_stu901").await?;
    ///     println!("Domain: {} ({:?})", domain.domain, domain.status);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, domain_id: &str) -> Result<Domain> {
        self.client.get(&format!("domains/{}", domain_id)).await
    }

    /// Trigger domain verification
    ///
    /// Re-checks DNS records to verify domain ownership.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     
    ///     let domain = client.domains().verify("dom_stu901").await?;
    ///     println!("Verification status: {:?}", domain.status);
    ///     Ok(())
    /// }
    /// ```
    pub async fn verify(&self, domain_id: &str) -> Result<Domain> {
        self.client
            .post_empty(&format!("domains/{}/verify", domain_id))
            .await
    }

    /// Delete a custom domain
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.domains().delete("dom_stu901").await?;
    ///     println!("Domain deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, domain_id: &str) -> Result<()> {
        self.client.delete(&format!("domains/{}", domain_id)).await
    }
}
