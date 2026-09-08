//! Conversations API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Conversations endpoints.
#[derive(Clone, Debug)]
pub struct ConversationsApi {
    client: BrevoClient,
}

impl ConversationsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Sets agent's status to online for 2-3 minutes
    pub async fn post_conversations_agent_online_ping(
        &self,
        body: &PostConversationsAgentOnlinePingRequest,
    ) -> Result<serde_json::Value> {
        let path = "/conversations/agentOnlinePing";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send a message as an agent
    pub async fn post_conversations_messages(
        &self,
        body: &PostConversationsMessagesRequest,
    ) -> Result<serde_json::Value> {
        let path = "/conversations/messages";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a message sent by an agent
    pub async fn delete_conversations_messages_id(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/conversations/messages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a message
    pub async fn get_conversations_messages_id(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/conversations/messages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a message sent by an agent
    pub async fn put_conversations_messages_id(
        &self,
        id: &str,
        body: &PutConversationsMessagesIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/conversations/messages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Send an automated message to a visitor
    pub async fn post_conversations_pushed_messages(
        &self,
        body: &PostConversationsPushedMessagesRequest,
    ) -> Result<serde_json::Value> {
        let path = "/conversations/pushedMessages";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an automated message
    pub async fn delete_conversations_pushed_messages_id(
        &self,
        id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/conversations/pushedMessages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get an automated message
    pub async fn get_conversations_pushed_messages_id(
        &self,
        id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/conversations/pushedMessages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update an automated message
    pub async fn put_conversations_pushed_messages_id(
        &self,
        id: &str,
        body: &PutConversationsPushedMessagesIdRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/conversations/pushedMessages/{id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Set visitor group assignment
    pub async fn put_conversations_visitor_group(
        &self,
        body: &PutConversationsVisitorGroupRequest,
    ) -> Result<PutConversationsVisitorGroupResponse> {
        let path = "/conversations/visitorGroup";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
