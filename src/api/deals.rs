//! Deals API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Deals endpoints.
#[derive(Clone, Debug)]
pub struct DealsApi {
    client: BrevoClient,
}

impl DealsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get deal attributes
    pub async fn get_crm_attributes_deals(&self) -> Result<Vec<GetCrmAttributesDealsResponseItem>> {
        let path = "/crm/attributes/deals";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all deals
    pub async fn get_crm_deals(
        &self,
        filters_attributes_deal_name: Option<&str>,
        filters_attributes_deal_owner: Option<&str>,
        filters_attributes_deal_stage: Option<&str>,
        filters_attributes_pipeline: Option<&str>,
        filters_linked_companies_ids: Option<&str>,
        filters_linked_contacts_ids: Option<&str>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
        sort: Option<GetCrmDealsSort>,
        sort_by: Option<&str>,
    ) -> Result<GetCrmDealsResponse> {
        let path = "/crm/deals";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = filters_attributes_deal_name {
                query_pairs.append_pair("filters[attributes.deal_name]", &val.to_string());
            }
            if let Some(val) = filters_attributes_deal_owner {
                query_pairs.append_pair("filters[attributes.deal_owner]", &val.to_string());
            }
            if let Some(val) = filters_attributes_deal_stage {
                query_pairs.append_pair("filters[attributes.deal_stage]", &val.to_string());
            }
            if let Some(val) = filters_attributes_pipeline {
                query_pairs.append_pair("filters[attributes.pipeline]", &val.to_string());
            }
            if let Some(val) = filters_linked_companies_ids {
                query_pairs.append_pair("filters[linkedCompaniesIds]", &val.to_string());
            }
            if let Some(val) = filters_linked_contacts_ids {
                query_pairs.append_pair("filters[linkedContactsIds]", &val.to_string());
            }
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = sort_by {
                query_pairs.append_pair("sortBy", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a deal
    pub async fn post_crm_deals(&self, body: &PostCrmDealsRequest) -> Result<PostCrmDealsResponse> {
        let path = "/crm/deals";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Import deals(creation and updation)
    pub async fn post_crm_deals_import(&self) -> Result<PostCrmDealsImportResponse> {
        let path = "/crm/deals/import";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Link and Unlink a deal with contacts and companies
    pub async fn patch_crm_deals_link_unlink_id(
        &self,
        id: &DealId,
        body: &PatchCrmDealsLinkUnlinkIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/deals/link-unlink/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a deal
    pub async fn delete_crm_deals_id(&self, id: &DealId) -> Result<serde_json::Value> {
        let path = format!("/crm/deals/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a deal
    pub async fn get_crm_deals_id(&self, id: &DealId) -> Result<serde_json::Value> {
        let path = format!("/crm/deals/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a deal
    pub async fn patch_crm_deals_id(
        &self,
        id: &DealId,
        body: &PatchCrmDealsIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/deals/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get pipeline stages
    pub async fn get_crm_pipeline_details(&self) -> Result<serde_json::Value> {
        let path = "/crm/pipeline/details";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all pipelines
    pub async fn get_crm_pipeline_details_all(&self) -> Result<serde_json::Value> {
        let path = "/crm/pipeline/details/all";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a pipeline
    pub async fn get_crm_pipeline_details_pipeline_id(
        &self,
        pipeline_id: &PipelineId,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/pipeline/details/{pipeline_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
