//! Webhooks API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Webhooks endpoints.
#[derive(Clone, Debug)]
pub struct WebhooksApi {
    client: BrevoClient,
}

impl WebhooksApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all webhooks
    pub async fn get_webhooks(
        &self,
        type_: Option<GetWebhooksType>,
        sort: Option<GetWebhooksSort>,
    ) -> Result<GetWebhooksResponse> {
        let path = "/webhooks";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = type_ {
                query_pairs.append_pair("type", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a webhook
    pub async fn create_webhook(&self, body: &CreateWebhookRequest) -> Result<serde_json::Value> {
        let path = "/webhooks";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Export webhook event history as CSV file
    pub async fn export_webhooks_history(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = "/webhooks/export";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a webhook
    pub async fn delete_webhook(&self, webhook_id: WebhookId) -> Result<serde_json::Value> {
        let path = format!("/webhooks/{webhook_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a webhook details
    pub async fn get_webhook(&self, webhook_id: WebhookId) -> Result<serde_json::Value> {
        let path = format!("/webhooks/{webhook_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a webhook
    pub async fn update_webhook(
        &self,
        webhook_id: WebhookId,
        body: &UpdateWebhookRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/webhooks/{webhook_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
