//! Tier API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Tier endpoints.
#[derive(Clone, Debug)]
pub struct TierApi {
    client: BrevoClient,
}

impl TierApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Assign a tier
    pub async fn add_subscription_to_tier(
        &self,
        pid: &str,
        cid: &str,
        tid: &str,
    ) -> Result<AddSubscriptionToTierResponse> {
        let path = format!("/loyalty/tier/programs/{pid}/contacts/{cid}/tiers/{tid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// List tier groups
    pub async fn get_list_of_tier_groups(
        &self,
        pid: &str,
        version: Option<GetListOfTierGroupsVersion>,
    ) -> Result<GetListOfTierGroupsResponse> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = version {
                query_pairs.append_pair("version", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a tier group
    pub async fn create_tier_group(
        &self,
        pid: &str,
        body: &CreateTierGroupRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete tier group
    pub async fn delete_tier_group(&self, pid: &str, gid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups/{gid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get tier group
    pub async fn get_tier_group(
        &self,
        pid: &str,
        gid: &str,
        version: Option<GetTierGroupVersion>,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups/{gid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = version {
                query_pairs.append_pair("version", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update tier group
    pub async fn update_tier_group(
        &self,
        pid: &str,
        gid: &str,
        body: &UpdateTierGroupRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups/{gid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a tier
    pub async fn create_tier_for_tier_group(
        &self,
        pid: &str,
        gid: &str,
        body: &CreateTierForTierGroupRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tier-groups/{gid}/tiers");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// List tiers
    pub async fn get_loyalty_program_tier(
        &self,
        pid: &str,
        version: Option<GetLoyaltyProgramTierVersion>,
    ) -> Result<GetLoyaltyProgramTierResponse> {
        let path = format!("/loyalty/tier/programs/{pid}/tiers");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = version {
                query_pairs.append_pair("version", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete tier
    pub async fn delete_tier(&self, pid: &str, tid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tiers/{tid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update tier
    pub async fn update_tier(
        &self,
        pid: &str,
        tid: &str,
        body: &UpdateTierRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/tier/programs/{pid}/tiers/{tid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
