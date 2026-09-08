//! WhatsAppCampaigns API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for WhatsAppCampaigns endpoints.
#[derive(Clone, Debug)]
pub struct WhatsAppCampaignsApi {
    client: BrevoClient,
}

impl WhatsAppCampaignsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Return all your created WhatsApp campaigns
    pub async fn get_whats_app_campaigns(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetWhatsAppCampaignsSort>,
    ) -> Result<GetWhatsAppCampaignsResponse> {
        let path = "/whatsappCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
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

    /// Create and Send a WhatsApp campaign
    pub async fn create_whats_app_campaign(
        &self,
        body: &CreateWhatsAppCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = "/whatsappCampaigns";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get your WhatsApp API account information
    pub async fn get_whats_app_config(&self) -> Result<GetWhatsAppConfigResponse> {
        let path = "/whatsappCampaigns/config";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a WhatsApp template
    pub async fn create_whats_app_template(
        &self,
        body: &CreateWhatsAppTemplateRequest,
    ) -> Result<serde_json::Value> {
        let path = "/whatsappCampaigns/template";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Return all your created WhatsApp templates
    pub async fn get_whats_app_templates(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetWhatsAppTemplatesSort>,
        source: Option<GetWhatsAppTemplatesSource>,
    ) -> Result<GetWhatsAppTemplatesResponse> {
        let path = "/whatsappCampaigns/template-list";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
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
            if let Some(val) = source {
                query_pairs.append_pair("source", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send your WhatsApp template for approval
    pub async fn send_whats_app_template_approval(
        &self,
        template_id: i64,
    ) -> Result<serde_json::Value> {
        let path = format!("/whatsappCampaigns/template/approval/{template_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a WhatsApp campaign
    pub async fn delete_whats_app_campaign(&self, campaign_id: i64) -> Result<serde_json::Value> {
        let path = format!("/whatsappCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a WhatsApp campaign
    pub async fn get_whats_app_campaign(
        &self,
        campaign_id: i64,
    ) -> Result<GetWhatsAppCampaignResponse> {
        let path = format!("/whatsappCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a WhatsApp campaign
    pub async fn update_whats_app_campaign(
        &self,
        campaign_id: i64,
        body: &UpdateWhatsAppCampaignRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/whatsappCampaigns/{campaign_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
