//! SMSCampaigns API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for SMSCampaigns endpoints.
#[derive(Clone, Debug)]
pub struct SMSCampaignsApi {
    client: BrevoClient,
}

impl SMSCampaignsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Returns the information for all your created SMS campaigns
    pub async fn get_sms_campaigns(
        &self,
        status: Option<GetSmsCampaignsStatus>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetSmsCampaignsSort>,
    ) -> Result<GetSmsCampaignsResponse> {
        let path = "/smsCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = status {
                query_pairs.append_pair("status", &val.to_string());
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Creates an SMS campaign
    pub async fn create_sms_campaign(
        &self,
        body: &CreateSmsCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = "/smsCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an SMS campaign
    pub async fn delete_sms_campaign(&self, campaign_id: i64) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get an SMS campaign
    pub async fn get_sms_campaign(&self, campaign_id: i64) -> Result<GetSmsCampaignResponse> {
        let path = format!("/smsCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an SMS campaign
    pub async fn update_sms_campaign(
        &self,
        campaign_id: i64,
        body: &UpdateSmsCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Export an SMS campaign's recipients
    pub async fn request_sms_recipient_export(
        &self,
        campaign_id: i64,
        body: &RequestSmsRecipientExportRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}/exportRecipients");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send your SMS campaign immediately
    pub async fn send_sms_campaign_now(&self, campaign_id: i64) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}/sendNow");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send an SMS campaign's report
    pub async fn send_sms_report(
        &self,
        campaign_id: i64,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}/sendReport");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send a test SMS campaign
    pub async fn send_test_sms(
        &self,
        campaign_id: i64,
        body: &SendTestSmsRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}/sendTest");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a campaign's status
    pub async fn update_sms_campaign_status(
        &self,
        campaign_id: i64,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/smsCampaigns/{campaign_id}/status");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
