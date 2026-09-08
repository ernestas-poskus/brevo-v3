//! ExternalFeeds API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for ExternalFeeds endpoints.
#[derive(Clone, Debug)]
pub struct ExternalFeedsApi {
    client: BrevoClient,
}

impl ExternalFeedsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all external feeds
    pub async fn get_all_external_feeds(
        &self,
        search: Option<&str>,
        start_date: Option<time::Date>,
        end_date: Option<time::Date>,
        sort: Option<GetAllExternalFeedsSort>,
        auth_type: Option<GetAllExternalFeedsAuthType>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<GetAllExternalFeedsResponse> {
        let path = "/feeds";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = search {
                query_pairs.append_pair("search", &val.to_string());
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = auth_type {
                query_pairs.append_pair("authType", &val.to_string());
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

    /// Create external feed
    pub async fn create_external_feed(
        &self,
        body: &CreateExternalFeedRequest,
    ) -> Result<CreateExternalFeedResponse> {
        let path = "/feeds";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete external feed
    pub async fn delete_external_feed(&self, uuid: &str) -> Result<serde_json::Value> {
        let path = format!("/feeds/{uuid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get external feed details
    pub async fn get_external_feed_by_uuid(
        &self,
        uuid: &str,
    ) -> Result<GetExternalFeedByUUIDResponse> {
        let path = format!("/feeds/{uuid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update external feed
    pub async fn update_external_feed(
        &self,
        uuid: &str,
        body: &UpdateExternalFeedRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/feeds/{uuid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
