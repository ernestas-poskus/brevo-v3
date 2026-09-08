//! Reward API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
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
    pub async fn get_code_count(
        &self,
        pid: &ProgramId,
        cpid: &str,
    ) -> Result<GetCodeCountResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/code-pools/{cpid}/codes-count");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get Reward Page API
    pub async fn get_loyalty_offer_programs_pid_offers(
        &self,
        pid: &ProgramId,
        limit: Option<i64>,
        offset: Option<i64>,
        state: Option<&str>,
        version: Option<GetLoyaltyOfferProgramsPidOffersVersion>,
    ) -> Result<GetLoyaltyOfferProgramsPidOffersResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/offers");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = state {
                query_pairs.append_pair("state", &val.to_string());
            }
            if let Some(val) = version {
                query_pairs.append_pair("version", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a reward
    pub async fn create_reward(
        &self,
        pid: &ProgramId,
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
        pid: &ProgramId,
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
        pid: &ProgramId,
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
        pid: &ProgramId,
        tid: &TierId,
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
        pid: &ProgramId,
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
        pid: &ProgramId,
        body: &ValidateRewardRequest,
    ) -> Result<ValidateRewardResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/validate");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get reward information
    pub async fn get_loyalty_offer_programs_pid_rewards_rid(
        &self,
        pid: &ProgramId,
        rid: &RewardId,
        version: Option<GetLoyaltyOfferProgramsPidRewardsRidVersion>,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/rewards/{rid}");
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

    /// Get voucher for a contact
    pub async fn get_loyalty_offer_programs_pid_vouchers(
        &self,
        pid: &ProgramId,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetLoyaltyOfferProgramsPidVouchersSort>,
        sort_field: Option<GetLoyaltyOfferProgramsPidVouchersSortField>,
        contact_id: ContactId,
        metadata_key_value: Option<&str>,
        reward_id: Option<&str>,
    ) -> Result<GetLoyaltyOfferProgramsPidVouchersResponse> {
        let path = format!("/loyalty/offer/programs/{pid}/vouchers");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = sort_field {
                query_pairs.append_pair("sortField", &val.to_string());
            }
            query_pairs.append_pair("contactId", &contact_id.to_string());
            if let Some(val) = metadata_key_value {
                query_pairs.append_pair("metadata_key_value", &val.to_string());
            }
            if let Some(val) = reward_id {
                query_pairs.append_pair("rewardId", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
