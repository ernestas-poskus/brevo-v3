//! Senders API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Senders endpoints.
#[derive(Clone, Debug)]
pub struct SendersApi {
    client: BrevoClient,
}

impl SendersApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get email senders
    pub async fn get_senders(
        &self,
        ip: Option<&str>,
        domain: Option<&DomainName>,
    ) -> Result<GetSendersResponse> {
        let path = "/senders";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = ip {
                query_pairs.append_pair("ip", &val.to_string());
            }
            if let Some(val) = domain {
                query_pairs.append_pair("domain", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create email sender
    pub async fn create_sender(&self, body: &CreateSenderRequest) -> Result<CreateSenderResponse> {
        let path = "/senders";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all the dedicated IPs for your account
    pub async fn get_ips(&self) -> Result<GetIpsResponse> {
        let path = "/senders/ips";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a sender
    pub async fn delete_sender(&self, sender_id: SenderId) -> Result<serde_json::Value> {
        let path = format!("/senders/{sender_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a sender
    pub async fn update_sender(
        &self,
        sender_id: SenderId,
        body: &UpdateSenderRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/senders/{sender_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all the dedicated IPs for a specific sender
    pub async fn get_ips_from_sender(
        &self,
        sender_id: SenderId,
    ) -> Result<GetIpsFromSenderResponse> {
        let path = format!("/senders/{sender_id}/ips");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Validate a sender using OTP
    pub async fn validate_sender_by_otp(
        &self,
        sender_id: SenderId,
        body: &ValidateSenderByOTPRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/senders/{sender_id}/validate");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
