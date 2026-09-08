//! CustomObjects API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for CustomObjects endpoints.
#[derive(Clone, Debug)]
pub struct CustomObjectsApi {
    client: BrevoClient,
}

impl CustomObjectsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Create/Update object records in bulk
    pub async fn upsertrecords(
        &self,
        object_type: &str,
        body: &UpsertrecordsRequest,
    ) -> Result<UpsertrecordsResponse> {
        let path = format!("/objects/{object_type}/batch/upsert");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the list of object records and total records count for an object.
    pub async fn getrecords(
        &self,
        object_type: &str,
        limit: i64,
        page_num: i64,
        sort: Option<GetrecordsSort>,
        association: Option<GetrecordsAssociation>,
    ) -> Result<GetrecordsResponse> {
        let path = format!("/objects/{object_type}/records");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("limit", &limit.to_string());
            query_pairs.append_pair("page_num", &page_num.to_string());
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = association {
                query_pairs.append_pair("association", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete multiple object records (up to 1000) asynchronously
    pub async fn batch_delete_object_records(
        &self,
        object_type: &str,
        body: &BatchDeleteObjectRecordsRequest,
    ) -> Result<BatchDeleteObjectRecordsResponse> {
        let path = format!("/objects/{object_type}/batch/delete");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the records associated with an object record
    pub async fn get_associated_records(
        &self,
        object_type: &str,
        id: Option<i64>,
        ext_id: Option<&str>,
        email: Option<&str>,
        sms: Option<&str>,
        type_: Option<&[String]>,
        offset: Option<i64>,
    ) -> Result<GetAssociatedRecordsResponse> {
        let path = format!("/objects/{object_type}/associated-records");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = id {
                query_pairs.append_pair("id", &val.to_string());
            }
            if let Some(val) = ext_id {
                query_pairs.append_pair("ext_id", &val.to_string());
            }
            if let Some(val) = email {
                query_pairs.append_pair("email", &val.to_string());
            }
            if let Some(val) = sms {
                query_pairs.append_pair("sms", &val.to_string());
            }
            if let Some(items) = type_ {
                for item in items {
                    query_pairs.append_pair("type", &item.to_string());
                }
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
