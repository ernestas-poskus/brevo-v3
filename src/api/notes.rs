//! Notes API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Notes endpoints.
#[derive(Clone, Debug)]
pub struct NotesApi {
    client: BrevoClient,
}

impl NotesApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all notes
    pub async fn get_crm_notes(
        &self,
        entity: Option<GetCrmNotesEntity>,
        entity_ids: Option<&str>,
        date_from: Option<i64>,
        date_to: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
        sort: Option<GetCrmNotesSort>,
    ) -> Result<Vec<serde_json::Value>> {
        let path = "/crm/notes";
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

    /// Create a note
    pub async fn post_crm_notes(&self, body: &serde_json::Value) -> Result<PostCrmNotesResponse> {
        let path = "/crm/notes";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a note
    pub async fn delete_crm_notes_id(&self, id: &NoteId) -> Result<serde_json::Value> {
        let path = format!("/crm/notes/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a note
    pub async fn get_crm_notes_id(&self, id: &NoteId) -> Result<serde_json::Value> {
        let path = format!("/crm/notes/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a note
    pub async fn patch_crm_notes_id(
        &self,
        id: &NoteId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/notes/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
