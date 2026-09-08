//! Companies API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Companies endpoints.
#[derive(Clone, Debug)]
pub struct CompaniesApi {
    client: BrevoClient,
}

impl CompaniesApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all Companies
    pub async fn get_companies(
        &self,
        filters_attributes_name: Option<&str>,
        linked_contacts_ids: Option<i64>,
        linked_deals_ids: Option<&str>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
        page: Option<i64>,
        limit: Option<i64>,
        sort: Option<GetCompaniesSort>,
        sort_by: Option<&str>,
    ) -> Result<GetCompaniesResponse> {
        let path = "/companies";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = filters_attributes_name {
                query_pairs.append_pair("filters[attributes.name]", &val.to_string());
            }
            if let Some(val) = linked_contacts_ids {
                query_pairs.append_pair("linkedContactsIds", &val.to_string());
            }
            if let Some(val) = linked_deals_ids {
                query_pairs.append_pair("linkedDealsIds", &val.to_string());
            }
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
            if let Some(val) = page {
                query_pairs.append_pair("page", &val.to_string());
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

    /// Create a company
    pub async fn post_companies(
        &self,
        body: &PostCompaniesRequest,
    ) -> Result<PostCompaniesResponse> {
        let path = "/companies";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Import companies(creation and updation)
    pub async fn post_companies_import(&self) -> Result<PostCompaniesImportResponse> {
        let path = "/companies/import";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Link and Unlink company with contact and deal
    pub async fn patch_companies_link_unlink_id(
        &self,
        id: &CompanyId,
        body: &PatchCompaniesLinkUnlinkIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/companies/link-unlink/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a company
    pub async fn delete_companies_id(&self, id: &CompanyId) -> Result<serde_json::Value> {
        let path = format!("/companies/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a company
    pub async fn get_companies_id(&self, id: &CompanyId) -> Result<serde_json::Value> {
        let path = format!("/companies/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a company
    pub async fn patch_companies_id(
        &self,
        id: &CompanyId,
        body: &PatchCompaniesIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/companies/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a company/deal attribute
    pub async fn post_crm_attributes(
        &self,
        body: &PostCrmAttributesRequest,
    ) -> Result<PostCrmAttributesResponse> {
        let path = "/crm/attributes";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an attribute
    pub async fn delete_crm_attributes_id(&self, id: &CompanyId) -> Result<serde_json::Value> {
        let path = format!("/crm/attributes/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an attribute
    pub async fn patch_crm_attributes_id(
        &self,
        id: &CompanyId,
        body: &PatchCrmAttributesIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/attributes/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get company attributes
    pub async fn get_crm_attributes_companies(
        &self,
    ) -> Result<Vec<GetCrmAttributesCompaniesResponseItem>> {
        let path = "/crm/attributes/companies";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
