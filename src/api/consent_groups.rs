//! ConsentGroups API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for ConsentGroups endpoints.
#[derive(Clone, Debug)]
pub struct ConsentGroupsApi {
    client: BrevoClient,
}

impl ConsentGroupsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// List all consent groups
    pub async fn get_consent_groups(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        id: Option<i64>,
        name: Option<&str>,
        signup_mode: Option<GetConsentGroupsSignupMode>,
    ) -> Result<serde_json::Value> {
        let path = "/contacts/consent-groups";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = id {
                query_pairs.append_pair("id", &val.to_string());
            }
            if let Some(val) = name {
                query_pairs.append_pair("name", &val.to_string());
            }
            if let Some(val) = signup_mode {
                query_pairs.append_pair("signupMode", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a consent group
    pub async fn create_consent_group(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = "/contacts/consent-groups";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a consent group
    pub async fn get_consent_group(&self, id: i64) -> Result<serde_json::Value> {
        let path = format!("/contacts/consent-groups/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a consent group
    pub async fn update_consent_group(
        &self,
        id: i64,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/consent-groups/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a consent group
    pub async fn delete_consent_group(&self, id: i64) -> Result<serde_json::Value> {
        let path = format!("/contacts/consent-groups/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
