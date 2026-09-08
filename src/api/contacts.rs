//! Contacts API endpoints.

#![allow(unused_imports, unused_mut, clippy::all)]

use crate::client::BrevoClient;
use crate::error::{Error, Result};
use crate::models::*;
use crate::types::*;
use reqwest::Method;

/// Client for Contacts endpoints.
#[derive(Clone, Debug)]
pub struct ContactsApi {
    client: BrevoClient,
}

impl ContactsApi {
    pub(crate) fn new(client: BrevoClient) -> Self {
        Self { client }
    }

    /// Get all the contacts
    pub async fn get_contacts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        modified_since: Option<&str>,
        created_since: Option<&str>,
        sort: Option<GetContactsSort>,
        ids: Option<&[i64]>,
        segment_id: Option<SegmentId>,
        list_ids: Option<&[i64]>,
        filter: Option<&str>,
    ) -> Result<serde_json::Value> {
        let path = "/contacts";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = limit {
                query_pairs.append_pair("limit", &val.to_string());
            }
            if let Some(val) = offset {
                query_pairs.append_pair("offset", &val.to_string());
            }
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
            }
            if let Some(val) = created_since {
                query_pairs.append_pair("createdSince", &val.to_string());
            }
            if let Some(val) = sort {
                query_pairs.append_pair("sort", &val.to_string());
            }
            if let Some(items) = ids {
                for item in items {
                    query_pairs.append_pair("ids", &item.to_string());
                }
            }
            if let Some(val) = segment_id {
                query_pairs.append_pair("segmentId", &val.to_string());
            }
            if let Some(items) = list_ids {
                for item in items {
                    query_pairs.append_pair("listIds", &item.to_string());
                }
            }
            if let Some(val) = filter {
                query_pairs.append_pair("filter", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create a contact
    pub async fn create_contact(
        &self,
        body: &CreateContactRequest,
    ) -> Result<CreateContactResponse> {
        let path = "/contacts";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// List all attributes
    pub async fn get_attributes(&self) -> Result<GetAttributesResponse> {
        let path = "/contacts/attributes";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete an attribute
    pub async fn delete_attribute(
        &self,
        attribute_category: DeleteAttributeAttributeCategory,
        attribute_name: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/attributes/{attribute_category}/{attribute_name}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create contact attribute
    pub async fn create_attribute(
        &self,
        attribute_category: CreateAttributeAttributeCategory,
        attribute_name: &str,
        body: &CreateAttributeRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/attributes/{attribute_category}/{attribute_name}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update contact attribute
    pub async fn update_attribute(
        &self,
        attribute_category: UpdateAttributeAttributeCategory,
        attribute_name: &str,
        body: &UpdateAttributeRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/attributes/{attribute_category}/{attribute_name}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a multiple-choice attribute option
    pub async fn delete_multi_attribute_options(
        &self,
        attribute_type: DeleteMultiAttributeOptionsAttributeType,
        multiple_choice_attribute: &str,
        multiple_choice_attribute_option: &str,
    ) -> Result<serde_json::Value> {
        let path = format!(
            "/contacts/attributes/{attribute_type}/{multiple_choice_attribute}/{multiple_choice_attribute_option}"
        );
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update multiple contacts
    pub async fn update_batch_contacts(
        &self,
        body: &UpdateBatchContactsRequest,
    ) -> Result<serde_json::Value> {
        let path = "/contacts/batch";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Create Contact via DOI (Double-Opt-In) Flow
    pub async fn create_doi_contact(
        &self,
        body: &CreateDoiContactRequest,
    ) -> Result<serde_json::Value> {
        let path = "/contacts/doubleOptinConfirmation";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Export contacts
    pub async fn request_contact_export(
        &self,
        body: &RequestContactExportRequest,
    ) -> Result<serde_json::Value> {
        let path = "/contacts/export";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all folders
    pub async fn get_folders(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetFoldersSort>,
    ) -> Result<GetFoldersResponse> {
        let path = "/contacts/folders";
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

    /// Create a folder
    pub async fn create_folder(&self, body: &serde_json::Value) -> Result<serde_json::Value> {
        let path = "/contacts/folders";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a folder (and all its lists)
    pub async fn delete_folder(&self, folder_id: FolderId) -> Result<serde_json::Value> {
        let path = format!("/contacts/folders/{folder_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Returns a folder's details
    pub async fn get_folder(&self, folder_id: FolderId) -> Result<serde_json::Value> {
        let path = format!("/contacts/folders/{folder_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a folder
    pub async fn update_folder(
        &self,
        folder_id: FolderId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/folders/{folder_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get lists in a folder
    pub async fn get_folder_lists(
        &self,
        folder_id: FolderId,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetFolderListsSort>,
    ) -> Result<GetFolderListsResponse> {
        let path = format!("/contacts/folders/{folder_id}/lists");
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

    /// Import contacts
    pub async fn import_contacts(&self, body: &ImportContactsRequest) -> Result<serde_json::Value> {
        let path = "/contacts/import";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all the lists
    pub async fn get_lists(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetListsSort>,
    ) -> Result<GetListsResponse> {
        let path = "/contacts/lists";
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

    /// Create a list
    pub async fn create_list(&self, body: &CreateListRequest) -> Result<serde_json::Value> {
        let path = "/contacts/lists";
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a list
    pub async fn delete_list(&self, list_id: ListId) -> Result<serde_json::Value> {
        let path = format!("/contacts/lists/{list_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a list's details
    pub async fn get_list(
        &self,
        list_id: ListId,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<GetListResponse> {
        let path = format!("/contacts/lists/{list_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a list
    pub async fn update_list(
        &self,
        list_id: ListId,
        body: &UpdateListRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/lists/{list_id}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get contacts in a list
    pub async fn get_contacts_from_list(
        &self,
        list_id: ListId,
        modified_since: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetContactsFromListSort>,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/lists/{list_id}/contacts");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = modified_since {
                query_pairs.append_pair("modifiedSince", &val.to_string());
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
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Add existing contacts to a list
    pub async fn add_contact_to_list(
        &self,
        list_id: ListId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/lists/{list_id}/contacts/add");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Delete a contact from a list
    pub async fn remove_contact_from_list(
        &self,
        list_id: ListId,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/lists/{list_id}/contacts/remove");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        let mut req_builder = self.client.http().request(Method::POST, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get all the segments
    pub async fn get_segments(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        sort: Option<GetSegmentsSort>,
    ) -> Result<GetSegmentsResponse> {
        let path = "/contacts/segments";
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

    /// Delete a contact
    pub async fn delete_contact(
        &self,
        identifier: &Email,
        identifier_type: Option<DeleteContactIdentifierType>,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/{identifier}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = identifier_type {
                query_pairs.append_pair("identifierType", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::DELETE, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get a contact's details
    pub async fn get_contact_info(
        &self,
        identifier: &Email,
        identifier_type: Option<GetContactInfoIdentifierType>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<GetContactInfoResponse> {
        let path = format!("/contacts/{identifier}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = identifier_type {
                query_pairs.append_pair("identifierType", &val.to_string());
            }
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Update a contact
    pub async fn update_contact(
        &self,
        identifier: &Email,
        body: &UpdateContactRequest,
        identifier_type: Option<UpdateContactIdentifierType>,
    ) -> Result<serde_json::Value> {
        let path = format!("/contacts/{identifier}");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = identifier_type {
                query_pairs.append_pair("identifierType", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::PUT, url);
        req_builder = req_builder.json(body);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }

    /// Get email campaigns' statistics for a contact
    pub async fn get_contact_stats(
        &self,
        identifier: &Email,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<GetContactStatsResponse> {
        let path = format!("/contacts/{identifier}/campaignStats");
        let mut url = self.client.base_url().join(path.trim_start_matches('/'))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(val) = start_date {
                query_pairs.append_pair("startDate", &val.to_string());
            }
            if let Some(val) = end_date {
                query_pairs.append_pair("endDate", &val.to_string());
            }
        }
        let mut req_builder = self.client.http().request(Method::GET, url);
        let req = req_builder.build()?;
        self.client.execute(req).await
    }
}
