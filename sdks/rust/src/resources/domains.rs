use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{AddDomainRequest, Domain, PaginatedResponse, PaginationParams};

#[derive(Debug, Clone)]
pub struct Domains { inner: Arc<ClientInner> }

impl Domains {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn add(&self, request: AddDomainRequest) -> Result<Domain> {
        let response = self.inner.post("/domains").json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Domain>> {
        let mut request = self.inner.get("/domains");
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, domain_id: &str) -> Result<Domain> {
        let response = self.inner.get(&format!("/domains/{}", domain_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn verify(&self, domain_id: &str) -> Result<Domain> {
        let response = self.inner.post(&format!("/domains/{}/verify", domain_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, domain_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/domains/{}", domain_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }
}
