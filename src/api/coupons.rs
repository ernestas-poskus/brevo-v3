//! Coupons API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Coupons endpoints.
#[derive(Clone, Debug)]
pub struct CouponsApi {
    client: BrevoClient,
}

impl CouponsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all your coupon collections
    pub async fn get_coupon_collections(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetCouponCollectionsSort>,
        sort_by: Option<GetCouponCollectionsSortBy>,
    ) -> Result<serde_json::Value> {
        let path = "/couponCollections";
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
            if let Some(val) = sort_by {
                query_pairs.append_pair("sortBy", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create а coupon collection
    pub async fn create_coupon_collection(
        &self,
        body: &CreateCouponCollectionRequest,
    ) -> Result<CreateCouponCollectionResponse> {
        let path = "/couponCollections";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a coupon collection by id
    pub async fn get_coupon_collection(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/couponCollections/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a coupon collection by id
    pub async fn update_coupon_collection(
        &self,
        id: &str,
        body: &UpdateCouponCollectionRequest,
    ) -> Result<UpdateCouponCollectionResponse> {
        let path = format!("/couponCollections/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create coupons for a coupon collection
    pub async fn create_coupons(&self, body: &CreateCouponsRequest) -> Result<serde_json::Value> {
        let path = "/coupons";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
