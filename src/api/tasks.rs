//! Tasks API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Tasks endpoints.
#[derive(Clone, Debug)]
pub struct TasksApi {
    client: BrevoClient,
}

impl TasksApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all tasks
    pub async fn get_crm_tasks(
        &self,
        filter_type: Option<&str>,
        filter_status: Option<GetCrmTasksFilterStatus>,
        filter_date: Option<GetCrmTasksFilterDate>,
        filter_assign_to: Option<&str>,
        filter_contacts: Option<&str>,
        filter_deals: Option<&str>,
        filter_companies: Option<&str>,
        date_from: Option<i64>,
        date_to: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
        sort: Option<GetCrmTasksSort>,
        sort_by: Option<&str>,
    ) -> Result<GetCrmTasksResponse> {
        let path = "/crm/tasks";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = filter_type {
                query_pairs.append_pair("filter[type]", &val.to_string());
            }
            if let Some(val) = filter_status {
                query_pairs.append_pair("filter[status]", &val.to_string());
            }
            if let Some(val) = filter_date {
                query_pairs.append_pair("filter[date]", &val.to_string());
            }
            if let Some(val) = filter_assign_to {
                query_pairs.append_pair("filter[assignTo]", &val.to_string());
            }
            if let Some(val) = filter_contacts {
                query_pairs.append_pair("filter[contacts]", &val.to_string());
            }
            if let Some(val) = filter_deals {
                query_pairs.append_pair("filter[deals]", &val.to_string());
            }
            if let Some(val) = filter_companies {
                query_pairs.append_pair("filter[companies]", &val.to_string());
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
            if let Some(val) = sort_by {
                query_pairs.append_pair("sortBy", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a task
    pub async fn post_crm_tasks(&self, body: &PostCrmTasksRequest) -> Result<PostCrmTasksResponse> {
        let path = "/crm/tasks";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a task
    pub async fn delete_crm_tasks_id(&self, id: &TaskId) -> Result<serde_json::Value> {
        let path = format!("/crm/tasks/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a task
    pub async fn get_crm_tasks_id(&self, id: &TaskId) -> Result<serde_json::Value> {
        let path = format!("/crm/tasks/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a task
    pub async fn patch_crm_tasks_id(
        &self,
        id: &TaskId,
        body: &PatchCrmTasksIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/crm/tasks/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PATCH, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all task types
    pub async fn get_crm_tasktypes(&self) -> Result<Vec<GetCrmTasktypesResponseItem>> {
        let path = "/crm/tasktypes";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
