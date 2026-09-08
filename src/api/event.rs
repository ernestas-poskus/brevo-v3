//! Event API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Event endpoints.
#[derive(Clone, Debug)]
pub struct EventApi {
    client: BrevoClient,
}

impl EventApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get events
    pub async fn get_events(
        &self,
        contact_id: Option<ContactId>,
        event_name: Option<&[String]>,
        object_type: Option<&[String]>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<serde_json::Value> {
        let path = "/events";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = contact_id {
                query_pairs.append_pair("contact_id", &val.to_string());
            }
            if let Some(items) = event_name {
                for item in items {
                    query_pairs.append_pair("event_name", &item.to_string());
                }
            }
            if let Some(items) = object_type {
                for item in items {
                    query_pairs.append_pair("object_type", &item.to_string());
                }
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
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

    /// Create an event
    pub async fn create_event(&self, body: &CreateEventRequest) -> Result<serde_json::Value> {
        let path = "/events";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create events in batch
    pub async fn create_batch_events(
        &self,
        body: &CreateBatchEventsRequest,
    ) -> Result<serde_json::Value> {
        let path = "/events/batch";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
