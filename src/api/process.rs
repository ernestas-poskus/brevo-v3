//! Process API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for Process endpoints.
#[derive(Clone, Debug)]
pub struct ProcessApi {
    client: BrevoClient,
}

impl ProcessApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get background processes
    pub async fn get_processes(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetProcessesSort>,
    ) -> Result<GetProcessesResponse> {
        let path = "/processes";
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

    /// Get process details
    pub async fn get_process(&self, process_id: i64) -> Result<GetProcessResponse> {
        let path = format!("/processes/{process_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
