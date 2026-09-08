//! Reward API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Reward endpoints.
#[derive(Clone, Debug)]
pub struct RewardApi {
    client: BrevoClient,
}

impl RewardApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get code count
    pub async fn get_code_count(&self, pid: &str, cpid: &str) -> Result<GetCodeCountResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/code-pools/{cpid}/codes-count");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a reward
    pub async fn create_reward(
        &self,
        pid: &str,
        body: &CreateRewardRequest,
    ) -> Result<CreateRewardResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/offers");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a voucher
    pub async fn create_voucher(
        &self,
        pid: &str,
        body: &CreateVoucherRequest,
    ) -> Result<CreateVoucherResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/attribute");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create redeem voucher request
    pub async fn redeem_voucher(
        &self,
        pid: &str,
        body: &RedeemVoucherRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/redeem");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Complete redeem voucher request
    pub async fn complete_redeem_transaction(
        &self,
        pid: &str,
        tid: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/redeem/{tid}/complete");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Revoke vouchers
    pub async fn revoke_vouchers(
        &self,
        pid: &str,
        attributed_reward_ids: Option<&str>,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/revoke");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = attributed_reward_ids {
                query_pairs.append_pair("attributedRewardIds", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Validate a reward
    pub async fn validate_reward(
        &self,
        pid: &str,
        body: &ValidateRewardRequest,
    ) -> Result<ValidateRewardResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/validate");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
