use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::types::{CreatePodRequest, PaginatedResponse, PaginationParams, Pod, UpdatePodRequest};

#[derive(Debug, Clone)]
pub struct Pods { inner: Arc<ClientInner> }

impl Pods {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self { Self { inner } }

    pub async fn create(&self, request: CreatePodRequest) -> Result<Pod> {
        let response = self.inner.post("/pods").json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn list(&self, params: PaginationParams) -> Result<PaginatedResponse<Pod>> {
        let mut request = self.inner.get("/pods");
        for (key, value) in params.to_query_params() { request = request.query(&[(key, value)]); }
        let response = request.send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn get(&self, pod_id: &str) -> Result<Pod> {
        let response = self.inner.get(&format!("/pods/{}", pod_id)).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn update(&self, pod_id: &str, request: UpdatePodRequest) -> Result<Pod> {
        let response = self.inner.patch(&format!("/pods/{}", pod_id)).json(&request).send().await?;
        self.inner.handle_response(response).await
    }

    pub async fn delete(&self, pod_id: &str) -> Result<()> {
        let response = self.inner.delete(&format!("/pods/{}", pod_id)).send().await?;
        self.inner.handle_empty_response(response).await
    }
}
