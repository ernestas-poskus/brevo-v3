//! Ecommerce API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Ecommerce endpoints.
#[derive(Clone, Debug)]
pub struct EcommerceApi {
    client: BrevoClient,
}

impl EcommerceApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Return all your categories
    pub async fn get_categories(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetCategoriesSort>,
        ids: Option<&[String]>,
        name: Option<&str>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
        is_deleted: Option<&str>,
    ) -> Result<GetCategoriesResponse> {
        let path = "/categories";
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
            if let Some(items) = ids {
                for item in items {
                    query_pairs.append_pair("ids", &item.to_string());
                }
            }
            if let Some(val) = name {
                query_pairs.append_pair("name", &val.to_string());
            }
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
            if let Some(val) = is_deleted {
                query_pairs.append_pair("isDeleted", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create/Update a category
    pub async fn create_update_category(
        &self,
        body: &CreateUpdateCategoryRequest,
    ) -> Result<CreateUpdateCategoryResponse> {
        let path = "/categories";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create categories in batch
    pub async fn create_update_batch_category(
        &self,
        body: &CreateUpdateBatchCategoryRequest,
    ) -> Result<CreateUpdateBatchCategoryResponse> {
        let path = "/categories/batch";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a category details
    pub async fn get_category_info(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/categories/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Set the ISO 4217 compliant display currency code for your Brevo account
    pub async fn set_config_display_currency(
        &self,
        body: &SetConfigDisplayCurrencyRequest,
    ) -> Result<SetConfigDisplayCurrencyResponse> {
        let path = "/ecommerce/config/displayCurrency";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get order details
    pub async fn get_orders(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetOrdersSort>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
    ) -> Result<serde_json::Value> {
        let path = "/orders";
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
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Managing the status of the order
    pub async fn create_order(&self, body: &serde_json::Value) -> Result<serde_json::Value> {
        let path = "/orders/status";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create orders in batch
    pub async fn create_batch_order(
        &self,
        body: &CreateBatchOrderRequest,
    ) -> Result<CreateBatchOrderResponse> {
        let path = "/orders/status/batch";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Return all your products
    pub async fn get_products(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetProductsSort>,
        ids: Option<&[String]>,
        search: Option<&str>,
        name: Option<&str>,
        price_lte: Option<f64>,
        price_gte: Option<f64>,
        price_lt: Option<f64>,
        price_gt: Option<f64>,
        price_eq: Option<f64>,
        price_ne: Option<f64>,
        alternative_price_lte: Option<f64>,
        alternative_price_gte: Option<f64>,
        alternative_price_lt: Option<f64>,
        alternative_price_gt: Option<f64>,
        alternative_price_eq: Option<f64>,
        alternative_price_ne: Option<f64>,
        categories: Option<&[String]>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
        sort_by_field: Option<GetProductsSortByField>,
        is_deleted: Option<&str>,
    ) -> Result<GetProductsResponse> {
        let path = "/products";
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
            if let Some(items) = ids {
                for item in items {
                    query_pairs.append_pair("ids", &item.to_string());
                }
            }
            if let Some(val) = search {
                query_pairs.append_pair("search", &val.to_string());
            }
            if let Some(val) = name {
                query_pairs.append_pair("name", &val.to_string());
            }
            if let Some(val) = price_lte {
                query_pairs.append_pair("price[lte]", &val.to_string());
            }
            if let Some(val) = price_gte {
                query_pairs.append_pair("price[gte]", &val.to_string());
            }
            if let Some(val) = price_lt {
                query_pairs.append_pair("price[lt]", &val.to_string());
            }
            if let Some(val) = price_gt {
                query_pairs.append_pair("price[gt]", &val.to_string());
            }
            if let Some(val) = price_eq {
                query_pairs.append_pair("price[eq]", &val.to_string());
            }
            if let Some(val) = price_ne {
                query_pairs.append_pair("price[ne]", &val.to_string());
            }
            if let Some(val) = alternative_price_lte {
                query_pairs.append_pair("alternativePrice[lte]", &val.to_string());
            }
            if let Some(val) = alternative_price_gte {
                query_pairs.append_pair("alternativePrice[gte]", &val.to_string());
            }
            if let Some(val) = alternative_price_lt {
                query_pairs.append_pair("alternativePrice[lt]", &val.to_string());
            }
            if let Some(val) = alternative_price_gt {
                query_pairs.append_pair("alternativePrice[gt]", &val.to_string());
            }
            if let Some(val) = alternative_price_eq {
                query_pairs.append_pair("alternativePrice[eq]", &val.to_string());
            }
            if let Some(val) = alternative_price_ne {
                query_pairs.append_pair("alternativePrice[ne]", &val.to_string());
            }
            if let Some(items) = categories {
                for item in items {
                    query_pairs.append_pair("categories", &item.to_string());
                }
            }
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
            if let Some(val) = sort_by_field {
                query_pairs.append_pair("sortByField", &val.to_string());
            }
            if let Some(val) = is_deleted {
                query_pairs.append_pair("isDeleted", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create/Update a product
    pub async fn create_update_product(
        &self,
        body: &CreateUpdateProductRequest,
    ) -> Result<CreateUpdateProductResponse> {
        let path = "/products";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create products in batch
    pub async fn create_update_batch_products(
        &self,
        body: &CreateUpdateBatchProductsRequest,
    ) -> Result<CreateUpdateBatchProductsResponse> {
        let path = "/products/batch";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a product's details
    pub async fn get_product_info(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/products/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a product alert for a contact
    pub async fn create_product_alert(
        &self,
        id: &str,
        type_: CreateProductAlertType,
        body: &CreateProductAlertRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/products/{id}/alerts/{type_}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
