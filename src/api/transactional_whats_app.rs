//! TransactionalWhatsApp API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for TransactionalWhatsApp endpoints.
#[derive(Clone, Debug)]
pub struct TransactionalWhatsAppApi {
    client: BrevoClient,
}

impl TransactionalWhatsAppApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Send a WhatsApp message
    pub async fn send_whatsapp_message(
        &self,
        body: &serde_json::Value,
    ) -> Result<SendWhatsappMessageResponse> {
        let path = "/whatsapp/sendMessage";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all your WhatsApp activity (unaggregated events)
    pub async fn get_whatsapp_event_report(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        contact_number: Option<&PhoneNumber>,
        event: Option<GetWhatsappEventReportEvent>,
        sort: Option<GetWhatsappEventReportSort>,
    ) -> Result<GetWhatsappEventReportResponse> {
        let path = "/whatsapp/statistics/events";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = days {
                query_pairs.append_pair("days", &val.to_string());
            }
            if let Some(val) = contact_number {
                query_pairs.append_pair("contactNumber", &val.to_string());
            }
            if let Some(val) = event {
                query_pairs.append_pair("event", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
