//! Pods resource

use crate::client::ApiClient;
use crate::error::Result;
use crate::types::{CreatePodRequest, PaginatedResponse, PaginationParams, Pod, UpdatePodRequest};

/// Operations on pods
pub struct PodsResource<'a> {
    client: &'a ApiClient,
}

impl<'a> PodsResource<'a> {
    pub(crate) fn new(client: &'a ApiClient) -> Self {
        Self { client }
    }

    /// Create a new pod
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, CreatePodRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let pod = client.pods().create(
    ///         CreatePodRequest::new("Production").client_id("prod-001")
    ///     ).await?;
    ///     println!("Created pod: {}", pod.pod_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, request: CreatePodRequest) -> Result<Pod> {
        self.client.post("pods", &request).await
    }

    /// List all pods
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, PaginationParams};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let pods = client.pods().list(PaginationParams::new().limit(10)).await?;
    ///     for pod in pods.items {
    ///         println!("Pod: {} ({})", pod.name, pod.pod_id);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Pod>> {
        let query = self.client.build_pagination_query(&params);
        self.client.get_with_query("pods", &query).await
    }

    /// Get a pod by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let pod = client.pods().get("pod_xyz789").await?;
    ///     println!("Pod: {}", pod.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, pod_id: &str) -> Result<Pod> {
        self.client.get(&format!("pods/{}", pod_id)).await
    }

    /// Update a pod
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::{OpenAgentMail, UpdatePodRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     let pod = client.pods().update(
    ///         "pod_xyz789",
    ///         UpdatePodRequest::new().name("Production v2")
    ///     ).await?;
    ///     println!("Updated pod: {}", pod.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(&self, pod_id: &str, request: UpdatePodRequest) -> Result<Pod> {
        self.client.put(&format!("pods/{}", pod_id), &request).await
    }

    /// Delete a pod
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openagentmail::OpenAgentMail;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OpenAgentMail::new("your-api-key")?;
    ///     client.pods().delete("pod_xyz789").await?;
    ///     println!("Pod deleted");
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, pod_id: &str) -> Result<()> {
        self.client.delete(&format!("pods/{}", pod_id)).await
    }
}
