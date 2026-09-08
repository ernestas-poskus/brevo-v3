//! Program API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Program endpoints.
#[derive(Clone, Debug)]
pub struct ProgramApi {
    client: BrevoClient,
}

impl ProgramApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get loyalty program list
    pub async fn get_lp_list(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort_field: Option<GetLPListSortField>,
        sort: Option<GetLPListSort>,
    ) -> Result<GetLPListResponse> {
        let path = "/loyalty/config/programs";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = sort_field {
                query_pairs.append_pair("sort_field", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create loyalty program
    pub async fn create_new_lp(&self, body: &CreateNewLPRequest) -> Result<serde_json::Value> {
        let path = "/loyalty/config/programs";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete Loyalty Program
    pub async fn delete_loyalty_program(&self, pid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get loyalty program Info
    pub async fn get_loyalty_program_info(&self, pid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Partially update loyalty program
    pub async fn partially_update_loyalty_program(
        &self,
        pid: &str,
        body: &PartiallyUpdateLoyaltyProgramRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update loyalty program
    pub async fn update_loyalty_program(
        &self,
        pid: &str,
        body: &UpdateLoyaltyProgramRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get Subscription Data
    pub async fn get_parameter_subscription_info(
        &self,
        pid: &str,
        contact_id: Option<&str>,
        params: Option<&str>,
        loyalty_subscription_id: Option<&str>,
        include_internal: Option<bool>,
    ) -> Result<GetParameterSubscriptionInfoResponse> {
        let path = format!("/loyalty/config/programs/{pid}/account-info");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = contact_id {
                query_pairs.append_pair("contactId", &val.to_string());
            }
            if let Some(val) = params {
                query_pairs.append_pair("params", &val.to_string());
            }
            if let Some(val) = loyalty_subscription_id {
                query_pairs.append_pair("loyaltySubscriptionId", &val.to_string());
            }
            if let Some(val) = include_internal {
                query_pairs.append_pair("includeInternal", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete subscription
    pub async fn delete_contact_subscription(
        &self,
        pid: &str,
        cid: i64,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}/contact/{cid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Publish loyalty program
    pub async fn publish_loyalty_program(&self, pid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}/publish");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete subscription member
    pub async fn delete_contact_members(
        &self,
        pid: &str,
        member_contact_ids: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/config/programs/{pid}/subscription-members");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("memberContactIds", &member_contact_ids.to_string());
        }
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create subscription member
    pub async fn subscribe_member_to_a_subscription(
        &self,
        pid: &str,
        body: &SubscribeMemberToASubscriptionRequest,
    ) -> Result<SubscribeMemberToASubscriptionResponse> {
        let path = format!("/loyalty/config/programs/{pid}/subscription-members");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create subscription
    pub async fn subscribe_to_loyalty_program(
        &self,
        pid: &str,
        body: &SubscribeToLoyaltyProgramRequest,
    ) -> Result<SubscribeToLoyaltyProgramResponse> {
        let path = format!("/loyalty/config/programs/{pid}/subscriptions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
