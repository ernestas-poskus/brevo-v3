//! MasterAccount API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for MasterAccount endpoints.
#[derive(Clone, Debug)]
pub struct MasterAccountApi {
    client: BrevoClient,
}

impl MasterAccountApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get the list of groups
    pub async fn get_sub_account_groups(&self) -> Result<Vec<GetSubAccountGroupsResponseItem>> {
        let path = "/corporate/groups";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the list of all admin users
    pub async fn get_corporate_invited_users_list(
        &self,
        type_: Option<&str>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<GetCorporateInvitedUsersListResponse> {
        let path = "/corporate/invited/users";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = type_ {
                query_pairs.append_pair("type", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send invitation to an admin user
    pub async fn invite_admin_user(
        &self,
        body: &InviteAdminUserRequest,
    ) -> Result<InviteAdminUserResponse> {
        let path = "/corporate/user/invitation/send";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Check admin user permissions
    pub async fn get_corporate_user_permission(
        &self,
        email: &str,
    ) -> Result<GetCorporateUserPermissionResponse> {
        let path = format!("/corporate/user/{email}/permissions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
