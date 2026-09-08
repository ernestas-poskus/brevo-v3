//! Domains API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Domains endpoints.
#[derive(Clone, Debug)]
pub struct DomainsApi {
    client: BrevoClient,
}

impl DomainsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get the list of all sender domains
    pub async fn get_domains(&self) -> Result<GetDomainsResponse> {
        let path = "/senders/domains";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a new sender domain
    pub async fn create_domain(&self, body: &CreateDomainRequest) -> Result<CreateDomainResponse> {
        let path = "/senders/domains";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a sender domain
    pub async fn delete_domain(&self, domain_name: &DomainName) -> Result<serde_json::Value> {
        let path = format!("/senders/domains/{domain_name}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a sender domain configuration
    pub async fn get_domain_configuration(
        &self,
        domain_name: &DomainName,
    ) -> Result<GetDomainConfigurationResponse> {
        let path = format!("/senders/domains/{domain_name}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Authenticate a sender domain
    pub async fn authenticate_domain(
        &self,
        domain_name: &DomainName,
    ) -> Result<AuthenticateDomainResponse> {
        let path = format!("/senders/domains/{domain_name}/authenticate");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
