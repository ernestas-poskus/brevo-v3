//! Wallet API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Wallet endpoints.
#[derive(Clone, Debug)]
pub struct WalletApi {
    client: BrevoClient,
}

impl WalletApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get a pass installation URL for a contact
    pub async fn get_wallet_pass_install_url(
        &self,
        pass_id: &PassId,
        contact_id: ContactId,
    ) -> Result<serde_json::Value> {
        let path = format!("/wallet/passes/{pass_id}/installUrl/{contact_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
