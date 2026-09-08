//! TransactionalSMS API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for TransactionalSMS endpoints.
#[derive(Clone, Debug)]
pub struct TransactionalSMSApi {
    client: BrevoClient,
}

impl TransactionalSMSApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Send SMS message asynchronously to a mobile number
    pub async fn send_async_transactional_sms(
        &self,
        body: &serde_json::Value,
    ) -> Result<SendAsyncTransactionalSmsResponse> {
        let path = "/transactionalSMS/send";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send SMS message to a mobile number
    pub async fn send_transac_sms(
        &self,
        body: &serde_json::Value,
    ) -> Result<SendTransacSmsResponse> {
        let path = "/transactionalSMS/sms";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get your SMS activity aggregated over a period of time
    pub async fn get_transac_aggregated_sms_report(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        tag: Option<&str>,
    ) -> Result<GetTransacAggregatedSmsReportResponse> {
        let path = "/transactionalSMS/statistics/aggregatedReport";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = days {
                query_pairs.append_pair("days", &val.to_string());
            }
            if let Some(val) = tag {
                query_pairs.append_pair("tag", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all your SMS activity (unaggregated events)
    pub async fn get_sms_events(
        &self,
        limit: Option<i64>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        offset: Option<i64>,
        days: Option<i64>,
        phone_number: Option<&PhoneNumber>,
        event: Option<GetSmsEventsEvent>,
        tags: Option<&str>,
        sort: Option<GetSmsEventsSort>,
    ) -> Result<GetSmsEventsResponse> {
        let path = "/transactionalSMS/statistics/events";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = days {
                query_pairs.append_pair("days", &val.to_string());
            }
            if let Some(val) = phone_number {
                query_pairs.append_pair("phoneNumber", &val.to_string());
            }
            if let Some(val) = event {
                query_pairs.append_pair("event", &val.to_string());
            }
            if let Some(val) = tags {
                query_pairs.append_pair("tags", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get your SMS activity aggregated per day
    pub async fn get_transac_sms_report(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        tag: Option<&str>,
        sort: Option<GetTransacSmsReportSort>,
    ) -> Result<GetTransacSmsReportResponse> {
        let path = "/transactionalSMS/statistics/reports";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = days {
                query_pairs.append_pair("days", &val.to_string());
            }
            if let Some(val) = tag {
                query_pairs.append_pair("tag", &val.to_string());
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
