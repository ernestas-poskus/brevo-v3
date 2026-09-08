//! Account API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Account endpoints.
#[derive(Clone, Debug)]
pub struct AccountApi {
    client: BrevoClient,
}

impl AccountApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get account details
    pub async fn get_account(&self) -> Result<GetAccountResponse> {
        let path = "/account";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get user activity logs
    pub async fn get_account_activity(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        email: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<GetAccountActivityResponse> {
        let path = "/organization/activities";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = email {
                query_pairs.append_pair("email", &val.to_string());
            }
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
