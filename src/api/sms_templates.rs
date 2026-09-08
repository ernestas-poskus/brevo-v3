//! SMSTemplates API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for SMSTemplates endpoints.
#[derive(Clone, Debug)]
pub struct SMSTemplatesApi {
    client: BrevoClient,
}

impl SMSTemplatesApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Return all your created SMS templates
    pub async fn get_sms_templates(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetSMSTemplatesSort>,
    ) -> Result<GetSMSTemplatesResponse> {
        let path = "/transactionalSMS/templates";
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
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
