//! MasterAccount API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
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

    /// Create a new group of sub-accounts
    pub async fn post_corporate_group(
        &self,
        body: &PostCorporateGroupRequest,
    ) -> Result<PostCorporateGroupResponse> {
        let path = "/corporate/group";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete sub-account from group
    pub async fn put_corporate_group_unlink_group_id_sub_accounts(
        &self,
        group_id: &str,
        body: &PutCorporateGroupUnlinkGroupIdSubAccountsRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/group/unlink/{group_id}/subAccounts");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a group
    pub async fn delete_corporate_group_id(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/corporate/group/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// GET a group details
    pub async fn get_corporate_group_id(&self, id: &str) -> Result<GetCorporateGroupIdResponse> {
        let path = format!("/corporate/group/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a group of sub-accounts
    pub async fn put_corporate_group_id(
        &self,
        id: &str,
        body: &PutCorporateGroupIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/group/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
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

    /// List of all IPs
    pub async fn get_corporate_ip(&self) -> Result<Vec<GetCorporateIpResponseItem>> {
        let path = "/corporate/ip";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the details of requested master account
    pub async fn get_corporate_master_account(&self) -> Result<GetCorporateMasterAccountResponse> {
        let path = "/corporate/masterAccount";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Generate SSO token to access admin account
    pub async fn post_corporate_sso_token(
        &self,
        body: &PostCorporateSsoTokenRequest,
    ) -> Result<serde_json::Value> {
        let path = "/corporate/ssoToken";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get the list of all the sub-accounts of the master account.
    pub async fn get_corporate_sub_account(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<GetCorporateSubAccountResponse> {
        let path = "/corporate/subAccount";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("offset", &offset.to_string());
            query_pairs.append_pair("limit", &limit.to_string());
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a new sub-account under a master account.
    pub async fn post_corporate_sub_account(
        &self,
        body: &PostCorporateSubAccountRequest,
    ) -> Result<PostCorporateSubAccountResponse> {
        let path = "/corporate/subAccount";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Associate an IP to sub-accounts
    pub async fn post_corporate_sub_account_ip_associate(
        &self,
        body: &PostCorporateSubAccountIpAssociateRequest,
    ) -> Result<serde_json::Value> {
        let path = "/corporate/subAccount/ip/associate";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Dissociate an IP to sub-accounts
    pub async fn put_corporate_sub_account_ip_dissociate(
        &self,
        body: &PutCorporateSubAccountIpDissociateRequest,
    ) -> Result<serde_json::Value> {
        let path = "/corporate/subAccount/ip/dissociate";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create an API key for a sub-account
    pub async fn post_corporate_sub_account_key(
        &self,
        body: &PostCorporateSubAccountKeyRequest,
    ) -> Result<PostCorporateSubAccountKeyResponse> {
        let path = "/corporate/subAccount/key";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Generate SSO token to access sub-account
    pub async fn post_corporate_sub_account_sso_token(
        &self,
        body: &PostCorporateSubAccountSsoTokenRequest,
    ) -> Result<serde_json::Value> {
        let path = "/corporate/subAccount/ssoToken";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a sub-account
    pub async fn delete_corporate_sub_account_id(&self, id: i64) -> Result<serde_json::Value> {
        let path = format!("/corporate/subAccount/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get sub-account details
    pub async fn get_corporate_sub_account_id(
        &self,
        id: i64,
    ) -> Result<GetCorporateSubAccountIdResponse> {
        let path = format!("/corporate/subAccount/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Enable/disable sub-account application(s)
    pub async fn put_corporate_sub_account_id_applications_toggle(
        &self,
        id: i64,
        body: &PutCorporateSubAccountIdApplicationsToggleRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/subAccount/{id}/applications/toggle");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update sub-account plan
    pub async fn put_corporate_sub_account_id_plan(
        &self,
        id: i64,
        body: &PutCorporateSubAccountIdPlanRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/subAccount/{id}/plan");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update sub-accounts plan
    pub async fn put_corporate_sub_accounts_plan(
        &self,
        body: &PutCorporateSubAccountsPlanRequest,
    ) -> Result<serde_json::Value> {
        let path = "/corporate/subAccounts/plan";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
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

    /// Resend / cancel admin user invitation
    pub async fn put_corporate_user_invitation_action_email(
        &self,
        action: PutCorporateUserInvitationActionEmailAction,
        email: &Email,
    ) -> Result<PutCorporateUserInvitationActionEmailResponse> {
        let path = format!("/corporate/user/invitation/{action}/{email}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Revoke an admin user
    pub async fn delete_corporate_user_revoke_email(
        &self,
        email: &Email,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/user/revoke/{email}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Check admin user permissions
    pub async fn get_corporate_user_permission(
        &self,
        email: &Email,
    ) -> Result<GetCorporateUserPermissionResponse> {
        let path = format!("/corporate/user/{email}/permissions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Change admin user permissions
    pub async fn put_corporate_user_email_permissions(
        &self,
        email: &Email,
        body: &PutCorporateUserEmailPermissionsRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/corporate/user/{email}/permissions");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
