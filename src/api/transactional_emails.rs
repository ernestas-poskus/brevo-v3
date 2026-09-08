//! TransactionalEmails API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for TransactionalEmails endpoints.
#[derive(Clone, Debug)]
pub struct TransactionalEmailsApi {
    client: BrevoClient,
}

impl TransactionalEmailsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get the list of blocked or unsubscribed transactional contacts
    pub async fn get_transac_blocked_contacts(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        senders: Option<&[String]>,
        sort: Option<GetTransacBlockedContactsSort>,
    ) -> Result<GetTransacBlockedContactsResponse> {
        let path = "/smtp/blockedContacts";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
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
            if let Some(items) = senders {
                for item in items {
                    query_pairs.append_pair("senders", &item.to_string());
                }
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the list of blocked domains
    pub async fn get_blocked_domains(&self) -> Result<GetBlockedDomainsResponse> {
        let path = "/smtp/blockedDomains";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Add a new domain to the list of blocked domains
    pub async fn block_new_domain(
        &self,
        body: &BlockNewDomainRequest,
    ) -> Result<serde_json::Value> {
        let path = "/smtp/blockedDomains";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Unblock an existing domain from the list of blocked domains
    pub async fn delete_blocked_domain(&self, domain: &str) -> Result<serde_json::Value> {
        let path = format!("/smtp/blockedDomains/{domain}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete hardbounces
    pub async fn delete_hardbounces(
        &self,
        body: &DeleteHardbouncesRequest,
    ) -> Result<serde_json::Value> {
        let path = "/smtp/deleteHardbounces";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send a transactional email
    pub async fn send_transac_email(
        &self,
        body: &SendTransacEmailRequest,
    ) -> Result<SendTransacEmailResponse> {
        let path = "/smtp/email";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete scheduled emails by batchId or messageId
    pub async fn delete_scheduled_email_by_id(
        &self,
        identifier: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/smtp/email/{identifier}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Fetch scheduled emails by batchId or messageId
    pub async fn get_scheduled_email_by_id(
        &self,
        identifier: &str,
        start_date: Option<time::Date>,
        end_date: Option<time::Date>,
        sort: Option<GetScheduledEmailByIdSort>,
        status: Option<GetScheduledEmailByIdStatus>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<serde_json::Value> {
        let path = format!("/smtp/emailStatus/{identifier}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(val) = status {
                query_pairs.append_pair("status", &val.to_string());
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

    /// Get the list of transactional emails on the basis of allowed filters
    pub async fn get_transac_emails_list(
        &self,
        email: Option<&str>,
        template_id: Option<i64>,
        message_id: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        sort: Option<GetTransacEmailsListSort>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<GetTransacEmailsListResponse> {
        let path = "/smtp/emails";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = email {
                query_pairs.append_pair("email", &val.to_string());
            }
            if let Some(val) = template_id {
                query_pairs.append_pair("templateId", &val.to_string());
            }
            if let Some(val) = message_id {
                query_pairs.append_pair("messageId", &val.to_string());
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

    /// Get the personalized content of a sent transactional email
    pub async fn get_transac_email_content(
        &self,
        uuid: &str,
    ) -> Result<GetTransacEmailContentResponse> {
        let path = format!("/smtp/emails/{uuid}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get your transactional email activity aggregated over a period of time
    pub async fn get_aggregated_smtp_report(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        tag: Option<&str>,
    ) -> Result<GetAggregatedSmtpReportResponse> {
        let path = "/smtp/statistics/aggregatedReport";
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

    /// Get all your transactional email activity (unaggregated events)
    pub async fn get_email_event_report(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        email: Option<&str>,
        event: Option<GetEmailEventReportEvent>,
        tags: Option<&str>,
        message_id: Option<&str>,
        template_id: Option<i64>,
        sort: Option<GetEmailEventReportSort>,
    ) -> Result<GetEmailEventReportResponse> {
        let path = "/smtp/statistics/events";
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
            if let Some(val) = email {
                query_pairs.append_pair("email", &val.to_string());
            }
            if let Some(val) = event {
                query_pairs.append_pair("event", &val.to_string());
            }
            if let Some(val) = tags {
                query_pairs.append_pair("tags", &val.to_string());
            }
            if let Some(val) = message_id {
                query_pairs.append_pair("messageId", &val.to_string());
            }
            if let Some(val) = template_id {
                query_pairs.append_pair("templateId", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get your transactional email activity aggregated per day
    pub async fn get_smtp_report(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        days: Option<i64>,
        tag: Option<&str>,
        sort: Option<GetSmtpReportSort>,
    ) -> Result<GetSmtpReportResponse> {
        let path = "/smtp/statistics/reports";
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

    /// Generate the rendered preview of transactional template
    pub async fn post_preview_smtp_email_templates(
        &self,
        body: &PostPreviewSmtpEmailTemplatesRequest,
    ) -> Result<PostPreviewSmtpEmailTemplatesResponse> {
        let path = "/smtp/template/preview";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the list of email templates
    pub async fn get_smtp_templates(
        &self,
        template_status: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetSmtpTemplatesSort>,
        editor_type: Option<GetSmtpTemplatesEditorType>,
    ) -> Result<GetSmtpTemplatesResponse> {
        let path = "/smtp/templates";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = template_status {
                query_pairs.append_pair("templateStatus", &val.to_string());
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
            if let Some(val) = editor_type {
                query_pairs.append_pair("editorType", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create an email template
    pub async fn create_smtp_template(
        &self,
        body: &CreateSmtpTemplateRequest,
    ) -> Result<serde_json::Value> {
        let path = "/smtp/templates";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an inactive email template
    pub async fn delete_smtp_template(&self, template_id: i64) -> Result<serde_json::Value> {
        let path = format!("/smtp/templates/{template_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Returns the template information
    pub async fn get_smtp_template(
        &self,
        template_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/smtp/templates/{template_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an email template
    pub async fn update_smtp_template(
        &self,
        template_id: serde_json::Value,
        body: &UpdateSmtpTemplateRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/smtp/templates/{template_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send a template to your test list
    pub async fn send_test_template(
        &self,
        template_id: i64,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/smtp/templates/{template_id}/sendTest");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
