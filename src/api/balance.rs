//! Balance API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Balance endpoints.
#[derive(Clone, Debug)]
pub struct BalanceApi {
    client: BrevoClient,
}

impl BalanceApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get balance definition list
    pub async fn get_balance_definition_list(
        &self,
        pid: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        sort_field: Option<GetBalanceDefinitionListSortField>,
        sort: Option<GetBalanceDefinitionListSort>,
        version: Option<GetBalanceDefinitionListVersion>,
    ) -> Result<GetBalanceDefinitionListResponse> {
        let path = format!("/loyalty/balance/programs/{pid}/balance-definitions");
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
                query_pairs.append_pair("sortField", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = version {
                query_pairs.append_pair("version", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete balance definition
    pub async fn delete_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get balance definition
    pub async fn get_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
        version: Option<GetBalanceDefinitionVersion>,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}");
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

    /// Update balance definition
    pub async fn update_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
        body: &UpdateBalanceDefinitionRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create balance limits
    pub async fn create_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        body: &CreateBalanceLimitRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}/limits");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete balance limit
    pub async fn delete_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
    ) -> Result<serde_json::Value> {
        let path =
            format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}/limits/{blid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get balance limits
    pub async fn get_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
        version: Option<GetBalanceLimitVersion>,
    ) -> Result<serde_json::Value> {
        let path =
            format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}/limits/{blid}");
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

    /// Updates balance limit
    pub async fn update_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
        body: &UpdateBalanceLimitRequest,
    ) -> Result<serde_json::Value> {
        let path =
            format!("/loyalty/balance/programs/{pid}/balance-definitions/{bdid}/limits/{blid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get balance list
    pub async fn get_contact_balances(
        &self,
        pid: &str,
        include_internal: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetContactBalancesSort>,
        sort_field: Option<GetContactBalancesSortField>,
        balance_definition_id: &str,
    ) -> Result<GetContactBalancesResponse> {
        let path = format!("/loyalty/balance/programs/{pid}/contact-balances");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = include_internal {
                query_pairs.append_pair("includeInternal", &val.to_string());
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
            if let Some(val) = sort_field {
                query_pairs.append_pair("sortField", &val.to_string());
            }
            query_pairs.append_pair("balanceDefinitionId", &balance_definition_id.to_string());
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create balance order
    pub async fn create_balance_order(
        &self,
        pid: &str,
        body: &CreateBalanceOrderRequest,
    ) -> Result<CreateBalanceOrderResponse> {
        let path = format!("/loyalty/balance/programs/{pid}/create-order");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get subscription balances
    pub async fn get_subscription_balances(
        &self,
        cid: &str,
        pid: &str,
        include_internal: Option<bool>,
    ) -> Result<GetSubscriptionBalancesResponse> {
        let path = format!("/loyalty/balance/programs/{pid}/subscriptions/{cid}/balances");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = include_internal {
                query_pairs.append_pair("includeInternal", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create new transaction
    pub async fn begin_transaction(
        &self,
        pid: &str,
        body: &BeginTransactionRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/transactions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Cancel transaction
    pub async fn cancel_transaction(&self, pid: &str, tid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/transactions/{tid}/cancel");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Complete transaction
    pub async fn complete_transaction(&self, pid: &str, tid: &str) -> Result<serde_json::Value> {
        let path = format!("/loyalty/balance/programs/{pid}/transactions/{tid}/complete");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
