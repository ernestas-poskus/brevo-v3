//! Files API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Files endpoints.
#[derive(Clone, Debug)]
pub struct FilesApi {
    client: BrevoClient,
}

impl FilesApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all files
    pub async fn get_crm_files(
        &self,
        entity: Option<GetCrmFilesEntity>,
        entity_ids: Option<&str>,
        date_from: Option<i64>,
        date_to: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
        sort: Option<GetCrmFilesSort>,
    ) -> Result<Vec<serde_json::Value>> {
        let path = "/crm/files";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = entity {
                query_pairs.append_pair("entity", &val.to_string());
            }
            if let Some(val) = entity_ids {
                query_pairs.append_pair("entityIds", &val.to_string());
            }
            if let Some(val) = date_from {
                query_pairs.append_pair("dateFrom", &val.to_string());
            }
            if let Some(val) = date_to {
                query_pairs.append_pair("dateTo", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Upload a file
    pub async fn post_crm_files(&self) -> Result<serde_json::Value> {
        let path = "/crm/files";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a file
    pub async fn delete_crm_files_id(&self, id: &MessageId) -> Result<serde_json::Value> {
        let path = format!("/crm/files/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Download a file
    pub async fn get_crm_files_id(&self, id: &MessageId) -> Result<GetCrmFilesIdResponse> {
        let path = format!("/crm/files/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get file details
    pub async fn get_crm_files_id_data(&self, id: &MessageId) -> Result<serde_json::Value> {
        let path = format!("/crm/files/{id}/data");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
