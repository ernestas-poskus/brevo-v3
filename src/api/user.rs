//! User API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use reqwest::Method;

/// Client for User endpoints.
#[derive(Clone, Debug)]
pub struct UserApi {
    client: BrevoClient,
}

impl UserApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get the list of all your users
    pub async fn get_invited_users_list(&self) -> Result<GetInvitedUsersListResponse> {
        let path = "/organization/invited/users";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Revoke user permission
    pub async fn put_revoke_user_permission(
        &self,
        email: &str,
    ) -> Result<PutRevokeUserPermissionResponse> {
        let path = format!("/organization/user/invitation/revoke/{email}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send invitation to user
    pub async fn inviteuser(&self, body: &serde_json::Value) -> Result<InviteuserResponse> {
        let path = "/organization/user/invitation/send";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Resend / Cancel invitation
    pub async fn putresendcancelinvitation(
        &self,
        action: PutresendcancelinvitationAction,
        email: &str,
    ) -> Result<PutresendcancelinvitationResponse> {
        let path = format!("/organization/user/invitation/{action}/{email}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update permission for a user
    pub async fn edit_user_permission(
        &self,
        body: &serde_json::Value,
    ) -> Result<EditUserPermissionResponse> {
        let path = "/organization/user/update/permissions";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Check user permission
    pub async fn get_user_permission(&self, email: &str) -> Result<GetUserPermissionResponse> {
        let path = format!("/organization/user/{email}/permissions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
