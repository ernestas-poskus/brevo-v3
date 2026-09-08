//! Payments API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Payments endpoints.
#[derive(Clone, Debug)]
pub struct PaymentsApi {
    client: BrevoClient,
}

impl PaymentsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Create a payment request
    pub async fn create_payment_request(
        &self,
        body: &CreatePaymentRequestRequest,
    ) -> Result<CreatePaymentRequestResponse> {
        let path = "/payments/requests";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a payment request.
    pub async fn delete_payment_request(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/payments/requests/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get payment request details
    pub async fn get_payment_request(&self, id: &str) -> Result<GetPaymentRequestResponse> {
        let path = format!("/payments/requests/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
