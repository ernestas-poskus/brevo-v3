//! EmailCampaigns API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for EmailCampaigns endpoints.
#[derive(Clone, Debug)]
pub struct EmailCampaignsApi {
    client: BrevoClient,
}

impl EmailCampaignsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Return all your created email campaigns
    pub async fn get_email_campaigns(
        &self,
        type_: Option<GetEmailCampaignsType>,
        status: Option<GetEmailCampaignsStatus>,
        statistics: Option<GetEmailCampaignsStatistics>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetEmailCampaignsSort>,
        exclude_html_content: Option<bool>,
        exclude_pdf_attachment: Option<bool>,
    ) -> Result<GetEmailCampaignsResponse> {
        let path = "/emailCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = type_ {
                query_pairs.append_pair("type", &val.to_string());
            }
            if let Some(val) = status {
                query_pairs.append_pair("status", &val.to_string());
            }
            if let Some(val) = statistics {
                query_pairs.append_pair("statistics", &val.to_string());
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
            if let Some(val) = exclude_html_content {
                query_pairs.append_pair("excludeHtmlContent", &val.to_string());
            }
            if let Some(val) = exclude_pdf_attachment {
                query_pairs.append_pair("excludePdfAttachment", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create an email campaign
    pub async fn create_email_campaign(
        &self,
        body: &CreateEmailCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = "/emailCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Upload an image to your account's image gallery
    pub async fn upload_image_to_gallery(
        &self,
        body: &UploadImageToGalleryRequest,
    ) -> Result<UploadImageToGalleryResponse> {
        let path = "/emailCampaigns/images";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an email campaign
    pub async fn delete_email_campaign(
        &self,
        campaign_id: CampaignId,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get an email campaign report
    pub async fn get_email_campaign(
        &self,
        campaign_id: CampaignId,
        statistics: Option<GetEmailCampaignStatistics>,
        exclude_html_content: Option<bool>,
    ) -> Result<GetEmailCampaignResponse> {
        let path = format!("/emailCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = statistics {
                query_pairs.append_pair("statistics", &val.to_string());
            }
            if let Some(val) = exclude_html_content {
                query_pairs.append_pair("excludeHtmlContent", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an email campaign
    pub async fn update_email_campaign(
        &self,
        campaign_id: CampaignId,
        body: &UpdateEmailCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get an A/B test email campaign results
    pub async fn get_ab_test_campaign_result(
        &self,
        campaign_id: CampaignId,
    ) -> Result<GetAbTestCampaignResultResponse> {
        let path = format!("/emailCampaigns/{campaign_id}/abTestCampaignResult");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Export the recipients of an email campaign
    pub async fn email_export_recipients(
        &self,
        campaign_id: CampaignId,
        body: &EmailExportRecipientsRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}/exportRecipients");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send an email campaign immediately, based on campaignId
    pub async fn send_email_campaign_now(
        &self,
        campaign_id: CampaignId,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}/sendNow");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send the report of a campaign
    pub async fn send_report(
        &self,
        campaign_id: CampaignId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}/sendReport");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send an email campaign to your test list
    pub async fn send_test_email(
        &self,
        campaign_id: CampaignId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}/sendTest");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a shared template url
    pub async fn get_shared_template_url(
        &self,
        campaign_id: CampaignId,
    ) -> Result<GetSharedTemplateUrlResponse> {
        let path = format!("/emailCampaigns/{campaign_id}/sharedUrl");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an email campaign status
    pub async fn update_campaign_status(
        &self,
        campaign_id: CampaignId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/emailCampaigns/{campaign_id}/status");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
