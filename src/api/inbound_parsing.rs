//! InboundParsing API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for InboundParsing endpoints.
#[derive(Clone, Debug)]
pub struct InboundParsingApi {
    client: BrevoClient,
}

impl InboundParsingApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get the list of all the events for the received emails.
    pub async fn get_inbound_email_events(
        &self,
        sender: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetInboundEmailEventsSort>,
    ) -> Result<GetInboundEmailEventsResponse> {
        let path = "/inbound/events";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = sender {
                query_pairs.append_pair("sender", &val.to_string());
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
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Fetch all events history for one particular received email.
    pub async fn get_inbound_email_events_by_uuid(
        &self,
        uuid: &str,
    ) -> Result<GetInboundEmailEventsByUuidResponse> {
        let path = format!("/inbound/events/{uuid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Retrieve inbound attachment with download token.
    pub async fn get_inbound_email_attachment(
        &self,
        download_token: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/inbound/attachments/{download_token}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
