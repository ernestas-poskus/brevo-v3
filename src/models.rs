//! Strongly typed Brevo v3 API request and response data models.

#![allow(unused_imports, clippy::all)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorModel {
    /// Error code displayed in case of a failure
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Readable message associated to the failure
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSsoToken {
    /// Session token, it will remain valid for 15 days.
    #[serde(rename = "token")]
    pub token: String,
}

/// Feature name
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InviteuserPrivilegesItemFeature {
    #[serde(rename = "email_campaigns")]
    EmailCampaigns,
    #[serde(rename = "sms_campaigns")]
    SmsCampaigns,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "templates")]
    Templates,
    #[serde(rename = "workflows")]
    Workflows,
    #[serde(rename = "landing_pages")]
    LandingPages,
    #[serde(rename = "transactional_emails")]
    TransactionalEmails,
    #[serde(rename = "smtp_api")]
    SmtpApi,
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "sales_platform")]
    SalesPlatform,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "conversations")]
    Conversations,
    #[serde(rename = "senders_domains_dedicated_ips")]
    SendersDomainsDedicatedIps,
    #[serde(rename = "push_notifications")]
    PushNotifications,
    #[serde(rename = "companies")]
    Companies,
}
impl Default for InviteuserPrivilegesItemFeature {
    fn default() -> Self {
        Self::EmailCampaigns
    }
}
impl core::fmt::Display for InviteuserPrivilegesItemFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailCampaigns => write!(f, "email_campaigns"),
            Self::SmsCampaigns => write!(f, "sms_campaigns"),
            Self::Contacts => write!(f, "contacts"),
            Self::Templates => write!(f, "templates"),
            Self::Workflows => write!(f, "workflows"),
            Self::LandingPages => write!(f, "landing_pages"),
            Self::TransactionalEmails => write!(f, "transactional_emails"),
            Self::SmtpApi => write!(f, "smtp_api"),
            Self::UserManagement => write!(f, "user_management"),
            Self::SalesPlatform => write!(f, "sales_platform"),
            Self::Phone => write!(f, "phone"),
            Self::Conversations => write!(f, "conversations"),
            Self::SendersDomainsDedicatedIps => write!(f, "senders_domains_dedicated_ips"),
            Self::PushNotifications => write!(f, "push_notifications"),
            Self::Companies => write!(f, "companies"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InviteuserPrivilegesItemPermissionsItem {
    #[serde(rename = "create_edit_delete")]
    CreateEditDelete,
    #[serde(rename = "send_schedule_suspend")]
    SendScheduleSuspend,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "export")]
    Export,
    #[serde(rename = "list_and_attributes")]
    ListAndAttributes,
    #[serde(rename = "forms")]
    Forms,
    #[serde(rename = "activate_deactivate")]
    ActivateDeactivate,
    #[serde(rename = "activate_deactivate_pause")]
    ActivateDeactivatePause,
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "schedule_pause")]
    SchedulePause,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "assign")]
    Assign,
    #[serde(rename = "configure")]
    Configure,
    #[serde(rename = "create_edit_deals")]
    CreateEditDeals,
    #[serde(rename = "delete_deals")]
    DeleteDeals,
    #[serde(rename = "manage_others_deals_tasks")]
    ManageOthersDealsTasks,
    #[serde(rename = "manage_owned_companies")]
    ManageOwnedCompanies,
    #[serde(rename = "manage_others_companies")]
    ManageOthersCompanies,
    #[serde(rename = "reports")]
    Reports,
    #[serde(rename = "senders_management")]
    SendersManagement,
    #[serde(rename = "domains_management")]
    DomainsManagement,
    #[serde(rename = "dedicated_ips_management")]
    DedicatedIpsManagement,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "smtp")]
    Smtp,
    #[serde(rename = "api_keys")]
    ApiKeys,
    #[serde(rename = "authorized_ips")]
    AuthorizedIps,
    #[serde(rename = "none")]
    None,
}
impl Default for InviteuserPrivilegesItemPermissionsItem {
    fn default() -> Self {
        Self::CreateEditDelete
    }
}
impl core::fmt::Display for InviteuserPrivilegesItemPermissionsItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CreateEditDelete => write!(f, "create_edit_delete"),
            Self::SendScheduleSuspend => write!(f, "send_schedule_suspend"),
            Self::View => write!(f, "view"),
            Self::Import => write!(f, "import"),
            Self::Export => write!(f, "export"),
            Self::ListAndAttributes => write!(f, "list_and_attributes"),
            Self::Forms => write!(f, "forms"),
            Self::ActivateDeactivate => write!(f, "activate_deactivate"),
            Self::ActivateDeactivatePause => write!(f, "activate_deactivate_pause"),
            Self::Settings => write!(f, "settings"),
            Self::SchedulePause => write!(f, "schedule_pause"),
            Self::All => write!(f, "all"),
            Self::Logs => write!(f, "logs"),
            Self::Access => write!(f, "access"),
            Self::Assign => write!(f, "assign"),
            Self::Configure => write!(f, "configure"),
            Self::CreateEditDeals => write!(f, "create_edit_deals"),
            Self::DeleteDeals => write!(f, "delete_deals"),
            Self::ManageOthersDealsTasks => write!(f, "manage_others_deals_tasks"),
            Self::ManageOwnedCompanies => write!(f, "manage_owned_companies"),
            Self::ManageOthersCompanies => write!(f, "manage_others_companies"),
            Self::Reports => write!(f, "reports"),
            Self::SendersManagement => write!(f, "senders_management"),
            Self::DomainsManagement => write!(f, "domains_management"),
            Self::DedicatedIpsManagement => write!(f, "dedicated_ips_management"),
            Self::Send => write!(f, "send"),
            Self::Smtp => write!(f, "smtp"),
            Self::ApiKeys => write!(f, "api_keys"),
            Self::AuthorizedIps => write!(f, "authorized_ips"),
            Self::None => write!(f, "none"),
        }
    }
}

/// Privileges given to the user
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteuserPrivilegesItem {
    /// Feature name
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none", default)]
    pub feature: Option<InviteuserPrivilegesItemFeature>,
    /// Permissions for a given feature
    #[serde(
        rename = "permissions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub permissions: Option<Vec<InviteuserPrivilegesItemPermissionsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inviteuser {
    /// All access to the features
    #[serde(rename = "all_features_access")]
    pub all_features_access: bool,
    /// Email address for the organization
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    #[serde(rename = "privileges")]
    pub privileges: Vec<InviteuserPrivilegesItem>,
}

/// Type of webhook (marketing, transactional, or inbound)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWebhookType {
    #[serde(rename = "marketing")]
    Marketing,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "inbound")]
    Inbound,
}
impl Default for GetWebhookType {
    fn default() -> Self {
        Self::Marketing
    }
}
impl core::fmt::Display for GetWebhookType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Marketing => write!(f, "marketing"),
            Self::Transactional => write!(f, "transactional"),
            Self::Inbound => write!(f, "inbound"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhook {
    /// Authentication header to be send with the webhook requests
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none", default)]
    pub auth: Option<serde_json::Value>,
    /// Batching configuration of the webhook, we send batched webhooks if its
    /// true
    #[serde(rename = "batched", skip_serializing_if = "Option::is_none", default)]
    pub batched: Option<bool>,
    /// Creation UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Description of the webhook
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "events")]
    pub events: Vec<String>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<serde_json::Value>>,
    /// ID of the webhook
    #[serde(rename = "id")]
    pub id: i64,
    /// Last modification UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Type of webhook (marketing, transactional, or inbound)
    #[serde(rename = "type")]
    pub type_: GetWebhookType,
    /// Inbound domain of the webhook, only returned for inbound type webhooks
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    /// URL of the webhook
    #[serde(rename = "url")]
    pub url: String,
}

/// The contact's subscription status within this consent group
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConsentGroupItemStatus {
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl Default for ConsentGroupItemStatus {
    fn default() -> Self {
        Self::Subscribed
    }
}
impl core::fmt::Display for ConsentGroupItemStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Subscribed => write!(f, "subscribed"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

/// A contact's membership status in a single consent group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentGroupItem {
    /// ID of the consent group
    #[serde(rename = "id")]
    pub id: i64,
    /// The contact's subscription status within this consent group
    #[serde(rename = "status")]
    pub status: ConsentGroupItemStatus,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactDetails {
    /// Set of attributes of the contact
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
    /// Creation UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Email address of the contact for which you requested the details
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Blacklist status for email campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "emailBlacklisted")]
    pub email_blacklisted: bool,
    /// ID of the contact for which you requested the details
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "listIds")]
    pub list_ids: Vec<i64>,
    #[serde(
        rename = "listUnsubscribed",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub list_unsubscribed: Option<Vec<i64>>,
    /// Last modification UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Blacklist status for SMS campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "smsBlacklisted")]
    pub sms_blacklisted: bool,
    /// Blacklist status for WhatsApp campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "whatsappBlacklisted")]
    pub whatsapp_blacklisted: bool,
    /// Consent groups the contact belongs to, with their subscription status. Only present when the Consent Groups feature is enabled for your account.
    #[serde(
        rename = "consentGroups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub consent_groups: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContacts {
    #[serde(rename = "contacts")]
    pub contacts: Vec<serde_json::Value>,
    /// Number of contacts
    #[serde(rename = "count")]
    pub count: i64,
}

/// Error code displayed in case of a failure
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ContactErrorModelCode {
    #[serde(rename = "invalid_parameter")]
    InvalidParameter,
    #[serde(rename = "missing_parameter")]
    MissingParameter,
    #[serde(rename = "document_not_found")]
    DocumentNotFound,
    #[serde(rename = "account_in_process")]
    AccountInProcess,
    #[serde(rename = "duplicate_parameter")]
    DuplicateParameter,
    #[serde(rename = "method_not_allowed")]
    MethodNotAllowed,
    #[serde(rename = "out_of_range")]
    OutOfRange,
}
impl Default for ContactErrorModelCode {
    fn default() -> Self {
        Self::InvalidParameter
    }
}
impl core::fmt::Display for ContactErrorModelCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidParameter => write!(f, "invalid_parameter"),
            Self::MissingParameter => write!(f, "missing_parameter"),
            Self::DocumentNotFound => write!(f, "document_not_found"),
            Self::AccountInProcess => write!(f, "account_in_process"),
            Self::DuplicateParameter => write!(f, "duplicate_parameter"),
            Self::MethodNotAllowed => write!(f, "method_not_allowed"),
            Self::OutOfRange => write!(f, "out_of_range"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactErrorModel {
    /// Error code displayed in case of a failure
    #[serde(rename = "code")]
    pub code: ContactErrorModelCode,
    /// Readable message associated to the failure
    #[serde(rename = "message")]
    pub message: String,
    /// Additional information about the error
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none", default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFolder {
    /// ID of the folder
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the folder
    #[serde(rename = "name")]
    pub name: String,
    /// Number of blacklisted contacts in the folder
    #[serde(rename = "totalBlacklisted")]
    pub total_blacklisted: i64,
    /// Number of contacts in the folder
    #[serde(rename = "totalSubscribers")]
    pub total_subscribers: i64,
    /// Number of unique contacts in the folder
    #[serde(rename = "uniqueSubscribers")]
    pub unique_subscribers: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateFolder {
    /// Name of the folder
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetList {
    /// ID of the list
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the list
    #[serde(rename = "name")]
    pub name: String,
    /// Number of blacklisted contacts in the list
    #[serde(rename = "totalBlacklisted")]
    pub total_blacklisted: i64,
    /// Number of contacts in the list
    #[serde(rename = "totalSubscribers")]
    pub total_subscribers: i64,
    /// Number of unique contacts in the list
    #[serde(rename = "uniqueSubscribers")]
    pub unique_subscribers: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCampaignStats {
    /// Numbers of times your email has been opened automatically through Apple MPP.
    #[serde(rename = "appleMppOpens")]
    pub apple_mpp_opens: i64,
    /// Number of total clicks for the campaign
    #[serde(rename = "clickers")]
    pub clickers: i64,
    /// Number of complaints (Spam reports) for the campaign
    #[serde(rename = "complaints")]
    pub complaints: i64,
    /// Number of deferred emails for the campaign
    #[serde(rename = "deferred", skip_serializing_if = "Option::is_none", default)]
    pub deferred: Option<i64>,
    /// Number of delivered emails for the campaign
    #[serde(rename = "delivered")]
    pub delivered: i64,
    /// Rate of recipients without any privacy protection option enabled in their email client, applied to all delivered emails
    #[serde(
        rename = "estimatedViews",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub estimated_views: Option<i64>,
    /// Number of hard bounces for the campaign
    #[serde(rename = "hardBounces")]
    pub hard_bounces: i64,
    /// List Id of email campaign (only in case of get email campaign(s)(not for global stats))
    #[serde(rename = "listId", skip_serializing_if = "Option::is_none", default)]
    pub list_id: Option<i64>,
    /// Percentage of recipients who open the email out of your total number of recipients. Depending on your Campaign settings, they may include Apple MPP opens.
    #[serde(rename = "opensRate")]
    pub opens_rate: f32,
    /// Total number of non-delivered campaigns for a particular campaign id.
    #[serde(
        rename = "returnBounce",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub return_bounce: Option<i64>,
    /// Number of sent emails for the campaign
    #[serde(rename = "sent")]
    pub sent: i64,
    /// Number of softbounce for the campaign
    #[serde(rename = "softBounces")]
    pub soft_bounces: i64,
    /// Recipients without any privacy protection option enabled in their email email client
    #[serde(rename = "trackableViews")]
    pub trackable_views: i64,
    /// Rate of recipients without any privacy protection option enabled in their email client
    #[serde(
        rename = "trackableViewsRate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub trackable_views_rate: Option<f32>,
    /// Number of unique clicks for the campaign
    #[serde(rename = "uniqueClicks")]
    pub unique_clicks: i64,
    /// Number of unique openings for the campaign
    #[serde(rename = "uniqueViews")]
    pub unique_views: i64,
    /// Number of unsubscription for the campaign
    #[serde(rename = "unsubscriptions")]
    pub unsubscriptions: i64,
    /// Number of openings for the campaign
    #[serde(rename = "viewed")]
    pub viewed: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContactInfoContacts {
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none", default)]
    pub failure: Option<serde_json::Value>,
    /// Id of the process created to remove contacts from list when user opts for "all" option.
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none", default)]
    pub process_id: Option<i64>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none", default)]
    pub success: Option<serde_json::Value>,
    /// Displays the count of total number of contacts removed from list when user opts for "all" option.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none", default)]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContactInfo {
    #[serde(rename = "contacts")]
    pub contacts: PostContactInfoContacts,
}

/// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConsentGroupSignupMode {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
}
impl Default for ConsentGroupSignupMode {
    fn default() -> Self {
        Self::Manual
    }
}
impl core::fmt::Display for ConsentGroupSignupMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Automatic => write!(f, "automatic"),
        }
    }
}

/// A consent group representing a category of contact opt-in/opt-out preferences.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentGroup {
    /// Unique identifier for the consent group
    #[serde(rename = "id")]
    pub id: i64,
    /// Display name of the consent group
    #[serde(rename = "name")]
    pub name: String,
    /// Optional description of the consent group
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
    #[serde(rename = "signupMode")]
    pub signup_mode: ConsentGroupSignupMode,
    /// Number of contacts currently in this consent group
    #[serde(rename = "contactsCount")]
    pub contacts_count: i64,
    /// Whether this is a system-created default consent group
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    /// UTC date-time when the consent group was created (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: time::OffsetDateTime,
    /// UTC date-time when the consent group was last updated (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt")]
    pub updated_at: time::OffsetDateTime,
}

/// Paginated list of consent groups.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentGroupsListResponse {
    /// List of consent groups matching the filter
    #[serde(rename = "consentGroups")]
    pub consent_groups: Vec<serde_json::Value>,
    /// Total number of consent groups matching the filter
    #[serde(rename = "total")]
    pub total: i64,
    /// Maximum number of results returned
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Number of results skipped
    #[serde(rename = "offset")]
    pub offset: i64,
}

/// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateConsentGroupRequestSignupMode {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
}
impl Default for CreateConsentGroupRequestSignupMode {
    fn default() -> Self {
        Self::Manual
    }
}
impl core::fmt::Display for CreateConsentGroupRequestSignupMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Automatic => write!(f, "automatic"),
        }
    }
}

/// Request body for creating a consent group.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConsentGroupRequest {
    /// Unique name for the consent group (max 255 characters)
    #[serde(rename = "name")]
    pub name: String,
    /// Optional description (max 500 characters)
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
    #[serde(rename = "signupMode")]
    pub signup_mode: CreateConsentGroupRequestSignupMode,
    /// Optional list of contact list IDs. Contacts from these lists will be copied once into this consent group at creation time.
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
}

/// New signup mode
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateConsentGroupRequestSignupMode {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
}
impl Default for UpdateConsentGroupRequestSignupMode {
    fn default() -> Self {
        Self::Manual
    }
}
impl core::fmt::Display for UpdateConsentGroupRequestSignupMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Automatic => write!(f, "automatic"),
        }
    }
}

/// Request body for updating a consent group. At least one field must be provided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConsentGroupRequest {
    /// New name for the consent group (max 255 characters)
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// New description (max 500 characters)
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// New signup mode
    #[serde(
        rename = "signupMode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub signup_mode: Option<UpdateConsentGroupRequestSignupMode>,
}

/// Image information (only present for image files).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageAttachmentsItemImageInfo {
    /// Width of the image in pixels.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none", default)]
    pub width: Option<i64>,
    /// Height of the image in pixels.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none", default)]
    pub height: Option<i64>,
    /// URL of the image preview.
    #[serde(
        rename = "previewLink",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageAttachmentsItem {
    /// The name of the file.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// The URL of the file.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none", default)]
    pub link: Option<String>,
    /// The MIME type of the file.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none", default)]
    pub mime_type: Option<String>,
    /// `true` for images.
    #[serde(rename = "isImage", skip_serializing_if = "Option::is_none", default)]
    pub is_image: Option<bool>,
    /// `true` for sticker files.
    #[serde(rename = "isSticker", skip_serializing_if = "Option::is_none", default)]
    pub is_sticker: Option<bool>,
    /// `true` for inline files.
    #[serde(rename = "isInline", skip_serializing_if = "Option::is_none", default)]
    pub is_inline: Option<bool>,
    /// The ID of the inline file.
    #[serde(rename = "inlineId", skip_serializing_if = "Option::is_none", default)]
    pub inline_id: Option<String>,
    /// The size of the file in bytes.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    pub size: Option<i64>,
    /// Image information (only present for image files).
    #[serde(rename = "imageInfo", skip_serializing_if = "Option::is_none", default)]
    pub image_info: Option<ConversationsMessageAttachmentsItemImageInfo>,
    /// Whether the file extension is allowed for download.
    #[serde(
        rename = "isAllowedFileType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_allowed_file_type: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageBccItem {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageCcItem {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Image info, present when the file is an image.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageFileImageInfo {
    /// Width of the image in pixels.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none", default)]
    pub width: Option<i64>,
    /// Height of the image in pixels.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none", default)]
    pub height: Option<i64>,
    /// URL of the image preview.
    #[serde(
        rename = "previewLink",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageFile {
    /// Name of the file.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// URL of the file.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none", default)]
    pub link: Option<String>,
    /// MIME type of the file.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none", default)]
    pub mime_type: Option<String>,
    /// Whether the file is an image.
    #[serde(rename = "isImage", skip_serializing_if = "Option::is_none", default)]
    pub is_image: Option<bool>,
    /// Whether the file is a sticker.
    #[serde(rename = "isSticker", skip_serializing_if = "Option::is_none", default)]
    pub is_sticker: Option<bool>,
    /// Whether the file is inline.
    #[serde(rename = "isInline", skip_serializing_if = "Option::is_none", default)]
    pub is_inline: Option<bool>,
    /// The ID of the inline file.
    #[serde(rename = "inlineId", skip_serializing_if = "Option::is_none", default)]
    pub inline_id: Option<String>,
    /// Size in bytes.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    pub size: Option<i64>,
    /// Image info, present when the file is an image.
    #[serde(rename = "imageInfo", skip_serializing_if = "Option::is_none", default)]
    pub image_info: Option<ConversationsMessageFileImageInfo>,
    /// Whether the file extension is allowed for download.
    #[serde(
        rename = "isAllowedFileType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_allowed_file_type: Option<bool>,
}

/// Status of the message forwarding to the source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageForwardedToSourceStatus {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none", default)]
    pub error: Option<String>,
    #[serde(rename = "isSuccess", skip_serializing_if = "Option::is_none", default)]
    pub is_success: Option<bool>,
}

/// An object containing details about the email sender (applicable only to messages in email threads).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageFrom {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// An object containing details of the reply-to email address (applicable only to messages in email threads).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageReplyTo {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessageToItem {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// `"agent"` for agents’ messages, `"visitor"` for visitors’ messages.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConversationsMessageType {
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "visitor")]
    Visitor,
}
impl Default for ConversationsMessageType {
    fn default() -> Self {
        Self::Agent
    }
}
impl core::fmt::Display for ConversationsMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::Visitor => write!(f, "visitor"),
        }
    }
}

/// a Conversations message
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMessage {
    /// ID of the agent on whose behalf the message was sent (only in messages sent by an agent).
    #[serde(rename = "agentId", skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<String>,
    /// Agent’s name as displayed to the visitor. Only in the messages sent by an agent.
    #[serde(rename = "agentName", skip_serializing_if = "Option::is_none", default)]
    pub agent_name: Option<String>,
    /// Only set if the agent has uploaded a profile picture.
    #[serde(
        rename = "agentUserpic",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub agent_userpic: Option<String>,
    /// An array of file attachments.
    #[serde(
        rename = "attachments",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachments: Option<Vec<ConversationsMessageAttachmentsItem>>,
    /// An array containing details of the blind carbon copy (BCC) recipients (applicable only to messages in email threads).
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none", default)]
    pub bcc: Option<Vec<ConversationsMessageBccItem>>,
    /// An array containing details of the carbon copy (CC) recipients (applicable only to messages in email threads).
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none", default)]
    pub cc: Option<Vec<ConversationsMessageCcItem>>,
    /// Timestamp in milliseconds.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<i64>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none", default)]
    pub file: Option<ConversationsMessageFile>,
    /// Status of the message forwarding to the source.
    #[serde(
        rename = "forwardedToSourceStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub forwarded_to_source_status: Option<ConversationsMessageForwardedToSourceStatus>,
    /// An object containing details about the email sender (applicable only to messages in email threads).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none", default)]
    pub from: Option<ConversationsMessageFrom>,
    /// The HTML content of the message.
    #[serde(rename = "html", skip_serializing_if = "Option::is_none", default)]
    pub html: Option<String>,
    /// Message ID. It can be used for further manipulations with the message.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Integration details.
    #[serde(
        rename = "integrations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub integrations: Option<serde_json::Value>,
    /// `true` for automated messages generated by an AI bot.
    #[serde(rename = "isBot", skip_serializing_if = "Option::is_none", default)]
    pub is_bot: Option<bool>,
    /// `true` for missed and offline messages.
    #[serde(rename = "isMissed", skip_serializing_if = "Option::is_none", default)]
    pub is_missed: Option<bool>,
    /// `true` for unread agent’s messages in finished chats.
    #[serde(
        rename = "isMissedByVisitor",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_missed_by_visitor: Option<bool>,
    /// `true` for pushed messages
    #[serde(rename = "isPushed", skip_serializing_if = "Option::is_none", default)]
    pub is_pushed: Option<bool>,
    /// `true` for automatic messages from “Targeted chats & triggers” and API (<https://developers.brevo.com/docs/javascript-api-reference#sendautomessage>)
    #[serde(rename = "isTrigger", skip_serializing_if = "Option::is_none", default)]
    pub is_trigger: Option<bool>,
    /// Unescaped HTML content of the message (may include unsafe HTML).
    #[serde(
        rename = "rawUnsafeHtml",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub raw_unsafe_html: Option<String>,
    /// In two-way integrations, messages sent via REST API can be marked with receivedFrom property and then filtered out when received in a webhook to avoid infinite loop.
    #[serde(
        rename = "receivedFrom",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub received_from: Option<String>,
    /// An object containing details of the reply-to email address (applicable only to messages in email threads).
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<ConversationsMessageReplyTo>,
    /// The ID of the message assigned by the integration source.
    #[serde(
        rename = "sourceMessageId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub source_message_id: Option<String>,
    /// The subject line of the email message (only for messages sent to email threads).
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Message text or name of the attached file
    #[serde(rename = "text", skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,
    /// An array containing details of the recipients (applicable only to messages in email threads).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none", default)]
    pub to: Option<Vec<ConversationsMessageToItem>>,
    /// `"agent"` for agents’ messages, `"visitor"` for visitors’ messages.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<ConversationsMessageType>,
    /// ‘`true` if the message was sent via JavaScript API.’
    #[serde(
        rename = "isSentViaJsApi",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_sent_via_js_api: Option<bool>,
    /// The type of message content (e.g. for integration-specific message types).
    #[serde(
        rename = "messageType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub message_type: Option<String>,
    /// ‘`true` if the message is a forwarded message.’
    #[serde(rename = "isForward", skip_serializing_if = "Option::is_none", default)]
    pub is_forward: Option<bool>,
    /// Source information for the message.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none", default)]
    pub source: Option<serde_json::Value>,
    /// visitor’s ID
    #[serde(rename = "visitorId", skip_serializing_if = "Option::is_none", default)]
    pub visitor_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryDetails {
    /// Creation UTC date-time of the category (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Category ID for which you requested the details
    #[serde(rename = "id")]
    pub id: String,
    /// category deleted from the shop's database
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    /// Last modification UTC date-time of the category (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Name of the category for which you requested the details
    #[serde(rename = "name")]
    pub name: String,
    /// URL to the category
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCouponCollection {
    /// Datetime on which the collection was created.
    #[serde(rename = "createdAt")]
    pub created_at: time::OffsetDateTime,
    /// The default coupon of the collection.
    #[serde(rename = "defaultCoupon")]
    pub default_coupon: String,
    /// Expiration date for the coupon collection in RFC3339 format.
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// The id of the collection.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the collection.
    #[serde(rename = "name")]
    pub name: String,
    /// Number of coupons that have not been sent yet.
    #[serde(rename = "remainingCoupons")]
    pub remaining_coupons: i64,
    /// If present, an email notification is going to be sent when the total number of available coupons falls below the defined threshold.
    #[serde(
        rename = "remainingCouponsAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_coupons_alert: Option<i64>,
    /// If present, an email notification is going to be sent the defined amount of days before to the expiration date.
    #[serde(
        rename = "remainingDaysAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_days_alert: Option<i64>,
    /// Total number of coupons in the collection.
    #[serde(rename = "totalCoupons")]
    pub total_coupons: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConversionSourceMetricsConversionSource {
    #[serde(rename = "email_campaign")]
    EmailCampaign,
    #[serde(rename = "sms_campaign")]
    SmsCampaign,
    #[serde(rename = "automation_workflow_email")]
    AutomationWorkflowEmail,
    #[serde(rename = "automation_workflow_sms")]
    AutomationWorkflowSms,
}
impl Default for ConversionSourceMetricsConversionSource {
    fn default() -> Self {
        Self::EmailCampaign
    }
}
impl core::fmt::Display for ConversionSourceMetricsConversionSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::AutomationWorkflowEmail => write!(f, "automation_workflow_email"),
            Self::AutomationWorkflowSms => write!(f, "automation_workflow_sms"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionSourceMetrics {
    #[serde(rename = "averageBasket")]
    pub average_basket: f32,
    #[serde(rename = "conversionSource")]
    pub conversion_source: ConversionSourceMetricsConversionSource,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "ordersCount")]
    pub orders_count: f64,
    #[serde(rename = "revenue")]
    pub revenue: f32,
}

/// Billing details of an order.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBilling {
    /// Full billing address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none", default)]
    pub address: Option<String>,
    /// Exact city of the address.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none", default)]
    pub city: Option<String>,
    /// Billing country name.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none", default)]
    pub country: Option<String>,
    /// Billing country 2-letter ISO code.
    #[serde(
        rename = "countryCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_code: Option<String>,
    /// How the visitor will pay for the item(s), e.g. paypal, check, etc.
    #[serde(
        rename = "paymentMethod",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub payment_method: Option<String>,
    /// Billing phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none", default)]
    pub phone: Option<String>,
    /// Postcode for delivery and billing.
    #[serde(rename = "postCode", skip_serializing_if = "Option::is_none", default)]
    pub post_code: Option<String>,
    /// Exact region (state/province) for delivery and billing.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none", default)]
    pub region: Option<String>,
}

/// Identifies the contact associated with the order.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderIdentifiers {
    /// Email of the contact associated with the order
    #[serde(rename = "email_id", skip_serializing_if = "Option::is_none", default)]
    pub email_id: Option<String>,
    /// ext_id associated with the order
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// loyalty_subscription_id associated with the order
    #[serde(
        rename = "loyalty_subscription_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Phone number of the contact associated with the order
    #[serde(rename = "phone_id", skip_serializing_if = "Option::is_none", default)]
    pub phone_id: Option<String>,
}

/// Details for the Products in an order.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderProductsItem {
    /// The price of a unit of product
    #[serde(rename = "price")]
    pub price: f64,
    /// ID of the product.
    #[serde(rename = "productId")]
    pub product_id: String,
    /// Product ID of the red color shirts.
    #[serde(rename = "variantId", skip_serializing_if = "Option::is_none", default)]
    pub variant_id: Option<String>,
    /// **Required if quantityFloat is empty.**
    ///
    /// Number of product units added to the cart (whole numbers only, e.g., 10)
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// **Required if quantity is empty.**
    ///
    /// Number of product units added to the cart(supports decimals, e.g., 20.52)
    #[serde(
        rename = "quantityFloat",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub quantity_float: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Total amount of the order, including all shipping expenses, tax and the price of items.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Billing details of an order.
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none", default)]
    pub billing: Option<OrderBilling>,
    /// Coupons applied to the order. Stored case insensitive.
    #[serde(rename = "coupons", skip_serializing_if = "Option::is_none", default)]
    pub coupons: Option<Vec<String>>,
    /// Event occurrence UTC date-time (YYYY-MM-DDTHH:mm:ssZ), when order is actually created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Unique ID of the order.
    #[serde(rename = "id")]
    pub id: String,
    /// Identifies the contact associated with the order.
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<OrderIdentifiers>,
    /// Meta data of order to store additional detail such as custom message, customer type, source.
    #[serde(rename = "metaInfo", skip_serializing_if = "Option::is_none", default)]
    pub meta_info: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "products")]
    pub products: Vec<OrderProductsItem>,
    /// State of the order.
    #[serde(rename = "status")]
    pub status: String,
    /// ID of store where the order is placed
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none", default)]
    pub store_id: Option<String>,
    /// Event updated UTC date-time (YYYY-MM-DDTHH:mm:ssZ), when the status of the order is actually changed/updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Currency code for the payment amount.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CartCurrency {
    #[serde(rename = "EUR")]
    EUR,
}
impl Default for CartCurrency {
    fn default() -> Self {
        Self::EUR
    }
}
impl core::fmt::Display for CartCurrency {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EUR => write!(f, "EUR"),
        }
    }
}

/// Specify the payment currency and amount.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cart {
    /// Currency code for the payment amount.
    #[serde(rename = "currency")]
    pub currency: CartCurrency,
    /// Payment amount, in cents. e.g. if you want to request €12.00, then the amount in cents is 1200.
    #[serde(rename = "specificAmount")]
    pub specific_amount: i64,
}

/// Optional. Redirect contact to a custom success page once payment is successful. If empty the default Brevo page will be displayed once a payment is validated
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// Absolute URL of the custom success page.
    #[serde(rename = "customSuccessUrl")]
    pub custom_success_url: String,
}

/// Channel used to send the notifications.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NotificationChannel {
    #[serde(rename = "email")]
    Email,
}
impl Default for NotificationChannel {
    fn default() -> Self {
        Self::Email
    }
}
impl core::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
        }
    }
}

/// Optional. Use this object if you want to let Brevo send an email to the contact, with the payment request URL. If empty, no notifications (message and reminders) will be sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// Channel used to send the notifications.
    #[serde(rename = "channel")]
    pub channel: NotificationChannel,
    /// Use this field if you want to give more context to your contact about the payment request.
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductDetails {
    /// Brand of the product
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none", default)]
    pub brand: Option<String>,
    /// Category ID-s of the product
    #[serde(
        rename = "categories",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub categories: Option<Vec<String>>,
    /// Creation UTC date-time of the product (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Description of the product
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Product ID for which you requested the details
    #[serde(rename = "id")]
    pub id: String,
    /// Absolute URL to the cover image of the product
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    /// Product deleted from the shop's database but not from Brevo
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none", default)]
    pub is_deleted: Option<bool>,
    /// Meta data of product such as description, vendor, producer, stock level, etc.
    #[serde(rename = "metaInfo", skip_serializing_if = "Option::is_none", default)]
    pub meta_info: Option<serde_json::Value>,
    /// Last modification UTC date-time of the product (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Name of the product for which you requested the details
    #[serde(rename = "name")]
    pub name: String,
    /// Parent product id of the product
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none", default)]
    pub parent_id: Option<String>,
    /// Price of the product
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f32>,
    /// Alternative price of the product
    #[serde(
        rename = "alternativePrice",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub alternative_price: Option<f32>,
    /// S3 url of original image
    #[serde(
        rename = "s3Original",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub s3_original: Option<String>,
    /// S3 thumbnail url of original image in 120x120 dimension for analytics section
    #[serde(rename = "s3ThumbAnalytics")]
    pub s3_thumb_analytics: String,
    /// S3 thumbnail url of original image in 600x400 dimension for editor section
    #[serde(rename = "s3ThumbEditor")]
    pub s3_thumb_editor: String,
    /// Product identifier from the shop
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none", default)]
    pub sku: Option<String>,
    /// Current stock value of the product from the shop's database
    #[serde(rename = "stock", skip_serializing_if = "Option::is_none", default)]
    pub stock: Option<f64>,
    /// URL to the product
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsListEventsItem {
    /// Contact ID associated with the event
    #[serde(
        rename = "contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_id: Option<i64>,
    /// Date and time of the event
    #[serde(
        rename = "event_date",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_date: Option<time::OffsetDateTime>,
    /// Name of the event
    #[serde(
        rename = "event_name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_name: Option<String>,
    /// Filter ID of the event
    #[serde(
        rename = "event_filter_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_filter_id: Option<String>,
    /// Object type associated with the event
    #[serde(
        rename = "object_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub object_type: Option<String>,
    /// Event-level custom properties
    #[serde(
        rename = "event_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Contact-level properties at time of event
    #[serde(
        rename = "contact_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Response containing a list of events and the total count for pagination
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsList {
    /// List of matching events ordered by event_date descending
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<GetEventsListEventsItem>>,
    /// Total count of events matching the filters (use for pagination)
    #[serde(rename = "count")]
    pub count: i64,
}

/// Response returned when all events in the batch are accepted for async processing
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAcceptedResponse {
    /// Confirmation message indicating the batch was accepted
    #[serde(rename = "message")]
    pub message: String,
    /// Number of events queued for processing
    #[serde(rename = "count")]
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEventsResponseErrorsItem {
    /// Index positions (0-based) of the events that caused the error
    #[serde(
        rename = "eventIndex",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_index: Option<Vec<i32>>,
    /// Description of the error
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
}

/// Response returned when a batch of events is processed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEventsResponse {
    /// Status of the batch request
    #[serde(rename = "status")]
    pub status: String,
    /// Total number of events submitted in the batch
    #[serde(rename = "total_events")]
    pub total_events: i32,
    /// Number of events that were successfully processed
    #[serde(rename = "successful_events")]
    pub successful_events: i32,
    /// Number of events that failed to be processed
    #[serde(rename = "failed_events")]
    pub failed_events: i32,
    /// List of errors for the failed events
    #[serde(rename = "errors")]
    pub errors: Vec<BatchEventsResponseErrorsItem>,
}

/// Unit of balance measurement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionUnit {
    #[serde(rename = "POINTS")]
    POINTS,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "BRL")]
    BRL,
}
impl Default for BalanceDefinitionUnit {
    fn default() -> Self {
        Self::POINTS
    }
}
impl core::fmt::Display for BalanceDefinitionUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::POINTS => write!(f, "POINTS"),
            Self::EUR => write!(f, "EUR"),
            Self::USD => write!(f, "USD"),
            Self::MXN => write!(f, "MXN"),
            Self::GBP => write!(f, "GBP"),
            Self::INR => write!(f, "INR"),
            Self::CAD => write!(f, "CAD"),
            Self::SGD => write!(f, "SGD"),
            Self::RON => write!(f, "RON"),
            Self::JPY => write!(f, "JPY"),
            Self::MYR => write!(f, "MYR"),
            Self::CLP => write!(f, "CLP"),
            Self::PEN => write!(f, "PEN"),
            Self::MAD => write!(f, "MAD"),
            Self::AUD => write!(f, "AUD"),
            Self::CHF => write!(f, "CHF"),
            Self::BRL => write!(f, "BRL"),
        }
    }
}

/// Partial enables partial credit of balance if maximum balance limit is reached. Strict enables rejection of a transaction if it will breach the max balance limit.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "partial")]
    Partial,
}
impl Default for BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    fn default() -> Self {
        Self::Strict
    }
}
impl core::fmt::Display for BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Partial => write!(f, "partial"),
        }
    }
}

/// Rounding strategy for credit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionBalanceOptionCreditRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for BalanceDefinitionBalanceOptionCreditRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display for BalanceDefinitionBalanceOptionCreditRounding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Rounding strategy for debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionBalanceOptionDebitRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for BalanceDefinitionBalanceOptionDebitRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display for BalanceDefinitionBalanceOptionDebitRounding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Unit of time for the balance availability duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionBalanceAvailabilityDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for BalanceDefinitionBalanceAvailabilityDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for BalanceDefinitionBalanceAvailabilityDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Modifier for balance availability duration. startOfPeriod causes expiry at the start of the period, endOfPeriod at the end. noModification uses the exact duration from the credit date.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceDefinitionBalanceAvailabilityDurationModifier {
    #[serde(rename = "noModification")]
    NoModification,
    #[serde(rename = "startOfPeriod")]
    StartOfPeriod,
    #[serde(rename = "endOfPeriod")]
    EndOfPeriod,
}
impl Default for BalanceDefinitionBalanceAvailabilityDurationModifier {
    fn default() -> Self {
        Self::NoModification
    }
}
impl core::fmt::Display for BalanceDefinitionBalanceAvailabilityDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoModification => write!(f, "noModification"),
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceDefinition {
    /// Unique identifier for the balance definition.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Name of the balance definition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Short description of the balance definition.
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Optional image reference URL.
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Additional metadata for the balance definition.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Unit of balance measurement.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<BalanceDefinitionUnit>,
    /// Minimum allowable balance.
    #[serde(rename = "minAmount", skip_serializing_if = "Option::is_none", default)]
    pub min_amount: Option<f64>,
    /// Maximum allowable balance.
    #[serde(rename = "maxAmount", skip_serializing_if = "Option::is_none", default)]
    pub max_amount: Option<f64>,
    /// Maximum credit allowed per operation.
    #[serde(
        rename = "maxCreditAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_credit_amount_limit: Option<f64>,
    /// Maximum debit allowed per operation.
    #[serde(
        rename = "maxDebitAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_debit_amount_limit: Option<f64>,
    /// Partial enables partial credit of balance if maximum balance limit is reached. Strict enables rejection of a transaction if it will breach the max balance limit.
    #[serde(
        rename = "balanceOptionAmountOvertakingStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_amount_overtaking_strategy:
        Option<BalanceDefinitionBalanceOptionAmountOvertakingStrategy>,
    /// Rounding strategy for credit transactions.
    #[serde(
        rename = "balanceOptionCreditRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_credit_rounding: Option<BalanceDefinitionBalanceOptionCreditRounding>,
    /// Rounding strategy for debit transactions.
    #[serde(
        rename = "balanceOptionDebitRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_debit_rounding: Option<BalanceDefinitionBalanceOptionDebitRounding>,
    /// Number of time units before the balance expires.
    #[serde(
        rename = "balanceAvailabilityDurationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_value: Option<i64>,
    /// Unit of time for the balance availability duration.
    #[serde(
        rename = "balanceAvailabilityDurationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_unit:
        Option<BalanceDefinitionBalanceAvailabilityDurationUnit>,
    /// Modifier for balance availability duration. startOfPeriod causes expiry at the start of the period, endOfPeriod at the end. noModification uses the exact duration from the credit date.
    #[serde(
        rename = "balanceAvailabilityDurationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_modifier:
        Option<BalanceDefinitionBalanceAvailabilityDurationModifier>,
    /// Fixed expiration date in dd/mm format. The balance expires when this date next appears in the calendar. Only one of balanceExpirationDate or balance availability duration fields can be used.
    #[serde(
        rename = "balanceExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_expiration_date: Option<String>,
    /// Timestamp of balance definition creation.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp of the last update.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Timestamp of balance definition deletion (nullable).
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none", default)]
    pub deleted_at: Option<time::OffsetDateTime>,
}

/// Specifies whether the limit applies to credit or debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceLimitTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for BalanceLimitTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for BalanceLimitTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// Defines the type of constraint (transaction count or amount).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceLimitConstraintType {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "amount")]
    Amount,
}
impl Default for BalanceLimitConstraintType {
    fn default() -> Self {
        Self::Transaction
    }
}
impl core::fmt::Display for BalanceLimitConstraintType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transaction => write!(f, "transaction"),
            Self::Amount => write!(f, "amount"),
        }
    }
}

/// Time unit for the balance limit.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BalanceLimitDurationUnit {
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for BalanceLimitDurationUnit {
    fn default() -> Self {
        Self::Hour
    }
}
impl core::fmt::Display for BalanceLimitDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceLimit {
    /// Unique identifier for the balance limit.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Balance definition ID.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Specifies whether the limit applies to credit or debit transactions.
    #[serde(
        rename = "transactionType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_type: Option<BalanceLimitTransactionType>,
    /// Defines the type of constraint (transaction count or amount).
    #[serde(
        rename = "constraintType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub constraint_type: Option<BalanceLimitConstraintType>,
    /// Number of time units the balance limit applies to.
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Time unit for the balance limit.
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<BalanceLimitDurationUnit>,
    /// The maximum allowed value for the defined constraint.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
    /// Indicates if the limit resets periodically based on a sliding schedule.
    #[serde(
        rename = "slidingSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sliding_schedule: Option<bool>,
    /// Timestamp of when the balance limit was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp of the last update to the balance limit.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// The type of the transaction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for TransactionTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for TransactionTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// The current status of the transaction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Completed => write!(f, "completed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

/// Represents a transaction involving a balance, including status and timestamps.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Unique identifier (UUID) of the transaction.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// The transaction amount.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f32>,
    /// The contact's total balance for this balance definition after the transaction was applied. Only returned when the transaction actually updated the balance (i.e. it was completed or auto-completed).
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none", default)]
    pub balance: Option<f32>,
    /// The type of the transaction.
    #[serde(
        rename = "transactionType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_type: Option<TransactionTransactionType>,
    /// Optional metadata associated with the transaction.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The current status of the transaction.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<TransactionStatus>,
    /// Unique identifier (UUID) of the associated loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact associated with the transaction.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Optional timestamp indicating when the transaction event occurred (ISO 8601 format).
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none", default)]
    pub event_time: Option<time::OffsetDateTime>,
    /// Reason for rejection if the transaction was declined (nullable).
    #[serde(
        rename = "rejectReason",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reject_reason: Option<String>,
    /// Timestamp when the transaction was rejected (nullable).
    #[serde(
        rename = "rejectedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub rejected_at: Option<time::OffsetDateTime>,
    /// Expiry date of the transaction (nullable).
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// Timestamp when the transaction was completed (nullable).
    #[serde(
        rename = "completedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub completed_at: Option<time::OffsetDateTime>,
    /// Timestamp when the transaction was canceled (nullable).
    #[serde(
        rename = "cancelledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub cancelled_at: Option<time::OffsetDateTime>,
    /// Timestamp when the transaction was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the transaction was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// Current state of the loyalty program.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LoyaltyProgramState {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for LoyaltyProgramState {
    fn default() -> Self {
        Self::Inactive
    }
}
impl core::fmt::Display for LoyaltyProgramState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Inactive => write!(f, "inactive"),
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyProgram {
    /// Loyalty Program unique identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Loyalty Program name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Loyalty Program description.
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Loyalty Program metadata.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Current state of the loyalty program.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none", default)]
    pub state: Option<LoyaltyProgramState>,
    /// Loyalty Program subscription pool ID.
    #[serde(
        rename = "subscriptionPoolId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subscription_pool_id: Option<String>,
    /// Loyalty Program subscription generator ID.
    #[serde(
        rename = "subscriptionGeneratorId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subscription_generator_id: Option<String>,
    /// Subscription code pattern.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none", default)]
    pub pattern: Option<String>,
    /// Number of available subscription codes.
    #[serde(rename = "codeCount", skip_serializing_if = "Option::is_none", default)]
    pub code_count: Option<i64>,
    /// Associated document identifier.
    #[serde(
        rename = "documentId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub document_id: Option<String>,
    /// Contact attribute name used for birthday tracking.
    #[serde(
        rename = "birthdayAttribute",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub birthday_attribute: Option<String>,
    /// Timestamp when the loyalty program was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the loyalty program was last modified.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redeem {
    /// Timestamp when the redemption was cancelled
    #[serde(
        rename = "cancelledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub cancelled_at: Option<time::OffsetDateTime>,
    /// Timestamp when the redemption was completed
    #[serde(
        rename = "completedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub completed_at: Option<time::OffsetDateTime>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Timestamp when the redemption was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Unique identifier for the debit transaction
    #[serde(
        rename = "debitTransactionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub debit_transaction_id: Option<String>,
    /// Timestamp when the redemption expires
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none", default)]
    pub expires_at: Option<time::OffsetDateTime>,
    /// Unique identifier for the redemption
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Unique identifier for the loyalty program
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata associated with the redemption
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Reason for rejection if the redemption was rejected
    #[serde(
        rename = "rejectReason",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reject_reason: Option<String>,
    /// Timestamp when the redemption was rejected
    #[serde(
        rename = "rejectedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub rejected_at: Option<time::OffsetDateTime>,
    /// Unique identifier for the reward attribution
    #[serde(
        rename = "rewardAttributionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reward_attribution_id: Option<String>,
    /// Current status of the redemption
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    /// Timestamp when the redemption was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleConditionResponse {
    /// Metric to indicate AND between rules
    #[serde(rename = "and", skip_serializing_if = "Option::is_none", default)]
    pub and: Option<Vec<serde_json::Value>>,
    /// Condition of the rule
    #[serde(rename = "lhs", skip_serializing_if = "Option::is_none", default)]
    pub lhs: Option<serde_json::Value>,
    /// Selected operator for the rule
    #[serde(rename = "op", skip_serializing_if = "Option::is_none", default)]
    pub op: Option<String>,
    /// Metric to indicate OR between rules
    #[serde(rename = "or", skip_serializing_if = "Option::is_none", default)]
    pub or: Option<Vec<serde_json::Value>>,
    /// Action of the rule
    #[serde(rename = "rhs", skip_serializing_if = "Option::is_none", default)]
    pub rhs: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueResponse {
    /// Array values to define the rule
    #[serde(rename = "array", skip_serializing_if = "Option::is_none", default)]
    pub array: Option<Vec<serde_json::Value>>,
    /// Boolean values for rule definition
    #[serde(rename = "boolean", skip_serializing_if = "Option::is_none", default)]
    pub boolean: Option<bool>,
    /// string
    #[serde(
        rename = "contactProperty",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_property: Option<String>,
    /// Selected date for rule definition
    #[serde(rename = "date", skip_serializing_if = "Option::is_none", default)]
    pub date: Option<String>,
    /// Selected event property for rule definition
    #[serde(
        rename = "eventProperty",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_property: Option<String>,
    /// Created expression for rule definition
    #[serde(
        rename = "expression",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expression: Option<serde_json::Value>,
    /// Boolean values for rule definition
    #[serde(rename = "number", skip_serializing_if = "Option::is_none", default)]
    pub number: Option<f64>,
    /// String values for rule definition
    #[serde(rename = "string", skip_serializing_if = "Option::is_none", default)]
    pub string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeResponse {
    /// array\[object\]
    #[serde(rename = "args", skip_serializing_if = "Option::is_none", default)]
    pub args: Option<Vec<serde_json::Value>>,
    /// Array values for rule definition
    #[serde(rename = "array", skip_serializing_if = "Option::is_none", default)]
    pub array: Option<Vec<serde_json::Value>>,
    /// Boolean values for rule definition
    #[serde(rename = "boolean", skip_serializing_if = "Option::is_none", default)]
    pub boolean: Option<bool>,
    /// Description for rule definition
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Float values for rule definition
    #[serde(rename = "float", skip_serializing_if = "Option::is_none", default)]
    pub float: Option<f64>,
    /// int64
    #[serde(rename = "int", skip_serializing_if = "Option::is_none", default)]
    pub int: Option<i64>,
    /// Operator selected for rule definition
    #[serde(rename = "op", skip_serializing_if = "Option::is_none", default)]
    pub op: Option<String>,
    /// Boolean values for rule definition
    #[serde(rename = "string", skip_serializing_if = "Option::is_none", default)]
    pub string: Option<String>,
}

/// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupDowngradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for TierGroupDowngradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for TierGroupDowngradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

/// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupUpgradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for TierGroupUpgradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for TierGroupUpgradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

/// Duration unit for the schedule.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupUpgradeScheduleDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for TierGroupUpgradeScheduleDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for TierGroupUpgradeScheduleDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Modifier for the duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupUpgradeScheduleDurationModifier {
    #[serde(rename = "start_of_period")]
    StartOfPeriod,
    #[serde(rename = "end_of_period")]
    EndOfPeriod,
}
impl Default for TierGroupUpgradeScheduleDurationModifier {
    fn default() -> Self {
        Self::StartOfPeriod
    }
}
impl core::fmt::Display for TierGroupUpgradeScheduleDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "start_of_period"),
            Self::EndOfPeriod => write!(f, "end_of_period"),
        }
    }
}

/// Schedule configuration for tier upgrades.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierGroupUpgradeSchedule {
    /// Schedule strategy type.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none", default)]
    pub strategy: Option<String>,
    /// Duration value for the schedule.
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<TierGroupUpgradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(
        rename = "durationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_modifier: Option<TierGroupUpgradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(
        rename = "scheduledDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_date: Option<String>,
}

/// Duration unit for the schedule.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupDowngradeScheduleDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for TierGroupDowngradeScheduleDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for TierGroupDowngradeScheduleDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Modifier for the duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TierGroupDowngradeScheduleDurationModifier {
    #[serde(rename = "start_of_period")]
    StartOfPeriod,
    #[serde(rename = "end_of_period")]
    EndOfPeriod,
}
impl Default for TierGroupDowngradeScheduleDurationModifier {
    fn default() -> Self {
        Self::StartOfPeriod
    }
}
impl core::fmt::Display for TierGroupDowngradeScheduleDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "start_of_period"),
            Self::EndOfPeriod => write!(f, "end_of_period"),
        }
    }
}

/// Schedule configuration for tier downgrades.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierGroupDowngradeSchedule {
    /// Schedule strategy type.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none", default)]
    pub strategy: Option<String>,
    /// Duration value for the schedule.
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<TierGroupDowngradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(
        rename = "durationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_modifier: Option<TierGroupDowngradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(
        rename = "scheduledDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierGroup {
    /// Timestamp when the tier group was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(
        rename = "downgradeStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub downgrade_strategy: Option<TierGroupDowngradeStrategy>,
    /// Tier group unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Associated loyalty program Id
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Tier group name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder", skip_serializing_if = "Option::is_none", default)]
    pub tier_order: Option<Vec<String>>,
    /// Timestamp when the tier group was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(
        rename = "upgradeStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub upgrade_strategy: Option<TierGroupUpgradeStrategy>,
    /// Schedule configuration for tier upgrades.
    #[serde(
        rename = "upgradeSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub upgrade_schedule: Option<TierGroupUpgradeSchedule>,
    /// Schedule configuration for tier downgrades.
    #[serde(
        rename = "downgradeSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub downgrade_schedule: Option<TierGroupDowngradeSchedule>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierAccessConditionsItem {
    /// Balance definition identifier
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Minimum value required to access this tier
    #[serde(
        rename = "minimumValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub minimum_value: Option<i64>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierTierRewardsItem {
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Reward to be attributed on tier assignment
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    /// Conditions required to access this tier
    #[serde(
        rename = "accessConditions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub access_conditions: Option<Vec<TierAccessConditionsItem>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Associated group Id
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none", default)]
    pub group_id: Option<String>,
    /// Tier image reference
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Associated loyalty program Id
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Tier name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Tier id
    #[serde(rename = "tierId", skip_serializing_if = "Option::is_none", default)]
    pub tier_id: Option<String>,
    /// Rewards associated with this tier
    #[serde(
        rename = "tierRewards",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub tier_rewards: Option<Vec<TierTierRewardsItem>>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPassInstallUrl {
    /// The wallet installation URL for the contact. Opening it lets the contact add the pass to their Apple Wallet or Google Wallet.
    #[serde(rename = "url")]
    pub url: String,
}

/// Status of the campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetExtendedCampaignOverviewStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "in_process")]
    InProcess,
    #[serde(rename = "in_review")]
    InReview,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl Default for GetExtendedCampaignOverviewStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for GetExtendedCampaignOverviewStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Sent => write!(f, "sent"),
            Self::Archive => write!(f, "archive"),
            Self::Queued => write!(f, "queued"),
            Self::Suspended => write!(f, "suspended"),
            Self::InProcess => write!(f, "in_process"),
            Self::InReview => write!(f, "in_review"),
            Self::Cancelling => write!(f, "cancelling"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Type of campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetExtendedCampaignOverviewType {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "trigger")]
    Trigger,
}
impl Default for GetExtendedCampaignOverviewType {
    fn default() -> Self {
        Self::Classic
    }
}
impl core::fmt::Display for GetExtendedCampaignOverviewType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Classic => write!(f, "classic"),
            Self::Trigger => write!(f, "trigger"),
        }
    }
}

/// Unit of the duration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetExtendedCampaignOverviewEmailExpirationDateUnit {
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}
impl Default for GetExtendedCampaignOverviewEmailExpirationDateUnit {
    fn default() -> Self {
        Self::Days
    }
}
impl core::fmt::Display for GetExtendedCampaignOverviewEmailExpirationDateUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Days => write!(f, "days"),
            Self::Weeks => write!(f, "weeks"),
            Self::Months => write!(f, "months"),
        }
    }
}

/// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedCampaignOverviewEmailExpirationDate {
    /// Duration of the email expiry
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<i64>,
    /// Unit of the duration
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<GetExtendedCampaignOverviewEmailExpirationDateUnit>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedCampaignOverviewSender {
    /// Sender email of the campaign
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Sender id of the campaign
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Sender name of the campaign
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedCampaignOverview {
    /// Url of the attachment file. Only available if the campaign has an attachment.
    #[serde(
        rename = "attachmentFile",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_file: Option<String>,
    /// Status of A/B Test for the campaign. abTesting = false means it is disabled, & abTesting = true means it is enabled.
    #[serde(rename = "abTesting", skip_serializing_if = "Option::is_none", default)]
    pub ab_testing: Option<bool>,
    /// ID of the campaign
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the campaign
    #[serde(rename = "name")]
    pub name: String,
    /// Preview text or preheader of the email campaign
    #[serde(
        rename = "previewText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_text: Option<String>,
    /// UTC date-time on which campaign is scheduled (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// It is true if you have chosen to send your campaign at best time, otherwise it is false
    #[serde(
        rename = "sendAtBestTime",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub send_at_best_time: Option<bool>,
    /// The size of your ab-test groups. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "splitRule", skip_serializing_if = "Option::is_none", default)]
    pub split_rule: Option<i64>,
    /// Status of the campaign
    #[serde(rename = "status")]
    pub status: GetExtendedCampaignOverviewStatus,
    /// Subject of the campaign. Only available if `abTesting` flag of the campaign is `false`
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Subject A of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "subjectA", skip_serializing_if = "Option::is_none", default)]
    pub subject_a: Option<String>,
    /// Subject B of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "subjectB", skip_serializing_if = "Option::is_none", default)]
    pub subject_b: Option<String>,
    /// Type of campaign
    #[serde(rename = "type")]
    pub type_: GetExtendedCampaignOverviewType,
    /// Criteria for the winning version. Only available if `abTesting` flag of the campaign is `true`
    #[serde(
        rename = "winnerCriteria",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_criteria: Option<String>,
    /// The duration of the test in hours at the end of which the winning version will be sent. Only available if `abTesting` flag of the campaign is `true`
    #[serde(
        rename = "winnerDelay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_delay: Option<i64>,
    /// URL of the attachment file associated with the campaign. Empty string if no attachment is present.
    #[serde(
        rename = "attachmentUrl",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_url: Option<String>,
    /// Creation UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
    #[serde(
        rename = "emailExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_expiration_date: Option<GetExtendedCampaignOverviewEmailExpirationDate>,
    /// Footer of the campaign
    #[serde(rename = "footer")]
    pub footer: String,
    /// Header of the campaign
    #[serde(rename = "header")]
    pub header: String,
    /// HTML content of the campaign
    #[serde(rename = "htmlContent")]
    pub html_content: String,
    /// Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email.
    #[serde(
        rename = "inlineImageActivation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub inline_image_activation: Option<bool>,
    /// Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign
    #[serde(
        rename = "mirrorActive",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mirror_active: Option<bool>,
    /// UTC date-time of last modification of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// FOR TRIGGER ONLY ! Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none", default)]
    pub recurring: Option<bool>,
    /// Email defined as the "Reply to" of the campaign
    #[serde(rename = "replyTo")]
    pub reply_to: crate::types::Email,
    /// Total number of non-delivered campaigns for a particular campaign id.
    #[serde(
        rename = "returnBounce",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub return_bounce: Option<i64>,
    #[serde(rename = "sender")]
    pub sender: GetExtendedCampaignOverviewSender,
    /// Sent UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ). Only available if 'status' of the campaign is 'sent'
    #[serde(rename = "sentDate", skip_serializing_if = "Option::is_none", default)]
    pub sent_date: Option<String>,
    /// Link to share the campaign on social media. For trigger campaigns, this returns a descriptive message instead of a URL. For classic campaigns that have not been sent, this also returns a descriptive message.
    #[serde(rename = "shareLink", skip_serializing_if = "Option::is_none", default)]
    pub share_link: Option<String>,
    /// Tag of the campaign
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// List of tags of the campaign
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    /// Retrieved the status of test email sending. (true=Test email has been sent  false=Test email has not been sent)
    #[serde(rename = "testSent")]
    pub test_sent: bool,
    /// Customisation of the "to" field of the campaign
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none", default)]
    pub to_field: Option<String>,
    /// The utm_campaign value associated with the campaign. Only present if a UTM campaign value was set.
    #[serde(
        rename = "utmCampaignValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_campaign_value: Option<String>,
    /// The utm_content value associated with the campaign. Only present if a utm_content value was set on create or update.
    #[serde(
        rename = "utmContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_content: Option<String>,
    /// The campaign ID used as utm_id parameter. Only present if UTM campaign tracking with ID is enabled.
    #[serde(rename = "utmID", skip_serializing_if = "Option::is_none", default)]
    pub utm_id: Option<i64>,
    /// The utm_medium value. Set to "EMAIL" when UTM campaign tracking is enabled.
    #[serde(rename = "utmMedium", skip_serializing_if = "Option::is_none", default)]
    pub utm_medium: Option<String>,
    /// The utm_source value. Set to "Brevo" when UTM campaign tracking is enabled.
    #[serde(rename = "utmSource", skip_serializing_if = "Option::is_none", default)]
    pub utm_source: Option<String>,
    /// The utm_term value associated with the campaign. Only present if a utm_term value was set on create or update.
    #[serde(rename = "utmTerm", skip_serializing_if = "Option::is_none", default)]
    pub utm_term: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCampaignRecipients {
    /// Segment IDs excluded from the campaign
    #[serde(
        rename = "excludedSegments",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub excluded_segments: Option<Vec<i64>>,
    #[serde(rename = "exclusionLists")]
    pub exclusion_lists: Vec<i64>,
    #[serde(rename = "lists")]
    pub lists: Vec<i64>,
    /// Segment IDs included in the campaign
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none", default)]
    pub segments: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceBrowserStats {
    /// Number of total clicks for the campaign using the particular browser
    #[serde(rename = "clickers")]
    pub clickers: i64,
    /// Number of unique clicks for the campaign using the particular browser
    #[serde(rename = "uniqueClicks")]
    pub unique_clicks: i64,
    /// Number of unique openings for the campaign using the particular browser
    #[serde(rename = "uniqueViews")]
    pub unique_views: i64,
    /// Number of openings for the campaign using the particular browser
    #[serde(rename = "viewed")]
    pub viewed: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedCampaignStatsStatsByDevice {
    /// Statistics of the campaign on the basis of desktop devices
    #[serde(rename = "desktop", skip_serializing_if = "Option::is_none", default)]
    pub desktop: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Statistics of the campaign on the basis of mobile devices
    #[serde(rename = "mobile", skip_serializing_if = "Option::is_none", default)]
    pub mobile: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Statistics of the campaign on the basis of tablet devices
    #[serde(rename = "tablet", skip_serializing_if = "Option::is_none", default)]
    pub tablet: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Statistics of the campaign on the basis of unknown devices
    #[serde(rename = "unknown", skip_serializing_if = "Option::is_none", default)]
    pub unknown: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedCampaignStats {
    /// List-wise statistics of the campaign.
    #[serde(rename = "campaignStats")]
    pub campaign_stats: Vec<serde_json::Value>,
    /// Overall statistics of the campaign
    #[serde(rename = "globalStats")]
    pub global_stats: serde_json::Value,
    /// Statistics about the number of clicks for each link in the campaign. Keys are the link URLs, values are click counts. Only populated when the `statistics` query parameter is set to `linksStats`.
    #[serde(rename = "linksStats")]
    pub links_stats: std::collections::HashMap<String, i64>,
    /// Number of clicks on mirror link
    #[serde(rename = "mirrorClick")]
    pub mirror_click: i64,
    /// Number of remaining emails to send
    #[serde(rename = "remaining")]
    pub remaining: i64,
    /// Statistics of the campaign grouped by browser. Only available when retrieving a single campaign with the `statistics` query parameter set to `statsByBrowser`.
    #[serde(
        rename = "statsByBrowser",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub stats_by_browser: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "statsByDevice",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub stats_by_device: Option<GetExtendedCampaignStatsStatsByDevice>,
    /// Statistics of the campaign grouped by email domain. Only populated when the `statistics` query parameter is set to `statsByDomain`.
    #[serde(
        rename = "statsByDomain",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub stats_by_domain: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbTestVersionClicksItem {
    /// Percentage of clicks of link with respect to total clicks
    #[serde(rename = "clickRate")]
    pub click_rate: String,
    /// Number of times a link is clicked
    #[serde(rename = "clicksCount")]
    pub clicks_count: i64,
    /// URL of the link
    #[serde(rename = "link")]
    pub link: String,
}

/// Percentage of a particular event for both versions
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbTestVersionStats {
    /// percentage of an event for version A
    #[serde(rename = "Version A")]
    pub version_a: String,
    /// percentage of an event for version B
    #[serde(rename = "Version B")]
    pub version_b: String,
}

/// Custom attributes for the report email.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendReportEmail {
    /// Custom text message to be presented in the report email.
    #[serde(rename = "body")]
    pub body: String,
    /// Email addresses of the recipients
    #[serde(rename = "to")]
    pub to: Vec<crate::types::Email>,
}

/// Language of email content for campaign report sending.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SendReportLanguage {
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
}
impl Default for SendReportLanguage {
    fn default() -> Self {
        Self::Fr
    }
}
impl core::fmt::Display for SendReportLanguage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fr => write!(f, "fr"),
            Self::Es => write!(f, "es"),
            Self::Pt => write!(f, "pt"),
            Self::It => write!(f, "it"),
            Self::De => write!(f, "de"),
            Self::En => write!(f, "en"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendReport {
    /// Custom attributes for the report email.
    #[serde(rename = "email")]
    pub email: SendReportEmail,
    /// Language of email content for campaign report sending.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none", default)]
    pub language: Option<SendReportLanguage>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTestEmail {
    /// List of the email addresses of the recipients whom you wish to send the test mail. _If left empty, the test mail will be sent to your entire test list. You can not send more than 50 test emails per day_.
    #[serde(rename = "emailTo", skip_serializing_if = "Option::is_none", default)]
    pub email_to: Option<Vec<crate::types::Email>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSendFailed {
    #[serde(
        rename = "blackListedEmails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub black_listed_emails: Option<Vec<crate::types::Email>>,
    /// Response code
    #[serde(rename = "code")]
    pub code: String,
    /// Response message
    #[serde(rename = "message")]
    pub message: String,
    #[serde(
        rename = "unexistingEmails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unexisting_emails: Option<Vec<crate::types::Email>>,
    #[serde(
        rename = "withoutListEmails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub without_list_emails: Option<Vec<crate::types::Email>>,
}

/// Note:- **replicateTemplate** status will be available **only for template type campaigns.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateCampaignStatusStatus {
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "darchive")]
    Darchive,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "replicate")]
    Replicate,
    #[serde(rename = "replicateTemplate")]
    ReplicateTemplate,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for UpdateCampaignStatusStatus {
    fn default() -> Self {
        Self::Suspended
    }
}
impl core::fmt::Display for UpdateCampaignStatusStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Suspended => write!(f, "suspended"),
            Self::Archive => write!(f, "archive"),
            Self::Darchive => write!(f, "darchive"),
            Self::Sent => write!(f, "sent"),
            Self::Queued => write!(f, "queued"),
            Self::Replicate => write!(f, "replicate"),
            Self::ReplicateTemplate => write!(f, "replicateTemplate"),
            Self::Cancel => write!(f, "cancel"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// Status of the campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCampaignStatus {
    /// Note:- **replicateTemplate** status will be available **only for template type campaigns.**
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<UpdateCampaignStatusStatus>,
}

/// Status of the SMS Campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsCampaignOverviewStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "inProcess")]
    InProcess,
}
impl Default for GetSmsCampaignOverviewStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for GetSmsCampaignOverviewStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Sent => write!(f, "sent"),
            Self::Archive => write!(f, "archive"),
            Self::Queued => write!(f, "queued"),
            Self::Suspended => write!(f, "suspended"),
            Self::InProcess => write!(f, "inProcess"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignOverview {
    /// Content of the SMS Campaign
    #[serde(rename = "content")]
    pub content: String,
    /// Creation UTC date-time of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// ID of the SMS Campaign
    #[serde(rename = "id")]
    pub id: i64,
    /// UTC date-time of last modification of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Name of the SMS Campaign
    #[serde(rename = "name")]
    pub name: String,
    /// A recognizable prefix added as the Brand Name before the message content. Empty string if not set.
    #[serde(
        rename = "organisationPrefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation_prefix: Option<String>,
    /// UTC date-time on which SMS campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format. Empty string if not scheduled.
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Sender of the SMS Campaign
    #[serde(rename = "sender")]
    pub sender: String,
    /// UTC date-time on which the SMS campaign was sent (YYYY-MM-DDTHH:mm:ss.SSSZ). Only available if the campaign status is 'sent'.
    #[serde(rename = "sentDate", skip_serializing_if = "Option::is_none", default)]
    pub sent_date: Option<String>,
    /// Status of the SMS Campaign
    #[serde(rename = "status")]
    pub status: GetSmsCampaignOverviewStatus,
    /// Instructions to unsubscribe from future communications. Empty string if not set.
    #[serde(
        rename = "unsubscribeInstruction",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribe_instruction: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignStats {
    /// Number of replies to the SMS
    #[serde(rename = "answered")]
    pub answered: i64,
    /// Number of delivered SMS
    #[serde(rename = "delivered")]
    pub delivered: i64,
    /// Number of hardbounced SMS
    #[serde(rename = "hardBounces")]
    pub hard_bounces: i64,
    /// Number of processing SMS
    #[serde(rename = "processing")]
    pub processing: i64,
    /// Number of sent SMS
    #[serde(rename = "sent")]
    pub sent: i64,
    /// Number of softbounced SMS
    #[serde(rename = "softBounces")]
    pub soft_bounces: i64,
    /// Number of unsubscription SMS
    #[serde(rename = "unsubscriptions")]
    pub unsubscriptions: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhatsappCampStats {
    #[serde(rename = "delivered")]
    pub delivered: i64,
    #[serde(rename = "notSent")]
    pub not_sent: i64,
    #[serde(rename = "read")]
    pub read: i64,
    #[serde(rename = "sent")]
    pub sent: i64,
    #[serde(rename = "unsubscribe")]
    pub unsubscribe: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesItems {
    #[serde(rename = "datatype", skip_serializing_if = "Option::is_none", default)]
    pub datatype: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none", default)]
    pub default: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Company Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    /// Company attributes with values
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Unique company id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Contact ids for contacts linked to this company
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Deal ids for deals linked to this company
    #[serde(
        rename = "linkedDealsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_deals_ids: Option<Vec<String>>,
}

/// Deal Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deal {
    /// Unique deal id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Deal attributes with values
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Contact ids for contacts linked to this deal
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Companies ids for companies linked to this deal
    #[serde(
        rename = "linkedCompaniesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_companies_ids: Option<Vec<String>>,
}

/// File data that is uploaded
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    /// Unique file id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Name of uploaded file
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Account id of user which created the file
    #[serde(rename = "authorId", skip_serializing_if = "Option::is_none", default)]
    pub author_id: Option<String>,
    /// Contact id of contact on which file is uploaded
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Deal id linked to a file
    #[serde(rename = "dealId", skip_serializing_if = "Option::is_none", default)]
    pub deal_id: Option<String>,
    /// Company id linked to a file
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none", default)]
    pub company_id: Option<String>,
    /// Size of file in bytes
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    pub size: Option<i64>,
    /// File created date/time
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
}

/// Note Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// Unique note Id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Content of the note. Supports HTML for rich text formatting. Supported tags include: `<p>` (paragraph), `<b>` / `<strong>` (bold), `<i>` / `<em>` (italic), `<u>` (underline), `<br>` (line break), `<a href="...">` (labelled hyperlink). Example labelled link: `<a href="<https://example.com">Link> text</a>`.
    #[serde(rename = "text")]
    pub text: String,
    /// Contact ids linked to a note
    #[serde(
        rename = "contactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_ids: Option<Vec<i64>>,
    /// Company ids linked to a note
    #[serde(
        rename = "companyIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_ids: Option<Vec<String>>,
    /// Deal ids linked to a note
    #[serde(rename = "dealIds", skip_serializing_if = "Option::is_none", default)]
    pub deal_ids: Option<Vec<String>>,
    /// Account details of user which created the note
    #[serde(rename = "authorId", skip_serializing_if = "Option::is_none", default)]
    pub author_id: Option<serde_json::Value>,
    /// Note created date/time
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Note updated date/time
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// Note data to be saved
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteData {
    /// Company Ids linked to a note
    #[serde(
        rename = "companyIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_ids: Option<Vec<String>>,
    /// Contact Ids linked to a note
    #[serde(
        rename = "contactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_ids: Option<Vec<i64>>,
    /// Deal Ids linked to a note
    #[serde(rename = "dealIds", skip_serializing_if = "Option::is_none", default)]
    pub deal_ids: Option<Vec<String>>,
    /// Content of the note. Supports HTML for rich text formatting. Supported tags include: `<p>` (paragraph), `<b>` / `<strong>` (bold), `<i>` / `<em>` (italic), `<u>` (underline), `<br>` (line break), `<a href="...">` (labelled hyperlink). Example labelled link: `<a href="<https://example.com">Link> text</a>`.
    #[serde(rename = "text")]
    pub text: String,
}

/// Stage details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStagesItem {
    /// Stage id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Stage name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Pipeline details with stages
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    /// Pipeline id
    #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none", default)]
    pub pipeline: Option<String>,
    /// Pipeline Name
    #[serde(
        rename = "pipeline_name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pipeline_name: Option<String>,
    /// List of pipeline stages
    #[serde(rename = "stages", skip_serializing_if = "Option::is_none", default)]
    pub stages: Option<Vec<PipelineStagesItem>>,
}

/// Task Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Unique task id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(rename = "taskTypeId")]
    pub task_type_id: String,
    /// Name of task
    #[serde(rename = "name")]
    pub name: String,
    /// Companies ids for companies a task is linked to
    #[serde(
        rename = "companiesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub companies_ids: Option<Vec<String>>,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds", skip_serializing_if = "Option::is_none", default)]
    pub deals_ids: Option<Vec<String>>,
    /// Contact ids for contacts linked to this task
    #[serde(
        rename = "contactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contacts_ids: Option<Vec<i64>>,
    /// Account id of the user assigned to this task
    #[serde(
        rename = "assignToId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub assign_to_id: Option<String>,
    /// Task due date and time
    #[serde(rename = "date")]
    pub date: time::OffsetDateTime,
    /// Notes added to a task
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    /// Whether the task is marked as done
    #[serde(rename = "done", skip_serializing_if = "Option::is_none", default)]
    pub done: Option<bool>,
    /// Task creation date/time
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Task last update date/time
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskReminderTypesItem {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "push")]
    Push,
}
impl Default for TaskReminderTypesItem {
    fn default() -> Self {
        Self::Email
    }
}
impl core::fmt::Display for TaskReminderTypesItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Push => write!(f, "push"),
        }
    }
}

/// Unit of time before reminder is to be sent
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskReminderUnit {
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "days")]
    Days,
}
impl Default for TaskReminderUnit {
    fn default() -> Self {
        Self::Minutes
    }
}
impl core::fmt::Display for TaskReminderUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Minutes => write!(f, "minutes"),
            Self::Hours => write!(f, "hours"),
            Self::Weeks => write!(f, "weeks"),
            Self::Days => write!(f, "days"),
        }
    }
}

/// Task reminder date/time for a task
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskReminder {
    /// Type of task reminder e.g email, push
    #[serde(rename = "types")]
    pub types: Vec<TaskReminderTypesItem>,
    /// Unit of time before reminder is to be sent
    #[serde(rename = "unit")]
    pub unit: TaskReminderUnit,
    /// Value of time unit before reminder is to be sent
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmtpTemplateOverviewSender {
    /// From email for the template
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Sender id of the template
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Sender name for the template
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmtpTemplateOverview {
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// It is true if template is a valid Double opt-in (DOI) template, otherwise it is false. This field will be available only in case of single template detail call.
    #[serde(
        rename = "doiTemplate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub doi_template: Option<bool>,
    /// HTML content of the template
    #[serde(rename = "htmlContent")]
    pub html_content: String,
    /// ID of the template
    #[serde(rename = "id")]
    pub id: i64,
    /// Status of template (true=active, false=inactive)
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Last modification UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Name of the template
    #[serde(rename = "name")]
    pub name: String,
    /// Email defined as the "Reply to" for the template
    #[serde(rename = "replyTo")]
    pub reply_to: crate::types::Email,
    #[serde(rename = "sender")]
    pub sender: GetSmtpTemplateOverviewSender,
    /// Subject of the template
    #[serde(rename = "subject")]
    pub subject: String,
    /// Tag of the template
    #[serde(rename = "tag")]
    pub tag: String,
    /// Status of test sending for the template (true=test email has been sent, false=test email has not been sent)
    #[serde(rename = "testSent")]
    pub test_sent: bool,
    /// Customisation of the "to" field for the template
    #[serde(rename = "toField")]
    pub to_field: String,
    /// Custom template identifier, if one was assigned during template creation. Only present when the template has a custom ID.
    #[serde(
        rename = "customTemplateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub custom_template_id: Option<String>,
}

/// Type of the SMS. Marketing SMS messages are those sent typically with marketing content. Transactional SMS messages are sent to individuals and are triggered in response to some action, such as a sign-up, purchase, etc.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SendTransacSmsType {
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "marketing")]
    Marketing,
}
impl Default for SendTransacSmsType {
    fn default() -> Self {
        Self::Transactional
    }
}
impl core::fmt::Display for SendTransacSmsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transactional => write!(f, "transactional"),
            Self::Marketing => write!(f, "marketing"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacSms {
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "organisationPrefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation_prefix: Option<String>,
    /// Mobile number to send SMS with the country code. Must contain between 6 and 15 digits, optionally prefixed with '+'.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters.** Alphanumeric sender names (up to 11 characters) must contain only letters and digits. Numeric sender names (12-15 characters) must contain only digits.
    #[serde(rename = "sender")]
    pub sender: String,
    /// Tag of the message. Can be a single string or an array of strings (maximum 10 tags). Each tag must be a non-empty string.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<serde_json::Value>,
    /// Type of the SMS. Marketing SMS messages are those sent typically with marketing content. Transactional SMS messages are sent to individuals and are triggered in response to some action, such as a sign-up, purchase, etc.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<SendTransacSmsType>,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(
        rename = "unicodeEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unicode_enabled: Option<bool>,
    /// Webhook to call for each event triggered by the message (delivered etc.)
    #[serde(rename = "webUrl", skip_serializing_if = "Option::is_none", default)]
    pub web_url: Option<String>,
    /// Pass the set of attributes to customize the template. For example, {"FNAME":"Joe", "LNAME":"Doe"}. These are the placeholder variables in the template that will be replaced with the corresponding values passed in the params object. Applicable only if `templateId` is used.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
}

/// Address informations
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponseAddress {
    /// City information
    #[serde(rename = "city")]
    pub city: String,
    /// Country information
    #[serde(rename = "country")]
    pub country: String,
    /// Street information
    #[serde(rename = "street")]
    pub street: String,
    /// Zip Code information
    #[serde(rename = "zipCode")]
    pub zip_code: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponseMarketingAutomation {
    /// Status of Marketing Automation Plateform activation
    /// for your account (true=enabled, false=disabled)
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Marketing Automation Tracker ID
    #[serde(rename = "key", skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
}

/// This is the type of the credit, "Send Limit" is one
/// of the possible types of credit of a user. "Send
/// Limit" implies the total number of emails you can
/// send to the subscribers in your account.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAccountResponsePlanItemCreditsType {
    #[serde(rename = "sendLimit")]
    SendLimit,
}
impl Default for GetAccountResponsePlanItemCreditsType {
    fn default() -> Self {
        Self::SendLimit
    }
}
impl core::fmt::Display for GetAccountResponsePlanItemCreditsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SendLimit => write!(f, "sendLimit"),
        }
    }
}

/// Displays the plan type of the user
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAccountResponsePlanItemType {
    #[serde(rename = "payAsYouGo")]
    PayAsYouGo,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "subscription")]
    Subscription,
    #[serde(rename = "sms")]
    Sms,
}
impl Default for GetAccountResponsePlanItemType {
    fn default() -> Self {
        Self::PayAsYouGo
    }
}
impl core::fmt::Display for GetAccountResponsePlanItemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PayAsYouGo => write!(f, "payAsYouGo"),
            Self::Free => write!(f, "free"),
            Self::Subscription => write!(f, "subscription"),
            Self::Sms => write!(f, "sms"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponsePlanItem {
    /// Remaining credits of the user
    #[serde(rename = "credits")]
    pub credits: f32,
    /// This is the type of the credit, "Send Limit" is one
    /// of the possible types of credit of a user. "Send
    /// Limit" implies the total number of emails you can
    /// send to the subscribers in your account.
    #[serde(rename = "creditsType")]
    pub credits_type: GetAccountResponsePlanItemCreditsType,
    /// Date of the period from which the plan will end
    /// (Unix timestamp format)
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none", default)]
    pub end_date: Option<String>,
    /// Date of the period from which the plan will start
    /// (Unix timestamp format)
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none", default)]
    pub start_date: Option<String>,
    /// Displays the plan type of the user
    #[serde(rename = "type")]
    pub type_: GetAccountResponsePlanItemType,
}

/// Category of the plan
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAccountResponsePlanVerticalsItemPlanCategory {
    #[serde(rename = "Marketing")]
    Marketing,
    #[serde(rename = "Chat")]
    Chat,
    #[serde(rename = "CRM")]
    CRM,
}
impl Default for GetAccountResponsePlanVerticalsItemPlanCategory {
    fn default() -> Self {
        Self::Marketing
    }
}
impl core::fmt::Display for GetAccountResponsePlanVerticalsItemPlanCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Marketing => write!(f, "Marketing"),
            Self::Chat => write!(f, "Chat"),
            Self::CRM => write!(f, "CRM"),
        }
    }
}

/// Current status of the plan
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAccountResponsePlanVerticalsItemStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}
impl Default for GetAccountResponsePlanVerticalsItemStatus {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetAccountResponsePlanVerticalsItemStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
        }
    }
}

/// User seat information for the plan
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponsePlanVerticalsItemUsers {
    /// Number of purchased user seats
    #[serde(rename = "purchasedSeats")]
    pub purchased_seats: String,
    /// Number of used user seats
    #[serde(rename = "usedSeats")]
    pub used_seats: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponsePlanVerticalsItem {
    /// Category of the plan
    #[serde(rename = "planCategory")]
    pub plan_category: GetAccountResponsePlanVerticalsItemPlanCategory,
    /// Type of plan within the category
    #[serde(rename = "planType")]
    pub plan_type: String,
    /// Name of the plan
    #[serde(rename = "name")]
    pub name: String,
    /// Current status of the plan
    #[serde(rename = "status")]
    pub status: GetAccountResponsePlanVerticalsItemStatus,
    /// Plan start date (Unix timestamp)
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none", default)]
    pub start_date: Option<String>,
    /// Plan end date (Unix timestamp)
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none", default)]
    pub end_date: Option<String>,
    /// User seat information for the plan
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<GetAccountResponsePlanVerticalsItemUsers>,
    /// Credits allocated for this plan category (optional, may not be present for all plan categories)
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none", default)]
    pub credits: Option<String>,
}

/// Data regarding the transactional email account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponseRelayData {
    /// Port used for SMTP Relay
    #[serde(rename = "port")]
    pub port: i64,
    /// URL of the SMTP Relay
    #[serde(rename = "relay")]
    pub relay: String,
    /// Email to use as login on transactional platform
    #[serde(rename = "userName")]
    pub user_name: crate::types::Email,
}

/// Information about your transactional email account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponseRelay {
    /// Data regarding the transactional email account
    #[serde(rename = "data")]
    pub data: GetAccountResponseRelayData,
    /// Status of your transactional email Account
    /// (true=Enabled, false=Disabled)
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponse {
    /// Unique identifier for the organization
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    /// Unique identifier for the user
    #[serde(rename = "user_id")]
    pub user_id: i64,
    /// Indicates if the account has enterprise features
    #[serde(rename = "enterprise")]
    pub enterprise: bool,
    /// Name of the company
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Login Email
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// First Name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Last Name
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Address informations
    #[serde(rename = "address", skip_serializing_if = "Option::is_none", default)]
    pub address: Option<GetAccountResponseAddress>,
    #[serde(
        rename = "marketingAutomation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub marketing_automation: Option<GetAccountResponseMarketingAutomation>,
    /// Information about your plans and credits
    #[serde(rename = "plan")]
    pub plan: Vec<GetAccountResponsePlanItem>,
    /// Detailed information about different plan categories
    #[serde(
        rename = "planVerticals",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub plan_verticals: Option<Vec<GetAccountResponsePlanVerticalsItem>>,
    /// Information about your transactional email account
    #[serde(rename = "relay")]
    pub relay: GetAccountResponseRelay,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateGroupRequest {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName")]
    pub group_name: String,
    /// Pass the list of sub-account Ids to be included in the group
    #[serde(
        rename = "subAccountIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_account_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateGroupResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
    /// List of sub-account ids
    #[serde(rename = "subAccountIds")]
    pub sub_account_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateGroupIdResponseGroup {
    /// Group creation date
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Name of the group
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none", default)]
    pub group_name: Option<String>,
    /// Group id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateGroupIdResponseSubAccountsItem {
    /// Name of the sub-account organzation
    #[serde(
        rename = "companyName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_name: Option<String>,
    /// Creation date of the sub-account organzation
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Id of the sub-account organzation
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateGroupIdResponseUsersItem {
    /// Email address of the user
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// First name of the user
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none", default)]
    pub first_name: Option<String>,
    /// Last name of the user
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none", default)]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateGroupIdResponse {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none", default)]
    pub group: Option<GetCorporateGroupIdResponseGroup>,
    #[serde(
        rename = "sub-accounts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_accounts: Option<Vec<GetCorporateGroupIdResponseSubAccountsItem>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<Vec<GetCorporateGroupIdResponseUsersItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateGroupIdRequest {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none", default)]
    pub group_name: Option<String>,
    /// Pass the list of sub-account Ids to be included in the group
    #[serde(
        rename = "subAccountIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_account_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountGroupsResponseItem {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none", default)]
    pub group_name: Option<String>,
    /// Unique id of the group
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

/// Feature accessiblity given to the user. (Required only
/// if status is active)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateInvitedUsersListResponseUsersItemFeatureAccess {
    /// Analytics dashboard accessibility
    #[serde(rename = "analytics", skip_serializing_if = "Option::is_none", default)]
    pub analytics: Option<Vec<String>>,
    /// Api keys accessiblity.
    #[serde(rename = "api_keys", skip_serializing_if = "Option::is_none", default)]
    pub api_keys: Option<Vec<String>>,
    /// Apps management accessiblity | Not available in
    /// ENTv2
    #[serde(
        rename = "apps_management",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub apps_management: Option<Vec<String>>,
    /// Authorization to create sub-organization in the
    /// admin account. If the user creating the
    /// sub-organization, belongs to a group, the user must
    /// choose a group at the sub-organization creation.
    #[serde(
        rename = "create_sub_organizations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub create_sub_organizations: Option<Vec<String>>,
    /// Authorization to manage and access sub-organizations
    /// in the admin account.
    #[serde(
        rename = "manage_sub_organizations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub manage_sub_organizations: Option<Vec<String>>,
    /// My plan accessiblity.
    #[serde(rename = "my_plan", skip_serializing_if = "Option::is_none", default)]
    pub my_plan: Option<Vec<String>>,
    /// Group creation, modification or deletion
    /// accessibility
    #[serde(
        rename = "sub_organization_groups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_organization_groups: Option<Vec<String>>,
    /// User management accessiblity.
    #[serde(
        rename = "user_management",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub user_management: Option<Vec<String>>,
}

/// Admin user groups list
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateInvitedUsersListResponseUsersItemGroups {
    /// group id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// group name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateInvitedUsersListResponseUsersItem {
    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// Feature accessiblity given to the user. (Required only
    /// if status is active)
    #[serde(rename = "feature_access")]
    pub feature_access: GetCorporateInvitedUsersListResponseUsersItemFeatureAccess,
    /// Admin user groups list
    #[serde(rename = "groups")]
    pub groups: GetCorporateInvitedUsersListResponseUsersItemGroups,
    /// Flag for indicating is user owner of the organization.
    #[serde(rename = "is_owner")]
    pub is_owner: String,
    /// Status of the invited user.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateInvitedUsersListResponse {
    /// Get invited users list
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<Vec<GetCorporateInvitedUsersListResponseUsersItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateIpResponseItem {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
    #[serde(
        rename = "transactional",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transactional: Option<bool>,
}

/// Billing address of master account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponseBillingInfoAddress {
    /// Country code
    #[serde(
        rename = "countryCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_code: Option<String>,
    /// Locality
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none", default)]
    pub locality: Option<String>,
    /// Postal code
    #[serde(
        rename = "postalCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub postal_code: Option<String>,
    /// State code
    #[serde(rename = "stateCode", skip_serializing_if = "Option::is_none", default)]
    pub state_code: Option<String>,
    /// Street address
    #[serde(
        rename = "streetAddress",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub street_address: Option<String>,
}

/// Billing name of master account holder
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponseBillingInfoName {
    /// Last name for billing
    #[serde(
        rename = "familyName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub family_name: Option<String>,
    /// First name for billing
    #[serde(rename = "givenName", skip_serializing_if = "Option::is_none", default)]
    pub given_name: Option<String>,
}

/// Billing details of the master account organization
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponseBillingInfo {
    /// Billing address of master account
    #[serde(rename = "address", skip_serializing_if = "Option::is_none", default)]
    pub address: Option<GetCorporateMasterAccountResponseBillingInfoAddress>,
    /// Company name of master account
    #[serde(
        rename = "companyName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_name: Option<String>,
    /// Billing email id of master account
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// Billing name of master account holder
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<GetCorporateMasterAccountResponseBillingInfoName>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponsePlanInfoFeaturesItem {
    /// Name of the feature
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Quantity provided in the plan
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Quantity with overages provided in the plan (only
    /// applicable on ENTv2)
    #[serde(
        rename = "quantityWithOverages",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub quantity_with_overages: Option<i64>,
    /// Quantity remaining in the plan
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
    /// Unit value of the feature
    #[serde(rename = "unitValue", skip_serializing_if = "Option::is_none", default)]
    pub unit_value: Option<String>,
    /// Quantity consumed by master
    #[serde(rename = "used", skip_serializing_if = "Option::is_none", default)]
    pub used: Option<i64>,
    /// Quantity consumed by sub-organizations over the
    /// admin plan limit (only applicable on ENTv2)
    #[serde(
        rename = "usedOverages",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub used_overages: Option<i64>,
}

/// Plan period type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCorporateMasterAccountResponsePlanInfoPlanPeriod {
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for GetCorporateMasterAccountResponsePlanInfoPlanPeriod {
    fn default() -> Self {
        Self::Month
    }
}
impl core::fmt::Display for GetCorporateMasterAccountResponsePlanInfoPlanPeriod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Plan details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponsePlanInfo {
    /// Plan currency
    #[serde(
        rename = "currencyCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub currency_code: Option<String>,
    /// List of provided features in the plan
    #[serde(rename = "features", skip_serializing_if = "Option::is_none", default)]
    pub features: Option<Vec<GetCorporateMasterAccountResponsePlanInfoFeaturesItem>>,
    /// Timestamp of next billing date
    #[serde(
        rename = "nextBillingAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub next_billing_at: Option<i64>,
    /// Plan period type
    #[serde(
        rename = "planPeriod",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub plan_period: Option<GetCorporateMasterAccountResponsePlanInfoPlanPeriod>,
    /// Plan amount
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f64>,
    /// Number of sub-accounts
    #[serde(
        rename = "subAccounts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_accounts: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateMasterAccountResponse {
    /// Billing details of the master account organization
    #[serde(
        rename = "billingInfo",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub billing_info: Option<GetCorporateMasterAccountResponseBillingInfo>,
    /// Company name of master account organization
    #[serde(
        rename = "companyName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_name: Option<String>,
    /// Currency code of the master account organization
    #[serde(
        rename = "currencyCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub currency_code: Option<String>,
    /// Email id of master account
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// Unique identifier of the master account organization
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Plan details
    #[serde(rename = "planInfo", skip_serializing_if = "Option::is_none", default)]
    pub plan_info: Option<GetCorporateMasterAccountResponsePlanInfo>,
    /// Timezone of the master account organization
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none", default)]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSsoTokenRequest {
    /// User email of admin account
    #[serde(rename = "email")]
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountResponseSubAccountsItemGroupsItem {
    /// Group identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Name of the group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountResponseSubAccountsItem {
    /// Whether the sub-account is active or not
    #[serde(rename = "active")]
    pub active: bool,
    /// Name of the sub-account company
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Timestamp when the sub-account was created
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Group details
    #[serde(rename = "groups")]
    pub groups: Vec<GetCorporateSubAccountResponseSubAccountsItemGroupsItem>,
    /// id of the sub-account
    #[serde(rename = "id")]
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountResponse {
    /// Total number of subaccounts
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(
        rename = "subAccounts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_accounts: Option<Vec<GetCorporateSubAccountResponseSubAccountsItem>>,
}

/// Set the language of the sub-account
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostCorporateSubAccountRequestLanguage {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "de")]
    De,
}
impl Default for PostCorporateSubAccountRequestLanguage {
    fn default() -> Self {
        Self::En
    }
}
impl core::fmt::Display for PostCorporateSubAccountRequestLanguage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Fr => write!(f, "fr"),
            Self::It => write!(f, "it"),
            Self::Es => write!(f, "es"),
            Self::Pt => write!(f, "pt"),
            Self::De => write!(f, "de"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountRequest {
    /// Set the name of the sub-account company
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Email address for the organization
    #[serde(rename = "email")]
    pub email: String,
    /// Set the group(s) for the sub-account
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none", default)]
    pub group_ids: Option<Vec<String>>,
    /// Set the language of the sub-account
    #[serde(rename = "language", skip_serializing_if = "Option::is_none", default)]
    pub language: Option<PostCorporateSubAccountRequestLanguage>,
    /// Set the timezone of the sub-account
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none", default)]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountResponse {
    /// ID of the sub-account created
    #[serde(rename = "id")]
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountIpAssociateRequest {
    /// Pass the list of sub-account Ids to be associated with the IP
    /// address
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
    /// IP address
    #[serde(rename = "ip")]
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountIpDissociateRequest {
    /// Pass the list of sub-account Ids to be dissociated from the IP
    /// address
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
    /// IP address
    #[serde(rename = "ip")]
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountKeyRequest {
    /// Id of the sub-account organization
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the API key
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountKeyResponse {
    /// API key
    #[serde(rename = "key")]
    pub key: String,
    /// Status of the API operation.
    #[serde(rename = "status")]
    pub status: String,
}

/// **Set target after login success** * **automation** - Redirect
/// to Automation after login * **email_campaign** - Redirect to
/// Email Campaign after login * **contacts** - Redirect to Contacts
/// after login * **landing_pages** - Redirect to Landing Pages
/// after login * **email_transactional** - Redirect to Email
/// Transactional after login * **senders** - Redirect to Senders
/// after login * **sms_campaign** - Redirect to Sms Campaign after
/// login * **sms_transactional** - Redirect to Sms Transactional
/// after login
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostCorporateSubAccountSsoTokenRequestTarget {
    #[serde(rename = "automation")]
    Automation,
    #[serde(rename = "email_campaign")]
    EmailCampaign,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "landing_pages")]
    LandingPages,
    #[serde(rename = "email_transactional")]
    EmailTransactional,
    #[serde(rename = "senders")]
    Senders,
    #[serde(rename = "sms_campaign")]
    SmsCampaign,
    #[serde(rename = "sms_transactional")]
    SmsTransactional,
}
impl Default for PostCorporateSubAccountSsoTokenRequestTarget {
    fn default() -> Self {
        Self::Automation
    }
}
impl core::fmt::Display for PostCorporateSubAccountSsoTokenRequestTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Automation => write!(f, "automation"),
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::Contacts => write!(f, "contacts"),
            Self::LandingPages => write!(f, "landing_pages"),
            Self::EmailTransactional => write!(f, "email_transactional"),
            Self::Senders => write!(f, "senders"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::SmsTransactional => write!(f, "sms_transactional"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCorporateSubAccountSsoTokenRequest {
    /// User email of sub-account organization
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// Id of the sub-account organization
    #[serde(rename = "id")]
    pub id: i64,
    /// **Set target after login success** * **automation** - Redirect
    /// to Automation after login * **email_campaign** - Redirect to
    /// Email Campaign after login * **contacts** - Redirect to Contacts
    /// after login * **landing_pages** - Redirect to Landing Pages
    /// after login * **email_transactional** - Redirect to Email
    /// Transactional after login * **senders** - Redirect to Senders
    /// after login * **sms_campaign** - Redirect to Sms Campaign after
    /// login * **sms_transactional** - Redirect to Sms Transactional
    /// after login
    #[serde(rename = "target", skip_serializing_if = "Option::is_none", default)]
    pub target: Option<PostCorporateSubAccountSsoTokenRequestTarget>,
    /// Set the full target URL after login success. The user will land
    /// directly on this target URL after login
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponseGroupsItem {
    /// Group id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Name of the group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Email credits remaining on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsEmails {
    /// Quantity of email messaging limits provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available email messaging limits for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// externalFeeds credits remaining on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds {
    /// Quantity of externalFeeds messaging limits
    /// provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available externalFeeds messaging limits for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// SMS credits remaining on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsSms {
    /// Quantity of SMS messaging limits provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available SMS messaging limits for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Whatsapp credits remaining on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp {
    /// Quantity of whatsapp messaging limits provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available whatsapp messaging limits for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Push credits remaining on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers {
    /// Quantity of Push sending limits provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available Push sending limits for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Credits quota and remaining credits on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoCredits {
    /// Email credits remaining on the sub-account
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none", default)]
    pub emails: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsEmails>,
    /// externalFeeds credits remaining on the sub-account
    #[serde(
        rename = "externalFeeds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub external_feeds: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds>,
    /// SMS credits remaining on the sub-account
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none", default)]
    pub sms: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsSms>,
    /// Whatsapp credits remaining on the sub-account
    #[serde(rename = "whatsapp", skip_serializing_if = "Option::is_none", default)]
    pub whatsapp: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp>,
    /// Push credits remaining on the sub-account
    #[serde(
        rename = "wpSubscribers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub wp_subscribers: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers>,
}

/// Inbox details / Not applicable on ENTv2
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox {
    /// Quantity of inbox provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available inboxes for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Landing page details / Not applicable on ENTv2
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage {
    /// Quantity of landing pages provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available landing pages for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Sales and service users to use phone, sales and
/// conversations
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers {
    /// Quantity of sales users provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available sales users for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Marketing users to manage the marketing channels
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers {
    /// Quantity of marketing users provided
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<i64>,
    /// Available marketing users for use
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none", default)]
    pub remaining: Option<i64>,
}

/// Features available on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeatures {
    /// Inbox details / Not applicable on ENTv2
    #[serde(rename = "inbox", skip_serializing_if = "Option::is_none", default)]
    pub inbox: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox>,
    /// Landing page details / Not applicable on ENTv2
    #[serde(
        rename = "landingPage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landing_page: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage>,
    /// Sales and service users to use phone, sales and
    /// conversations
    #[serde(
        rename = "salesUsers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sales_users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers>,
    /// Marketing users to manage the marketing channels
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers>,
}

/// Sub-account plan details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponsePlanInfo {
    /// Credits quota and remaining credits on the sub-account
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none", default)]
    pub credits: Option<GetCorporateSubAccountIdResponsePlanInfoCredits>,
    /// Features available on the sub-account
    #[serde(rename = "features", skip_serializing_if = "Option::is_none", default)]
    pub features: Option<GetCorporateSubAccountIdResponsePlanInfoFeatures>,
    /// type of the plan
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none", default)]
    pub plan_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateSubAccountIdResponse {
    /// Sub-account company name
    #[serde(
        rename = "companyName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company_name: Option<String>,
    /// Email id of the sub-account organization
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// List of groups associated with the sub-account
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<Vec<GetCorporateSubAccountIdResponseGroupsItem>>,
    /// Name of the sub-account user
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Sub-account plan details
    #[serde(rename = "planInfo", skip_serializing_if = "Option::is_none", default)]
    pub plan_info: Option<GetCorporateSubAccountIdResponsePlanInfo>,
}

/// List of enable/disable applications on the sub-account
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountIdApplicationsToggleRequest {
    /// Set this field to enable or disable Automation on the
    /// sub-account
    #[serde(
        rename = "automation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub automation: Option<bool>,
    /// Set this field to enable or disable Conversations on the
    /// sub-account
    #[serde(
        rename = "conversations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub conversations: Option<bool>,
    /// Set this field to enable or disable Sales CRM on the sub-account
    #[serde(rename = "crm", skip_serializing_if = "Option::is_none", default)]
    pub crm: Option<bool>,
    /// Set this field to enable or disable Email Campaigns on the
    /// sub-account
    #[serde(
        rename = "email-campaigns",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_campaigns: Option<bool>,
    /// Set this field to enable or disable Facebook ads on the
    /// sub-account
    #[serde(
        rename = "facebook-ads",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub facebook_ads: Option<bool>,
    /// Set this field to enable or disable Inbox on the sub-account /
    /// Not applicable on ENTv2
    #[serde(rename = "inbox", skip_serializing_if = "Option::is_none", default)]
    pub inbox: Option<bool>,
    /// Set this field to enable or disable Landing pages on the
    /// sub-account
    #[serde(
        rename = "landing-pages",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landing_pages: Option<bool>,
    /// Set this field to enable or disable Meetings on the sub-account
    #[serde(rename = "meetings", skip_serializing_if = "Option::is_none", default)]
    pub meetings: Option<bool>,
    /// Set this field to enable or disable SMS Marketing on the
    /// sub-account
    #[serde(
        rename = "sms-campaigns",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_campaigns: Option<bool>,
    /// Set this field to enable or disable Transactional Email on the
    /// sub-account
    #[serde(
        rename = "transactional-emails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transactional_emails: Option<bool>,
    /// Set this field to enable or disable Transactional SMS on the
    /// sub-account
    #[serde(
        rename = "transactional-sms",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transactional_sms: Option<bool>,
    /// Set this field to enable or disable Web Push on the sub-account
    #[serde(rename = "web-push", skip_serializing_if = "Option::is_none", default)]
    pub web_push: Option<bool>,
    /// Set this field to enable or disable Whatsapp campaigns on the
    /// sub-account
    #[serde(rename = "whatsapp", skip_serializing_if = "Option::is_none", default)]
    pub whatsapp: Option<bool>,
}

/// Credit details to update
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountIdPlanRequestCredits {
    /// Number of email credits | Pass the value -1 for unlimited
    /// emails in ENTv2 only
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<i64>,
    /// Number of externalFeeds credits, possible values are 0 or 1
    /// |available in ENTv2 only
    #[serde(
        rename = "externalFeeds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub external_feeds: Option<f64>,
    /// Number of SMS credits | Pass the value -1 for unlimited SMS
    /// in ENTv2 only
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none", default)]
    pub sms: Option<f32>,
    /// Number of whatsapp credits | Pass the value -1 for unlimited
    /// whatsapp in ENTv2 only
    #[serde(rename = "whatsapp", skip_serializing_if = "Option::is_none", default)]
    pub whatsapp: Option<f32>,
    /// Number of Push credits, possible value is 0 and -1 |
    /// available in ENT-v2 only
    #[serde(
        rename = "wpSubscribers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub wp_subscribers: Option<i64>,
}

/// Features details to update
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountIdPlanRequestFeatures {
    /// Number of inboxes / Not required on ENTv2
    #[serde(rename = "inbox", skip_serializing_if = "Option::is_none", default)]
    pub inbox: Option<i64>,
    /// Number of landing pages
    #[serde(
        rename = "landingPage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landing_page: Option<i64>,
    /// Number of sales and service users | only available in ENT-V2
    #[serde(
        rename = "salesUsers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sales_users: Option<i64>,
    /// Number of multi-users
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<i64>,
}

/// Details of the plan to be changed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountIdPlanRequest {
    /// Credit details to update
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none", default)]
    pub credits: Option<PutCorporateSubAccountIdPlanRequestCredits>,
    /// Features details to update
    #[serde(rename = "features", skip_serializing_if = "Option::is_none", default)]
    pub features: Option<PutCorporateSubAccountIdPlanRequestFeatures>,
}

/// Credit details to update
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountsPlanRequestCredits {
    /// Number of email credits | Pass the value -1 for unlimited
    /// emails in ENTv2 only
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<i64>,
    /// Number of externalFeeds credits, possible values are 0 or 1
    /// |available in ENTv2 only
    #[serde(
        rename = "externalFeeds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub external_feeds: Option<f64>,
    /// Number of SMS credits | Pass the value -1 for unlimited SMS
    /// in ENTv2 only
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none", default)]
    pub sms: Option<f32>,
    /// Number of whatsapp credits | Pass the value -1 for unlimited
    /// whatsapp in ENTv2 only
    #[serde(rename = "whatsapp", skip_serializing_if = "Option::is_none", default)]
    pub whatsapp: Option<f32>,
    /// Number of Push credits, possible value is 0 and -1 |
    /// available in ENT-v2 only
    #[serde(
        rename = "wpSubscribers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub wp_subscribers: Option<i64>,
}

/// Features details to update
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountsPlanRequestFeatures {
    /// Number of landing pages
    #[serde(
        rename = "landingPage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landing_page: Option<i64>,
    /// Number of sales and service users | only available in ENT-V2
    #[serde(
        rename = "salesUsers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sales_users: Option<i64>,
    /// Number of multi-users
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<i64>,
}

/// Details of the plan to be changed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateSubAccountsPlanRequest {
    /// Credit details to update
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none", default)]
    pub credits: Option<PutCorporateSubAccountsPlanRequestCredits>,
    /// Features details to update
    #[serde(rename = "features", skip_serializing_if = "Option::is_none", default)]
    pub features: Option<PutCorporateSubAccountsPlanRequestFeatures>,
    /// List of sub-account ids
    #[serde(
        rename = "subAccountIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_account_ids: Option<Vec<i64>>,
}

/// Feature name
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InviteAdminUserRequestPrivilegesItemFeature {
    #[serde(rename = "my_plan")]
    MyPlan,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "app_management")]
    AppManagement,
    #[serde(rename = "sub_organization_groups")]
    SubOrganizationGroups,
    #[serde(rename = "create_sub_organizations")]
    CreateSubOrganizations,
    #[serde(rename = "manage_sub_organizations")]
    ManageSubOrganizations,
    #[serde(rename = "analytics")]
    Analytics,
    #[serde(rename = "security")]
    Security,
}
impl Default for InviteAdminUserRequestPrivilegesItemFeature {
    fn default() -> Self {
        Self::MyPlan
    }
}
impl core::fmt::Display for InviteAdminUserRequestPrivilegesItemFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MyPlan => write!(f, "my_plan"),
            Self::Api => write!(f, "api"),
            Self::UserManagement => write!(f, "user_management"),
            Self::AppManagement => write!(f, "app_management"),
            Self::SubOrganizationGroups => write!(f, "sub_organization_groups"),
            Self::CreateSubOrganizations => write!(f, "create_sub_organizations"),
            Self::ManageSubOrganizations => write!(f, "manage_sub_organizations"),
            Self::Analytics => write!(f, "analytics"),
            Self::Security => write!(f, "security"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InviteAdminUserRequestPrivilegesItemPermissionsItem {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "edit_delete")]
    EditDelete,
    #[serde(rename = "download_data")]
    DownloadData,
    #[serde(rename = "create_alerts")]
    CreateAlerts,
}
impl Default for InviteAdminUserRequestPrivilegesItemPermissionsItem {
    fn default() -> Self {
        Self::All
    }
}
impl core::fmt::Display for InviteAdminUserRequestPrivilegesItemPermissionsItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Create => write!(f, "create"),
            Self::EditDelete => write!(f, "edit_delete"),
            Self::DownloadData => write!(f, "download_data"),
            Self::CreateAlerts => write!(f, "create_alerts"),
        }
    }
}

/// Privileges given to the user
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteAdminUserRequestPrivilegesItem {
    /// Feature name
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none", default)]
    pub feature: Option<InviteAdminUserRequestPrivilegesItemFeature>,
    /// Permissions for a given feature
    #[serde(
        rename = "permissions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub permissions: Option<Vec<InviteAdminUserRequestPrivilegesItemPermissionsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteAdminUserRequest {
    /// All access to the features
    #[serde(rename = "all_features_access")]
    pub all_features_access: bool,
    /// Email address for the organization
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Ids of Group
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none", default)]
    pub group_ids: Option<Vec<String>>,
    #[serde(rename = "privileges")]
    pub privileges: Vec<InviteAdminUserRequestPrivilegesItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteAdminUserResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PutCorporateUserInvitationActionEmailAction {
    #[serde(rename = "resend")]
    Resend,
    #[serde(rename = "cancel")]
    Cancel,
}
impl Default for PutCorporateUserInvitationActionEmailAction {
    fn default() -> Self {
        Self::Resend
    }
}
impl core::fmt::Display for PutCorporateUserInvitationActionEmailAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Resend => write!(f, "resend"),
            Self::Cancel => write!(f, "cancel"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateUserInvitationActionEmailResponse {
    /// Action success message
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
}

/// Granular feature permissions given to the user.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateUserPermissionResponseFeatureAccess {
    /// Permission on analytics
    #[serde(rename = "analytics", skip_serializing_if = "Option::is_none", default)]
    pub analytics: Option<Vec<String>>,
    /// Permission on api keys
    #[serde(rename = "api_keys", skip_serializing_if = "Option::is_none", default)]
    pub api_keys: Option<Vec<String>>,
    /// Permission on apps management
    #[serde(
        rename = "apps_management",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub apps_management: Option<Vec<String>>,
    /// Permission on create sub-accounts
    #[serde(
        rename = "create_sub_organizations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub create_sub_organizations: Option<Vec<String>>,
    /// Permission on manage sub-accounts
    #[serde(
        rename = "manage_sub_organizations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub manage_sub_organizations: Option<Vec<String>>,
    /// Permission on my plan
    #[serde(rename = "my_plan", skip_serializing_if = "Option::is_none", default)]
    pub my_plan: Option<Vec<String>>,
    /// Permission on groups
    #[serde(
        rename = "sub_organization_groups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sub_organization_groups: Option<Vec<String>>,
    /// Permission on user management
    #[serde(
        rename = "user_management",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub user_management: Option<Vec<String>>,
}

/// Groups details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateUserPermissionResponseGroupsItem {
    /// group identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Group name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Check admin user permissions
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCorporateUserPermissionResponse {
    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// Granular feature permissions given to the user.
    #[serde(rename = "feature_access")]
    pub feature_access: GetCorporateUserPermissionResponseFeatureAccess,
    #[serde(rename = "groups")]
    pub groups: Vec<GetCorporateUserPermissionResponseGroupsItem>,
    /// Status of the invited user.
    #[serde(rename = "status")]
    pub status: String,
}

/// feature name
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "my_plan")]
    MyPlan,
    #[serde(rename = "apps_management")]
    AppsManagement,
    #[serde(rename = "analytics")]
    Analytics,
    #[serde(rename = "sub_organization_groups")]
    SubOrganizationGroups,
    #[serde(rename = "create_sub_organizations")]
    CreateSubOrganizations,
    #[serde(rename = "manage_sub_organizations")]
    ManageSubOrganizations,
    #[serde(rename = "security")]
    Security,
}
impl Default for PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    fn default() -> Self {
        Self::UserManagement
    }
}
impl core::fmt::Display for PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UserManagement => write!(f, "user_management"),
            Self::Api => write!(f, "api"),
            Self::MyPlan => write!(f, "my_plan"),
            Self::AppsManagement => write!(f, "apps_management"),
            Self::Analytics => write!(f, "analytics"),
            Self::SubOrganizationGroups => write!(f, "sub_organization_groups"),
            Self::CreateSubOrganizations => write!(f, "create_sub_organizations"),
            Self::ManageSubOrganizations => write!(f, "manage_sub_organizations"),
            Self::Security => write!(f, "security"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "edit_delete")]
    EditDelete,
    #[serde(rename = "create_alerts")]
    CreateAlerts,
    #[serde(rename = "download_data")]
    DownloadData,
    #[serde(rename = "my_looks")]
    MyLooks,
    #[serde(rename = "explore_create")]
    ExploreCreate,
}
impl Default for PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    fn default() -> Self {
        Self::All
    }
}
impl core::fmt::Display for PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Create => write!(f, "create"),
            Self::EditDelete => write!(f, "edit_delete"),
            Self::CreateAlerts => write!(f, "create_alerts"),
            Self::DownloadData => write!(f, "download_data"),
            Self::MyLooks => write!(f, "my_looks"),
            Self::ExploreCreate => write!(f, "explore_create"),
        }
    }
}

/// Permission on features
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    /// feature name
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none", default)]
    pub feature: Option<PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature>,
    /// Permission for the feature
    #[serde(
        rename = "permissions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub permissions:
        Option<Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCorporateUserEmailPermissionsRequest {
    /// All access to the features
    #[serde(rename = "all_features_access")]
    pub all_features_access: bool,
    #[serde(rename = "privileges")]
    pub privileges: Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountActivityResponseLogsItem {
    /// Type of activity in the account.
    #[serde(rename = "action")]
    pub action: String,
    /// Time of the activity.
    #[serde(rename = "date")]
    pub date: String,
    /// Browser details of the user who performed the activity.
    #[serde(rename = "user_agent")]
    pub user_agent: String,
    /// Email address of the user who performed activity in the
    /// account.
    #[serde(rename = "user_email")]
    pub user_email: String,
    /// IP address of the user who performed activity in the
    /// account.
    #[serde(rename = "user_ip")]
    pub user_ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountActivityResponse {
    /// Get user activity logs
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none", default)]
    pub logs: Option<Vec<GetAccountActivityResponseLogsItem>>,
}

/// Feature accessiblity given to the user.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInvitedUsersListResponseUsersItemFeatureAccess {
    /// Conversations features accessiblity.
    #[serde(
        rename = "conversations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub conversations: Option<String>,
    /// CRM features accessiblity.
    #[serde(rename = "crm", skip_serializing_if = "Option::is_none", default)]
    pub crm: Option<String>,
    /// Marketing features accessiblity.
    #[serde(rename = "marketing", skip_serializing_if = "Option::is_none", default)]
    pub marketing: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInvitedUsersListResponseUsersItem {
    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// Feature accessiblity given to the user.
    #[serde(rename = "feature_access")]
    pub feature_access: GetInvitedUsersListResponseUsersItemFeatureAccess,
    /// Flag for indicating is user owner of the organization.
    #[serde(rename = "is_owner")]
    pub is_owner: String,
    /// Status of the invited user.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInvitedUsersListResponse {
    /// Get invited users list
    #[serde(rename = "users", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<Vec<GetInvitedUsersListResponseUsersItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutRevokeUserPermissionResponse {
    /// Credit note
    #[serde(
        rename = "credit_notes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub credit_notes: Option<Vec<String>>,
    /// Status of the API operation.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteuserResponse {
    /// Invoice id
    #[serde(
        rename = "invoice_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub invoice_id: Option<String>,
    /// Status of the API operation.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PutresendcancelinvitationAction {
    #[serde(rename = "resend")]
    Resend,
    #[serde(rename = "cancel")]
    Cancel,
}
impl Default for PutresendcancelinvitationAction {
    fn default() -> Self {
        Self::Resend
    }
}
impl core::fmt::Display for PutresendcancelinvitationAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Resend => write!(f, "resend"),
            Self::Cancel => write!(f, "cancel"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutresendcancelinvitationResponse {
    /// Credit note
    #[serde(
        rename = "credit_notes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub credit_notes: Option<Vec<String>>,
    /// Status of the API operation.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditUserPermissionResponse {
    /// Credit note
    #[serde(
        rename = "credit_notes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub credit_notes: Option<Vec<String>>,
    /// Invoice id
    #[serde(
        rename = "invoice_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub invoice_id: Option<String>,
    /// Status of the API operation.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserPermissionResponsePrivilegesItem {
    #[serde(rename = "feature")]
    pub feature: String,
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

/// Check user permission
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserPermissionResponse {
    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// Granular feature permissions given to the user.
    #[serde(rename = "privileges")]
    pub privileges: Vec<GetUserPermissionResponsePrivilegesItem>,
    /// Status of the invited user.
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProcessesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetProcessesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetProcessesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Name/type of the process
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProcessesResponseProcessesItemName {
    #[serde(rename = "IMPORTUSER")]
    IMPORTUSER,
    #[serde(rename = "SEARCH_EXPORT_USERS")]
    SEARCHEXPORTUSERS,
    #[serde(rename = "TRANS-CALC")]
    TRANSCALC,
    #[serde(rename = "TRANS-GLOBAL-CALC")]
    TRANSGLOBALCALC,
    #[serde(rename = "CAMPAIGN_PROCESSING")]
    CAMPAIGNPROCESSING,
    #[serde(rename = "LIST_EXPORT")]
    LISTEXPORT,
    #[serde(rename = "CONTACT_EXPORT")]
    CONTACTEXPORT,
}
impl Default for GetProcessesResponseProcessesItemName {
    fn default() -> Self {
        Self::IMPORTUSER
    }
}
impl core::fmt::Display for GetProcessesResponseProcessesItemName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IMPORTUSER => write!(f, "IMPORTUSER"),
            Self::SEARCHEXPORTUSERS => write!(f, "SEARCH_EXPORT_USERS"),
            Self::TRANSCALC => write!(f, "TRANS-CALC"),
            Self::TRANSGLOBALCALC => write!(f, "TRANS-GLOBAL-CALC"),
            Self::CAMPAIGNPROCESSING => write!(f, "CAMPAIGN_PROCESSING"),
            Self::LISTEXPORT => write!(f, "LIST_EXPORT"),
            Self::CONTACTEXPORT => write!(f, "CONTACT_EXPORT"),
        }
    }
}

/// Current status of the process
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProcessesResponseProcessesItemStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl Default for GetProcessesResponseProcessesItemStatus {
    fn default() -> Self {
        Self::Queued
    }
}
impl core::fmt::Display for GetProcessesResponseProcessesItemStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Import process details with URLs to CSV reports
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessesResponseProcessesItemInfoImport {
    /// URL to CSV file containing invalid email addresses, or null if none
    #[serde(
        rename = "invalid_emails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub invalid_emails: Option<String>,
    /// URL to CSV file containing duplicate contact IDs, or null if none
    #[serde(
        rename = "duplicate_contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_contact_id: Option<String>,
    /// URL to CSV file containing duplicate external IDs, or null if none
    #[serde(
        rename = "duplicate_ext_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_ext_id: Option<String>,
    /// URL to CSV file containing duplicate email IDs, or null if none
    #[serde(
        rename = "duplicate_email_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_email_id: Option<String>,
    /// URL to CSV file containing duplicate phone numbers, or null if none
    #[serde(
        rename = "duplicate_phone_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_phone_id: Option<String>,
    /// URL to CSV file containing duplicate WhatsApp numbers, or null if none
    #[serde(
        rename = "duplicate_whatsapp_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_whatsapp_id: Option<String>,
    /// URL to CSV file containing duplicate landline numbers, or null if none
    #[serde(
        rename = "duplicate_landline_number_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_landline_number_id: Option<String>,
}

/// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessesResponseProcessesItemInfo {
    /// Import process details with URLs to CSV reports
    #[serde(rename = "import", skip_serializing_if = "Option::is_none", default)]
    pub import: Option<GetProcessesResponseProcessesItemInfoImport>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessesResponseProcessesItem {
    /// Unique identifier of the process
    #[serde(rename = "id")]
    pub id: i64,
    /// Name/type of the process
    #[serde(rename = "name")]
    pub name: GetProcessesResponseProcessesItemName,
    /// Current status of the process
    #[serde(rename = "status")]
    pub status: GetProcessesResponseProcessesItemStatus,
    /// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
    #[serde(rename = "info", skip_serializing_if = "Option::is_none", default)]
    pub info: Option<GetProcessesResponseProcessesItemInfo>,
    /// Download URL for completed export processes (returned for SEARCH_EXPORT_USERS, SEARCH_EXPORT_USERS_API, CAMPAIGN_USER_DETAILS, and EXPORT_WEBHOOK process types)
    #[serde(
        rename = "export_url",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessesResponse {
    /// Total number of processes available on your account
    #[serde(rename = "count")]
    pub count: i64,
    /// List of background processes on your account
    #[serde(rename = "processes")]
    pub processes: Vec<GetProcessesResponseProcessesItem>,
}

/// Name/type of the process
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProcessResponseName {
    #[serde(rename = "IMPORTUSER")]
    IMPORTUSER,
    #[serde(rename = "SEARCH_EXPORT_USERS")]
    SEARCHEXPORTUSERS,
    #[serde(rename = "TRANS-CALC")]
    TRANSCALC,
    #[serde(rename = "TRANS-GLOBAL-CALC")]
    TRANSGLOBALCALC,
    #[serde(rename = "CAMPAIGN_PROCESSING")]
    CAMPAIGNPROCESSING,
    #[serde(rename = "LIST_EXPORT")]
    LISTEXPORT,
    #[serde(rename = "CONTACT_EXPORT")]
    CONTACTEXPORT,
}
impl Default for GetProcessResponseName {
    fn default() -> Self {
        Self::IMPORTUSER
    }
}
impl core::fmt::Display for GetProcessResponseName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IMPORTUSER => write!(f, "IMPORTUSER"),
            Self::SEARCHEXPORTUSERS => write!(f, "SEARCH_EXPORT_USERS"),
            Self::TRANSCALC => write!(f, "TRANS-CALC"),
            Self::TRANSGLOBALCALC => write!(f, "TRANS-GLOBAL-CALC"),
            Self::CAMPAIGNPROCESSING => write!(f, "CAMPAIGN_PROCESSING"),
            Self::LISTEXPORT => write!(f, "LIST_EXPORT"),
            Self::CONTACTEXPORT => write!(f, "CONTACT_EXPORT"),
        }
    }
}

/// Current status of the process
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProcessResponseStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl Default for GetProcessResponseStatus {
    fn default() -> Self {
        Self::Queued
    }
}
impl core::fmt::Display for GetProcessResponseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Import process details with URLs to CSV reports
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessResponseInfoImport {
    /// URL to CSV file containing invalid email addresses, or null if none
    #[serde(
        rename = "invalid_emails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub invalid_emails: Option<String>,
    /// URL to CSV file containing duplicate contact IDs, or null if none
    #[serde(
        rename = "duplicate_contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_contact_id: Option<String>,
    /// URL to CSV file containing duplicate external IDs, or null if none
    #[serde(
        rename = "duplicate_ext_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_ext_id: Option<String>,
    /// URL to CSV file containing duplicate email IDs, or null if none
    #[serde(
        rename = "duplicate_email_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_email_id: Option<String>,
    /// URL to CSV file containing duplicate phone numbers, or null if none
    #[serde(
        rename = "duplicate_phone_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_phone_id: Option<String>,
    /// URL to CSV file containing duplicate WhatsApp numbers, or null if none
    #[serde(
        rename = "duplicate_whatsapp_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_whatsapp_id: Option<String>,
    /// URL to CSV file containing duplicate landline numbers, or null if none
    #[serde(
        rename = "duplicate_landline_number_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duplicate_landline_number_id: Option<String>,
}

/// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessResponseInfo {
    /// Import process details with URLs to CSV reports
    #[serde(rename = "import", skip_serializing_if = "Option::is_none", default)]
    pub import: Option<GetProcessResponseInfoImport>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessResponse {
    /// Unique identifier of the process
    #[serde(rename = "id")]
    pub id: i64,
    /// Name/type of the process
    #[serde(rename = "name")]
    pub name: GetProcessResponseName,
    /// Current status of the process
    #[serde(rename = "status")]
    pub status: GetProcessResponseStatus,
    /// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
    #[serde(rename = "info", skip_serializing_if = "Option::is_none", default)]
    pub info: Option<GetProcessResponseInfo>,
    /// Download URL for completed export processes (returned for SEARCH_EXPORT_USERS, SEARCH_EXPORT_USERS_API, CAMPAIGN_USER_DETAILS, and EXPORT_WEBHOOK process types)
    #[serde(
        rename = "export_url",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSendersResponseSendersItemIpsItem {
    /// Domain of the IP
    #[serde(rename = "domain")]
    pub domain: String,
    /// Dedicated IP available in your account
    #[serde(rename = "ip")]
    pub ip: String,
    /// Weight of the IP for this sender
    #[serde(rename = "weight")]
    pub weight: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSendersResponseSendersItem {
    /// Status of sender (true=activated, false=deactivated)
    #[serde(rename = "active")]
    pub active: bool,
    /// From Email associated to the sender
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Id of the sender
    #[serde(rename = "id")]
    pub id: i64,
    /// List of dedicated IP(s) available in the account. This
    /// data is displayed only for dedicated IPs. For standard accounts,
    /// this will be an empty array.
    #[serde(rename = "ips")]
    pub ips: Vec<GetSendersResponseSendersItemIpsItem>,
    /// From Name associated to the sender
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSendersResponse {
    /// List of the senders available in your account
    #[serde(rename = "senders", skip_serializing_if = "Option::is_none", default)]
    pub senders: Option<Vec<GetSendersResponseSendersItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSenderRequestIpsItem {
    /// Domain of the IP
    #[serde(rename = "domain")]
    pub domain: String,
    /// Dedicated IP available in your account
    #[serde(rename = "ip")]
    pub ip: String,
    /// Weight to apply to the IP. Sum of all IP weights must be
    /// 100. Should be passed for either ALL or NONE of the IPs.
    /// If it's not passed, the sending will be equally balanced
    /// on all IPs.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none", default)]
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSenderRequest {
    /// From email to use for the sender. A verification email will be
    /// sent to this address.
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// **Mandatory in case of dedicated IP**. IPs to associate to the
    /// sender. Not required for standard accounts.
    #[serde(rename = "ips", skip_serializing_if = "Option::is_none", default)]
    pub ips: Option<Vec<CreateSenderRequestIpsItem>>,
    /// From Name to use for the sender
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSenderResponse {
    /// Status of DKIM configuration for the sender (true = DKIM not
    /// well configured, false = DKIM well configured)
    #[serde(rename = "dkimError", skip_serializing_if = "Option::is_none", default)]
    pub dkim_error: Option<bool>,
    /// ID of the Sender created
    #[serde(rename = "id")]
    pub id: i64,
    /// Status of SPF configuration for the sender (true = SPF not
    /// well configured, false = SPF well configured)
    #[serde(rename = "spfError", skip_serializing_if = "Option::is_none", default)]
    pub spf_error: Option<bool>,
}

/// Information about who created the domain
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsResponseDomainsItemCreator {
    /// Creator's user ID
    #[serde(rename = "id")]
    pub id: String,
    /// Creator's email address
    #[serde(rename = "email")]
    pub email: String,
    /// Domain creation date
    #[serde(rename = "creationDate")]
    pub creation_date: time::OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsResponseDomainsItem {
    /// Unique identifier of the domain
    #[serde(rename = "id")]
    pub id: String,
    /// Domain name
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    /// Status of domain authentication (true=authenticated,
    /// false=non authenticated)
    #[serde(rename = "authenticated")]
    pub authenticated: bool,
    /// Status of domain verification (true=verified, false=non
    /// verified)
    #[serde(rename = "verified")]
    pub verified: bool,
    /// Dedicated IP associated with domain (null if none)
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
    /// DNS provider for the domain
    #[serde(rename = "provider")]
    pub provider: String,
    /// Information about who created the domain
    #[serde(rename = "creator")]
    pub creator: GetDomainsResponseDomainsItemCreator,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsResponse {
    /// List of the domains available in your account
    #[serde(rename = "domains")]
    pub domains: Vec<GetDomainsResponseDomainsItem>,
    /// Total number of domains
    #[serde(rename = "count")]
    pub count: i64,
    /// Current page number
    #[serde(rename = "current_page")]
    pub current_page: i64,
    /// Total number of pages
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainRequest {
    /// Domain name to be added
    #[serde(rename = "name")]
    pub name: String,
}

/// Brevo verification code record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainResponseDnsRecordsBrevoCode {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DKIM authentication record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainResponseDnsRecordsDkimRecord {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DMARC policy record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainResponseDnsRecordsDmarcRecord {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DNS records required for domain authentication
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainResponseDnsRecords {
    /// Brevo verification code record
    #[serde(rename = "brevo_code")]
    pub brevo_code: CreateDomainResponseDnsRecordsBrevoCode,
    /// DKIM authentication record
    #[serde(rename = "dkim_record")]
    pub dkim_record: CreateDomainResponseDnsRecordsDkimRecord,
    /// DMARC policy record
    #[serde(rename = "dmarc_record")]
    pub dmarc_record: CreateDomainResponseDnsRecordsDmarcRecord,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainResponse {
    /// ID of the Domain created
    #[serde(rename = "id")]
    pub id: i64,
    /// Domain name
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    /// Detected domain provider
    #[serde(
        rename = "domain_provider",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub domain_provider: Option<String>,
    /// Success message with next steps
    #[serde(rename = "message")]
    pub message: String,
    /// DNS records required for domain authentication
    #[serde(
        rename = "dns_records",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub dns_records: Option<CreateDomainResponseDnsRecords>,
}

/// Brevo verification code record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainConfigurationResponseDnsRecordsBrevoCode {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DKIM authentication record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainConfigurationResponseDnsRecordsDkimRecord {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DMARC policy record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainConfigurationResponseDnsRecordsDmarcRecord {
    /// DNS record hostname
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// DNS record type
    #[serde(rename = "type")]
    pub type_: String,
    /// DNS record value
    #[serde(rename = "value")]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(rename = "status")]
    pub status: bool,
}

/// DNS records configuration and status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainConfigurationResponseDnsRecords {
    /// Brevo verification code record
    #[serde(rename = "brevo_code")]
    pub brevo_code: GetDomainConfigurationResponseDnsRecordsBrevoCode,
    /// DKIM authentication record
    #[serde(rename = "dkim_record")]
    pub dkim_record: GetDomainConfigurationResponseDnsRecordsDkimRecord,
    /// DMARC policy record
    #[serde(rename = "dmarc_record")]
    pub dmarc_record: GetDomainConfigurationResponseDnsRecordsDmarcRecord,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainConfigurationResponse {
    /// Domain name
    #[serde(rename = "domain")]
    pub domain: String,
    /// Status of domain verification (true=verified, false=non
    /// verified)
    #[serde(rename = "verified")]
    pub verified: bool,
    /// Status of domain authentication (true=authenticated, false=non
    /// authenticated)
    #[serde(rename = "authenticated")]
    pub authenticated: bool,
    /// DNS records configuration and status
    #[serde(rename = "dns_records")]
    pub dns_records: GetDomainConfigurationResponseDnsRecords,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateDomainResponse {
    /// Domain name that was authenticated
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    /// Success message confirming authentication
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIpsResponseIpsItem {
    /// ID of the dedicated IP
    #[serde(rename = "id")]
    pub id: i64,
    /// Status of the IP (true=active, false=inactive)
    #[serde(rename = "active")]
    pub active: bool,
    /// Domain associated to the IP
    #[serde(rename = "domain")]
    pub domain: String,
    /// Dedicated IP address
    #[serde(rename = "ip")]
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIpsResponse {
    /// Dedicated IP(s) available on your account. Returns empty array
    /// for standard accounts without dedicated IPs.
    #[serde(rename = "ips")]
    pub ips: Vec<GetIpsResponseIpsItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSenderRequestIpsItem {
    /// Domain of the IP
    #[serde(rename = "domain")]
    pub domain: String,
    /// Dedicated IP available in your account
    #[serde(rename = "ip")]
    pub ip: String,
    /// Weight to apply to the IP. Sum of all IP weights must be
    /// 100. Should be passed for either ALL or NONE of the IPs.
    /// If it's not passed, the sending will be equally balanced
    /// on all IPs.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none", default)]
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSenderRequest {
    /// From Email to update the sender
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// **Only in case of dedicated IP**. IPs to associate to the
    /// sender. If passed, will replace all the existing IPs. Not required for standard accounts.
    #[serde(rename = "ips", skip_serializing_if = "Option::is_none", default)]
    pub ips: Option<Vec<UpdateSenderRequestIpsItem>>,
    /// From Name to update the sender
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIpsFromSenderResponseIpsItem {
    /// Domain associated to the IP
    #[serde(rename = "domain")]
    pub domain: String,
    /// ID of the dedicated IP
    #[serde(rename = "id")]
    pub id: i64,
    /// Dedicated IP address
    #[serde(rename = "ip")]
    pub ip: String,
    /// Weight of the IP for this sender
    #[serde(rename = "weight")]
    pub weight: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIpsFromSenderResponse {
    /// Dedicated IP(s) linked to a sender. Returns empty array for standard accounts
    /// without dedicated IPs.
    #[serde(rename = "ips")]
    pub ips: Vec<GetIpsFromSenderResponseIpsItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSenderByOTPRequest {
    /// 6 digit OTP received on email
    #[serde(rename = "otp")]
    pub otp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWebhooksType {
    #[serde(rename = "marketing")]
    Marketing,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "inbound")]
    Inbound,
}
impl Default for GetWebhooksType {
    fn default() -> Self {
        Self::Marketing
    }
}
impl core::fmt::Display for GetWebhooksType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Marketing => write!(f, "marketing"),
            Self::Transactional => write!(f, "transactional"),
            Self::Inbound => write!(f, "inbound"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWebhooksSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetWebhooksSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetWebhooksSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhooksResponse {
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<serde_json::Value>,
}

/// Add authentication on webhook url
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequestAuth {
    /// Webhook authentication token
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Type of authentication
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

/// Channel of the webhook
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateWebhookRequestChannel {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
}
impl Default for CreateWebhookRequestChannel {
    fn default() -> Self {
        Self::Email
    }
}
impl core::fmt::Display for CreateWebhookRequestChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateWebhookRequestEventsItem {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "hardBounce")]
    HardBounce,
    #[serde(rename = "softBounce")]
    SoftBounce,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "uniqueOpened")]
    UniqueOpened,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "listAddition")]
    ListAddition,
    #[serde(rename = "contactUpdated")]
    ContactUpdated,
    #[serde(rename = "contactDeleted")]
    ContactDeleted,
    #[serde(rename = "inboundEmailProcessed")]
    InboundEmailProcessed,
    #[serde(rename = "reply")]
    Reply,
}
impl Default for CreateWebhookRequestEventsItem {
    fn default() -> Self {
        Self::Sent
    }
}
impl core::fmt::Display for CreateWebhookRequestEventsItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::SoftBounce => write!(f, "softBounce"),
            Self::Blocked => write!(f, "blocked"),
            Self::Spam => write!(f, "spam"),
            Self::Delivered => write!(f, "delivered"),
            Self::Request => write!(f, "request"),
            Self::Click => write!(f, "click"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Opened => write!(f, "opened"),
            Self::UniqueOpened => write!(f, "uniqueOpened"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::ListAddition => write!(f, "listAddition"),
            Self::ContactUpdated => write!(f, "contactUpdated"),
            Self::ContactDeleted => write!(f, "contactDeleted"),
            Self::InboundEmailProcessed => write!(f, "inboundEmailProcessed"),
            Self::Reply => write!(f, "reply"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequestHeadersItem {
    /// Header key name
    #[serde(rename = "key", skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    /// Header value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

/// Type of the webhook
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateWebhookRequestType {
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "marketing")]
    Marketing,
    #[serde(rename = "inbound")]
    Inbound,
}
impl Default for CreateWebhookRequestType {
    fn default() -> Self {
        Self::Transactional
    }
}
impl core::fmt::Display for CreateWebhookRequestType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transactional => write!(f, "transactional"),
            Self::Marketing => write!(f, "marketing"),
            Self::Inbound => write!(f, "inbound"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequest {
    /// Add authentication on webhook url
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none", default)]
    pub auth: Option<CreateWebhookRequestAuth>,
    /// To send batched webhooks
    #[serde(rename = "batched", skip_serializing_if = "Option::is_none", default)]
    pub batched: Option<bool>,
    /// Channel of the webhook
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none", default)]
    pub channel: Option<CreateWebhookRequestChannel>,
    /// Description of the webhook
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Inbound domain of webhook, required in case of event type
    /// `inbound`
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    /// Events triggering the webhook. Required for transactional and
    /// marketing types, optional for inbound type (defaults to
    /// `inboundEmailProcessed`). Possible values for **Transactional**
    /// type webhook: `sent` OR `request`, `delivered`, `hardBounce`,
    /// `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`,
    /// `opened`, `uniqueOpened` and `unsubscribed`. Possible values for
    /// **Marketing** type webhook: `spam`, `opened`, `click`,
    /// `hardBounce`, `softBounce`, `unsubscribed`, `listAddition`,
    /// `delivered`, `contactUpdated` & `contactDeleted`. Possible values
    /// for **Inbound** type webhook: `inboundEmailProcessed`.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<CreateWebhookRequestEventsItem>>,
    /// Custom headers to be send with webhooks
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<CreateWebhookRequestHeadersItem>>,
    /// Type of the webhook
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<CreateWebhookRequestType>,
    /// URL of the webhook
    #[serde(rename = "url")]
    pub url: String,
}

/// Add authentication on webhook url
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequestAuth {
    /// Webhook authentication token
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Type of authentication
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateWebhookRequestEventsItem {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "hardBounce")]
    HardBounce,
    #[serde(rename = "softBounce")]
    SoftBounce,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "uniqueOpened")]
    UniqueOpened,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "listAddition")]
    ListAddition,
    #[serde(rename = "contactUpdated")]
    ContactUpdated,
    #[serde(rename = "contactDeleted")]
    ContactDeleted,
    #[serde(rename = "inboundEmailProcessed")]
    InboundEmailProcessed,
    #[serde(rename = "reply")]
    Reply,
}
impl Default for UpdateWebhookRequestEventsItem {
    fn default() -> Self {
        Self::Sent
    }
}
impl core::fmt::Display for UpdateWebhookRequestEventsItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::SoftBounce => write!(f, "softBounce"),
            Self::Blocked => write!(f, "blocked"),
            Self::Spam => write!(f, "spam"),
            Self::Delivered => write!(f, "delivered"),
            Self::Request => write!(f, "request"),
            Self::Click => write!(f, "click"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Opened => write!(f, "opened"),
            Self::UniqueOpened => write!(f, "uniqueOpened"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::ListAddition => write!(f, "listAddition"),
            Self::ContactUpdated => write!(f, "contactUpdated"),
            Self::ContactDeleted => write!(f, "contactDeleted"),
            Self::InboundEmailProcessed => write!(f, "inboundEmailProcessed"),
            Self::Reply => write!(f, "reply"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequestHeadersItem {
    /// Header key name
    #[serde(rename = "key", skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    /// Header value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest {
    /// Add authentication on webhook url
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none", default)]
    pub auth: Option<UpdateWebhookRequestAuth>,
    /// To send batched webhooks
    #[serde(rename = "batched", skip_serializing_if = "Option::is_none", default)]
    pub batched: Option<bool>,
    /// Description of the webhook
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Inbound domain of webhook, used in case of event type `inbound`
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    /// - Events triggering the webhook. Possible values for
    /// **Transactional** type webhook: #### `sent` OR `request`,
    /// `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`,
    /// `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and
    /// `unsubscribed` - Possible values for **Marketing** type webhook:
    /// #### `spam`, `opened`, `click`, `hardBounce`, `softBounce`,
    /// `unsubscribed`, `listAddition` & `delivered` - Possible values
    /// for **Inbound** type webhook: #### `inboundEmailProcessed`
    /// #### `reply`
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<UpdateWebhookRequestEventsItem>>,
    /// Custom headers to be send with webhooks
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<UpdateWebhookRequestHeadersItem>>,
    /// URL of the webhook
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAllExternalFeedsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetAllExternalFeedsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetAllExternalFeedsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAllExternalFeedsAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}
impl Default for GetAllExternalFeedsAuthType {
    fn default() -> Self {
        Self::Basic
    }
}
impl core::fmt::Display for GetAllExternalFeedsAuthType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
        }
    }
}

/// Authentication type for accessing the feed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAllExternalFeedsResponseFeedsItemAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}
impl Default for GetAllExternalFeedsResponseFeedsItemAuthType {
    fn default() -> Self {
        Self::Basic
    }
}
impl core::fmt::Display for GetAllExternalFeedsResponseFeedsItemAuthType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllExternalFeedsResponseFeedsItemHeadersItem {
    /// Header name
    #[serde(rename = "name")]
    pub name: String,
    /// Header value
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllExternalFeedsResponseFeedsItem {
    /// Unique identifier of the feed
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the feed
    #[serde(rename = "name")]
    pub name: String,
    /// URL of the external data source
    #[serde(rename = "url")]
    pub url: String,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType")]
    pub auth_type: GetAllExternalFeedsResponseFeedsItemAuthType,
    /// Username for basic authentication. Only returned when authType is 'basic'. Excluded when authType is 'token'.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
    /// Password for basic authentication. Only returned when authType is 'basic'. Excluded when authType is 'token'.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    /// Token for token-based authentication. Only returned when authType is 'token'. Excluded when authType is 'basic' or 'noAuth'.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// Whether to cache the feed response
    #[serde(rename = "cache")]
    pub cache: bool,
    /// Custom HTTP headers for the feed request
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<GetAllExternalFeedsResponseFeedsItemHeadersItem>>,
    /// Feed creation timestamp
    #[serde(rename = "createdAt")]
    pub created_at: time::OffsetDateTime,
    /// Feed last modification timestamp
    #[serde(rename = "modifiedAt")]
    pub modified_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllExternalFeedsResponse {
    /// Total number of feeds
    #[serde(rename = "count")]
    pub count: i64,
    /// List of external feeds
    #[serde(rename = "feeds")]
    pub feeds: Vec<GetAllExternalFeedsResponseFeedsItem>,
}

/// Authentication type for accessing the feed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateExternalFeedRequestAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}
impl Default for CreateExternalFeedRequestAuthType {
    fn default() -> Self {
        Self::Basic
    }
}
impl core::fmt::Display for CreateExternalFeedRequestAuthType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalFeedRequestHeadersItem {
    /// Header name
    #[serde(rename = "name")]
    pub name: String,
    /// Header value
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalFeedRequest {
    /// Name of the feed
    #[serde(rename = "name")]
    pub name: String,
    /// URL of the external data source
    #[serde(rename = "url")]
    pub url: String,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none", default)]
    pub auth_type: Option<CreateExternalFeedRequestAuthType>,
    /// Username for basic authentication (required if authType is 'basic')
    #[serde(rename = "username", skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
    /// Password for basic authentication (required if authType is 'basic')
    #[serde(rename = "password", skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    /// Token for token-based authentication (required if authType is 'token')
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Maximum number of retry attempts for failed requests
    #[serde(
        rename = "maxRetries",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_retries: Option<i64>,
    /// Whether to cache the feed response
    #[serde(rename = "cache", skip_serializing_if = "Option::is_none", default)]
    pub cache: Option<bool>,
    /// Custom HTTP headers for the feed request
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<CreateExternalFeedRequestHeadersItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalFeedResponse {
    /// UUID of the created feed
    #[serde(rename = "id")]
    pub id: String,
}

/// Authentication type for accessing the feed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetExternalFeedByUUIDResponseAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}
impl Default for GetExternalFeedByUUIDResponseAuthType {
    fn default() -> Self {
        Self::Basic
    }
}
impl core::fmt::Display for GetExternalFeedByUUIDResponseAuthType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExternalFeedByUUIDResponseHeadersItem {
    /// Header name
    #[serde(rename = "name")]
    pub name: String,
    /// Header value
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExternalFeedByUUIDResponse {
    /// Unique identifier of the feed
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the feed
    #[serde(rename = "name")]
    pub name: String,
    /// URL of the external data source
    #[serde(rename = "url")]
    pub url: String,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType")]
    pub auth_type: GetExternalFeedByUUIDResponseAuthType,
    /// Username for basic authentication. Only returned when authType is 'basic'. Excluded from response when authType is 'token'.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
    /// Password for basic authentication. Only returned when authType is 'basic'. Excluded from response when authType is 'token'.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    /// Token for token-based authentication. Only returned when authType is 'token'. Excluded from response when authType is 'basic' or 'noAuth'.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Custom HTTP headers for the feed request
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<GetExternalFeedByUUIDResponseHeadersItem>>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    pub max_retries: i64,
    /// Whether to cache the feed response
    #[serde(rename = "cache")]
    pub cache: bool,
    /// Feed creation timestamp
    #[serde(rename = "createdAt")]
    pub created_at: time::OffsetDateTime,
    /// Feed last modification timestamp
    #[serde(rename = "modifiedAt")]
    pub modified_at: time::OffsetDateTime,
}

/// Authentication type for accessing the feed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateExternalFeedRequestAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}
impl Default for UpdateExternalFeedRequestAuthType {
    fn default() -> Self {
        Self::Basic
    }
}
impl core::fmt::Display for UpdateExternalFeedRequestAuthType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExternalFeedRequestHeadersItem {
    /// Header name
    #[serde(rename = "name")]
    pub name: String,
    /// Header value
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExternalFeedRequest {
    /// Name of the feed
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// URL of the external data source
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none", default)]
    pub auth_type: Option<UpdateExternalFeedRequestAuthType>,
    /// Username for basic authentication
    #[serde(rename = "username", skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
    /// Password for basic authentication
    #[serde(rename = "password", skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    /// Token for token-based authentication
    #[serde(rename = "token", skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    /// Custom HTTP headers for the feed request
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<Vec<UpdateExternalFeedRequestHeadersItem>>,
    /// Maximum number of retry attempts for failed requests
    #[serde(
        rename = "maxRetries",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_retries: Option<i64>,
    /// Whether to cache the feed response
    #[serde(rename = "cache", skip_serializing_if = "Option::is_none", default)]
    pub cache: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertrecordsRequest {
    /// List of object records to be upsert. Each record can have attributes, identifiers, and associations.
    #[serde(rename = "records")]
    pub records: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertrecordsResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
    /// Unique Id for the batch process used to track the status of the batch. **How to use this processId:**   Refer to the [Get process status API](<https://developers.brevo.com/reference/get-process>) to check the execution status of this batch using the returned `processId`.
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none", default)]
    pub process_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetrecordsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetrecordsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetrecordsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetrecordsAssociation {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}
impl Default for GetrecordsAssociation {
    fn default() -> Self {
        Self::True
    }
}
impl core::fmt::Display for GetrecordsAssociation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

/// Identifiers for the object record. Supports `id` (singular, not `ids`) or `ext_id`.
/// - `ext_id`: Your external system's identifier for this record.
/// - `id`: Internal Brevo record ID.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetrecordsResponseRecordsItemIdentifiers {
    /// External ID of the record in your system.
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Internal ID of the object record generated by Brevo.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetrecordsResponseRecordsItem {
    /// List of associations for the object record. If association query param is true it will return 5 associated records per association.
    #[serde(
        rename = "associations",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub associations: Option<Vec<serde_json::Value>>,
    /// Key-value pairs of attribute data for the object record. Each key is the attribute **key** (e.g., `company_name`, `engine_type`), not the attribute label (e.g., "Company Name", "Engine Type"). Only attributes that already exist in the object schema will be present.
    /// For `category` or `multiple_category` attributes, the value is the option **key** (not the option label or option ID).
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Timestamp when the object record was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Identifiers for the object record. Supports `id` (singular, not `ids`) or `ext_id`.
    /// - `ext_id`: Your external system's identifier for this record.
    /// - `id`: Internal Brevo record ID.
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<GetrecordsResponseRecordsItemIdentifiers>,
    /// Timestamp when the object record was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetrecordsResponse {
    /// Total number of object records for an object type.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "records", skip_serializing_if = "Option::is_none", default)]
    pub records: Option<Vec<GetrecordsResponseRecordsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteObjectRecordsRequest {
    /// Either `ids` or `ext_ids` must be provided, but not both in the same request.
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteObjectRecordsResponse {
    /// Identifier for batch process tracking
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none", default)]
    pub process_id: Option<i64>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
}

/// The associated record. `ext_id`, `attributes`, `created_at` and `updated_at` are omitted when they are not available for the record.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssociatedRecordsResponseItemsItemObject {
    /// Key-value pairs of attribute data for the associated record. Each key is the attribute **key** (e.g., `company_name`, `engine_type`), not the attribute label (e.g., "Company Name", "Engine Type").
    /// For `category` or `multiple_category` attributes, the value is the option **key** (not the option label or option ID).
    /// For contacts, attribute keys are returned in lowercase, for example `email`, `first_name`, `last_name`, `sms` and `ext_id`.
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Timestamp when the associated record was created. Not returned for contacts.
    #[serde(
        rename = "created_at",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub created_at: Option<time::OffsetDateTime>,
    /// External ID of the associated record in your system. Not returned for contacts, whose external ID is available as `attributes.ext_id`.
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Internal ID of the associated record generated by Brevo.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Timestamp when the associated record was last updated. Not returned for contacts.
    #[serde(
        rename = "updated_at",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssociatedRecordsResponseItemsItem {
    /// The associated record. `ext_id`, `attributes`, `created_at` and `updated_at` are omitted when they are not available for the record.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none", default)]
    pub object: Option<GetAssociatedRecordsResponseItemsItemObject>,
    /// Object type of the associated record.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssociatedRecordsResponse {
    /// `true` when at least one more associated record exists after this page.
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none", default)]
    pub has_more: Option<bool>,
    /// Associated records for this page, up to 20 per page. Fewer when the last page is reached, empty when the source record has no matching associations.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<GetAssociatedRecordsResponseItemsItem>>,
    /// The offset that was requested.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none", default)]
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetContactsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetContactsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetContactsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactRequest {
    /// Pass the set of attributes and their values. The attribute's parameter should be passed in capital letter while creating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored. **These attributes must be present in your Brevo account**. For eg: **{"FNAME":"Elly", "LNAME":"Roger", "COUNTRIES": ["India","China"]}**
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Email address of the user. **Mandatory if "ext_id"  & "SMS" field is not passed.**
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Set this field to blacklist the contact for emails (emailBlacklisted = true)
    #[serde(
        rename = "emailBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to create a contact.
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// Set this field to blacklist the contact for SMS (smsBlacklisted = true)
    #[serde(
        rename = "smsBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_blacklisted: Option<bool>,
    /// transactional email forbidden sender for contact. Use only for email Contact ( only available if updateEnabled = true )
    #[serde(
        rename = "smtpBlacklistSender",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub smtp_blacklist_sender: Option<Vec<crate::types::Email>>,
    /// Facilitate to update the existing contact in the same request (updateEnabled = true)
    #[serde(
        rename = "updateEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_enabled: Option<bool>,
    /// When true, if the contact being created shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    #[serde(
        rename = "forceMerge",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub force_merge: Option<bool>,
    /// When true, the response returns the `id` of the surviving contact after merge.
    #[serde(rename = "getId", skip_serializing_if = "Option::is_none", default)]
    pub get_id: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactResponse {
    /// ID of the contact when a new contact is created
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
}

/// Category of the attribute
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAttributesResponseAttributesItemCategory {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "calculated")]
    Calculated,
    #[serde(rename = "global")]
    Global,
}
impl Default for GetAttributesResponseAttributesItemCategory {
    fn default() -> Self {
        Self::Normal
    }
}
impl core::fmt::Display for GetAttributesResponseAttributesItemCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Transactional => write!(f, "transactional"),
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesResponseAttributesItemEnumerationItem {
    /// Label of the "category" type attribute
    #[serde(rename = "label")]
    pub label: String,
    /// Numeric ID of the "category" type attribute value. Set to 0 when the raw value cannot be converted to an integer (for example non-numeric values such as "en" or "fr"). Refer to `valueStr` for the original string representation.
    #[serde(rename = "value")]
    pub value: i64,
    /// String representation of the "category" type attribute value. Always contains the original value as stored. Use this field when the attribute value is non-numeric (e.g. "en", "fr") or when you need the exact string form alongside the numeric `value`.
    #[serde(rename = "valueStr")]
    pub value_str: String,
}

/// Type of the attribute
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAttributesResponseAttributesItemType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "multiple-choice")]
    MultipleChoice,
    #[serde(rename = "user")]
    User,
}
impl Default for GetAttributesResponseAttributesItemType {
    fn default() -> Self {
        Self::Text
    }
}
impl core::fmt::Display for GetAttributesResponseAttributesItemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Date => write!(f, "date"),
            Self::Float => write!(f, "float"),
            Self::Id => write!(f, "id"),
            Self::Boolean => write!(f, "boolean"),
            Self::MultipleChoice => write!(f, "multiple-choice"),
            Self::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesResponseAttributesItem {
    /// Calculated value formula
    #[serde(
        rename = "calculatedValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub calculated_value: Option<String>,
    /// Category of the attribute
    #[serde(rename = "category")]
    pub category: GetAttributesResponseAttributesItemCategory,
    /// Parameter only available for "category" type attributes.
    #[serde(
        rename = "enumeration",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub enumeration: Option<Vec<GetAttributesResponseAttributesItemEnumerationItem>>,
    /// Parameter only available for "multiple-choice" type attributes.
    #[serde(
        rename = "multiCategoryOptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub multi_category_options: Option<Vec<String>>,
    /// Name of the attribute
    #[serde(rename = "name")]
    pub name: String,
    /// Type of the attribute
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<GetAttributesResponseAttributesItemType>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesResponse {
    /// Listing of available contact attributes in your account
    #[serde(rename = "attributes")]
    pub attributes: Vec<GetAttributesResponseAttributesItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeleteAttributeAttributeCategory {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "calculated")]
    Calculated,
    #[serde(rename = "global")]
    Global,
}
impl Default for DeleteAttributeAttributeCategory {
    fn default() -> Self {
        Self::Normal
    }
}
impl core::fmt::Display for DeleteAttributeAttributeCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Transactional => write!(f, "transactional"),
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateAttributeAttributeCategory {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "calculated")]
    Calculated,
    #[serde(rename = "global")]
    Global,
}
impl Default for CreateAttributeAttributeCategory {
    fn default() -> Self {
        Self::Normal
    }
}
impl core::fmt::Display for CreateAttributeAttributeCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Transactional => write!(f, "transactional"),
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttributeRequestEnumerationItem {
    /// Label of the value
    #[serde(rename = "label")]
    pub label: String,
    /// Id of the value
    #[serde(rename = "value")]
    pub value: i64,
}

/// Type of the attribute. **Use only if the attribute's category is 'normal', 'category' or 'transactional'** Type **user and multiple-choice** is only available if the category is **normal** attribute Type **id** is only available if the category is **transactional** attribute Type **category** is only available if the category is **category** attribute
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateAttributeRequestType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "multiple-choice")]
    MultipleChoice,
    #[serde(rename = "user")]
    User,
}
impl Default for CreateAttributeRequestType {
    fn default() -> Self {
        Self::Text
    }
}
impl core::fmt::Display for CreateAttributeRequestType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Date => write!(f, "date"),
            Self::Float => write!(f, "float"),
            Self::Boolean => write!(f, "boolean"),
            Self::Id => write!(f, "id"),
            Self::Category => write!(f, "category"),
            Self::MultipleChoice => write!(f, "multiple-choice"),
            Self::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttributeRequest {
    /// List of values and labels that the attribute can take. **Use only if the attribute's category is "category"**. None of the category options can exceed max 200 characters. For example: **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    #[serde(
        rename = "enumeration",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub enumeration: Option<Vec<CreateAttributeRequestEnumerationItem>>,
    /// Type of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(
        rename = "isRecurring",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_recurring: Option<bool>,
    /// List of options you want to add for multiple-choice attribute. **Use only if the attribute's category is "normal" and attribute's type is "multiple-choice". None of the multicategory options can exceed max 200 characters.** For example: **["USA","INDIA"]**
    #[serde(
        rename = "multiCategoryOptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub multi_category_options: Option<Vec<String>>,
    /// Type of the attribute. **Use only if the attribute's category is 'normal', 'category' or 'transactional'** Type **user and multiple-choice** is only available if the category is **normal** attribute Type **id** is only available if the category is **transactional** attribute Type **category** is only available if the category is **category** attribute
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<CreateAttributeRequestType>,
    /// Value of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateAttributeAttributeCategory {
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "calculated")]
    Calculated,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "normal")]
    Normal,
}
impl Default for UpdateAttributeAttributeCategory {
    fn default() -> Self {
        Self::Category
    }
}
impl core::fmt::Display for UpdateAttributeAttributeCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
            Self::Normal => write!(f, "normal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAttributeRequestEnumerationItem {
    /// Label of the value
    #[serde(rename = "label")]
    pub label: String,
    /// Id of the value
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAttributeRequest {
    /// List of the values and labels that the attribute can take. **Use only if the attribute's category is "category"** None of the category options can exceed max 200 characters. For example, **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    #[serde(
        rename = "enumeration",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub enumeration: Option<Vec<UpdateAttributeRequestEnumerationItem>>,
    /// Use this option to add multiple-choice attributes options only if the attribute's category is "normal". **This option is specifically designed for updating multiple-choice attributes. None of the multicategory options can exceed max 200 characters.**. For example: **["USA","INDIA"]**
    #[serde(
        rename = "multiCategoryOptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub multi_category_options: Option<Vec<String>>,
    /// Value of the attribute to update. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeleteMultiAttributeOptionsAttributeType {
    #[serde(rename = "multiple-choice")]
    MultipleChoice,
}
impl Default for DeleteMultiAttributeOptionsAttributeType {
    fn default() -> Self {
        Self::MultipleChoice
    }
}
impl core::fmt::Display for DeleteMultiAttributeOptionsAttributeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MultipleChoice => write!(f, "multiple-choice"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatchContactsRequestContactsItem {
    /// Pass the set of attributes to be updated. **These attributes must be present in your account**. To update existing email address of a contact with the new one please pass EMAIL in attribtes. For example, **{ "EMAIL":"newemail@domain.com", "FNAME":"Ellie", "LNAME":"Roger"}**. Keep in mind transactional attributes can be updated the same way as normal attributes. Mobile Number in **SMS** field should be passed with proper country code. For example: **{"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"}**
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Email address of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Set/unset this field to blacklist/allow the contact for emails (emailBlacklisted = true)
    #[serde(
        rename = "emailBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to update ext_id of a contact.
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// id of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// SMS of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none", default)]
    pub sms: Option<String>,
    /// Set/unset this field to blacklist/allow the contact for SMS (smsBlacklisted = true)
    #[serde(
        rename = "smsBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_blacklisted: Option<bool>,
    /// transactional email forbidden sender for contact. Use only for email Contact
    #[serde(
        rename = "smtpBlacklistSender",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub smtp_blacklist_sender: Option<Vec<crate::types::Email>>,
    /// Ids of the lists to remove the contact from
    #[serde(
        rename = "unlinkListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_list_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBatchContactsRequest {
    /// List of contacts to be updated
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none", default)]
    pub contacts: Option<Vec<UpdateBatchContactsRequestContactsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDoiContactRequest {
    /// Pass the set of attributes and their values. **These attributes must be present in your Brevo account**. For eg. **{'FNAME':'Elly', 'LNAME':'Roger', 'COUNTRIES': ['India','China']}**
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Email address where the confirmation email will be sent. This email address will be the identifier for all other contact attributes.
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Lists under user account where contact should not be added
    #[serde(
        rename = "excludeListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclude_list_ids: Option<Vec<i64>>,
    /// Lists under user account where contact should be added
    #[serde(rename = "includeListIds")]
    pub include_list_ids: Vec<i64>,
    /// URL of the web page that user will be redirected to after clicking on the double opt in URL. When editing your DOI template you can reference this URL by using the tag **{{ params.DOIurl }}**.
    #[serde(rename = "redirectionUrl")]
    pub redirection_url: String,
    /// Id of the Double opt-in (DOI) template
    #[serde(rename = "templateId")]
    pub template_id: i64,
}

/// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on contacts as per the list id. * **allContacts** - Fetch the list of all contacts for a particular list. * **subscribed & unsubscribed** - Fetch the list of subscribed / unsubscribed (blacklisted via any means) contacts for a particular list. * **unsubscribedPerList** - Fetch the list of contacts that are unsubscribed from a particular list only.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RequestContactExportRequestCustomContactFilterActionForContacts {
    #[serde(rename = "allContacts")]
    AllContacts,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "unsubscribedPerList")]
    UnsubscribedPerList,
}
impl Default for RequestContactExportRequestCustomContactFilterActionForContacts {
    fn default() -> Self {
        Self::AllContacts
    }
}
impl core::fmt::Display for RequestContactExportRequestCustomContactFilterActionForContacts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AllContacts => write!(f, "allContacts"),
            Self::Subscribed => write!(f, "subscribed"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::UnsubscribedPerList => write!(f, "unsubscribedPerList"),
        }
    }
}

/// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    #[serde(rename = "openers")]
    Openers,
    #[serde(rename = "nonOpeners")]
    NonOpeners,
    #[serde(rename = "clickers")]
    Clickers,
    #[serde(rename = "nonClickers")]
    NonClickers,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
}
impl Default for RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    fn default() -> Self {
        Self::Openers
    }
}
impl core::fmt::Display for RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Openers => write!(f, "openers"),
            Self::NonOpeners => write!(f, "nonOpeners"),
            Self::Clickers => write!(f, "clickers"),
            Self::NonClickers => write!(f, "nonClickers"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
        }
    }
}

/// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl Default for RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    fn default() -> Self {
        Self::HardBounces
    }
}
impl core::fmt::Display for RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

/// Set the filter for the contacts to be exported.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContactExportRequestCustomContactFilter {
    /// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on contacts as per the list id. * **allContacts** - Fetch the list of all contacts for a particular list. * **subscribed & unsubscribed** - Fetch the list of subscribed / unsubscribed (blacklisted via any means) contacts for a particular list. * **unsubscribedPerList** - Fetch the list of contacts that are unsubscribed from a particular list only.
    #[serde(
        rename = "actionForContacts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub action_for_contacts:
        Option<RequestContactExportRequestCustomContactFilterActionForContacts>,
    /// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
    #[serde(
        rename = "actionForEmailCampaigns",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub action_for_email_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForEmailCampaigns>,
    /// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
    #[serde(
        rename = "actionForSmsCampaigns",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub action_for_sms_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForSmsCampaigns>,
    /// Considered only if **actionForEmailCampaigns** is passed, ignored otherwise. **Mandatory if action is one of the following - openers, nonOpeners, clickers, nonClickers, unsubscribed.** The id of the email campaign for which the corresponding action shall be applied in the filter.
    #[serde(
        rename = "emailCampaignId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_campaign_id: Option<i64>,
    /// ID of the list. This is mandatory if actionForContacts is specified and segmentId is not provided. Either segmentId or listId must be included.
    #[serde(rename = "listId", skip_serializing_if = "Option::is_none", default)]
    pub list_id: Option<i64>,
    /// ID of the segment. This is mandatory if actionForContacts is specified and listId is not provided. Either segmentId or listId must be included.
    #[serde(rename = "segmentId", skip_serializing_if = "Option::is_none", default)]
    pub segment_id: Option<i64>,
    /// Considered only if **actionForSmsCampaigns** is passed, ignored otherwise. The id of sms campaign for which the corresponding action shall be applied in the filter.
    #[serde(
        rename = "smsCampaignId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_campaign_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContactExportRequest {
    /// Set the filter for the contacts to be exported.
    #[serde(rename = "customContactFilter")]
    pub custom_contact_filter: RequestContactExportRequestCustomContactFilter,
    /// To avoid generating the email notification upon contact export, pass **true**
    #[serde(
        rename = "disableNotification",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub disable_notification: Option<bool>,
    /// List of all the attributes that you want to export. **These attributes must be present in your contact database. It is required if exportMandatoryAttributes is set false. ** For example: **['fname', 'lname', 'email']**
    #[serde(
        rename = "exportAttributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_attributes: Option<Vec<String>>,
    /// To export mandatory attributes like EMAIL, ADDED_TIME, MODIFIED_TIME
    #[serde(
        rename = "exportMandatoryAttributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_mandatory_attributes: Option<bool>,
    /// Export metadata of contacts such as _listIds, ADDED_TIME, MODIFIED_TIME.
    #[serde(
        rename = "exportMetadata",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_metadata: Option<Vec<String>>,
    /// Specifies whether the date fields createdAt, modifiedAt in the exported data should be returned in UTC format.
    #[serde(
        rename = "exportDateInUTC",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_date_in_utc: Option<bool>,
    /// Export subscription status of contacts for email & sms marketting. Pass email_marketing to obtain the marketing email subscription status & sms_marketing to retrieve the marketing SMS status of the contact.
    #[serde(
        rename = "exportSubscriptionStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub export_subscription_status: Option<Vec<String>>,
    /// Webhook that will be called once the export process is finished. For reference, <https://help.brevo.com/hc/en-us/articles/360007666479>
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none", default)]
    pub notify_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetFoldersSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetFoldersSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetFoldersSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFoldersResponse {
    /// Number of folders available in your account
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "folders", skip_serializing_if = "Option::is_none", default)]
    pub folders: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetFolderListsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetFolderListsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetFolderListsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFolderListsResponse {
    /// Number of lists in the folder
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none", default)]
    pub lists: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportContactsRequestJsonBodyItem {
    /// List of attributes to be imported
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
}

/// To create a new list and import the contacts into it, pass the listName and an optional folderId.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportContactsRequestNewList {
    /// Id of the folder where this new list shall be created. **Mandatory if listName is not empty**
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none", default)]
    pub folder_id: Option<i64>,
    /// List with listName will be created first and users will be imported in it. **Mandatory if listIds is empty**.
    #[serde(rename = "listName", skip_serializing_if = "Option::is_none", default)]
    pub list_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportContactsRequest {
    /// To disable email notification
    #[serde(
        rename = "disableNotification",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub disable_notification: Option<bool>,
    /// To blacklist all the contacts for email
    #[serde(
        rename = "emailBlacklist",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_blacklist: Option<bool>,
    /// To facilitate the choice to erase any attribute of the existing contacts with empty value. emptyContactsAttributes = true means the empty fields in your import will erase any attribute that currently contain data in Brevo, & emptyContactsAttributes = false means the empty fields will not affect your existing data ( **only available if `updateExistingContacts` set to true **)
    #[serde(
        rename = "emptyContactsAttributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub empty_contacts_attributes: Option<bool>,
    /// **Mandatory if fileUrl and jsonBody is not defined.** CSV content to be imported. Use semicolon to separate multiple attributes. **Maximum allowed file body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of file body size while parsing. Please use fileUrl instead to import bigger files.
    #[serde(rename = "fileBody", skip_serializing_if = "Option::is_none", default)]
    pub file_body: Option<String>,
    /// **Mandatory if fileBody and jsonBody is not defined.** URL of the file to be imported (**no local file**). Possible file formats: #### .txt, .csv, .json
    #[serde(rename = "fileUrl", skip_serializing_if = "Option::is_none", default)]
    pub file_url: Option<String>,
    /// **Mandatory if fileUrl and fileBody is not defined.** JSON content to be imported. **Maximum allowed json body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of json body size while parsing. Please use fileUrl instead to import bigger files.
    #[serde(rename = "jsonBody", skip_serializing_if = "Option::is_none", default)]
    pub json_body: Option<Vec<ImportContactsRequestJsonBodyItem>>,
    /// **Mandatory if newList is not defined.** Ids of the lists in which the contacts shall be imported. For example, **[2, 4, 7]**.
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// **Optional.** Ids of the consent groups to which all imported contacts will be added. Requires consent groups to be enabled for the organisation. For example, **[1, 3]**.
    #[serde(
        rename = "consentGroupIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub consent_group_ids: Option<Vec<i64>>,
    /// To create a new list and import the contacts into it, pass the listName and an optional folderId.
    #[serde(rename = "newList", skip_serializing_if = "Option::is_none", default)]
    pub new_list: Option<ImportContactsRequestNewList>,
    /// URL that will be called once the import process is finished. For reference, <https://help.brevo.com/hc/en-us/articles/360007666479>
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none", default)]
    pub notify_url: Option<String>,
    /// To blacklist all the contacts for sms
    #[serde(
        rename = "smsBlacklist",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_blacklist: Option<bool>,
    /// To facilitate the choice to update the existing contacts
    #[serde(
        rename = "updateExistingContacts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_existing_contacts: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetListsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetListsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetListsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetListsResponseListsItem {
    /// ID of the folder
    #[serde(rename = "folderId")]
    pub folder_id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetListsResponse {
    /// Number of lists in your account
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    /// Listing of all the lists available in your account
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none", default)]
    pub lists: Option<Vec<GetListsResponseListsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListRequest {
    /// Id of the parent folder in which this list is to be created
    #[serde(rename = "folderId")]
    pub folder_id: i64,
    /// Name of the list
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetListResponseCampaignStatsItem {
    /// ID of the campaign
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    #[serde(rename = "stats")]
    pub stats: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetListResponse {
    #[serde(
        rename = "campaignStats",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub campaign_stats: Option<Vec<GetListResponseCampaignStatsItem>>,
    /// Creation UTC date-time of the list (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Status telling if the list is dynamic or not (true=dynamic, false=not dynamic)
    #[serde(
        rename = "dynamicList",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub dynamic_list: Option<bool>,
    /// ID of the folder
    #[serde(rename = "folderId")]
    pub folder_id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateListRequest {
    /// Id of the folder in which the list is to be moved. Either of the two parameters (name, folderId) can be updated at a time.
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none", default)]
    pub folder_id: Option<i64>,
    /// Name of the list. Either of the two parameters (name, folderId) can be updated at a time.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetContactsFromListSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetContactsFromListSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetContactsFromListSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSegmentsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSegmentsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSegmentsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSegmentsResponseSegmentsItem {
    /// Name of the Segment Category
    #[serde(rename = "categoryName")]
    pub category_name: String,
    /// ID of the segment
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the Segment
    #[serde(rename = "segmentName")]
    pub segment_name: String,
    /// Last update UTC date-time of the segment (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSegmentsResponse {
    /// Number of Segments in your account
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    /// Listing of all the segments available in your account
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none", default)]
    pub segments: Option<Vec<GetSegmentsResponseSegmentsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetConsentGroupsSignupMode {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
}
impl Default for GetConsentGroupsSignupMode {
    fn default() -> Self {
        Self::Manual
    }
}
impl core::fmt::Display for GetConsentGroupsSignupMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Automatic => write!(f, "automatic"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeleteContactIdentifierType {
    #[serde(rename = "email_id")]
    EmailId,
    #[serde(rename = "contact_id")]
    ContactId,
    #[serde(rename = "ext_id")]
    ExtId,
    #[serde(rename = "phone_id")]
    PhoneId,
    #[serde(rename = "whatsapp_id")]
    WhatsappId,
    #[serde(rename = "landline_number_id")]
    LandlineNumberId,
}
impl Default for DeleteContactIdentifierType {
    fn default() -> Self {
        Self::EmailId
    }
}
impl core::fmt::Display for DeleteContactIdentifierType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_id"),
            Self::ContactId => write!(f, "contact_id"),
            Self::ExtId => write!(f, "ext_id"),
            Self::PhoneId => write!(f, "phone_id"),
            Self::WhatsappId => write!(f, "whatsapp_id"),
            Self::LandlineNumberId => write!(f, "landline_number_id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetContactInfoIdentifierType {
    #[serde(rename = "email_id")]
    EmailId,
    #[serde(rename = "phone_id")]
    PhoneId,
    #[serde(rename = "contact_id")]
    ContactId,
    #[serde(rename = "ext_id")]
    ExtId,
    #[serde(rename = "whatsapp_id")]
    WhatsappId,
    #[serde(rename = "landline_number_id")]
    LandlineNumberId,
}
impl Default for GetContactInfoIdentifierType {
    fn default() -> Self {
        Self::EmailId
    }
}
impl core::fmt::Display for GetContactInfoIdentifierType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_id"),
            Self::PhoneId => write!(f, "phone_id"),
            Self::ContactId => write!(f, "contact_id"),
            Self::ExtId => write!(f, "ext_id"),
            Self::WhatsappId => write!(f, "whatsapp_id"),
            Self::LandlineNumberId => write!(f, "landline_number_id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsClickedItemLinksItem {
    /// Number of clicks on this link for the campaign
    #[serde(rename = "count")]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has clicked on the link
    #[serde(rename = "ip")]
    pub ip: String,
    /// URL of the clicked link
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsClickedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// Listing of the clicked links for the campaign
    #[serde(rename = "links")]
    pub links: Vec<GetContactInfoResponseStatisticsClickedItemLinksItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsComplaintsItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsDeliveredItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsHardBouncesItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsMessagesSentItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsOpenedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// Number of openings for the campaign
    #[serde(rename = "count")]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has opened the email
    #[serde(rename = "ip")]
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsSoftBouncesItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem {
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has been unsubscribed
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has unsubscribed
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
}

/// Listing of the unsubscription for the contact
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatisticsUnsubscriptions {
    /// Contact has been unsubscribed from the administrator
    #[serde(rename = "adminUnsubscription")]
    pub admin_unsubscription:
        Vec<GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem>,
    /// Contact unsubscribe via unsubscription link in a campaign
    #[serde(rename = "userUnsubscription")]
    pub user_unsubscription:
        Vec<GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem>,
}

/// Campaign statistics of the contact
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponseStatistics {
    /// Listing of the clicks generated by the contact
    #[serde(rename = "clicked", skip_serializing_if = "Option::is_none", default)]
    pub clicked: Option<Vec<GetContactInfoResponseStatisticsClickedItem>>,
    /// Listing of the complaints generated by the contact
    #[serde(
        rename = "complaints",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub complaints: Option<Vec<GetContactInfoResponseStatisticsComplaintsItem>>,
    /// Listing of the delivered campaign for the contact
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none", default)]
    pub delivered: Option<Vec<GetContactInfoResponseStatisticsDeliveredItem>>,
    /// Listing of the hard bounces generated by the contact
    #[serde(
        rename = "hardBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hard_bounces: Option<Vec<GetContactInfoResponseStatisticsHardBouncesItem>>,
    /// Listing of the sent campaign for the contact
    #[serde(
        rename = "messagesSent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub messages_sent: Option<Vec<GetContactInfoResponseStatisticsMessagesSentItem>>,
    /// Listing of the openings generated by the contact
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none", default)]
    pub opened: Option<Vec<GetContactInfoResponseStatisticsOpenedItem>>,
    /// Listing of the soft bounces generated by the contact
    #[serde(
        rename = "softBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub soft_bounces: Option<Vec<GetContactInfoResponseStatisticsSoftBouncesItem>>,
    /// Listing of the transactional attributes for the contact
    #[serde(
        rename = "transacAttributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transac_attributes: Option<Vec<serde_json::Value>>,
    /// Listing of the unsubscription for the contact
    #[serde(
        rename = "unsubscriptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscriptions: Option<GetContactInfoResponseStatisticsUnsubscriptions>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactInfoResponse {
    /// Campaign statistics of the contact
    #[serde(rename = "statistics")]
    pub statistics: GetContactInfoResponseStatistics,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateContactIdentifierType {
    #[serde(rename = "email_id")]
    EmailId,
    #[serde(rename = "contact_id")]
    ContactId,
    #[serde(rename = "ext_id")]
    ExtId,
    #[serde(rename = "phone_id")]
    PhoneId,
    #[serde(rename = "whatsapp_id")]
    WhatsappId,
    #[serde(rename = "landline_number_id")]
    LandlineNumberId,
}
impl Default for UpdateContactIdentifierType {
    fn default() -> Self {
        Self::EmailId
    }
}
impl core::fmt::Display for UpdateContactIdentifierType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_id"),
            Self::ContactId => write!(f, "contact_id"),
            Self::ExtId => write!(f, "ext_id"),
            Self::PhoneId => write!(f, "phone_id"),
            Self::WhatsappId => write!(f, "whatsapp_id"),
            Self::LandlineNumberId => write!(f, "landline_number_id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContactRequest {
    /// Pass the set of attributes to be updated. **These attributes must be present in your account**. To update existing email address of a contact with the new one please pass EMAIL in attributes. For example, **{ "EMAIL":"newemail@domain.com", "FNAME":"Ellie", "LNAME":"Roger", "COUNTRIES":["India","China"]}**. The attribute's parameter should be passed in capital letter while updating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored .Keep in mind transactional attributes can be updated the same way as normal attributes. Mobile Number in **SMS** field should be passed with proper country code. For example: **{"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"}**
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Set/unset this field to blacklist/allow the contact for emails (emailBlacklisted = true)
    #[serde(
        rename = "emailBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to update ext_id of a contact.
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// Set/unset this field to blacklist/allow the contact for SMS (smsBlacklisted = true)
    #[serde(
        rename = "smsBlacklisted",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sms_blacklisted: Option<bool>,
    /// transactional email forbidden sender for contact. Use only for email Contact
    #[serde(
        rename = "smtpBlacklistSender",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub smtp_blacklist_sender: Option<Vec<crate::types::Email>>,
    /// Ids of the lists to remove the contact from
    #[serde(
        rename = "unlinkListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_list_ids: Option<Vec<i64>>,
    /// When true, if the contact being updated shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    #[serde(
        rename = "forceMerge",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub force_merge: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseClickedItemLinksItem {
    /// Number of clicks on this link for the campaign
    #[serde(rename = "count")]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has clicked on the link
    #[serde(rename = "ip")]
    pub ip: String,
    /// URL of the clicked link
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseClickedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    #[serde(rename = "links")]
    pub links: Vec<GetContactStatsResponseClickedItemLinksItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseComplaintsItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseDeliveredItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseHardBouncesItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseMessagesSentItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseOpenedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// Number of openings of the campaign
    #[serde(rename = "count")]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has opened the campaign
    #[serde(rename = "ip")]
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseSoftBouncesItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseTransacAttributesItem {
    /// Date of the order
    #[serde(rename = "orderDate")]
    pub order_date: time::Date,
    /// ID of the order
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Price of the order
    #[serde(rename = "orderPrice")]
    pub order_price: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseUnsubscriptionsAdminUnsubscriptionItem {
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has been unsubscribed
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has unsubscribed
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponseUnsubscriptions {
    /// Contact has been unsubscribed from the administrator
    #[serde(rename = "adminUnsubscription")]
    pub admin_unsubscription: Vec<GetContactStatsResponseUnsubscriptionsAdminUnsubscriptionItem>,
    /// Contact has unsubscribed via the unsubscription link in the email
    #[serde(rename = "userUnsubscription")]
    pub user_unsubscription: Vec<GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem>,
}

/// Campaign Statistics for the contact
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactStatsResponse {
    #[serde(rename = "clicked", skip_serializing_if = "Option::is_none", default)]
    pub clicked: Option<Vec<GetContactStatsResponseClickedItem>>,
    #[serde(
        rename = "complaints",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub complaints: Option<Vec<GetContactStatsResponseComplaintsItem>>,
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none", default)]
    pub delivered: Option<Vec<GetContactStatsResponseDeliveredItem>>,
    #[serde(
        rename = "hardBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hard_bounces: Option<Vec<GetContactStatsResponseHardBouncesItem>>,
    #[serde(
        rename = "messagesSent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub messages_sent: Option<Vec<GetContactStatsResponseMessagesSentItem>>,
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none", default)]
    pub opened: Option<Vec<GetContactStatsResponseOpenedItem>>,
    #[serde(
        rename = "softBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub soft_bounces: Option<Vec<GetContactStatsResponseSoftBouncesItem>>,
    #[serde(
        rename = "transacAttributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transac_attributes: Option<Vec<GetContactStatsResponseTransacAttributesItem>>,
    #[serde(
        rename = "unsubscriptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscriptions: Option<GetContactStatsResponseUnsubscriptions>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostConversationsAgentOnlinePingRequest {
    /// Agent's email address. When sending online pings from a standalone system, it's hard to maintain a 1-to-1 relationship between the users of both systems. In this case, an agent can be specified by their email address. If there's no agent with the specified email address in your Brevo organization, a dummy agent will be created automatically.
    #[serde(
        rename = "agentEmail",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub agent_email: Option<crate::types::Email>,
    /// Agent ID. It can be found on the agent's page or received <a href="<https://developers.brevo.com/docs/conversations-webhooks">from> a webhook</a>. Alternatively, you can use `agentEmail` + `agentName` + `receivedFrom` instead (all 3 fields required).
    #[serde(rename = "agentId", skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<String>,
    /// Agent's name.
    #[serde(rename = "agentName", skip_serializing_if = "Option::is_none", default)]
    pub agent_name: Option<String>,
    /// Mark your messages to distinguish messages created by you from the others.
    #[serde(
        rename = "receivedFrom",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub received_from: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostConversationsMessagesRequest {
    /// Agent's email address. When sending messages from a standalone system, it's hard to maintain a 1-to-1 relationship between the users of both systems. In this case, an agent can be specified by their email address.
    #[serde(
        rename = "agentEmail",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub agent_email: Option<crate::types::Email>,
    /// Agent ID. It can be found on the agent's page or received <a href="<https://developers.brevo.com/docs/conversations-webhooks">from> a webhook</a>. Alternatively, you can use `agentEmail` + `agentName` + `receivedFrom` instead (all 3 fields required).
    #[serde(rename = "agentId", skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<String>,
    /// Agent's name.
    #[serde(rename = "agentName", skip_serializing_if = "Option::is_none", default)]
    pub agent_name: Option<String>,
    /// Mark your messages to distinguish messages created by you from the others.
    #[serde(
        rename = "receivedFrom",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub received_from: Option<String>,
    /// Message text.
    #[serde(rename = "text")]
    pub text: String,
    /// Visitor's ID received <a href="<https://developers.brevo.com/docs/conversations-webhooks">from> a webhook</a> or generated by you to <a href="<https://developers.brevo.com/docs/customize-the-widget#identifying-existing-users">bind> an existing user account to Conversations</a>.
    #[serde(rename = "visitorId")]
    pub visitor_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutConversationsMessagesIdRequest {
    /// The new message text.
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostConversationsPushedMessagesRequest {
    /// Agent ID. It can be found on the agent's page or received <a href="<https://developers.brevo.com/docs/conversations-webhooks">from> a webhook</a>.
    #[serde(rename = "agentId", skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<String>,
    /// Group ID. It can be found on the group's page.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none", default)]
    pub group_id: Option<String>,
    /// Message text.
    #[serde(rename = "text")]
    pub text: String,
    /// Visitor's ID received <a href="<https://developers.brevo.com/docs/conversations-webhooks">from> a webhook</a> or generated by you to <a href="<https://developers.brevo.com/docs/customize-the-widget#identifying-existing-users">bind> an existing user account to Conversations</a>.
    #[serde(rename = "visitorId")]
    pub visitor_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutConversationsPushedMessagesIdRequest {
    /// The new message text.
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutConversationsVisitorGroupRequest {
    /// Group ID. It can be found on group’s page.
    #[serde(rename = "groupId")]
    pub group_id: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutConversationsVisitorGroupResponse {
    /// group ID the visitor was assigned to
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none", default)]
    pub group_id: Option<serde_json::Value>,
    /// visitor ID
    #[serde(rename = "visitorId", skip_serializing_if = "Option::is_none", default)]
    pub visitor_id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCategoriesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCategoriesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCategoriesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<serde_json::Value>,
    /// Number of categories
    #[serde(rename = "count")]
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateCategoryRequest {
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the category deleted from the shop's database
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none", default)]
    pub deleted_at: Option<String>,
    /// Unique Category ID as saved in the shop
    #[serde(rename = "id")]
    pub id: String,
    /// category deleted from the shop's database
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none", default)]
    pub is_deleted: Option<bool>,
    /// **Mandatory in case of creation**. Name of the Category, as displayed in the shop
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Facilitate to update the existing category in the same request (updateEnabled = true)
    #[serde(
        rename = "updateEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_enabled: Option<bool>,
    /// URL to the category
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateCategoryResponse {
    /// ID of the category when a new category is created
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchCategoryRequestCategoriesItem {
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the category deleted from the shop's database
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none", default)]
    pub deleted_at: Option<String>,
    /// Unique Category ID as saved in the shop
    #[serde(rename = "id")]
    pub id: String,
    /// category deleted from the shop's database
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none", default)]
    pub is_deleted: Option<bool>,
    /// **Mandatory in case of creation**. Name of the Category, as displayed in the shop
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// URL to the category
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchCategoryRequest {
    /// array of categories objects
    #[serde(rename = "categories")]
    pub categories: Vec<CreateUpdateBatchCategoryRequestCategoriesItem>,
    /// Facilitate to update the existing categories in the same request (updateEnabled = true)
    #[serde(
        rename = "updateEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchCategoryResponse {
    /// Number of the new created categories
    #[serde(
        rename = "createdCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub created_count: Option<i64>,
    /// Number of the existing categories updated
    #[serde(
        rename = "updatedCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub updated_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCouponCollectionsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCouponCollectionsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCouponCollectionsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCouponCollectionsSortBy {
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "remainingCoupons")]
    RemainingCoupons,
    #[serde(rename = "expirationDate")]
    ExpirationDate,
}
impl Default for GetCouponCollectionsSortBy {
    fn default() -> Self {
        Self::CreatedAt
    }
}
impl core::fmt::Display for GetCouponCollectionsSortBy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "createdAt"),
            Self::RemainingCoupons => write!(f, "remainingCoupons"),
            Self::ExpirationDate => write!(f, "expirationDate"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCouponCollectionRequest {
    /// Default coupons collection name
    #[serde(rename = "defaultCoupon")]
    pub default_coupon: String,
    /// Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// Name of the coupons collection
    #[serde(rename = "name")]
    pub name: String,
    /// Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    #[serde(
        rename = "remainingCouponsAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_coupons_alert: Option<i64>,
    /// Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    #[serde(
        rename = "remainingDaysAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_days_alert: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCouponCollectionResponse {
    /// The id of the created collection
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCouponCollectionRequest {
    /// A default coupon to be used in case there are no coupons left
    #[serde(
        rename = "defaultCoupon",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub default_coupon: Option<String>,
    /// Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    #[serde(
        rename = "remainingCouponsAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_coupons_alert: Option<i64>,
    /// Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    #[serde(
        rename = "remainingDaysAlert",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_days_alert: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCouponCollectionResponse {
    /// The default coupon of the collection
    #[serde(rename = "defaultCoupon")]
    pub default_coupon: String,
    /// The id of the collection
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the collection
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCouponsRequest {
    /// The id of the coupon collection for which the coupons will be created
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "coupons")]
    pub coupons: Vec<String>,
}

/// Attribution list aggregated totals
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceAttributionMetricsResponseTotals {
    #[serde(rename = "averageBasket")]
    pub average_basket: f32,
    #[serde(rename = "ordersCount")]
    pub orders_count: f64,
    #[serde(rename = "revenue")]
    pub revenue: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceAttributionMetricsResponse {
    /// List of conversion attribution metrics
    #[serde(rename = "results")]
    pub results: Vec<serde_json::Value>,
    /// Attribution list aggregated totals
    #[serde(rename = "totals")]
    pub totals: GetEcommerceAttributionMetricsResponseTotals,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEcommerceAttributionMetricsConversionSourceConversionSourceIdConversionSource {
    #[serde(rename = "email_campaign")]
    EmailCampaign,
    #[serde(rename = "sms_campaign")]
    SmsCampaign,
    #[serde(rename = "automation_workflow_email")]
    AutomationWorkflowEmail,
    #[serde(rename = "automation_workflow_sms")]
    AutomationWorkflowSms,
}
impl Default for GetEcommerceAttributionMetricsConversionSourceConversionSourceIdConversionSource {
    fn default() -> Self {
        Self::EmailCampaign
    }
}
impl core::fmt::Display
    for GetEcommerceAttributionMetricsConversionSourceConversionSourceIdConversionSource
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::AutomationWorkflowEmail => write!(f, "automation_workflow_email"),
            Self::AutomationWorkflowSms => write!(f, "automation_workflow_sms"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse {
    #[serde(rename = "newCustomersCount")]
    pub new_customers_count: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEcommerceAttributionProductsConversionSourceConversionSourceIdConversionSource {
    #[serde(rename = "email_campaign")]
    EmailCampaign,
    #[serde(rename = "sms_campaign")]
    SmsCampaign,
    #[serde(rename = "automation_workflow_email")]
    AutomationWorkflowEmail,
    #[serde(rename = "automation_workflow_sms")]
    AutomationWorkflowSms,
}
impl Default for GetEcommerceAttributionProductsConversionSourceConversionSourceIdConversionSource {
    fn default() -> Self {
        Self::EmailCampaign
    }
}
impl core::fmt::Display
    for GetEcommerceAttributionProductsConversionSourceConversionSourceIdConversionSource
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::AutomationWorkflowEmail => write!(f, "automation_workflow_email"),
            Self::AutomationWorkflowSms => write!(f, "automation_workflow_sms"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[serde(
        rename = "ordersCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub orders_count: Option<i64>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f32>,
    #[serde(rename = "revenue", skip_serializing_if = "Option::is_none", default)]
    pub revenue: Option<f32>,
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none", default)]
    pub sku: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse {
    /// List of attributed products
    #[serde(rename = "products")]
    pub products:
        Vec<GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcommerceConfigDisplayCurrencyResponse {
    /// ISO 4217 compliant display currency code
    #[serde(rename = "code")]
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigDisplayCurrencyRequest {
    /// ISO 4217 compliant display currency code
    #[serde(rename = "code")]
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigDisplayCurrencyResponse {
    /// ISO 4217 compliant display currency code
    #[serde(rename = "code")]
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetOrdersSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetOrdersSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetOrdersSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchOrderRequest {
    /// Defines whether you want your orders to be considered as live data or as historical data (import of past data, synchronising data). True: orders will not trigger any automation workflows. False: orders will trigger workflows as usual.
    #[serde(
        rename = "historical",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub historical: Option<bool>,
    /// Webhook URL to receive the status of the batch request
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none", default)]
    pub notify_url: Option<String>,
    /// array of order objects
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchOrderResponse {
    /// Batch ID of the request
    #[serde(rename = "batchId")]
    pub batch_id: f64,
    /// Number of orders
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequestRequest {
    #[serde(rename = "cart")]
    pub cart: serde_json::Value,
    #[serde(
        rename = "configuration",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub configuration: Option<serde_json::Value>,
    /// Brevo ID of the contact requested to pay.
    #[serde(rename = "contactId")]
    pub contact_id: i64,
    /// Description of payment request.
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    #[serde(
        rename = "notification",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub notification: Option<serde_json::Value>,
    /// Reference of the payment request, it will appear on the payment page.
    #[serde(rename = "reference")]
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequestResponse {
    /// ID of the object created
    #[serde(rename = "id")]
    pub id: i64,
    /// URL of the payment request created
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

/// Status of the payment request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetPaymentRequestResponseStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "reminderSent")]
    ReminderSent,
    #[serde(rename = "paid")]
    Paid,
}
impl Default for GetPaymentRequestResponseStatus {
    fn default() -> Self {
        Self::Created
    }
}
impl core::fmt::Display for GetPaymentRequestResponseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Sent => write!(f, "sent"),
            Self::ReminderSent => write!(f, "reminderSent"),
            Self::Paid => write!(f, "paid"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPaymentRequestResponse {
    #[serde(rename = "cart")]
    pub cart: serde_json::Value,
    #[serde(
        rename = "configuration",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub configuration: Option<serde_json::Value>,
    /// Brevo ID of the contact requested to pay.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    #[serde(rename = "notification")]
    pub notification: serde_json::Value,
    /// number of reminders sent.
    #[serde(
        rename = "numberOfRemindersSent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub number_of_reminders_sent: Option<i64>,
    /// Reference of the payment request, it will appear on the payment page.
    #[serde(rename = "reference")]
    pub reference: String,
    /// Status of the payment request.
    #[serde(rename = "status")]
    pub status: GetPaymentRequestResponseStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProductsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetProductsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetProductsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetProductsSortByField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "price")]
    Price,
    #[serde(rename = "id")]
    Id,
}
impl Default for GetProductsSortByField {
    fn default() -> Self {
        Self::CreatedAt
    }
}
impl core::fmt::Display for GetProductsSortByField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Name => write!(f, "name"),
            Self::Price => write!(f, "price"),
            Self::Id => write!(f, "id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductsResponse {
    /// Number of products
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "products")]
    pub products: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateProductRequest {
    /// Brand of the product
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none", default)]
    pub brand: Option<String>,
    /// Category ID-s of the product
    #[serde(
        rename = "categories",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub categories: Option<Vec<String>>,
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the product deleted from the shop's database
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none", default)]
    pub deleted_at: Option<String>,
    /// Description of the product
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Product ID for which you requested the details
    #[serde(rename = "id")]
    pub id: String,
    /// Absolute URL to the cover image of the product
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    /// product deleted from the shop's database
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none", default)]
    pub is_deleted: Option<bool>,
    /// Meta data of product such as description, vendor, producer, stock level. The total characters of cumulative metaInfo shall not exceed **20000 characters**.
    #[serde(rename = "metaInfo", skip_serializing_if = "Option::is_none", default)]
    pub meta_info: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// **Mandatory in case of creation**. Name of the product, as displayed in the shop
    #[serde(rename = "name")]
    pub name: String,
    /// Parent product id of the product
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none", default)]
    pub parent_id: Option<String>,
    /// Price of the product
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f32>,
    /// Alternative price of the product
    #[serde(
        rename = "alternativePrice",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub alternative_price: Option<f32>,
    /// Product identifier from the shop
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none", default)]
    pub sku: Option<String>,
    /// Current stock value of the product from the shop's database
    #[serde(rename = "stock", skip_serializing_if = "Option::is_none", default)]
    pub stock: Option<f64>,
    /// Facilitate to update the existing product in the same request (updateEnabled = true)
    #[serde(
        rename = "updateEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_enabled: Option<bool>,
    /// URL to the product
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateProductResponse {
    /// ID of the product when a new product is created
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchProductsRequestProductsItem {
    /// Brand of the product
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none", default)]
    pub brand: Option<String>,
    /// Category ID-s of the product
    #[serde(
        rename = "categories",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub categories: Option<Vec<String>>,
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the product deleted from the shop's database
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none", default)]
    pub deleted_at: Option<String>,
    /// Description of the product
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Product ID for which you requested the details
    #[serde(rename = "id")]
    pub id: String,
    /// Absolute URL to the cover image of the product
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    /// product deleted from the shop's database
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none", default)]
    pub is_deleted: Option<bool>,
    /// Meta data of product such as description, vendor, producer, stock level. The total characters of cumulative metaInfo shall not exceed **20000 characters**.
    #[serde(rename = "metaInfo", skip_serializing_if = "Option::is_none", default)]
    pub meta_info: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// **Mandatory in case of creation**. Name of the product, as displayed in the shop
    #[serde(rename = "name")]
    pub name: String,
    /// Parent product id of the product
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none", default)]
    pub parent_id: Option<String>,
    /// Price of the product
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f32>,
    /// Alternative price of the product
    #[serde(
        rename = "alternativePrice",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub alternative_price: Option<f32>,
    /// Product identifier from the shop
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none", default)]
    pub sku: Option<String>,
    /// Current stock value of the product from the shop's database
    #[serde(rename = "stock", skip_serializing_if = "Option::is_none", default)]
    pub stock: Option<f64>,
    /// URL to the product
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchProductsRequest {
    /// array of products objects
    #[serde(rename = "products")]
    pub products: Vec<CreateUpdateBatchProductsRequestProductsItem>,
    /// Facilitate to update the existing products in the same request (updateEnabled = true)
    #[serde(
        rename = "updateEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBatchProductsResponse {
    /// Number of the new created products
    #[serde(
        rename = "createdCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub created_count: Option<i64>,
    /// Number of the existing products updated
    #[serde(
        rename = "updatedCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub updated_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateProductAlertType {
    #[serde(rename = "back_in_stock")]
    BackInStock,
}
impl Default for CreateProductAlertType {
    fn default() -> Self {
        Self::BackInStock
    }
}
impl core::fmt::Display for CreateProductAlertType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BackInStock => write!(f, "back_in_stock"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAlertRequestContactIdentifiers {
    /// Email address of the contact
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Contact ID in your system
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Sms of the contact
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none", default)]
    pub sms: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAlertRequest {
    #[serde(
        rename = "contactIdentifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_identifiers: Option<CreateProductAlertRequestContactIdentifiers>,
}

/// Identifies the contact associated with the event. At least one identifier is required.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequestIdentifiers {
    /// Internal unique contact ID. When present, this takes priority over all other identifiers for event attribution and contact resolution.
    #[serde(
        rename = "contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_id: Option<i64>,
    /// Email Id associated with the event
    #[serde(rename = "email_id", skip_serializing_if = "Option::is_none", default)]
    pub email_id: Option<String>,
    /// ext_id associated with the event
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// landline_number associated with the event
    #[serde(
        rename = "landline_number_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landline_number_id: Option<String>,
    /// SMS associated with the event
    #[serde(rename = "phone_id", skip_serializing_if = "Option::is_none", default)]
    pub phone_id: Option<String>,
    /// whatsapp associated with the event
    #[serde(
        rename = "whatsapp_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whatsapp_id: Option<String>,
}

/// Identifiers for the object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequestObjectIdentifiers {
    /// External object ID
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Internal object ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

/// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequestObject {
    /// Identifiers for the object.
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<CreateEventRequestObjectIdentifiers>,
    /// Type of object (e.g., subscription, vehicle, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    /// Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **"FIRSTNAME": "Jane" , "AGE": 37**
    #[serde(
        rename = "contact_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// ISO 8601 timestamp of when the event occurred (e.g. "2024-01-24T17:39:57+01:00"). If no value is passed, the timestamp of the event creation is used.
    #[serde(
        rename = "event_date",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_date: Option<time::OffsetDateTime>,
    /// The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters; only alphanumeric characters, hyphens (-), and underscores (_) are allowed.
    #[serde(rename = "event_name")]
    pub event_name: String,
    /// Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field types are supported: string, number, boolean (true/false), date (Timestamp e.g. "2024-01-24T17:39:57+01:00"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50KB.
    #[serde(
        rename = "event_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Identifies the contact associated with the event. At least one identifier is required.
    #[serde(rename = "identifiers")]
    pub identifiers: CreateEventRequestIdentifiers,
    /// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none", default)]
    pub object: Option<CreateEventRequestObject>,
}

/// Identifies the contact associated with the event. At least one identifier is required.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchEventsRequestEventsItemIdentifiers {
    /// Internal unique contact ID. When present, this takes priority over all other identifiers for event attribution and contact resolution.
    #[serde(
        rename = "contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_id: Option<i64>,
    /// Email Id associated with the event
    #[serde(rename = "email_id", skip_serializing_if = "Option::is_none", default)]
    pub email_id: Option<String>,
    /// ext_id associated with the event
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// landline_number associated with the event
    #[serde(
        rename = "landline_number_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub landline_number_id: Option<String>,
    /// SMS associated with the event
    #[serde(rename = "phone_id", skip_serializing_if = "Option::is_none", default)]
    pub phone_id: Option<String>,
    /// whatsapp associated with the event
    #[serde(
        rename = "whatsapp_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whatsapp_id: Option<String>,
}

/// Identifiers for the object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchEventsRequestEventsItemObjectIdentifiers {
    /// External object ID
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Internal object ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

/// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchEventsRequestEventsItemObject {
    /// Identifiers for the object.
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<CreateBatchEventsRequestEventsItemObjectIdentifiers>,
    /// Type of object (e.g., subscription, vehicle, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchEventsRequestEventsItem {
    /// Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **"FIRSTNAME": "Jane" , "AGE": 37**
    #[serde(
        rename = "contact_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Timestamp of when the event occurred (e.g. "2024-01-24T17:39:57+01:00"). If no value is passed, the timestamp of the event creation is used.
    #[serde(
        rename = "event_date",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_date: Option<String>,
    /// The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters, alphanumerical characters and - _ only.
    #[serde(rename = "event_name")]
    pub event_name: String,
    /// Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field type are supported: string, number, boolean (true/false), date (Timestamp e.g. "2024-01-24T17:39:57+01:00"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50Kb.
    #[serde(
        rename = "event_properties",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Identifies the contact associated with the event. At least one identifier is required.
    #[serde(rename = "identifiers")]
    pub identifiers: CreateBatchEventsRequestEventsItemIdentifiers,
    /// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none", default)]
    pub object: Option<CreateBatchEventsRequestEventsItemObject>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchEventsRequest {
    #[serde(rename = "events")]
    pub events: Vec<CreateBatchEventsRequestEventsItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetInboundEmailEventsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetInboundEmailEventsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetInboundEmailEventsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundEmailEventsResponseEventsItem {
    /// Date when email was received on SMTP relay
    #[serde(rename = "date")]
    pub date: time::OffsetDateTime,
    /// Recipient’s email address
    #[serde(rename = "recipient")]
    pub recipient: crate::types::Email,
    /// Sender’s email address
    #[serde(rename = "sender")]
    pub sender: crate::types::Email,
    /// UUID that can be used to fetch additional data
    #[serde(rename = "uuid")]
    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundEmailEventsResponse {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<GetInboundEmailEventsResponseEventsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundEmailEventsByUuidResponseAttachmentsItem {
    /// value of the Content-ID header of the attachment.
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none", default)]
    pub content_id: Option<String>,
    /// size of the attachment in bytes
    #[serde(
        rename = "contentLength",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub content_length: Option<i64>,
    /// value of the Content-Type header of the attachment
    #[serde(
        rename = "contentType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub content_type: Option<String>,
    /// filename specified in the Content-Disposition header of the attachment
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Type of the event
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetInboundEmailEventsByUuidResponseLogsItemType {
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "webhookFailed")]
    WebhookFailed,
    #[serde(rename = "webhookDelivered")]
    WebhookDelivered,
}
impl Default for GetInboundEmailEventsByUuidResponseLogsItemType {
    fn default() -> Self {
        Self::Received
    }
}
impl core::fmt::Display for GetInboundEmailEventsByUuidResponseLogsItemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Received => write!(f, "received"),
            Self::Processed => write!(f, "processed"),
            Self::WebhookFailed => write!(f, "webhookFailed"),
            Self::WebhookDelivered => write!(f, "webhookDelivered"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundEmailEventsByUuidResponseLogsItem {
    /// Date of the event
    #[serde(rename = "date", skip_serializing_if = "Option::is_none", default)]
    pub date: Option<time::OffsetDateTime>,
    /// Type of the event
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<GetInboundEmailEventsByUuidResponseLogsItemType>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundEmailEventsByUuidResponse {
    /// List of attachments of the email. This will be present only after the processing is done.
    #[serde(
        rename = "attachments",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachments: Option<Vec<GetInboundEmailEventsByUuidResponseAttachmentsItem>>,
    /// Date when email was delivered successfully to the client’s webhook
    #[serde(
        rename = "deliveredAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub delivered_at: Option<time::OffsetDateTime>,
    /// List of events/logs that describe the lifecycle of the email on the Brevo platform
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none", default)]
    pub logs: Option<Vec<GetInboundEmailEventsByUuidResponseLogsItem>>,
    /// Value of the Message-ID header. This will be present only after the processing is done.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none", default)]
    pub message_id: Option<String>,
    /// Date when email was received on SMTP relay
    #[serde(
        rename = "receivedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub received_at: Option<time::OffsetDateTime>,
    /// Recipient’s email address
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none", default)]
    pub recipient: Option<crate::types::Email>,
    /// Sender’s email address
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<crate::types::Email>,
    /// Value of the Subject header. This will be present only after the processing is done.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidActiveBalanceSortField {
    #[serde(rename = "createdAt")]
    CreatedAt,
}
impl Default for GetLoyaltyBalanceProgramsPidActiveBalanceSortField {
    fn default() -> Self {
        Self::CreatedAt
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidActiveBalanceSortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "createdAt"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidActiveBalanceSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetLoyaltyBalanceProgramsPidActiveBalanceSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidActiveBalanceSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem {
    /// Unique identifier of the balance entry.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Current balance amount.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f32>,
    /// Timestamp when the balance was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the balance expires (if applicable).
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none", default)]
    pub expires_at: Option<time::OffsetDateTime>,
}

/// Active balance response for a specific contact and balance definition.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponse {
    /// Total number of active balances.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Unique identifier of the balance definition.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// List of active balances.
    #[serde(
        rename = "activeBalances",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub active_balances:
        Option<Vec<GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetBalanceDefinitionListSortField {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
impl Default for GetBalanceDefinitionListSortField {
    fn default() -> Self {
        Self::Name
    }
}
impl core::fmt::Display for GetBalanceDefinitionListSortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetBalanceDefinitionListSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetBalanceDefinitionListSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetBalanceDefinitionListSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetBalanceDefinitionListVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetBalanceDefinitionListVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetBalanceDefinitionListVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceDefinitionListResponse {
    /// list of balance definitions
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

/// Defines when the balance expires within the selected duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationModifier {
    #[serde(rename = "noModification")]
    NoModification,
    #[serde(rename = "startOfPeriod")]
    StartOfPeriod,
    #[serde(rename = "endOfPeriod")]
    EndOfPeriod,
}
impl Default
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationModifier
{
    fn default() -> Self {
        Self::NoModification
    }
}
impl core::fmt::Display
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationModifier
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoModification => write!(f, "noModification"),
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
        }
    }
}

/// Unit of time for balance validity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationUnit
{
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationUnit
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Defines whether partial credit is allowed when reaching max balance.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionAmountOvertakingStrategy
{
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "partial")]
    Partial,
}
impl Default
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionAmountOvertakingStrategy
{
    fn default() -> Self {
        Self::Strict
    }
}
impl core::fmt::Display
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionAmountOvertakingStrategy
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Partial => write!(f, "partial"),
        }
    }
}

/// Defines rounding strategy for credit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionCreditRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionCreditRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionCreditRounding
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Defines rounding strategy for debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionDebitRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionDebitRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display
    for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionDebitRounding
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Unit of balance measurement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit {
    #[serde(rename = "POINTS")]
    POINTS,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "BRL")]
    BRL,
}
impl Default for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit {
    fn default() -> Self {
        Self::POINTS
    }
}
impl core::fmt::Display for PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::POINTS => write!(f, "POINTS"),
            Self::EUR => write!(f, "EUR"),
            Self::USD => write!(f, "USD"),
            Self::MXN => write!(f, "MXN"),
            Self::GBP => write!(f, "GBP"),
            Self::INR => write!(f, "INR"),
            Self::CAD => write!(f, "CAD"),
            Self::SGD => write!(f, "SGD"),
            Self::RON => write!(f, "RON"),
            Self::JPY => write!(f, "JPY"),
            Self::MYR => write!(f, "MYR"),
            Self::CLP => write!(f, "CLP"),
            Self::PEN => write!(f, "PEN"),
            Self::MAD => write!(f, "MAD"),
            Self::AUD => write!(f, "AUD"),
            Self::CHF => write!(f, "CHF"),
            Self::BRL => write!(f, "BRL"),
        }
    }
}

/// Payload for creating a new balance definition, specifying expiration rules, rounding strategies, and constraints.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequest {
    /// Defines when the balance expires within the selected duration.
    #[serde(
        rename = "balanceAvailabilityDurationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_modifier: Option<
        PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationModifier,
    >,
    /// Unit of time for balance validity.
    #[serde(
        rename = "balanceAvailabilityDurationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_unit: Option<
        PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationUnit,
    >,
    /// Number of time units before the balance expires.
    #[serde(
        rename = "balanceAvailabilityDurationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_value: Option<i64>,
    /// Fixed expiration date (`dd/mm` format) as an alternative to duration-based expiry.
    #[serde(
        rename = "balanceExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_expiration_date: Option<time::Date>,
    /// Defines whether partial credit is allowed when reaching max balance.
    #[serde(
        rename = "balanceOptionAmountOvertakingStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_amount_overtaking_strategy: Option<
        PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionAmountOvertakingStrategy,
    >,
    /// Defines rounding strategy for credit transactions.
    #[serde(
        rename = "balanceOptionCreditRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_credit_rounding:
        Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionCreditRounding>,
    /// Defines rounding strategy for debit transactions.
    #[serde(
        rename = "balanceOptionDebitRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_debit_rounding:
        Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionDebitRounding>,
    /// Short description of the balance definition.
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// URL of an optional image reference.
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Maximum allowable balance amount.
    #[serde(rename = "maxAmount", skip_serializing_if = "Option::is_none", default)]
    pub max_amount: Option<f64>,
    /// Maximum credit allowed per operation.
    #[serde(
        rename = "maxCreditAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_credit_amount_limit: Option<f64>,
    /// Maximum debit allowed per operation.
    #[serde(
        rename = "maxDebitAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_debit_amount_limit: Option<f64>,
    /// Additional metadata for the balance definition.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Minimum allowable balance amount.
    #[serde(rename = "minAmount", skip_serializing_if = "Option::is_none", default)]
    pub min_amount: Option<f64>,
    /// Name of the balance definition.
    #[serde(rename = "name")]
    pub name: String,
    /// Unit of balance measurement.
    #[serde(rename = "unit")]
    pub unit: PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetBalanceDefinitionVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetBalanceDefinitionVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetBalanceDefinitionVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// Defines when the balance expires within the selected duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier {
    #[serde(rename = "noModification")]
    NoModification,
    #[serde(rename = "startOfPeriod")]
    StartOfPeriod,
    #[serde(rename = "endOfPeriod")]
    EndOfPeriod,
}
impl Default for UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier {
    fn default() -> Self {
        Self::NoModification
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoModification => write!(f, "noModification"),
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
        }
    }
}

/// Unit of time for balance validity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Defines whether partial credit is allowed when reaching max balance.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "partial")]
    Partial,
}
impl Default for UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy {
    fn default() -> Self {
        Self::Strict
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Partial => write!(f, "partial"),
        }
    }
}

/// Rounding strategy for credit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestBalanceOptionCreditRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for UpdateBalanceDefinitionRequestBalanceOptionCreditRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestBalanceOptionCreditRounding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Rounding strategy for debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestBalanceOptionDebitRounding {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "natural")]
    Natural,
}
impl Default for UpdateBalanceDefinitionRequestBalanceOptionDebitRounding {
    fn default() -> Self {
        Self::Lower
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestBalanceOptionDebitRounding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lower => write!(f, "lower"),
            Self::Upper => write!(f, "upper"),
            Self::Natural => write!(f, "natural"),
        }
    }
}

/// Unit of balance measurement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceDefinitionRequestUnit {
    #[serde(rename = "POINTS")]
    POINTS,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "BRL")]
    BRL,
}
impl Default for UpdateBalanceDefinitionRequestUnit {
    fn default() -> Self {
        Self::POINTS
    }
}
impl core::fmt::Display for UpdateBalanceDefinitionRequestUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::POINTS => write!(f, "POINTS"),
            Self::EUR => write!(f, "EUR"),
            Self::USD => write!(f, "USD"),
            Self::MXN => write!(f, "MXN"),
            Self::GBP => write!(f, "GBP"),
            Self::INR => write!(f, "INR"),
            Self::CAD => write!(f, "CAD"),
            Self::SGD => write!(f, "SGD"),
            Self::RON => write!(f, "RON"),
            Self::JPY => write!(f, "JPY"),
            Self::MYR => write!(f, "MYR"),
            Self::CLP => write!(f, "CLP"),
            Self::PEN => write!(f, "PEN"),
            Self::MAD => write!(f, "MAD"),
            Self::AUD => write!(f, "AUD"),
            Self::CHF => write!(f, "CHF"),
            Self::BRL => write!(f, "BRL"),
        }
    }
}

/// Payload for updating an existing balance definition, including expiry rules, rounding strategies, and constraints.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBalanceDefinitionRequest {
    /// Defines when the balance expires within the selected duration.
    #[serde(
        rename = "balanceAvailabilityDurationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_modifier:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier>,
    /// Unit of time for balance validity.
    #[serde(
        rename = "balanceAvailabilityDurationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_unit:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit>,
    /// Number of time units before the balance expires.
    #[serde(
        rename = "balanceAvailabilityDurationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_availability_duration_value: Option<i64>,
    /// Expiration date (`dd/mm` format) or empty if not applicable.
    #[serde(
        rename = "balanceExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_expiration_date: Option<String>,
    /// Defines whether partial credit is allowed when reaching max balance.
    #[serde(
        rename = "balanceOptionAmountOvertakingStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_amount_overtaking_strategy:
        Option<UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy>,
    /// Rounding strategy for credit transactions.
    #[serde(
        rename = "balanceOptionCreditRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_credit_rounding:
        Option<UpdateBalanceDefinitionRequestBalanceOptionCreditRounding>,
    /// Rounding strategy for debit transactions.
    #[serde(
        rename = "balanceOptionDebitRounding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_option_debit_rounding:
        Option<UpdateBalanceDefinitionRequestBalanceOptionDebitRounding>,
    /// Short description of the balance definition.
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// URL of an optional image reference.
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Maximum allowable balance amount.
    #[serde(rename = "maxAmount", skip_serializing_if = "Option::is_none", default)]
    pub max_amount: Option<f64>,
    /// Maximum credit allowed per operation.
    #[serde(
        rename = "maxCreditAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_credit_amount_limit: Option<f64>,
    /// Maximum debit allowed per operation.
    #[serde(
        rename = "maxDebitAmountLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_debit_amount_limit: Option<f64>,
    /// Optional metadata for the balance definition.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Minimum allowable balance amount.
    #[serde(rename = "minAmount", skip_serializing_if = "Option::is_none", default)]
    pub min_amount: Option<f64>,
    /// Name of the balance definition.
    #[serde(rename = "name")]
    pub name: String,
    /// Unit of balance measurement.
    #[serde(rename = "unit")]
    pub unit: UpdateBalanceDefinitionRequestUnit,
}

/// Defines whether the limit applies to transaction count or amount.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateBalanceLimitRequestConstraintType {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "amount")]
    Amount,
}
impl Default for CreateBalanceLimitRequestConstraintType {
    fn default() -> Self {
        Self::Transaction
    }
}
impl core::fmt::Display for CreateBalanceLimitRequestConstraintType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transaction => write!(f, "transaction"),
            Self::Amount => write!(f, "amount"),
        }
    }
}

/// Unit of time for which the limit is applicable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateBalanceLimitRequestDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for CreateBalanceLimitRequestDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for CreateBalanceLimitRequestDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Specifies whether the limit applies to credit or debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateBalanceLimitRequestTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for CreateBalanceLimitRequestTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for CreateBalanceLimitRequestTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// Payload for setting transaction or amount-based limits on a balance.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBalanceLimitRequest {
    /// Defines whether the limit applies to transaction count or amount.
    #[serde(rename = "constraintType")]
    pub constraint_type: CreateBalanceLimitRequestConstraintType,
    /// Unit of time for which the limit is applicable.
    #[serde(rename = "durationUnit")]
    pub duration_unit: CreateBalanceLimitRequestDurationUnit,
    /// Number of time units for the balance limit.
    #[serde(rename = "durationValue")]
    pub duration_value: i64,
    /// Determines if the limit resets on a rolling schedule.
    #[serde(
        rename = "slidingSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sliding_schedule: Option<bool>,
    /// Specifies whether the limit applies to credit or debit transactions.
    #[serde(rename = "transactionType")]
    pub transaction_type: CreateBalanceLimitRequestTransactionType,
    /// Maximum allowed value for the specified constraint type.
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetBalanceLimitVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetBalanceLimitVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetBalanceLimitVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// Defines whether the limit applies to transaction count or amount.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceLimitRequestConstraintType {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "amount")]
    Amount,
}
impl Default for UpdateBalanceLimitRequestConstraintType {
    fn default() -> Self {
        Self::Transaction
    }
}
impl core::fmt::Display for UpdateBalanceLimitRequestConstraintType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transaction => write!(f, "transaction"),
            Self::Amount => write!(f, "amount"),
        }
    }
}

/// Unit of time for which the limit is applicable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceLimitRequestDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for UpdateBalanceLimitRequestDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for UpdateBalanceLimitRequestDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Specifies whether the limit applies to credit or debit transactions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateBalanceLimitRequestTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for UpdateBalanceLimitRequestTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for UpdateBalanceLimitRequestTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// Payload for updating an existing balance limit, specifying constraints on transactions or amounts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBalanceLimitRequest {
    /// Defines whether the limit applies to transaction count or amount.
    #[serde(rename = "constraintType")]
    pub constraint_type: UpdateBalanceLimitRequestConstraintType,
    /// Unit of time for which the limit is applicable.
    #[serde(rename = "durationUnit")]
    pub duration_unit: UpdateBalanceLimitRequestDurationUnit,
    /// Number of time units for the balance limit.
    #[serde(rename = "durationValue")]
    pub duration_value: i64,
    /// Determines if the limit resets on a rolling schedule.
    #[serde(
        rename = "slidingSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sliding_schedule: Option<bool>,
    /// Specifies whether the limit applies to credit or debit transactions.
    #[serde(rename = "transactionType")]
    pub transaction_type: UpdateBalanceLimitRequestTransactionType,
    /// Maximum allowed value for the specified constraint type.
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetContactBalancesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetContactBalancesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetContactBalancesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetContactBalancesSortField {
    #[serde(rename = "updatedAt")]
    UpdatedAt,
    #[serde(rename = "value")]
    Value,
}
impl Default for GetContactBalancesSortField {
    fn default() -> Self {
        Self::UpdatedAt
    }
}
impl core::fmt::Display for GetContactBalancesSortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UpdatedAt => write!(f, "updatedAt"),
            Self::Value => write!(f, "value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactBalancesResponseBalancesItem {
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactBalancesResponse {
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none", default)]
    pub balances: Option<Vec<GetContactBalancesResponseBalancesItem>>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
}

/// Specifies the origin of the order.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateBalanceOrderRequestSource {
    #[serde(rename = "engine")]
    Engine,
    #[serde(rename = "user")]
    User,
}
impl Default for CreateBalanceOrderRequestSource {
    fn default() -> Self {
        Self::Engine
    }
}
impl core::fmt::Display for CreateBalanceOrderRequestSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Engine => write!(f, "engine"),
            Self::User => write!(f, "user"),
        }
    }
}

/// Payload for creating an order linked to a balance definition.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBalanceOrderRequest {
    /// Order amount (must be non-zero).
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    pub balance_definition_id: String,
    /// Unique identifier of the contact placing the order (must be ≥ 1).
    #[serde(rename = "contactId")]
    pub contact_id: i64,
    /// RFC3339 timestamp specifying when the order is due.
    #[serde(rename = "dueAt")]
    pub due_at: String,
    /// Optional RFC3339 timestamp defining order expiration.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none", default)]
    pub expires_at: Option<String>,
    /// Optional metadata associated with the order.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Specifies the origin of the order.
    #[serde(rename = "source")]
    pub source: CreateBalanceOrderRequestSource,
}

/// Represents an order linked to a balance definition, including amount, due date, and transaction details.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBalanceOrderResponse {
    /// Order amount (must not be zero).
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Optional unique identifier (UUID) of the associated balance definition.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact placing the order (must be ≥ 1).
    #[serde(rename = "contactId")]
    pub contact_id: i64,
    /// RFC3339 timestamp indicating when the order was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// RFC3339 timestamp specifying when the order is due in the future.
    #[serde(rename = "dueAt")]
    pub due_at: String,
    /// Optional RFC3339 timestamp defining order expiration in the future.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none", default)]
    pub expires_at: Option<String>,
    /// Unique identifier for the balance order.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Unique identifier of the loyalty program associated with the order.
    #[serde(rename = "loyaltyProgramId")]
    pub loyalty_program_id: String,
    /// Optional metadata associated with the order.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Optional RFC3339 timestamp indicating when the order was processed.
    #[serde(
        rename = "processedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub processed_at: Option<String>,
    /// Optional reference to the associated transaction ID.
    #[serde(
        rename = "transactionid",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transactionid: Option<String>,
    /// RFC3339 timestamp indicating the last update to the order.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscriptionBalancesResponseBalanceItem {
    /// Balance definition ID.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Aggregate balance value for this definition.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscriptionBalancesResponse {
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Aggregate balance per balance definition.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none", default)]
    pub balance: Option<Vec<GetSubscriptionBalancesResponseBalanceItem>>,
}

/// Payload for creating a new balance linked to a specific balance definition.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
    /// Unique identifier (UUID) of the balance definition associated with the new balance.
    #[serde(rename = "balanceDefinitionId")]
    pub balance_definition_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse {
    /// Unique identifier for the balance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// The current amount available in the balance.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f32>,
    /// Loyalty program ID.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Organization ID.
    #[serde(
        rename = "organizationId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organization_id: Option<i64>,
    /// Contact ID.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Balance definition ID.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Timestamp of when the balance was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Expiration timestamp of the balance.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none", default)]
    pub expires_at: Option<time::OffsetDateTime>,
    /// Timestamp of when the balance was last consumed.
    #[serde(
        rename = "consumedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub consumed_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidTransactionHistorySortField {
    #[serde(rename = "createdAt")]
    CreatedAt,
}
impl Default for GetLoyaltyBalanceProgramsPidTransactionHistorySortField {
    fn default() -> Self {
        Self::CreatedAt
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidTransactionHistorySortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "createdAt"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidTransactionHistorySort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetLoyaltyBalanceProgramsPidTransactionHistorySort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidTransactionHistorySort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidTransactionHistoryStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl Default for GetLoyaltyBalanceProgramsPidTransactionHistoryStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidTransactionHistoryStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Completed => write!(f, "completed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidTransactionHistoryTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for GetLoyaltyBalanceProgramsPidTransactionHistoryTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for GetLoyaltyBalanceProgramsPidTransactionHistoryTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// The type of the transaction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType
{
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default
    for GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType
{
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display
    for GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// Represents a record of a past transaction, including status and key timestamps.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem {
    /// The transaction amount.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f32>,
    /// The type of the transaction.
    #[serde(
        rename = "transactionType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_type: Option<
        GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType,
    >,
    /// Expiration date of the balance associated with this transaction.
    #[serde(
        rename = "balanceExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_expiration_date: Option<String>,
    /// Timestamp when the transaction was canceled, if applicable.
    #[serde(
        rename = "cancelledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub cancelled_at: Option<String>,
    /// Timestamp when the transaction was successfully completed.
    #[serde(
        rename = "completedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub completed_at: Option<String>,
    /// Timestamp when the transaction was initiated.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Unique identifier of the transaction.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Optional metadata associated with the transaction.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Reason for rejection, if the transaction was declined.
    #[serde(
        rename = "rejectReason",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reject_reason: Option<String>,
    /// Timestamp when the transaction was rejected.
    #[serde(
        rename = "rejectedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub rejected_at: Option<String>,
    /// Current status of the transaction (e.g., pending, completed, rejected).
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
}

/// Response containing transaction history details for a specific balance and contact.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponse {
    /// Unique identifier of the associated balance definition.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact related to the transactions.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Total number of transactions in the history.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    /// Unique identifier of the associated loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// List of past transactions associated with the balance.
    #[serde(
        rename = "transactionHistory",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_history:
        Option<Vec<GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem>>,
}

/// Explicit transaction type. If not provided, the type is inferred from the sign of the amount (positive = credit, negative = debit).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BeginTransactionRequestTransactionType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
}
impl Default for BeginTransactionRequestTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}
impl core::fmt::Display for BeginTransactionRequestTransactionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
        }
    }
}

/// Payload for creating a transaction, specifying balance details and optional expiration.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginTransactionRequest {
    /// Transaction amount. A positive value creates a credit transaction and a negative value creates a debit transaction (unless transactionType is explicitly provided).
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Explicit transaction type. If not provided, the type is inferred from the sign of the amount (positive = credit, negative = debit).
    #[serde(
        rename = "transactionType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_type: Option<BeginTransactionRequestTransactionType>,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    pub balance_definition_id: String,
    /// Unique identifier of the contact involved in the transaction. Required unless `LoyaltySubscriptionId` is provided.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Unique identifier for the loyalty subscription. Required unless `contactId` is provided.
    #[serde(
        rename = "LoyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Optional metadata associated with the transaction.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Time-to-live for the transaction in seconds. Must be at least 10 seconds if provided.
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none", default)]
    pub ttl: Option<i64>,
    /// Timestamp specifying when the transaction event occurred (ISO 8601 / RFC 3339 format).
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none", default)]
    pub event_time: Option<time::OffsetDateTime>,
    /// Whether the transaction should be automatically completed.
    #[serde(
        rename = "autoComplete",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub auto_complete: Option<bool>,
    /// Expiry time for the balance in minutes. Must be greater than 0 if provided. Only applicable when autoComplete is true.
    #[serde(
        rename = "balanceExpiryInMinutes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_expiry_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLPListSortField {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
impl Default for GetLPListSortField {
    fn default() -> Self {
        Self::Name
    }
}
impl core::fmt::Display for GetLPListSortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLPListSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetLPListSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetLPListSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLPListResponse {
    /// Loyalty Program list
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewLPRequest {
    /// Optional description of the loyalty program (max 256 chars).
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Optional unique document ID.
    #[serde(
        rename = "documentId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub document_id: Option<String>,
    /// Optional metadata related to the loyalty program.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Required name of the loyalty program (max 128 chars).
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyUpdateLoyaltyProgramRequest {
    /// Loyalty Program description
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Loyalty Program meta data
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Loyalty Program name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLoyaltyProgramRequest {
    /// Loyalty Program description
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Loyalty Program meta data
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Loyalty Program name
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseBalanceBalancesItem {
    /// Unique identifier for the balance definition).
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Name of the balance definition.
    #[serde(
        rename = "balanceDefinitionName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_name: Option<String>,
    /// The amount of the balance.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
}

/// Balance details for the subscription.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseBalance {
    /// List of balance details associated with the contact.
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none", default)]
    pub balances: Option<Vec<GetParameterSubscriptionInfoResponseBalanceBalancesItem>>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseMembersItem {
    /// Timestamp when the member was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Unique identifier of the member.
    #[serde(
        rename = "memberContactId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub member_contact_id: Option<i64>,
    /// Timestamp when the member was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
}

/// Membership details of the subscription. Returned when the subscription could be resolved from the provided `contactId` or `loyaltySubscriptionId`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseMembership {
    /// Timestamp when the subscription was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Unique identifier of the loyalty subscription.
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Timestamp when the subscription was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// Unit the reward's value is expressed in — one of the currency codes listed below, or PERCENT. Omitted when the reward has no unit (e.g. free-product rewards).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetParameterSubscriptionInfoResponseRewardItemUnit {
    #[serde(rename = "PERCENT")]
    PERCENT,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "BRL")]
    BRL,
}
impl Default for GetParameterSubscriptionInfoResponseRewardItemUnit {
    fn default() -> Self {
        Self::PERCENT
    }
}
impl core::fmt::Display for GetParameterSubscriptionInfoResponseRewardItemUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PERCENT => write!(f, "PERCENT"),
            Self::EUR => write!(f, "EUR"),
            Self::USD => write!(f, "USD"),
            Self::MXN => write!(f, "MXN"),
            Self::GBP => write!(f, "GBP"),
            Self::INR => write!(f, "INR"),
            Self::CAD => write!(f, "CAD"),
            Self::SGD => write!(f, "SGD"),
            Self::RON => write!(f, "RON"),
            Self::JPY => write!(f, "JPY"),
            Self::MYR => write!(f, "MYR"),
            Self::CLP => write!(f, "CLP"),
            Self::PEN => write!(f, "PEN"),
            Self::MAD => write!(f, "MAD"),
            Self::AUD => write!(f, "AUD"),
            Self::CHF => write!(f, "CHF"),
            Self::BRL => write!(f, "BRL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseRewardItem {
    /// Reward code assigned to the contact.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Timestamp when the reward was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Expiration date of the reward.
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<String>,
    /// Unique identifier of the reward.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata related to the reward.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Customer-facing description of the reward, as configured on the reward definition.
    #[serde(
        rename = "publicDescription",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_description: Option<String>,
    /// Unique identifier of the reward definition.
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
    /// Customer-facing name of the reward. Falls back to the internal reward name when no public name is set.
    #[serde(
        rename = "rewardName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reward_name: Option<String>,
    /// Unit the reward's value is expressed in — one of the currency codes listed below, or PERCENT. Omitted when the reward has no unit (e.g. free-product rewards).
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<GetParameterSubscriptionInfoResponseRewardItemUnit>,
    /// Timestamp when the reward was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Date from which the voucher becomes valid.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none", default)]
    pub valid_from: Option<time::OffsetDateTime>,
    /// The value recorded when this reward was attributed to the contact — a snapshot, not necessarily the reward's current configured value. Omitted when not set.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponseTierItem {
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Timestamp when the tier was assigned.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Unique identifier of the group associated with the tier.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none", default)]
    pub group_id: Option<String>,
    /// Name of the group associated with the tier.
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none", default)]
    pub group_name: Option<String>,
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata related to the tier.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Unique identifier of the tier.
    #[serde(rename = "tierId", skip_serializing_if = "Option::is_none", default)]
    pub tier_id: Option<String>,
    /// Name of the tier.
    #[serde(rename = "tierName", skip_serializing_if = "Option::is_none", default)]
    pub tier_name: Option<String>,
    /// Timestamp when the tier was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameterSubscriptionInfoResponse {
    /// Balance details for the subscription.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none", default)]
    pub balance: Option<GetParameterSubscriptionInfoResponseBalance>,
    /// Name of the loyalty program.
    #[serde(
        rename = "loyaltyProgramName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_name: Option<String>,
    /// List of members associated with the subscription.
    #[serde(rename = "members", skip_serializing_if = "Option::is_none", default)]
    pub members: Option<Vec<GetParameterSubscriptionInfoResponseMembersItem>>,
    /// Membership details of the subscription. Returned when the subscription could be resolved from the provided `contactId` or `loyaltySubscriptionId`.
    #[serde(
        rename = "membership",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub membership: Option<GetParameterSubscriptionInfoResponseMembership>,
    /// List of rewards associated with the subscription.
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none", default)]
    pub reward: Option<Vec<GetParameterSubscriptionInfoResponseRewardItem>>,
    /// List of tier assignments for the subscription.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none", default)]
    pub tier: Option<Vec<GetParameterSubscriptionInfoResponseTierItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMemberToASubscriptionRequest {
    /// Required if LoyaltySubscriptionId is not provided, must be greater than 0
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Required if ContactId is not provided, max length 64
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Required, each item must be greater than or equal to 1
    #[serde(rename = "memberContactIds")]
    pub member_contact_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMemberToASubscriptionResponse {
    /// Unique identifier of the organization.
    #[serde(
        rename = "organizationId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organization_id: Option<i64>,
    /// Unique identifier of the subscription owner.
    #[serde(
        rename = "ownerContactId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub owner_contact_id: Option<i64>,
    /// List of unique member contact IDs.
    #[serde(
        rename = "memberContactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub member_contact_ids: Option<Vec<i64>>,
    /// Timestamp when the subscription member was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the subscription member was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeToLoyaltyProgramRequest {
    /// Required contact ID; must be greater than 0.
    #[serde(rename = "contactId")]
    pub contact_id: i64,
    /// Optional subscription ID (max length 64).
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Optional creation date in ISO 8601 format (YYYY-MM-DDThh:mm:ss.ffffff+HH:MM). Must be in the past.
    #[serde(
        rename = "creationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    /// Optional metadata associated with the subscription.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeToLoyaltyProgramResponse {
    /// Unique identifier of the organization.
    #[serde(
        rename = "organizationId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organization_id: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Version number of the subscription.
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none", default)]
    pub version_id: Option<i64>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Unique identifier of the subscription.
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Metadata associated with the subscription.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Timestamp when the subscription was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the subscription was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCodeCountResponse {
    /// Number of codes
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyOfferProgramsPidOffersVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetLoyaltyOfferProgramsPidOffersVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetLoyaltyOfferProgramsPidOffersVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidOffersResponseItemsItem {
    /// Timestamp for when this reward was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp for when this reward attribution ended
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none", default)]
    pub end_date: Option<time::OffsetDateTime>,
    /// Unique identifier for the reward
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Loyalty Program to which current reward is associated
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Name of the reward
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Public image of the reward
    #[serde(
        rename = "publicImage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_image: Option<String>,
    /// Timestamp for when this reward attribution started
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none", default)]
    pub start_date: Option<time::OffsetDateTime>,
    /// State of the reward
    #[serde(rename = "state", skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    /// Timestamp for when this reward attribution was updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidOffersResponse {
    /// Items for the current reward
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<GetLoyaltyOfferProgramsPidOffersResponseItemsItem>>,
    /// Count of the associated parameter in current reward
    #[serde(
        rename = "totalCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRewardRequest {
    /// Internal name of the reward
    #[serde(rename = "name")]
    pub name: String,
    /// Public facing description of the reward
    #[serde(
        rename = "publicDescription",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_description: Option<String>,
    /// URL of the public image for the reward
    #[serde(
        rename = "publicImage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_image: Option<String>,
    /// Public facing name of the reward
    #[serde(
        rename = "publicName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRewardResponse {
    /// Unique identifier for the reward
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Loyalty program to which the reward belongs
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Name of the reward
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Public description of the reward
    #[serde(
        rename = "publicDescription",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_description: Option<String>,
    /// Public image URL of the reward
    #[serde(
        rename = "publicImage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_image: Option<String>,
    /// Public name of the reward
    #[serde(
        rename = "publicName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_name: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the reward was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVoucherRequest {
    /// Code generated to attribute reward to a contact
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Contact to attribute the reward
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Reward expiration date
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<String>,
    /// One of contactId or loyaltySubscriptionId is required
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Offer meta information (key/value object)
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Reward id
    #[serde(rename = "rewardId")]
    pub reward_id: String,
    /// Value of the selected reward config
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
    /// Date from which the voucher becomes valid. Accepts RFC 3339 or DD/MM/YYYY HH:MM AM/PM format. Converted to UTC using the organization's timezone.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none", default)]
    pub valid_from: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVoucherResponse {
    /// Generated code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Timestamp for when this reward was consumed
    #[serde(
        rename = "consumedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub consumed_at: Option<String>,
    /// Id of the contact that attributed the reward
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Timestamp for when this reward was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Expiration date of the reward
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// Unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Loyalty Program Id to which attributed reward belongs
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Additional data to define the reward
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
    /// Timestamp for when this reward was updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Value of the selected reward config
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
    /// Date from which the voucher becomes valid
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none", default)]
    pub valid_from: Option<time::OffsetDateTime>,
}

/// Billing information for the order
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherRequestOrderBilling {
    /// Address of the user
    #[serde(rename = "address", skip_serializing_if = "Option::is_none", default)]
    pub address: Option<String>,
    /// City of the user
    #[serde(rename = "city", skip_serializing_if = "Option::is_none", default)]
    pub city: Option<String>,
    /// Country code of the user
    #[serde(
        rename = "countryCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_code: Option<String>,
    /// Payment method opted by the user
    #[serde(
        rename = "paymentMethod",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub payment_method: Option<String>,
    /// Phone number of the user
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none", default)]
    pub phone: Option<i64>,
    /// Postal Code of user's location
    #[serde(rename = "postCode", skip_serializing_if = "Option::is_none", default)]
    pub post_code: Option<i64>,
    /// Region where user resides
    #[serde(rename = "region", skip_serializing_if = "Option::is_none", default)]
    pub region: Option<String>,
}

/// Additional identifiers for the order
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherRequestOrderIdentifiers {
    /// External identifier for the order
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none", default)]
    pub ext_id: Option<String>,
    /// Identifier for the loyalty subscription
    #[serde(
        rename = "loyalty_subscription_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherRequestOrderProductsItem {
    /// List of categories the product belongs to
    #[serde(rename = "category", skip_serializing_if = "Option::is_none", default)]
    pub category: Option<Vec<String>>,
    /// Price of the product
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f64>,
    /// Unique identifier for the product
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none", default)]
    pub product_id: Option<String>,
    /// Quantity of the product
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none", default)]
    pub quantity: Option<f64>,
    /// Identifier for the product variant
    #[serde(rename = "variantId", skip_serializing_if = "Option::is_none", default)]
    pub variant_id: Option<String>,
}

/// Order details for the redemption
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherRequestOrder {
    /// Total amount of the order
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f64>,
    /// Billing information for the order
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none", default)]
    pub billing: Option<RedeemVoucherRequestOrderBilling>,
    /// Unique identifier for the contact
    #[serde(
        rename = "contact_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_id: Option<i64>,
    /// List of coupon codes applied to the order
    #[serde(rename = "coupons", skip_serializing_if = "Option::is_none", default)]
    pub coupons: Option<Vec<String>>,
    /// Timestamp when the order was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Email address associated with the order
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Unique identifier for the order
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Additional identifiers for the order
    #[serde(
        rename = "identifiers",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub identifiers: Option<RedeemVoucherRequestOrderIdentifiers>,
    /// List of products in the order
    #[serde(rename = "products", skip_serializing_if = "Option::is_none", default)]
    pub products: Option<Vec<RedeemVoucherRequestOrderProductsItem>>,
    /// Current status of the order
    #[serde(rename = "status", skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    /// Identifier for the store where the order was placed
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none", default)]
    pub store_id: Option<String>,
    /// Timestamp when the order was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherRequest {
    /// Unique identifier for the attributed reward
    #[serde(
        rename = "attributedRewardId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributed_reward_id: Option<String>,
    /// Redemption code for the reward
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Identifier for the loyalty subscription
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Additional metadata associated with the redeem request
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Order details for the redemption
    #[serde(rename = "order", skip_serializing_if = "Option::is_none", default)]
    pub order: Option<RedeemVoucherRequestOrder>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
    /// Time to live in seconds for the redemption request
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none", default)]
    pub ttl: Option<i64>,
    /// Whether the redemption should be automatically completed
    #[serde(
        rename = "autoComplete",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub auto_complete: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateRewardRequest {
    /// Unique identifier for the attributed reward
    #[serde(
        rename = "attributedRewardId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributed_reward_id: Option<String>,
    /// Validation code for the reward
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// Identifier for the loyalty subscription
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Identifier for the point of sale
    #[serde(
        rename = "pointOfSellId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub point_of_sell_id: Option<String>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateRewardResponse {
    /// Boolean value to authorize a reward or not
    #[serde(rename = "authorize", skip_serializing_if = "Option::is_none", default)]
    pub authorize: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyOfferProgramsPidRewardsRidVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetLoyaltyOfferProgramsPidRewardsRidVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetLoyaltyOfferProgramsPidRewardsRidVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// Select startOfPeriod to configure rewards expiry on start of day/week/month/year. Select endOfPeriod to configure reward expiry on end of day/week/month/year, else select noModification
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    #[serde(rename = "startOfPeriod")]
    StartOfPeriod,
    #[serde(rename = "endOfPeriod")]
    EndOfPeriod,
    #[serde(rename = "noModification")]
    NoModification,
}
impl Default for GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    fn default() -> Self {
        Self::StartOfPeriod
    }
}
impl core::fmt::Display for GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
            Self::NoModification => write!(f, "noModification"),
        }
    }
}

/// object
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator {
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Public facing description of the reward
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Unique identifier for the reward
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Name of the reward
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Generated pattern
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none", default)]
    pub pattern: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem {
    /// Timestamp when the reward limit was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Unit of time for the reward limit's availability (e.g., day/week/month/year).
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<String>,
    /// Number of days/weeks/month/year for reward limit
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Value of the limit
    #[serde(
        rename = "limitValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub limit_value: Option<i64>,
    /// Unique identifier for the reward limit
    #[serde(
        rename = "rewardLimitId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reward_limit_id: Option<String>,
    /// Select true to calculate all redeems/attributions from the previous value of selected durationUnit to the current time
    #[serde(
        rename = "slidingSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sliding_schedule: Option<bool>,
    /// Type of reward
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
    /// Timestamp when the reward limit was created
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem {
    /// Timestamp when the product was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// URL of the product image
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Unique identifier for the product
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none", default)]
    pub product_id: Option<String>,
    /// string
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

/// object
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs {
    /// Attribution config of the reward
    #[serde(
        rename = "attribution",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribution: Option<String>,
    /// Code config of the reward
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Value config of the reward
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

/// Selected event in the rule
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent {
    /// Name of the event
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Source of the event
    #[serde(rename = "source", skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem {
    /// Name of the rule
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Selected value of the parameter to define the rule
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem {
    /// Action for the defined rule
    #[serde(rename = "action", skip_serializing_if = "Option::is_none", default)]
    pub action: Option<String>,
    /// Parameters to define the reward
    #[serde(
        rename = "parameters",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub parameters:
        Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem>>,
    /// Selected service to define the reward
    #[serde(rename = "service", skip_serializing_if = "Option::is_none", default)]
    pub service: Option<String>,
}

/// Rule to define the reward
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRule {
    /// Selected rule condition
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none", default)]
    pub condition: Option<serde_json::Value>,
    /// Timestamp when the rule was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Description of the rule
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// Selected event in the rule
    #[serde(rename = "event", skip_serializing_if = "Option::is_none", default)]
    pub event: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent>,
    /// Metric to identify if it's an internal rule or not
    #[serde(
        rename = "isInternal",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_internal: Option<bool>,
    /// Loyalty Program id to which current rule is associated
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Loyalty Version id to which current rule is associated
    #[serde(
        rename = "loyaltyVersionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_version_id: Option<i64>,
    /// Additional data to define the rule
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Rule name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Results of the rule definition
    #[serde(rename = "results", skip_serializing_if = "Option::is_none", default)]
    pub results: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem>>,
    /// Unique identifier for the rule
    #[serde(rename = "ruleId", skip_serializing_if = "Option::is_none", default)]
    pub rule_id: Option<String>,
    /// Type of the rule
    #[serde(rename = "ruleType", skip_serializing_if = "Option::is_none", default)]
    pub rule_type: Option<String>,
    /// Timestamp when the rule was updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponse {
    /// Maximum number of times a consumer can be attributed this reward
    #[serde(
        rename = "attributionPerConsumer",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribution_per_consumer: Option<i64>,
    /// Unique identifier for the balance definition
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Unique code for the reward
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Total number of codes generated
    #[serde(rename = "codeCount", skip_serializing_if = "Option::is_none", default)]
    pub code_count: Option<i64>,
    /// Unique identifier for the code generator
    #[serde(
        rename = "codeGeneratorId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub code_generator_id: Option<String>,
    /// Unique identifier for the code pool
    #[serde(
        rename = "codePoolId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub code_pool_id: Option<String>,
    /// Configuration settings for the reward
    #[serde(rename = "config", skip_serializing_if = "Option::is_none", default)]
    pub config: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Disabled date of the reward
    #[serde(
        rename = "disabledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub disabled_at: Option<time::OffsetDateTime>,
    /// End date of the reward validity
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none", default)]
    pub end_date: Option<time::OffsetDateTime>,
    /// Expiration date of the reward
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    /// Select startOfPeriod to configure rewards expiry on start of day/week/month/year. Select endOfPeriod to configure reward expiry on end of day/week/month/year, else select noModification
    #[serde(
        rename = "expirationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_modifier: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier>,
    /// Unit of time for the rewards's availability (e.g., day/week/month/year).
    #[serde(
        rename = "expirationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_unit: Option<String>,
    /// Number of days/weeks/month/year for reward expiry
    #[serde(
        rename = "expirationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_value: Option<i64>,
    /// object
    #[serde(rename = "generator", skip_serializing_if = "Option::is_none", default)]
    pub generator: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator>,
    /// Unique identifier for the reward
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Attribution / Redeem Limits for the reward
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none", default)]
    pub limits: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem>>,
    /// Id of the loyalty program to which the current reward belongs to
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Additional data for reward definition
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Name of the reward
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Selected products for reward definition
    #[serde(rename = "products", skip_serializing_if = "Option::is_none", default)]
    pub products: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem>>,
    /// Public description for the reward
    #[serde(
        rename = "publicDescription",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_description: Option<String>,
    /// Public Image for the reward
    #[serde(
        rename = "publicImage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_image: Option<String>,
    /// Public name for the reward
    #[serde(
        rename = "publicName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_name: Option<String>,
    /// Defines the redeem limit for the consumer
    #[serde(
        rename = "redeemPerConsumer",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub redeem_per_consumer: Option<i64>,
    /// Rules defined to redeem a reward
    #[serde(
        rename = "redeemRules",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub redeem_rules: Option<Vec<String>>,
    /// object
    #[serde(
        rename = "rewardConfigs",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reward_configs: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs>,
    /// Rule to define the reward
    #[serde(rename = "rule", skip_serializing_if = "Option::is_none", default)]
    pub rule: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRule>,
    /// Start date of attribution of the reward
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none", default)]
    pub start_date: Option<time::OffsetDateTime>,
    /// Id of the selected balance while redeeming / attributing a reward
    #[serde(
        rename = "subtractBalanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subtract_balance_definition_id: Option<String>,
    /// Strategy of the Balance while redeeming / attributing a reward
    #[serde(
        rename = "subtractBalanceStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subtract_balance_strategy: Option<String>,
    /// Amount of balance to be selected while redeeming / attributing a reward
    #[serde(
        rename = "subtractBalanceValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subtract_balance_value: Option<i64>,
    /// Value to indicate to subtract full balance or not
    #[serde(
        rename = "subtractTotalBalance",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub subtract_total_balance: Option<bool>,
    /// Defines the limit to which a consumer can attribute a reward
    #[serde(
        rename = "totalAttribution",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub total_attribution: Option<i64>,
    /// Defines the limit to which a consumer can redeem a reward
    #[serde(
        rename = "totalRedeem",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub total_redeem: Option<i64>,
    /// Id of the Rule to be updated for that reward
    #[serde(rename = "triggerId", skip_serializing_if = "Option::is_none", default)]
    pub trigger_id: Option<String>,
    /// Selected unit of the balance
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<String>,
    /// Timestamp for when this reward was last updated.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
    /// Value of metric in selected config for reward definition
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
    /// Type of config selected for reward definition
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none", default)]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyOfferProgramsPidVouchersSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetLoyaltyOfferProgramsPidVouchersSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetLoyaltyOfferProgramsPidVouchersSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyOfferProgramsPidVouchersSortField {
    #[serde(rename = "updatedAt")]
    UpdatedAt,
    #[serde(rename = "createdAt")]
    CreatedAt,
}
impl Default for GetLoyaltyOfferProgramsPidVouchersSortField {
    fn default() -> Self {
        Self::UpdatedAt
    }
}
impl core::fmt::Display for GetLoyaltyOfferProgramsPidVouchersSortField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UpdatedAt => write!(f, "updatedAt"),
            Self::CreatedAt => write!(f, "createdAt"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem {
    /// Generated code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,
    /// Timestamp when the reward limit was consumed
    #[serde(
        rename = "consumedAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub consumed_at: Option<String>,
    /// Timestamp when the reward limit was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Reward expiration date
    #[serde(
        rename = "expirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<String>,
    /// Unique identifier for the contact
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Additional data for the reward
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
    /// Unit of the balance selected in the reward definition
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<String>,
    /// Timestamp when the reward limit was updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
    /// Value of the selected reward config
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<f64>,
    /// Date from which the voucher becomes valid
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none", default)]
    pub valid_from: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyOfferProgramsPidVouchersResponse {
    /// Contact id associated with the current reward
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    /// List of all the rewards for current contact
    #[serde(
        rename = "contactRewards",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_rewards: Option<Vec<GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem>>,
    /// Count of the rewards associated with the current contact
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    /// Loyalty Program Id for the contact
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// Loyalty Subscription Id for the contact
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_subscription_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSubscriptionToTierResponse {
    /// Contact to which the tier is assigned
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none", default)]
    pub contact_id: Option<i64>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<time::OffsetDateTime>,
    /// Group Id to which the tier is associated
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none", default)]
    pub group_id: Option<String>,
    /// Unique identifier for the assigned tier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Associated loyalty program Id
    #[serde(
        rename = "loyaltyProgramId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub loyalty_program_id: Option<String>,
    /// object
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetListOfTierGroupsVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetListOfTierGroupsVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetListOfTierGroupsVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetListOfTierGroupsResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

/// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestDowngradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for CreateTierGroupRequestDowngradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for CreateTierGroupRequestDowngradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

/// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestUpgradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for CreateTierGroupRequestUpgradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for CreateTierGroupRequestUpgradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

/// Duration unit for the schedule.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestUpgradeScheduleDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for CreateTierGroupRequestUpgradeScheduleDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for CreateTierGroupRequestUpgradeScheduleDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Modifier for the duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestUpgradeScheduleDurationModifier {
    #[serde(rename = "start_of_period")]
    StartOfPeriod,
    #[serde(rename = "end_of_period")]
    EndOfPeriod,
}
impl Default for CreateTierGroupRequestUpgradeScheduleDurationModifier {
    fn default() -> Self {
        Self::StartOfPeriod
    }
}
impl core::fmt::Display for CreateTierGroupRequestUpgradeScheduleDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "start_of_period"),
            Self::EndOfPeriod => write!(f, "end_of_period"),
        }
    }
}

/// Schedule configuration for tier upgrades. Required when upgradeStrategy is set to a schedule-based strategy.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierGroupRequestUpgradeSchedule {
    /// Duration value for the schedule.
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<CreateTierGroupRequestUpgradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(
        rename = "durationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_modifier: Option<CreateTierGroupRequestUpgradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(
        rename = "scheduledDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_date: Option<String>,
}

/// Duration unit for the schedule.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestDowngradeScheduleDurationUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl Default for CreateTierGroupRequestDowngradeScheduleDurationUnit {
    fn default() -> Self {
        Self::Day
    }
}
impl core::fmt::Display for CreateTierGroupRequestDowngradeScheduleDurationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}

/// Modifier for the duration.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateTierGroupRequestDowngradeScheduleDurationModifier {
    #[serde(rename = "start_of_period")]
    StartOfPeriod,
    #[serde(rename = "end_of_period")]
    EndOfPeriod,
}
impl Default for CreateTierGroupRequestDowngradeScheduleDurationModifier {
    fn default() -> Self {
        Self::StartOfPeriod
    }
}
impl core::fmt::Display for CreateTierGroupRequestDowngradeScheduleDurationModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "start_of_period"),
            Self::EndOfPeriod => write!(f, "end_of_period"),
        }
    }
}

/// Schedule configuration for tier downgrades. Required when downgradeStrategy is set to a schedule-based strategy.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierGroupRequestDowngradeSchedule {
    /// Duration value for the schedule.
    #[serde(
        rename = "durationValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(
        rename = "durationUnit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_unit: Option<CreateTierGroupRequestDowngradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(
        rename = "durationModifier",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration_modifier: Option<CreateTierGroupRequestDowngradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(
        rename = "scheduledDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierGroupRequest {
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(
        rename = "downgradeStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub downgrade_strategy: Option<CreateTierGroupRequestDowngradeStrategy>,
    /// Name of the tier group
    #[serde(rename = "name")]
    pub name: String,
    /// Additional metadata for the tier group.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder", skip_serializing_if = "Option::is_none", default)]
    pub tier_order: Option<Vec<String>>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(
        rename = "upgradeStrategy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub upgrade_strategy: Option<CreateTierGroupRequestUpgradeStrategy>,
    /// Schedule configuration for tier upgrades. Required when upgradeStrategy is set to a schedule-based strategy.
    #[serde(
        rename = "upgradeSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub upgrade_schedule: Option<CreateTierGroupRequestUpgradeSchedule>,
    /// Schedule configuration for tier downgrades. Required when downgradeStrategy is set to a schedule-based strategy.
    #[serde(
        rename = "downgradeSchedule",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub downgrade_schedule: Option<CreateTierGroupRequestDowngradeSchedule>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetTierGroupVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetTierGroupVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetTierGroupVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateTierGroupRequestDowngradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for UpdateTierGroupRequestDowngradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for UpdateTierGroupRequestDowngradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

/// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateTierGroupRequestUpgradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}
impl Default for UpdateTierGroupRequestUpgradeStrategy {
    fn default() -> Self {
        Self::RealTime
    }
}
impl core::fmt::Display for UpdateTierGroupRequestUpgradeStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RealTime => write!(f, "real_time"),
            Self::MembershipAnniversary => write!(f, "membership_anniversary"),
            Self::TierAnniversary => write!(f, "tier_anniversary"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTierGroupRequest {
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(rename = "downgradeStrategy")]
    pub downgrade_strategy: UpdateTierGroupRequestDowngradeStrategy,
    /// Name of the tier group
    #[serde(rename = "name")]
    pub name: String,
    /// Additional metadata for the tier group.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder")]
    pub tier_order: Vec<String>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(rename = "upgradeStrategy")]
    pub upgrade_strategy: UpdateTierGroupRequestUpgradeStrategy,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierForTierGroupRequestAccessConditionsItem {
    /// Balance definition identifier for accessing the tier
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Minimum value required to access the tier
    #[serde(
        rename = "minimumValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub minimum_value: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierForTierGroupRequestTierRewardsItem {
    /// Reward unique identifier
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTierForTierGroupRequest {
    #[serde(rename = "accessConditions")]
    pub access_conditions: Vec<CreateTierForTierGroupRequestAccessConditionsItem>,
    /// Image of the tier
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Name of the tier to be created
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "tierRewards",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub tier_rewards: Option<Vec<CreateTierForTierGroupRequestTierRewardsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetLoyaltyProgramTierVersion {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "draft")]
    Draft,
}
impl Default for GetLoyaltyProgramTierVersion {
    fn default() -> Self {
        Self::Active
    }
}
impl core::fmt::Display for GetLoyaltyProgramTierVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyaltyProgramTierResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTierRequestAccessConditionsItem {
    /// Balance definition identifier for accessing the tier
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub balance_definition_id: Option<String>,
    /// Minimum value required to access the tier
    #[serde(
        rename = "minimumValue",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub minimum_value: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTierRequestTierRewardsItem {
    /// Reward unique identifier
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none", default)]
    pub reward_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTierRequest {
    #[serde(rename = "accessConditions")]
    pub access_conditions: Vec<UpdateTierRequestAccessConditionsItem>,
    /// Image of the tier
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none", default)]
    pub image_ref: Option<String>,
    /// Name of the tier to be created
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tierRewards")]
    pub tier_rewards: Vec<UpdateTierRequestTierRewardsItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailCampaignsType {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "trigger")]
    Trigger,
}
impl Default for GetEmailCampaignsType {
    fn default() -> Self {
        Self::Classic
    }
}
impl core::fmt::Display for GetEmailCampaignsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Classic => write!(f, "classic"),
            Self::Trigger => write!(f, "trigger"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailCampaignsStatus {
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "inProcess")]
    InProcess,
    #[serde(rename = "inReview")]
    InReview,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl Default for GetEmailCampaignsStatus {
    fn default() -> Self {
        Self::Suspended
    }
}
impl core::fmt::Display for GetEmailCampaignsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Suspended => write!(f, "suspended"),
            Self::Archive => write!(f, "archive"),
            Self::Sent => write!(f, "sent"),
            Self::Queued => write!(f, "queued"),
            Self::Draft => write!(f, "draft"),
            Self::InProcess => write!(f, "inProcess"),
            Self::InReview => write!(f, "inReview"),
            Self::Cancelling => write!(f, "cancelling"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailCampaignsStatistics {
    #[serde(rename = "globalStats")]
    GlobalStats,
    #[serde(rename = "linksStats")]
    LinksStats,
    #[serde(rename = "statsByDomain")]
    StatsByDomain,
}
impl Default for GetEmailCampaignsStatistics {
    fn default() -> Self {
        Self::GlobalStats
    }
}
impl core::fmt::Display for GetEmailCampaignsStatistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GlobalStats => write!(f, "globalStats"),
            Self::LinksStats => write!(f, "linksStats"),
            Self::StatsByDomain => write!(f, "statsByDomain"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailCampaignsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetEmailCampaignsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetEmailCampaignsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmailCampaignsResponseCampaignsItem {
    #[serde(rename = "recipients")]
    pub recipients: serde_json::Value,
    /// Link to share the campaign on social medias
    #[serde(rename = "shareLink", skip_serializing_if = "Option::is_none", default)]
    pub share_link: Option<String>,
    #[serde(rename = "statistics")]
    pub statistics: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmailCampaignsResponse {
    #[serde(rename = "campaigns", skip_serializing_if = "Option::is_none", default)]
    pub campaigns: Option<Vec<GetEmailCampaignsResponseCampaignsItem>>,
    /// Number of Email campaigns retrieved
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

/// unit of the duration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateEmailCampaignRequestEmailExpirationDateUnit {
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}
impl Default for CreateEmailCampaignRequestEmailExpirationDateUnit {
    fn default() -> Self {
        Self::Days
    }
}
impl core::fmt::Display for CreateEmailCampaignRequestEmailExpirationDateUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Days => write!(f, "days"),
            Self::Weeks => write!(f, "weeks"),
            Self::Months => write!(f, "months"),
        }
    }
}

/// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy. Learn more about setting an email expiration date. For reference , ``<https://help.brevo.com/hc/en-us/articles/4413566705298-Create-an-email-campaign``>
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailCampaignRequestEmailExpirationDate {
    /// Duration of the email expiry. maximum duration can be 3600 days or 480 weeks or 120 months.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<i64>,
    /// unit of the duration
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<CreateEmailCampaignRequestEmailExpirationDateUnit>,
}

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailCampaignRequestRecipients {
    /// List ids to exclude from the campaign
    #[serde(
        rename = "exclusionListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Segment ids which have to be excluded from a campaign.
    #[serde(
        rename = "exclusionSegmentIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_segment_ids: Option<Vec<i64>>,
    /// **Mandatory if scheduledAt is not empty**. List Ids to send the campaign to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used**. Segment ids to send the campaign to.
    #[serde(
        rename = "segmentIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub segment_ids: Option<Vec<i64>>,
}

/// Sender details including id or email and name (optional). Only one of either Sender’s email or Sender’s ID shall be passed in one request at a time. Passing both `email` and `id` will result in an error. For example: **{"name":"xyz", "email":"example@abc.com"}** or **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailCampaignRequestSender {
    /// Sender email. Must be a valid email address. Cannot be used together with `id` in the same request.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Select the sender for the campaign on the basis of sender id. Cannot be used together with `email` in the same request. _In order to select a sender with a specific pool of IPs, dedicated IP users shall pass id (instead of email)._
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Sender Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateEmailCampaignRequestWinnerCriteria {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "click")]
    Click,
}
impl Default for CreateEmailCampaignRequestWinnerCriteria {
    fn default() -> Self {
        Self::Open
    }
}
impl core::fmt::Display for CreateEmailCampaignRequestWinnerCriteria {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Click => write!(f, "click"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailCampaignRequest {
    /// Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    #[serde(rename = "abTesting", skip_serializing_if = "Option::is_none", default)]
    pub ab_testing: Option<bool>,
    /// Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(
        rename = "attachmentUrl",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_url: Option<String>,
    /// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy. Learn more about setting an email expiration date. For reference , ``<https://help.brevo.com/hc/en-us/articles/4413566705298-Create-an-email-campaign``>
    #[serde(
        rename = "emailExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_expiration_date: Option<CreateEmailCampaignRequestEmailExpirationDate>,
    /// Footer of the email campaign
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none", default)]
    pub footer: Option<String>,
    /// Header of the email campaign
    #[serde(rename = "header", skip_serializing_if = "Option::is_none", default)]
    pub header: Option<String>,
    /// **Mandatory if htmlUrl and templateId are empty.** Body of the message (HTML). Must have more than 10 characters and be less than 1MB in size. Cannot be used together with `htmlUrl` or `templateId`.
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// **Mandatory if htmlContent and templateId are empty.** URL to the message (HTML). Cannot be used together with `htmlContent` or `templateId`. For example: **<https://html.domain.com**>
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none", default)]
    pub html_url: Option<String>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    #[serde(
        rename = "increaseRate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub increase_rate: Option<i64>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    #[serde(
        rename = "initialQuota",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub initial_quota: Option<i64>,
    /// Use true to embedded the images in your email. Final size of the email should be less than **4MB**. Campaigns with embedded images can _not be sent to more than 5000 contacts_
    #[serde(
        rename = "inlineImageActivation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub inline_image_activation: Option<bool>,
    /// **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    #[serde(
        rename = "ipWarmupEnable",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ip_warmup_enable: Option<bool>,
    /// Use true to enable the mirror link
    #[serde(
        rename = "mirrorActive",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mirror_active: Option<bool>,
    /// Name of the campaign
    #[serde(rename = "name")]
    pub name: String,
    /// Pass the set of attributes to customize the type classic campaign. For example: **{"FNAME":"Joe", "LNAME":"Doe"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    /// Preview text or preheader of the email campaign
    #[serde(
        rename = "previewText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_text: Option<String>,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(
        rename = "recipients",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub recipients: Option<CreateEmailCampaignRequestRecipients>,
    /// Email on which the campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<crate::types::Email>,
    /// Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result**. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part). For example: **2017-06-01T12:30:00+02:00**
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Set this to true if you want to send your campaign at best time.
    #[serde(
        rename = "sendAtBestTime",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub send_at_best_time: Option<bool>,
    /// Sender details including id or email and name (optional). Only one of either Sender’s email or Sender’s ID shall be passed in one request at a time. Passing both `email` and `id` will result in an error. For example: **{"name":"xyz", "email":"example@abc.com"}** or **{"name":"xyz", "id":123}**
    #[serde(rename = "sender")]
    pub sender: CreateEmailCampaignRequestSender,
    /// Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    #[serde(rename = "splitRule", skip_serializing_if = "Option::is_none", default)]
    pub split_rule: Option<i64>,
    /// Subject of the campaign. **Mandatory if abTesting is false**. Ignored if abTesting is true.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectA", skip_serializing_if = "Option::is_none", default)]
    pub subject_a: Option<String>,
    /// Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectB", skip_serializing_if = "Option::is_none", default)]
    pub subject_b: Option<String>,
    /// Tag of the campaign
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// **Mandatory if htmlContent and htmlUrl are empty.** Id of the transactional email template with status _active_. Used to copy only its content fetched from htmlContent/htmlUrl to an email campaign for RSS feature. Cannot be used together with `htmlContent` or `htmlUrl`.
    #[serde(
        rename = "templateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_id: Option<i64>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none", default)]
    pub to_field: Option<String>,
    /// Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page. If not entered, then the default unsubscription page will be used.
    #[serde(
        rename = "unsubscriptionPageId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscription_page_id: Option<String>,
    /// **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form. If not entered, then the default update profile form will be used.
    #[serde(
        rename = "updateFormId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_form_id: Option<String>,
    /// Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    #[serde(
        rename = "utmCampaign",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_campaign: Option<String>,
    /// Customize the utm_content value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(
        rename = "utmContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_content: Option<String>,
    /// Customize the utm_term value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmTerm", skip_serializing_if = "Option::is_none", default)]
    pub utm_term: Option<String>,
    /// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    #[serde(
        rename = "winnerCriteria",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_criteria: Option<CreateEmailCampaignRequestWinnerCriteria>,
    /// Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    #[serde(
        rename = "winnerDelay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_delay: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageToGalleryRequest {
    /// The absolute url of the image (**no local file**). Maximum allowed size for image is **2MB**. Allowed extensions for images are: #### jpeg, jpg, png, bmp, gif.
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    /// Name shown in your Brevo image gallery. Include the file extension, e.g. product-banner.png
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageToGalleryResponse {
    /// URL of the image uploaded
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailCampaignStatistics {
    #[serde(rename = "globalStats")]
    GlobalStats,
    #[serde(rename = "linksStats")]
    LinksStats,
    #[serde(rename = "statsByDomain")]
    StatsByDomain,
    #[serde(rename = "statsByDevice")]
    StatsByDevice,
    #[serde(rename = "statsByBrowser")]
    StatsByBrowser,
}
impl Default for GetEmailCampaignStatistics {
    fn default() -> Self {
        Self::GlobalStats
    }
}
impl core::fmt::Display for GetEmailCampaignStatistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GlobalStats => write!(f, "globalStats"),
            Self::LinksStats => write!(f, "linksStats"),
            Self::StatsByDomain => write!(f, "statsByDomain"),
            Self::StatsByDevice => write!(f, "statsByDevice"),
            Self::StatsByBrowser => write!(f, "statsByBrowser"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmailCampaignResponse {
    #[serde(rename = "recipients")]
    pub recipients: serde_json::Value,
    #[serde(rename = "statistics")]
    pub statistics: serde_json::Value,
}

/// unit of the duration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateEmailCampaignRequestEmailExpirationDateUnit {
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}
impl Default for UpdateEmailCampaignRequestEmailExpirationDateUnit {
    fn default() -> Self {
        Self::Days
    }
}
impl core::fmt::Display for UpdateEmailCampaignRequestEmailExpirationDateUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Days => write!(f, "days"),
            Self::Weeks => write!(f, "weeks"),
            Self::Months => write!(f, "months"),
        }
    }
}

/// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailCampaignRequestEmailExpirationDate {
    /// Duration of the email expiry. maximum duration can be 3600 days or 480 weeks or 120 months.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<i64>,
    /// unit of the duration
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none", default)]
    pub unit: Option<UpdateEmailCampaignRequestEmailExpirationDateUnit>,
}

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailCampaignRequestRecipients {
    /// List ids which have to be excluded from a campaign
    #[serde(
        rename = "exclusionListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Segment ids which have to be excluded from a campaign.
    #[serde(
        rename = "exclusionSegmentIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_segment_ids: Option<Vec<i64>>,
    /// Lists Ids to send the campaign to. **Campaign should only be updated with listIds if listIds were used to create it. REQUIRED if already not present in campaign and scheduledAt is not empty**
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used. Campaign should only be updated with segmentIds if segmentIds were used to create it.** Segment ids to send the campaign to.
    #[serde(
        rename = "segmentIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub segment_ids: Option<Vec<i64>>,
}

/// Sender details including id or email and name (optional). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailCampaignRequestSender {
    /// Sender email from which the campaign emails are sent
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Select the sender for the campaign on the basis of sender id. **In order to select a sender with specific pool of IP’s, dedicated ip users shall pass id (instead of email)**.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Sender Name from which the campaign emails are sent
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateEmailCampaignRequestWinnerCriteria {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "click")]
    Click,
}
impl Default for UpdateEmailCampaignRequestWinnerCriteria {
    fn default() -> Self {
        Self::Open
    }
}
impl core::fmt::Display for UpdateEmailCampaignRequestWinnerCriteria {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Click => write!(f, "click"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailCampaignRequest {
    /// Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    #[serde(rename = "abTesting", skip_serializing_if = "Option::is_none", default)]
    pub ab_testing: Option<bool>,
    /// Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps'
    #[serde(
        rename = "attachmentUrl",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_url: Option<String>,
    /// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy.
    #[serde(
        rename = "emailExpirationDate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_expiration_date: Option<UpdateEmailCampaignRequestEmailExpirationDate>,
    /// Footer of the email campaign
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none", default)]
    pub footer: Option<String>,
    /// Header of the email campaign
    #[serde(rename = "header", skip_serializing_if = "Option::is_none", default)]
    pub header: Option<String>,
    /// Body of the message (HTML version). If the campaign is designed using Drag & Drop editor via HTML content, then the design page will not have Drag & Drop editor access for that campaign. **REQUIRED if htmlUrl is empty**
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// Url which contents the body of the email message. **REQUIRED if htmlContent is empty**
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none", default)]
    pub html_url: Option<String>,
    /// Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    #[serde(
        rename = "increaseRate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub increase_rate: Option<i64>,
    /// Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    #[serde(
        rename = "initialQuota",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub initial_quota: Option<i64>,
    /// Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email. You cannot send a campaign of more than **4MB** with images embedded in the email. Campaigns with the images embedded in the email _must be sent to less than 5000 contacts_.
    #[serde(
        rename = "inlineImageActivation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub inline_image_activation: Option<bool>,
    /// **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    #[serde(
        rename = "ipWarmupEnable",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ip_warmup_enable: Option<bool>,
    /// Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign
    #[serde(
        rename = "mirrorActive",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mirror_active: Option<bool>,
    /// Name of the campaign
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Pass the set of attributes to customize the type classic campaign. For example: **{"FNAME":"Joe", "LNAME":"Doe"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    /// Preview text or preheader of the email campaign
    #[serde(
        rename = "previewText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_text: Option<String>,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(
        rename = "recipients",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub recipients: Option<UpdateEmailCampaignRequestRecipients>,
    /// **FOR TRIGGER ONLY !** Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none", default)]
    pub recurring: Option<bool>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<crate::types::Email>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.** If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part).
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Set this to true if you want to send your campaign at best time. Note:- **if true, warmup ip will be disabled.**
    #[serde(
        rename = "sendAtBestTime",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub send_at_best_time: Option<bool>,
    /// Sender details including id or email and name (optional). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<UpdateEmailCampaignRequestSender>,
    /// Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    #[serde(rename = "splitRule", skip_serializing_if = "Option::is_none", default)]
    pub split_rule: Option<i64>,
    /// Subject of the campaign
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectA", skip_serializing_if = "Option::is_none", default)]
    pub subject_a: Option<String>,
    /// Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectB", skip_serializing_if = "Option::is_none", default)]
    pub subject_b: Option<String>,
    /// Tag of the campaign
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none", default)]
    pub to_field: Option<String>,
    /// Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page.
    #[serde(
        rename = "unsubscriptionPageId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscription_page_id: Option<String>,
    /// **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form.
    #[serde(
        rename = "updateFormId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub update_form_id: Option<String>,
    /// Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    #[serde(
        rename = "utmCampaign",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_campaign: Option<String>,
    /// Customize the utm_content value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(
        rename = "utmContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub utm_content: Option<String>,
    /// Customize the utm_term value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmTerm", skip_serializing_if = "Option::is_none", default)]
    pub utm_term: Option<String>,
    /// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    #[serde(
        rename = "winnerCriteria",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_criteria: Option<UpdateEmailCampaignRequestWinnerCriteria>,
    /// Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    #[serde(
        rename = "winnerDelay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winner_delay: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAbTestCampaignResultResponseClickedLinks {
    #[serde(rename = "Version A")]
    pub version_a: serde_json::Value,
    #[serde(rename = "Version B")]
    pub version_b: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAbTestCampaignResultResponseStatistics {
    #[serde(rename = "clicks")]
    pub clicks: serde_json::Value,
    #[serde(rename = "complaints")]
    pub complaints: serde_json::Value,
    #[serde(rename = "hardBounces")]
    pub hard_bounces: serde_json::Value,
    #[serde(rename = "openers")]
    pub openers: serde_json::Value,
    #[serde(rename = "softBounces")]
    pub soft_bounces: serde_json::Value,
    #[serde(rename = "unsubscribed")]
    pub unsubscribed: serde_json::Value,
}

/// Criteria chosen for winning version (Open/Click)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAbTestCampaignResultResponseWinningCriteria {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Click")]
    Click,
}
impl Default for GetAbTestCampaignResultResponseWinningCriteria {
    fn default() -> Self {
        Self::Open
    }
}
impl core::fmt::Display for GetAbTestCampaignResultResponseWinningCriteria {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Click => write!(f, "Click"),
        }
    }
}

/// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetAbTestCampaignResultResponseWinningVersion {
    #[serde(rename = "notAvailable")]
    NotAvailable,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "tie")]
    Tie,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
}
impl Default for GetAbTestCampaignResultResponseWinningVersion {
    fn default() -> Self {
        Self::NotAvailable
    }
}
impl core::fmt::Display for GetAbTestCampaignResultResponseWinningVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAvailable => write!(f, "notAvailable"),
            Self::Pending => write!(f, "pending"),
            Self::Tie => write!(f, "tie"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAbTestCampaignResultResponse {
    /// Click rate for current winning version
    #[serde(rename = "clickRate", skip_serializing_if = "Option::is_none", default)]
    pub click_rate: Option<String>,
    #[serde(
        rename = "clickedLinks",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub clicked_links: Option<GetAbTestCampaignResultResponseClickedLinks>,
    /// Open rate for current winning version
    #[serde(rename = "openRate", skip_serializing_if = "Option::is_none", default)]
    pub open_rate: Option<String>,
    #[serde(
        rename = "statistics",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub statistics: Option<GetAbTestCampaignResultResponseStatistics>,
    /// Criteria chosen for winning version (Open/Click)
    #[serde(
        rename = "winningCriteria",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winning_criteria: Option<GetAbTestCampaignResultResponseWinningCriteria>,
    /// Subject Line of current winning version
    #[serde(
        rename = "winningSubjectLine",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winning_subject_line: Option<String>,
    /// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
    #[serde(
        rename = "winningVersion",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winning_version: Option<GetAbTestCampaignResultResponseWinningVersion>,
    /// Open/Click rate for the winner version
    #[serde(
        rename = "winningVersionRate",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub winning_version_rate: Option<String>,
}

/// Type of recipients to export for a campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EmailExportRecipientsRequestRecipientsType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "nonClickers")]
    NonClickers,
    #[serde(rename = "nonOpeners")]
    NonOpeners,
    #[serde(rename = "clickers")]
    Clickers,
    #[serde(rename = "openers")]
    Openers,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl Default for EmailExportRecipientsRequestRecipientsType {
    fn default() -> Self {
        Self::All
    }
}
impl core::fmt::Display for EmailExportRecipientsRequestRecipientsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::NonClickers => write!(f, "nonClickers"),
            Self::NonOpeners => write!(f, "nonOpeners"),
            Self::Clickers => write!(f, "clickers"),
            Self::Openers => write!(f, "openers"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailExportRecipientsRequest {
    /// Webhook called once the export process is finished. For reference, <https://help.brevo.com/hc/en-us/articles/360007666479>
    #[serde(rename = "notifyURL", skip_serializing_if = "Option::is_none", default)]
    pub notify_url: Option<String>,
    /// Type of recipients to export for a campaign
    #[serde(rename = "recipientsType")]
    pub recipients_type: EmailExportRecipientsRequestRecipientsType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedTemplateUrlResponse {
    /// A unique URL for the email campaign or transactional template. This URL can be shared with other Brevo users.
    #[serde(rename = "sharedUrl")]
    pub shared_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsCampaignsStatus {
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "inProcess")]
    InProcess,
}
impl Default for GetSmsCampaignsStatus {
    fn default() -> Self {
        Self::Suspended
    }
}
impl core::fmt::Display for GetSmsCampaignsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Suspended => write!(f, "suspended"),
            Self::Archive => write!(f, "archive"),
            Self::Sent => write!(f, "sent"),
            Self::Queued => write!(f, "queued"),
            Self::Draft => write!(f, "draft"),
            Self::InProcess => write!(f, "inProcess"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsCampaignsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSmsCampaignsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSmsCampaignsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignsResponseCampaignsItem {
    #[serde(rename = "recipients")]
    pub recipients: serde_json::Value,
    #[serde(rename = "statistics")]
    pub statistics: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignsResponse {
    #[serde(rename = "campaigns", skip_serializing_if = "Option::is_none", default)]
    pub campaigns: Option<Vec<GetSmsCampaignsResponseCampaignsItem>>,
    /// Number of SMS campaigns retrieved
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSmsCampaignRequestRecipients {
    /// List ids which have to be excluded from a campaign
    #[serde(
        rename = "exclusionListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Lists Ids to send the campaign to. **REQUIRED if scheduledAt is not empty**
    #[serde(rename = "listIds")]
    pub list_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSmsCampaignRequest {
    /// Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS.
    #[serde(rename = "content")]
    pub content: String,
    /// Name of the campaign
    #[serde(rename = "name")]
    pub name: String,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "organisationPrefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation_prefix: Option<String>,
    #[serde(
        rename = "recipients",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub recipients: Option<CreateSmsCampaignRequestRecipients>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    #[serde(rename = "sender")]
    pub sender: String,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(
        rename = "unicodeEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unicode_enabled: Option<bool>,
    /// Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "unsubscribeInstruction",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribe_instruction: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignResponseRecipientsExclusionListsItem {
    /// ID of the exclusion list
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Name of the exclusion list
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignResponseRecipientsListsItem {
    /// ID of the list
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Name of the list
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Recipients of the SMS campaign. For a single campaign, lists and exclusion lists are returned as objects with id and name.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignResponseRecipients {
    #[serde(
        rename = "exclusionLists",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_lists: Option<Vec<GetSmsCampaignResponseRecipientsExclusionListsItem>>,
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none", default)]
    pub lists: Option<Vec<GetSmsCampaignResponseRecipientsListsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsCampaignResponse {
    /// Recipients of the SMS campaign. For a single campaign, lists and exclusion lists are returned as objects with id and name.
    #[serde(rename = "recipients")]
    pub recipients: GetSmsCampaignResponseRecipients,
    #[serde(rename = "statistics")]
    pub statistics: serde_json::Value,
    /// Tags (labels) associated with the SMS campaign. Only available when retrieving a single campaign.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmsCampaignRequestRecipients {
    /// List ids which have to be excluded from a campaign
    #[serde(
        rename = "exclusionListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Lists Ids to send the campaign to. **REQUIRED if scheduledAt is not empty**
    #[serde(rename = "listIds")]
    pub list_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmsCampaignRequest {
    /// Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS
    #[serde(rename = "content", skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    /// Name of the campaign
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "organisationPrefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation_prefix: Option<String>,
    #[serde(
        rename = "recipients",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub recipients: Option<UpdateSmsCampaignRequestRecipients>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<String>,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(
        rename = "unicodeEnabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unicode_enabled: Option<bool>,
    /// Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "unsubscribeInstruction",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribe_instruction: Option<String>,
}

/// Filter the recipients based on how they interacted with the campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RequestSmsRecipientExportRequestRecipientsType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl Default for RequestSmsRecipientExportRequestRecipientsType {
    fn default() -> Self {
        Self::All
    }
}
impl core::fmt::Display for RequestSmsRecipientExportRequestRecipientsType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Delivered => write!(f, "delivered"),
            Self::Answered => write!(f, "answered"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSmsRecipientExportRequest {
    /// URL that will be called once the export process is finished. For reference, <https://help.brevo.com/hc/en-us/articles/360007666479>
    #[serde(rename = "notifyURL", skip_serializing_if = "Option::is_none", default)]
    pub notify_url: Option<String>,
    /// Filter the recipients based on how they interacted with the campaign
    #[serde(rename = "recipientsType")]
    pub recipients_type: RequestSmsRecipientExportRequestRecipientsType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTestSmsRequest {
    /// Mobile number of the recipient with the country code. This number **must belong to one of your contacts in Brevo account and must not be blacklisted**
    #[serde(
        rename = "phoneNumber",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppCampaignsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetWhatsAppCampaignsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetWhatsAppCampaignsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Status of the Whatsapp Campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "sent")]
    Sent,
}
impl Default for GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Running => write!(f, "running"),
            Self::Suspended => write!(f, "suspended"),
            Self::Rejected => write!(f, "rejected"),
            Self::Sent => write!(f, "sent"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignsResponseCampaignsItem {
    /// Name of the WhatsApp Campaign
    #[serde(rename = "campaignName")]
    pub campaign_name: String,
    /// Status of the Whatsapp Campaign
    #[serde(rename = "campaignStatus")]
    pub campaign_status: GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus,
    /// Creation UTC date-time of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Error Reason associated with the WhatsApp campaign sending
    #[serde(
        rename = "errorReason",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub error_reason: Option<String>,
    /// ID of the WhatsApp Campaign
    #[serde(rename = "id")]
    pub id: i64,
    /// Count of invalidated contacts
    #[serde(
        rename = "invalidatedContacts",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub invalidated_contacts: Option<i64>,
    /// UTC date-time of last modification of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Read percentage of the the whatsapp campaign created
    #[serde(
        rename = "readPercentage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub read_percentage: Option<f32>,
    /// UTC date-time on which WhatsApp campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: String,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none", default)]
    pub stats: Option<serde_json::Value>,
    /// Id of the WhatsApp template
    #[serde(rename = "templateId")]
    pub template_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignsResponse {
    #[serde(rename = "campaigns", skip_serializing_if = "Option::is_none", default)]
    pub campaigns: Option<Vec<GetWhatsAppCampaignsResponseCampaignsItem>>,
    /// Number of WhatsApp campaigns retrieved
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWhatsAppCampaignRequestRecipients {
    /// List ids to exclude from the campaign
    #[serde(
        rename = "excludedListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub excluded_list_ids: Option<Vec<i64>>,
    /// **Mandatory if scheduledAt is not empty**. List Ids to send the campaign to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used**. Segment ids to send the campaign to.
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none", default)]
    pub segments: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWhatsAppCampaignRequest {
    /// Name of the WhatsApp campaign creation
    #[serde(rename = "name")]
    pub name: String,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(rename = "recipients")]
    pub recipients: CreateWhatsAppCampaignRequestRecipients,
    /// Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00**
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: String,
    /// Id of the WhatsApp template in **approved** state
    #[serde(rename = "templateId")]
    pub template_id: i64,
}

/// Status of the name associated with WhatsApp Phone number
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppConfigResponsePhoneNumberNameStatus {
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "REJECTED")]
    REJECTED,
}
impl Default for GetWhatsAppConfigResponsePhoneNumberNameStatus {
    fn default() -> Self {
        Self::APPROVED
    }
}
impl core::fmt::Display for GetWhatsAppConfigResponsePhoneNumberNameStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::APPROVED => write!(f, "APPROVED"),
            Self::PENDING => write!(f, "PENDING"),
            Self::REJECTED => write!(f, "REJECTED"),
        }
    }
}

/// Quality status of phone number associated with WhatsApp account. There are three quality ratings. example - **High (GREEN) , Medium (YELLOW) and Low(RED)**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppConfigResponsePhoneNumberQuality {
    #[serde(rename = "GREEN")]
    GREEN,
    #[serde(rename = "YELLOW")]
    YELLOW,
    #[serde(rename = "RED")]
    RED,
}
impl Default for GetWhatsAppConfigResponsePhoneNumberQuality {
    fn default() -> Self {
        Self::GREEN
    }
}
impl core::fmt::Display for GetWhatsAppConfigResponsePhoneNumberQuality {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GREEN => write!(f, "GREEN"),
            Self::YELLOW => write!(f, "YELLOW"),
            Self::RED => write!(f, "RED"),
        }
    }
}

/// Status information related to WhatsApp Api account
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppConfigResponseWhatsappBusinessAccountStatus {
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "REJECTED")]
    REJECTED,
}
impl Default for GetWhatsAppConfigResponseWhatsappBusinessAccountStatus {
    fn default() -> Self {
        Self::APPROVED
    }
}
impl core::fmt::Display for GetWhatsAppConfigResponseWhatsappBusinessAccountStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::APPROVED => write!(f, "APPROVED"),
            Self::PENDING => write!(f, "PENDING"),
            Self::REJECTED => write!(f, "REJECTED"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppConfigResponse {
    /// Verification status information of the Business account
    #[serde(
        rename = "businessStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub business_status: Option<String>,
    /// Status of the name associated with WhatsApp Phone number
    #[serde(
        rename = "phoneNumberNameStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub phone_number_name_status: Option<GetWhatsAppConfigResponsePhoneNumberNameStatus>,
    /// Quality status of phone number associated with WhatsApp account. There are three quality ratings. example - **High (GREEN) , Medium (YELLOW) and Low(RED)**
    #[serde(
        rename = "phoneNumberQuality",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub phone_number_quality: Option<GetWhatsAppConfigResponsePhoneNumberQuality>,
    /// Sending limit of the WhatsApp account
    #[serde(
        rename = "sendingLimit",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sending_limit: Option<String>,
    /// Id of the WhatsApp business account
    #[serde(
        rename = "whatsappBusinessAccountId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whatsapp_business_account_id: Option<String>,
    /// Status information related to WhatsApp Api account
    #[serde(
        rename = "whatsappBusinessAccountStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whatsapp_business_account_status:
        Option<GetWhatsAppConfigResponseWhatsappBusinessAccountStatus>,
}

/// Category of the template
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateWhatsAppTemplateRequestCategory {
    #[serde(rename = "MARKETING")]
    MARKETING,
    #[serde(rename = "UTILITY")]
    UTILITY,
}
impl Default for CreateWhatsAppTemplateRequestCategory {
    fn default() -> Self {
        Self::MARKETING
    }
}
impl core::fmt::Display for CreateWhatsAppTemplateRequestCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MARKETING => write!(f, "MARKETING"),
            Self::UTILITY => write!(f, "UTILITY"),
        }
    }
}

/// source of the template
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateWhatsAppTemplateRequestSource {
    #[serde(rename = "Automation")]
    Automation,
    #[serde(rename = "Conversations")]
    Conversations,
}
impl Default for CreateWhatsAppTemplateRequestSource {
    fn default() -> Self {
        Self::Automation
    }
}
impl core::fmt::Display for CreateWhatsAppTemplateRequestSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Automation => write!(f, "Automation"),
            Self::Conversations => write!(f, "Conversations"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWhatsAppTemplateRequest {
    /// Body of the template. **Maximum allowed characters are 1024**
    #[serde(rename = "bodyText")]
    pub body_text: String,
    /// Category of the template
    #[serde(rename = "category")]
    pub category: CreateWhatsAppTemplateRequestCategory,
    /// Text content of the header in the template.  **Maximum allowed characters are 45** **Use this field to add text content in template header and if mediaUrl is empty**
    #[serde(
        rename = "headerText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub header_text: Option<String>,
    /// Language of the template. For Example :
    /// **en** for English
    #[serde(rename = "language")]
    pub language: String,
    /// Absolute url of the media file **(no local file)** for the header. **Use this field in you want to add media in Template header and headerText is empty** Allowed extensions for media files are: #### jpeg | png | mp4 | pdf
    #[serde(rename = "mediaUrl", skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<String>,
    /// Name of the template
    #[serde(rename = "name")]
    pub name: String,
    /// source of the template
    #[serde(rename = "source", skip_serializing_if = "Option::is_none", default)]
    pub source: Option<CreateWhatsAppTemplateRequestSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppTemplatesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetWhatsAppTemplatesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetWhatsAppTemplatesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppTemplatesSource {
    #[serde(rename = "Automation")]
    Automation,
    #[serde(rename = "Conversations")]
    Conversations,
}
impl Default for GetWhatsAppTemplatesSource {
    fn default() -> Self {
        Self::Automation
    }
}
impl core::fmt::Display for GetWhatsAppTemplatesSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Automation => write!(f, "Automation"),
            Self::Conversations => write!(f, "Conversations"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppTemplatesResponseTemplatesItem {
    /// category of the template
    #[serde(rename = "category")]
    pub category: String,
    /// Creation UTC date-time of the whatsApp template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Error reason in the template creation
    #[serde(
        rename = "errorReason",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub error_reason: Option<String>,
    /// ID of the whatsApp template
    #[serde(rename = "id")]
    pub id: i64,
    /// Language in which template exists
    #[serde(rename = "language")]
    pub language: String,
    /// UTC date-time of last modification of the whatsApp template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// Name of the whatsApp template
    #[serde(rename = "name")]
    pub name: String,
    /// Status of the whatsApp template
    #[serde(rename = "status")]
    pub status: String,
    /// Type of the whatsApp template
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppTemplatesResponse {
    /// Number of whatsApp templates retrieved
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none", default)]
    pub templates: Option<Vec<GetWhatsAppTemplatesResponseTemplatesItem>>,
}

/// Status of the WhatsApp Campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsAppCampaignResponseCampaignStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "sent")]
    Sent,
}
impl Default for GetWhatsAppCampaignResponseCampaignStatus {
    fn default() -> Self {
        Self::Draft
    }
}
impl core::fmt::Display for GetWhatsAppCampaignResponseCampaignStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Running => write!(f, "running"),
            Self::Suspended => write!(f, "suspended"),
            Self::Rejected => write!(f, "rejected"),
            Self::Sent => write!(f, "sent"),
        }
    }
}

/// Recipients of the WhatsApp Campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignResponseRecipients {
    /// List of excluded list IDs
    #[serde(
        rename = "excludedLists",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub excluded_lists: Option<Vec<i64>>,
    /// List of included list IDs
    #[serde(
        rename = "includedLists",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub included_lists: Option<Vec<i64>>,
    /// List of segment IDs
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none", default)]
    pub segments: Option<Vec<i64>>,
    /// Type of recipients (list or segment)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignResponseTemplateComponentsItem {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignResponseTemplate {
    /// array of variables item variables
    #[serde(
        rename = "body_variables",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub body_variables: Option<Vec<serde_json::Value>>,
    #[serde(
        rename = "button_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub button_type: Option<String>,
    /// description of the template
    #[serde(rename = "category", skip_serializing_if = "Option::is_none", default)]
    pub category: Option<String>,
    /// array of component item objects
    #[serde(
        rename = "components",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub components: Option<Vec<GetWhatsAppCampaignResponseTemplateComponentsItem>>,
    #[serde(
        rename = "contains_button",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contains_button: Option<bool>,
    #[serde(
        rename = "display_header",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub display_header: Option<bool>,
    /// type of header
    #[serde(
        rename = "header_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub header_type: Option<String>,
    /// array of variables item object
    #[serde(
        rename = "header_variables",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub header_variables: Option<Vec<serde_json::Value>>,
    #[serde(
        rename = "hide_footer",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hide_footer: Option<bool>,
    /// language of the template
    #[serde(rename = "language", skip_serializing_if = "Option::is_none", default)]
    pub language: Option<String>,
    /// name of the template
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsAppCampaignResponse {
    /// Name of the WhatsApp Campaign
    #[serde(rename = "campaignName")]
    pub campaign_name: String,
    /// Status of the WhatsApp Campaign
    #[serde(rename = "campaignStatus")]
    pub campaign_status: GetWhatsAppCampaignResponseCampaignStatus,
    /// Creation UTC date-time of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// ID of the WhatsApp Campaign
    #[serde(rename = "id")]
    pub id: i64,
    /// UTC date-time of last modification of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// UTC date-time on which WhatsApp campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<String>,
    /// Recipients of the WhatsApp Campaign
    #[serde(rename = "recipients")]
    pub recipients: GetWhatsAppCampaignResponseRecipients,
    /// Sender of the WhatsApp Campaign
    #[serde(rename = "senderNumber")]
    pub sender_number: String,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none", default)]
    pub stats: Option<serde_json::Value>,
    #[serde(rename = "template")]
    pub template: GetWhatsAppCampaignResponseTemplate,
}

/// Status of the campaign
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateWhatsAppCampaignRequestCampaignStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "suspended")]
    Suspended,
}
impl Default for UpdateWhatsAppCampaignRequestCampaignStatus {
    fn default() -> Self {
        Self::Scheduled
    }
}
impl core::fmt::Display for UpdateWhatsAppCampaignRequestCampaignStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::Suspended => write!(f, "suspended"),
        }
    }
}

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWhatsAppCampaignRequestRecipients {
    /// List ids to exclude from the campaign
    #[serde(
        rename = "excludedListIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub excluded_list_ids: Option<Vec<i64>>,
    /// **Mandatory if scheduledAt is not empty**. List Ids to send the campaign to
    #[serde(rename = "listIds", skip_serializing_if = "Option::is_none", default)]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used**. Segment ids to send the campaign to.
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none", default)]
    pub segments: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWhatsAppCampaignRequest {
    /// Name of the campaign
    #[serde(
        rename = "campaignName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub campaign_name: Option<String>,
    /// Status of the campaign
    #[serde(
        rename = "campaignStatus",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub campaign_status: Option<UpdateWhatsAppCampaignRequestCampaignStatus>,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(
        rename = "recipients",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub recipients: Option<UpdateWhatsAppCampaignRequestRecipients>,
    /// Reschedule the sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of campaign. **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00** Use this field to update the scheduledAt of any existing draft or scheduled WhatsApp campaign.
    #[serde(
        rename = "rescheduleFor",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reschedule_for: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCompaniesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCompaniesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCompaniesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// List of companies
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompaniesResponse {
    /// List of companies
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCompaniesRequest {
    /// Attributes for company creation
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Country code if phone_number is passed in attributes.
    #[serde(
        rename = "countryCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_code: Option<i64>,
    /// Contact ids to be linked with company
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Deal ids to be linked with company
    #[serde(
        rename = "linkedDealsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_deals_ids: Option<Vec<String>>,
    /// Name of company
    #[serde(rename = "name")]
    pub name: String,
}

/// Created company id
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCompaniesResponse {
    /// Unique company id
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCompaniesImportResponse {
    /// The ID of the import process
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none", default)]
    pub process_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCompaniesLinkUnlinkIdRequest {
    /// Contact ids for contacts to be linked with company
    #[serde(
        rename = "linkContactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub link_contact_ids: Option<Vec<i64>>,
    /// Deal ids for deals to be linked with company
    #[serde(
        rename = "linkDealsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub link_deals_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be unlinked from company
    #[serde(
        rename = "unlinkContactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_contact_ids: Option<Vec<i64>>,
    /// Deal ids for deals to be unlinked from company
    #[serde(
        rename = "unlinkDealsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_deals_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCompaniesIdRequest {
    /// Attributes for company update
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Country code if phone_number is passed in attributes.
    #[serde(
        rename = "countryCode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_code: Option<i64>,
    /// Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Warning - Using PATCH on linkedDealsIds replaces the list of linked deals. Omitted IDs will be removed.
    #[serde(
        rename = "linkedDealsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_deals_ids: Option<Vec<String>>,
    /// Name of company
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// The type of attribute (must be one of the defined enums)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostCrmAttributesRequestAttributeType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "single-select")]
    SingleSelect,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "multi-choice")]
    MultiChoice,
}
impl Default for PostCrmAttributesRequestAttributeType {
    fn default() -> Self {
        Self::Text
    }
}
impl core::fmt::Display for PostCrmAttributesRequestAttributeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::User => write!(f, "user"),
            Self::Number => write!(f, "number"),
            Self::SingleSelect => write!(f, "single-select"),
            Self::Date => write!(f, "date"),
            Self::Boolean => write!(f, "boolean"),
            Self::MultiChoice => write!(f, "multi-choice"),
        }
    }
}

/// The type of object the attribute belongs to. Must be either `companies` or `deals`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PostCrmAttributesRequestObjectType {
    #[serde(rename = "companies")]
    Companies,
    #[serde(rename = "deals")]
    Deals,
}
impl Default for PostCrmAttributesRequestObjectType {
    fn default() -> Self {
        Self::Companies
    }
}
impl core::fmt::Display for PostCrmAttributesRequestObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Companies => write!(f, "companies"),
            Self::Deals => write!(f, "deals"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmAttributesRequest {
    /// The type of attribute (must be one of the defined enums)
    #[serde(rename = "attributeType")]
    pub attribute_type: PostCrmAttributesRequestAttributeType,
    /// A description of the attribute
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    /// The label for the attribute (max 50 characters, cannot be empty)
    #[serde(rename = "label")]
    pub label: String,
    /// The type of object the attribute belongs to. Must be either `companies` or `deals`.
    #[serde(rename = "objectType")]
    pub object_type: PostCrmAttributesRequestObjectType,
    /// Options for multi-choice or single-select attributes
    #[serde(
        rename = "optionsLabels",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub options_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmAttributesResponse {
    /// Unique ID of the created attribute
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCrmAttributesIdRequestOptionsLabelsItem {
    /// Internal option identifier
    #[serde(rename = "key")]
    pub key: String,
    /// Display label for the option
    #[serde(rename = "value")]
    pub value: String,
}

/// The type of object the attribute belongs to, it cannot be updated after creation
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatchCrmAttributesIdRequestObjectType {
    #[serde(rename = "companies")]
    Companies,
    #[serde(rename = "deals")]
    Deals,
}
impl Default for PatchCrmAttributesIdRequestObjectType {
    fn default() -> Self {
        Self::Companies
    }
}
impl core::fmt::Display for PatchCrmAttributesIdRequestObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Companies => write!(f, "companies"),
            Self::Deals => write!(f, "deals"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCrmAttributesIdRequest {
    /// Attribute display label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    /// Updated labels for selectable options
    #[serde(
        rename = "optionsLabels",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub options_labels: Option<Vec<PatchCrmAttributesIdRequestOptionsLabelsItem>>,
    /// The type of object the attribute belongs to, it cannot be updated after creation
    #[serde(
        rename = "objectType",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub object_type: Option<PatchCrmAttributesIdRequestObjectType>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmAttributesCompaniesResponseItemAttributeOptionsItem {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

/// Company attribute details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmAttributesCompaniesResponseItem {
    /// Internal name of the attribute
    #[serde(
        rename = "internalName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub internal_name: Option<String>,
    /// Type of the attribute
    #[serde(
        rename = "attributeTypeName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribute_type_name: Option<String>,
    /// Display label of the attribute
    #[serde(rename = "label", skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    /// Options for single-select or multi-choice attributes
    #[serde(
        rename = "attributeOptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribute_options: Option<Vec<GetCrmAttributesCompaniesResponseItemAttributeOptionsItem>>,
    /// Whether this attribute is required
    #[serde(
        rename = "isRequired",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_required: Option<bool>,
    /// Whether this attribute value is read-only
    #[serde(
        rename = "isValueReadonly",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_value_readonly: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmAttributesDealsResponseItemAttributeOptionsItem {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

/// Deal attribute details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmAttributesDealsResponseItem {
    /// Internal name of the attribute
    #[serde(
        rename = "internalName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub internal_name: Option<String>,
    /// Type of the attribute
    #[serde(
        rename = "attributeTypeName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribute_type_name: Option<String>,
    /// Display label of the attribute
    #[serde(rename = "label", skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    /// Options for single-select or multi-choice attributes
    #[serde(
        rename = "attributeOptions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attribute_options: Option<Vec<GetCrmAttributesDealsResponseItemAttributeOptionsItem>>,
    /// Whether this attribute is required
    #[serde(
        rename = "isRequired",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_required: Option<bool>,
    /// Whether this attribute value is read-only
    #[serde(
        rename = "isValueReadonly",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_value_readonly: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmDealsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCrmDealsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCrmDealsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// List of Deals
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmDealsResponse {
    /// List of deals
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmDealsRequest {
    /// Attributes for deal creation To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you want to create a deal on a specific pipeline and stage you can use the following attributes `pipeline` and `deal_stage`. Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Company ids to be linked with deal
    #[serde(
        rename = "linkedCompaniesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_companies_ids: Option<Vec<String>>,
    /// Contact ids to be linked with deal
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Name of deal
    #[serde(rename = "name")]
    pub name: String,
}

/// Created deal id
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmDealsResponse {
    /// Unique deal id
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmDealsImportResponse {
    /// The ID of the import process
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none", default)]
    pub process_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCrmDealsLinkUnlinkIdRequest {
    /// Company ids to be linked with deal
    #[serde(
        rename = "linkCompanyIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub link_company_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be linked with deal
    #[serde(
        rename = "linkContactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub link_contact_ids: Option<Vec<i64>>,
    /// Company ids to be unlinked from deal
    #[serde(
        rename = "unlinkCompanyIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_company_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be unlinked from deal
    #[serde(
        rename = "unlinkContactIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unlink_contact_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCrmDealsIdRequest {
    /// Attributes for deal update To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you wish to update the pipeline of a deal you need to provide the `pipeline` and the `deal_stage` Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    #[serde(
        rename = "attributes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attributes: Option<serde_json::Value>,
    /// Warning - Using PATCH on linkedCompaniesIds replaces the list of linked companies. Omitted IDs will be removed.
    #[serde(
        rename = "linkedCompaniesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_companies_ids: Option<Vec<String>>,
    /// Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(
        rename = "linkedContactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Name of deal
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmFilesEntity {
    #[serde(rename = "companies")]
    Companies,
    #[serde(rename = "deals")]
    Deals,
    #[serde(rename = "contacts")]
    Contacts,
}
impl Default for GetCrmFilesEntity {
    fn default() -> Self {
        Self::Companies
    }
}
impl core::fmt::Display for GetCrmFilesEntity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Companies => write!(f, "companies"),
            Self::Deals => write!(f, "deals"),
            Self::Contacts => write!(f, "contacts"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmFilesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCrmFilesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCrmFilesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Downloadable file link
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmFilesIdResponse {
    /// A unique link to download the requested file.
    #[serde(rename = "fileUrl", skip_serializing_if = "Option::is_none", default)]
    pub file_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmNotesEntity {
    #[serde(rename = "companies")]
    Companies,
    #[serde(rename = "deals")]
    Deals,
    #[serde(rename = "contacts")]
    Contacts,
}
impl Default for GetCrmNotesEntity {
    fn default() -> Self {
        Self::Companies
    }
}
impl core::fmt::Display for GetCrmNotesEntity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Companies => write!(f, "companies"),
            Self::Deals => write!(f, "deals"),
            Self::Contacts => write!(f, "contacts"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmNotesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCrmNotesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCrmNotesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Created Note ID
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmNotesResponse {
    /// Unique note Id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmTasksFilterStatus {
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "undone")]
    Undone,
}
impl Default for GetCrmTasksFilterStatus {
    fn default() -> Self {
        Self::Done
    }
}
impl core::fmt::Display for GetCrmTasksFilterStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Done => write!(f, "done"),
            Self::Undone => write!(f, "undone"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmTasksFilterDate {
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "tomorrow")]
    Tomorrow,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "range")]
    Range,
}
impl Default for GetCrmTasksFilterDate {
    fn default() -> Self {
        Self::Overdue
    }
}
impl core::fmt::Display for GetCrmTasksFilterDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overdue => write!(f, "overdue"),
            Self::Today => write!(f, "today"),
            Self::Tomorrow => write!(f, "tomorrow"),
            Self::Week => write!(f, "week"),
            Self::Range => write!(f, "range"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetCrmTasksSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetCrmTasksSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetCrmTasksSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// List of tasks
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmTasksResponse {
    /// List of tasks
    #[serde(rename = "items", skip_serializing_if = "Option::is_none", default)]
    pub items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmTasksRequest {
    /// To assign a task to a user you can use either the account email or ID.
    #[serde(
        rename = "assignToId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub assign_to_id: Option<String>,
    /// Companies ids for companies a task is linked to
    #[serde(
        rename = "companiesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub companies_ids: Option<Vec<String>>,
    /// Contact ids for contacts linked to this task
    #[serde(
        rename = "contactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contacts_ids: Option<Vec<i64>>,
    /// Task due date and time
    #[serde(rename = "date")]
    pub date: time::OffsetDateTime,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds", skip_serializing_if = "Option::is_none", default)]
    pub deals_ids: Option<Vec<String>>,
    /// Task marked as done
    #[serde(rename = "done", skip_serializing_if = "Option::is_none", default)]
    pub done: Option<bool>,
    /// Duration of task in milliseconds [1 minute = 60000 ms]
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<i64>,
    /// Name of task
    #[serde(rename = "name")]
    pub name: String,
    /// Notes added to a task
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none", default)]
    pub reminder: Option<serde_json::Value>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(rename = "taskTypeId")]
    pub task_type_id: String,
}

/// Task Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCrmTasksResponse {
    /// Unique task id
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCrmTasksIdRequest {
    /// To assign a task to a user you can use either the account email or ID.
    #[serde(
        rename = "assignToId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub assign_to_id: Option<String>,
    /// Companies ids for companies a task is linked to
    #[serde(
        rename = "companiesIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub companies_ids: Option<Vec<String>>,
    /// Contact ids for contacts linked to this task
    #[serde(
        rename = "contactsIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contacts_ids: Option<Vec<i64>>,
    /// Task date/time
    #[serde(rename = "date", skip_serializing_if = "Option::is_none", default)]
    pub date: Option<time::OffsetDateTime>,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds", skip_serializing_if = "Option::is_none", default)]
    pub deals_ids: Option<Vec<String>>,
    /// Task marked as done
    #[serde(rename = "done", skip_serializing_if = "Option::is_none", default)]
    pub done: Option<bool>,
    /// Duration of task in milliseconds [1 minute = 60000 ms]
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<i64>,
    /// Name of task
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Notes added to a task
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none", default)]
    pub reminder: Option<serde_json::Value>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(
        rename = "taskTypeId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub task_type_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrmTasktypesResponseItem {
    /// Id of task type
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Title of task type
    #[serde(rename = "title", skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendWhatsappMessageResponse {
    /// messageId of sent message
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsappEventReportEvent {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
    #[serde(rename = "reply")]
    Reply,
    #[serde(rename = "soft-bounce")]
    SoftBounce,
}
impl Default for GetWhatsappEventReportEvent {
    fn default() -> Self {
        Self::Sent
    }
}
impl core::fmt::Display for GetWhatsappEventReportEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::Delivered => write!(f, "delivered"),
            Self::Read => write!(f, "read"),
            Self::Error => write!(f, "error"),
            Self::Unsubscribe => write!(f, "unsubscribe"),
            Self::Reply => write!(f, "reply"),
            Self::SoftBounce => write!(f, "soft-bounce"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsappEventReportSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetWhatsappEventReportSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetWhatsappEventReportSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Event which occurred
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetWhatsappEventReportResponseEventsItemEvent {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
    #[serde(rename = "reply")]
    Reply,
    #[serde(rename = "soft-bounce")]
    SoftBounce,
}
impl Default for GetWhatsappEventReportResponseEventsItemEvent {
    fn default() -> Self {
        Self::Sent
    }
}
impl core::fmt::Display for GetWhatsappEventReportResponseEventsItemEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::Delivered => write!(f, "delivered"),
            Self::Read => write!(f, "read"),
            Self::Error => write!(f, "error"),
            Self::Unsubscribe => write!(f, "unsubscribe"),
            Self::Reply => write!(f, "reply"),
            Self::SoftBounce => write!(f, "soft-bounce"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsappEventReportResponseEventsItem {
    /// Text of the reply (will be there only in case of `reply` event with text)
    #[serde(rename = "body", skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "contactNumber")]
    pub contact_number: String,
    /// UTC date-time on which the event has been generated
    #[serde(rename = "date")]
    pub date: String,
    /// Event which occurred
    #[serde(rename = "event")]
    pub event: GetWhatsappEventReportResponseEventsItemEvent,
    /// Url of the media reply (will be there only in case of `reply` event with media)
    #[serde(rename = "mediaUrl", skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<String>,
    /// Message ID which generated the event
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Reason for the event (will be there in case of `error` and `soft-bounce` events)
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none", default)]
    pub reason: Option<String>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "senderNumber")]
    pub sender_number: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWhatsappEventReportResponse {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<GetWhatsappEventReportResponseEventsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetTransacBlockedContactsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetTransacBlockedContactsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetTransacBlockedContactsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Reason code for blocking / unsubscribing (This code is safe for comparison)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetTransacBlockedContactsResponseContactsItemReasonCode {
    #[serde(rename = "unsubscribedViaMA")]
    UnsubscribedViaMA,
    #[serde(rename = "unsubscribedViaEmail")]
    UnsubscribedViaEmail,
    #[serde(rename = "adminBlocked")]
    AdminBlocked,
    #[serde(rename = "unsubscribedViaApi")]
    UnsubscribedViaApi,
    #[serde(rename = "hardBounce")]
    HardBounce,
    #[serde(rename = "contactFlaggedAsSpam")]
    ContactFlaggedAsSpam,
}
impl Default for GetTransacBlockedContactsResponseContactsItemReasonCode {
    fn default() -> Self {
        Self::UnsubscribedViaMA
    }
}
impl core::fmt::Display for GetTransacBlockedContactsResponseContactsItemReasonCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsubscribedViaMA => write!(f, "unsubscribedViaMA"),
            Self::UnsubscribedViaEmail => write!(f, "unsubscribedViaEmail"),
            Self::AdminBlocked => write!(f, "adminBlocked"),
            Self::UnsubscribedViaApi => write!(f, "unsubscribedViaApi"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::ContactFlaggedAsSpam => write!(f, "contactFlaggedAsSpam"),
        }
    }
}

/// Reason for blocking / unsubscribing
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacBlockedContactsResponseContactsItemReason {
    /// Reason code for blocking / unsubscribing (This code is safe for comparison)
    #[serde(rename = "code", skip_serializing_if = "Option::is_none", default)]
    pub code: Option<GetTransacBlockedContactsResponseContactsItemReasonCode>,
    /// Reason for blocking / unsubscribing (This string is not safe for comparison)
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacBlockedContactsResponseContactsItem {
    /// Date when the contact was blocked or unsubscribed on
    #[serde(rename = "blockedAt")]
    pub blocked_at: String,
    /// Email address of the blocked or unsubscribed contact
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Reason for blocking / unsubscribing
    #[serde(rename = "reason")]
    pub reason: GetTransacBlockedContactsResponseContactsItemReason,
    /// Sender email address of the blocked or unsubscribed contact
    #[serde(rename = "senderEmail")]
    pub sender_email: crate::types::Email,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacBlockedContactsResponse {
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none", default)]
    pub contacts: Option<Vec<GetTransacBlockedContactsResponseContactsItem>>,
    /// Count of blocked or unsubscribed contact
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockedDomainsResponse {
    /// List of all blocked domains
    #[serde(rename = "domains")]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockNewDomainRequest {
    /// name of the domain to be blocked
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteHardbouncesRequest {
    /// Target a specific email address
    #[serde(
        rename = "contactEmail",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_email: Option<crate::types::Email>,
    /// Ending date (YYYY-MM-DD) of the time period for deletion. The hardbounces until this date will be deleted. Must be greater than or equal to the startDate
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none", default)]
    pub end_date: Option<String>,
    /// Starting date (YYYY-MM-DD) of the time period for deletion. The hardbounces occurred after this date will be deleted. Must be less than or equal to the endDate
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none", default)]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestAttachmentItem {
    /// Base64-encoded attachment data
    #[serde(rename = "content", skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    /// Attachment filename. Required when `content` is provided.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Absolute URL of the attachment. Local file paths are not supported.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestBccItem {
    /// Consent of the recipient in bcc for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// BCC recipient email address
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the BCC recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestCcItem {
    /// Consent of the recipient in cc for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// CC recipient email address
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the CC recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestMessageVersionsItemBccItem {
    /// Consent of the recipient in bcc for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// BCC recipient email address
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the BCC recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestMessageVersionsItemCcItem {
    /// Consent of the recipient in cc for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// CC recipient email address
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the CC recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestMessageVersionsItemReplyTo {
    /// Email address in reply to
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name for the reply-to address. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestMessageVersionsItemToItem {
    /// Consent of the recipient for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// Email address of the recipient
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestMessageVersionsItem {
    /// Array of BCC recipient objects. Each object contains an email address and an optional name.
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none", default)]
    pub bcc: Option<Vec<SendTransacEmailRequestMessageVersionsItemBccItem>>,
    /// Array of CC recipient objects. Each object contains an email address and an optional name.
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none", default)]
    pub cc: Option<Vec<SendTransacEmailRequestMessageVersionsItemCcItem>>,
    /// HTML body content of the email. Required when `templateId` is not provided. Ignored when `templateId` is provided.
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// Key-value pairs for template variable substitution. Only applicable when the template uses the New Template Language format.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    /// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<SendTransacEmailRequestMessageVersionsItemReplyTo>,
    /// Email subject line for this message version
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Plain text body content of the email. Ignored when `templateId` is provided.
    #[serde(
        rename = "textContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub text_content: Option<String>,
    /// Array of recipient objects. Each object contains an email address and an optional display name.
    #[serde(rename = "to")]
    pub to: Vec<SendTransacEmailRequestMessageVersionsItemToItem>,
}

/// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestReplyTo {
    /// Email address in reply to
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name for the reply-to address. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

/// Sender information. Required when `templateId` is not provided. Specify either an email address (with optional name) or a sender ID. The `name` field is ignored when `id` is provided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestSender {
    /// Sender email address. Required when `id` is not provided.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Sender identifier. Required when `email` is not provided. Use this field to select a sender with a specific IP pool (dedicated IP users only).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Display name of the sender. Maximum length is 70 characters. Only applicable when `email` is provided.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequestToItem {
    /// Consent of the recipient for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(
        rename = "contactPixelTrackingConsent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// Email address of the recipient
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Display name of the recipient. Maximum length is 70 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailRequest {
    /// Array of attachment objects. Each attachment must include either an absolute URL (no local file paths) or base64-encoded content, along with the attachment filename. The `name` field is required when `content` is provided. Supported file extensions: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub, eps, odt, mp3, m4a, m4v, wma, ogg, flac, wav, aif, aifc, aiff, mp4, mov, avi, mkv, mpeg, mpg, wmv, pkpass, xlsm. When `templateId` is specified: if the template uses the New Template Language format, both `url` and `content` attachment types are supported; if the template uses the Old Template Language format, the `attachment` parameter is ignored.
    #[serde(
        rename = "attachment",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment: Option<Vec<SendTransacEmailRequestAttachmentItem>>,
    /// UUIDv4 identifier for the scheduled batch of transactional emails. If omitted, a valid UUIDv4 batch identifier is automatically generated.
    #[serde(rename = "batchId", skip_serializing_if = "Option::is_none", default)]
    pub batch_id: Option<String>,
    /// Array of BCC recipient objects. Each object contains an email address and an optional name.
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none", default)]
    pub bcc: Option<Vec<SendTransacEmailRequestBccItem>>,
    /// Array of CC recipient objects. Each object contains an email address and an optional name.
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none", default)]
    pub cc: Option<Vec<SendTransacEmailRequestCcItem>>,
    /// Custom email headers (non-standard headers) to include in the email. The `sender.ip` header can be set to specify the IP address used for sending transactional emails (dedicated IP users only). Header names must use Title-Case-Format (words separated by hyphens with the first letter of each word capitalized). Headers not in this format are automatically converted. Standard email headers are not supported. Example: `{"sender.ip":"1.2.3.4", "X-Mailin-custom":"some_custom_value", "Idempotency-Key":"abc-123"}`
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<serde_json::Value>,
    /// HTML body content of the email. Required when `templateId` is not provided. Ignored when `templateId` is provided.
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// Array of message version objects for sending customized email variants. The `templateId` can be customized per version only if a global `templateId` is provided. The `htmlContent` and `textContent` can be customized per version only if at least one of these is present in the global parameters. Global parameters such as `to` (required), `bcc`, `cc`, `replyTo`, and `subject` can be customized per version. Maximum total recipients per API request is 2000. Maximum recipients per message version is 99. Individual `params` objects must not exceed 100 KB. Cumulative `params` across all versions must not exceed 1000 KB. See <https://developers.brevo.com/docs/batch-send-transactional-emails> for detailed usage instructions.
    #[serde(
        rename = "messageVersions",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub message_versions: Option<Vec<SendTransacEmailRequestMessageVersionsItem>>,
    /// Key-value pairs for template variable substitution. Only applicable when the template uses the New Template Language format.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    /// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<SendTransacEmailRequestReplyTo>,
    /// UTC date-time when the email should be sent (format: YYYY-MM-DDTHH:mm:ss.SSSZ). Include timezone information in the date-time value. Scheduled emails may be delayed by up to 5 minutes.
    #[serde(
        rename = "scheduledAt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_at: Option<time::OffsetDateTime>,
    /// Sender information. Required when `templateId` is not provided. Specify either an email address (with optional name) or a sender ID. The `name` field is ignored when `id` is provided.
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<SendTransacEmailRequestSender>,
    /// Email subject line. Required when `templateId` is not provided.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Array of tags for categorizing and filtering emails
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    /// Template identifier
    #[serde(
        rename = "templateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_id: Option<i64>,
    /// Plain text body content of the email. Ignored when `templateId` is provided.
    #[serde(
        rename = "textContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub text_content: Option<String>,
    /// Array of recipient objects. Each object contains an email address and an optional display name. Required when `messageVersions` is not provided. Ignored when `messageVersions` is provided. Example: `[{"name":"Jimmy", "email":"jimmy@example.com"}, {"name":"Joe", "email":"joe@example.com"}]`
    #[serde(rename = "to", skip_serializing_if = "Option::is_none", default)]
    pub to: Option<Vec<SendTransacEmailRequestToItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacEmailResponse {
    /// Message ID of the transactional email sent
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none", default)]
    pub message_id: Option<String>,
    #[serde(
        rename = "messageIds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub message_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetScheduledEmailByIdSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetScheduledEmailByIdSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetScheduledEmailByIdSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetScheduledEmailByIdStatus {
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
}
impl Default for GetScheduledEmailByIdStatus {
    fn default() -> Self {
        Self::Processed
    }
}
impl core::fmt::Display for GetScheduledEmailByIdStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Processed => write!(f, "processed"),
            Self::InProgress => write!(f, "inProgress"),
            Self::Queued => write!(f, "queued"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetTransacEmailsListSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetTransacEmailsListSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetTransacEmailsListSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacEmailsListResponseTransactionalEmailsItem {
    /// Date on which transactional email was sent
    #[serde(rename = "date")]
    pub date: String,
    /// Email address to which transactional email has been sent
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Email address of the sender from which the email was sent
    #[serde(rename = "from", skip_serializing_if = "Option::is_none", default)]
    pub from: Option<crate::types::Email>,
    /// Message Id of the sent email
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Subject of the sent email
    #[serde(rename = "subject")]
    pub subject: String,
    /// Tags used for your email
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    /// Id of the template
    #[serde(
        rename = "templateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_id: Option<i64>,
    /// Unique id of the email sent to a particular contact
    #[serde(rename = "uuid")]
    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacEmailsListResponse {
    /// Total number of transactional emails available on your account according to the passed filter
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(
        rename = "transactionalEmails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transactional_emails: Option<Vec<GetTransacEmailsListResponseTransactionalEmailsItem>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacEmailContentResponseEventsItem {
    /// Name of the event that occurred on the sent email
    #[serde(rename = "name")]
    pub name: String,
    /// Time at which the event occurred
    #[serde(rename = "time")]
    pub time: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacEmailContentResponse {
    /// Count of the attachments that were sent in the email
    #[serde(
        rename = "attachmentCount",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_count: Option<i64>,
    /// Actual content of the transactional email that has been sent
    #[serde(rename = "body", skip_serializing_if = "Option::is_none", default)]
    pub body: Option<String>,
    /// Date on which transactional email was sent
    #[serde(rename = "date")]
    pub date: String,
    /// Email address to which transactional email has been sent
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Series of events which occurred on the transactional email
    #[serde(rename = "events")]
    pub events: Vec<GetTransacEmailContentResponseEventsItem>,
    /// Subject of the sent email
    #[serde(rename = "subject")]
    pub subject: String,
    /// Id of the template
    #[serde(
        rename = "templateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAggregatedSmtpReportResponse {
    /// Number of blocked contact emails for the timeframe
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none", default)]
    pub blocked: Option<i64>,
    /// Number of clicks for the timeframe
    #[serde(rename = "clicks", skip_serializing_if = "Option::is_none", default)]
    pub clicks: Option<i64>,
    /// Number of delivered emails for the timeframe
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none", default)]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the timeframe
    #[serde(
        rename = "hardBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hard_bounces: Option<i64>,
    /// Number of invalid emails for the timeframe
    #[serde(rename = "invalid", skip_serializing_if = "Option::is_none", default)]
    pub invalid: Option<i64>,
    /// Number of openings for the timeframe
    #[serde(rename = "opens", skip_serializing_if = "Option::is_none", default)]
    pub opens: Option<i64>,
    /// Time frame of the report
    #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
    pub range: Option<String>,
    /// Number of requests for the timeframe
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none", default)]
    pub requests: Option<i64>,
    /// Number of softbounces for the timeframe
    #[serde(
        rename = "softBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub soft_bounces: Option<i64>,
    /// Number of complaint (spam report) for the timeframe
    #[serde(
        rename = "spamReports",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub spam_reports: Option<i64>,
    /// Number of unique clicks for the timeframe
    #[serde(
        rename = "uniqueClicks",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unique_clicks: Option<i64>,
    /// Number of unique openings for the timeframe
    #[serde(
        rename = "uniqueOpens",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unique_opens: Option<i64>,
    /// Number of unsubscribed emails for the timeframe
    #[serde(
        rename = "unsubscribed",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailEventReportEvent {
    #[serde(rename = "bounces")]
    Bounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "loadedByProxy")]
    LoadedByProxy,
}
impl Default for GetEmailEventReportEvent {
    fn default() -> Self {
        Self::Bounces
    }
}
impl core::fmt::Display for GetEmailEventReportEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Spam => write!(f, "spam"),
            Self::Requests => write!(f, "requests"),
            Self::Opened => write!(f, "opened"),
            Self::Clicks => write!(f, "clicks"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Blocked => write!(f, "blocked"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Error => write!(f, "error"),
            Self::LoadedByProxy => write!(f, "loadedByProxy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailEventReportSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetEmailEventReportSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetEmailEventReportSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Event which occurred
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetEmailEventReportResponseEventsItemEvent {
    #[serde(rename = "bounces")]
    Bounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "loadedByProxy")]
    LoadedByProxy,
}
impl Default for GetEmailEventReportResponseEventsItemEvent {
    fn default() -> Self {
        Self::Bounces
    }
}
impl core::fmt::Display for GetEmailEventReportResponseEventsItemEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Spam => write!(f, "spam"),
            Self::Requests => write!(f, "requests"),
            Self::Opened => write!(f, "opened"),
            Self::Clicks => write!(f, "clicks"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Blocked => write!(f, "blocked"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Error => write!(f, "error"),
            Self::LoadedByProxy => write!(f, "loadedByProxy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmailEventReportResponseEventsItem {
    /// UTC date-time on which the event has been generated
    #[serde(rename = "date")]
    pub date: String,
    /// Email address which generates the event
    #[serde(rename = "email")]
    pub email: crate::types::Email,
    /// Event which occurred
    #[serde(rename = "event")]
    pub event: GetEmailEventReportResponseEventsItemEvent,
    /// Sender email from which the emails are sent
    #[serde(rename = "from", skip_serializing_if = "Option::is_none", default)]
    pub from: Option<crate::types::Email>,
    /// IP from which the user has opened the email or clicked on the link (only available if the event is opened or clicks)
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
    /// The link which is sent to the user (only available if the event is requests or opened or clicks)
    #[serde(rename = "link", skip_serializing_if = "Option::is_none", default)]
    pub link: Option<String>,
    /// Message ID which generated the event
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Reason of bounce (only available if the event is hardbounce or softbounce)
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none", default)]
    pub reason: Option<String>,
    /// Subject of the event
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Tag of the email which generated the event
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// ID of the template (only available if the email is template based)
    #[serde(
        rename = "templateId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmailEventReportResponse {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<GetEmailEventReportResponseEventsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmtpReportSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSmtpReportSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSmtpReportSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmtpReportResponseReportsItem {
    /// Number of blocked emails for the date
    #[serde(rename = "blocked")]
    pub blocked: i64,
    /// Number of clicks for the date
    #[serde(rename = "clicks")]
    pub clicks: i64,
    /// Date of the statistics
    #[serde(rename = "date")]
    pub date: time::Date,
    /// Number of delivered emails for the date
    #[serde(rename = "delivered")]
    pub delivered: i64,
    /// Number of hardbounces for the date
    #[serde(rename = "hardBounces")]
    pub hard_bounces: i64,
    /// Number of invalid emails for the date
    #[serde(rename = "invalid")]
    pub invalid: i64,
    /// Number of openings for the date
    #[serde(rename = "opens")]
    pub opens: i64,
    /// Number of requests for the date
    #[serde(rename = "requests")]
    pub requests: i64,
    /// Number of softbounces for the date
    #[serde(rename = "softBounces")]
    pub soft_bounces: i64,
    /// Number of complaints (spam reports) for the date
    #[serde(rename = "spamReports")]
    pub spam_reports: i64,
    /// Number of unique clicks for the date
    #[serde(rename = "uniqueClicks")]
    pub unique_clicks: i64,
    /// Number of unique openings for the date
    #[serde(rename = "uniqueOpens")]
    pub unique_opens: i64,
    /// Number of unsubscribed emails for the date
    #[serde(rename = "unsubscribed")]
    pub unsubscribed: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmtpReportResponse {
    #[serde(rename = "reports", skip_serializing_if = "Option::is_none", default)]
    pub reports: Option<Vec<GetSmtpReportResponseReportsItem>>,
}

/// Key-value pairs of dynamic parameters for template rendering.(Required if email not provided) For example: **{"Firstname":"John", "Lastname":"Doe"}**
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPreviewSmtpEmailTemplatesRequestParams {
    /// firstname of the contact. **Dynamic parameter**
    #[serde(rename = "Firstname", skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<String>,
    /// Lastname of the contact. **Dynamic parameter**
    #[serde(rename = "Lastname", skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPreviewSmtpEmailTemplatesRequest {
    /// Email of the contact.(Required if params not provided)
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    /// Key-value pairs of dynamic parameters for template rendering.(Required if email not provided) For example: **{"Firstname":"John", "Lastname":"Doe"}**
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<PostPreviewSmtpEmailTemplatesRequestParams>,
    /// Id of the template.
    #[serde(rename = "templateId")]
    pub template_id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPreviewSmtpEmailTemplatesResponse {
    /// Sender email address
    #[serde(rename = "fromEmail", skip_serializing_if = "Option::is_none", default)]
    pub from_email: Option<String>,
    /// Sender Name
    #[serde(rename = "fromName", skip_serializing_if = "Option::is_none", default)]
    pub from_name: Option<String>,
    /// Html content of the template
    #[serde(rename = "html", skip_serializing_if = "Option::is_none", default)]
    pub html: Option<String>,
    /// Preview text of the template
    #[serde(
        rename = "previewText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub preview_text: Option<String>,
    /// subject of the template
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    #[serde(
        rename = "usedFeedNames",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub used_feed_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmtpTemplatesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSmtpTemplatesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSmtpTemplatesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmtpTemplatesEditorType {
    #[serde(rename = "richTextEditor")]
    RichTextEditor,
}
impl Default for GetSmtpTemplatesEditorType {
    fn default() -> Self {
        Self::RichTextEditor
    }
}
impl core::fmt::Display for GetSmtpTemplatesEditorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RichTextEditor => write!(f, "richTextEditor"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmtpTemplatesResponse {
    /// Count of transactional email templates
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none", default)]
    pub templates: Option<Vec<serde_json::Value>>,
}

/// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSmtpTemplateRequestSender {
    /// Email of the sender
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Select the sender for the template on the basis of sender id. _In order to select a sender with specific pool of IP’s, dedicated ip users shall pass id (instead of email)_.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Name of the sender. **If not passed, will be set to default**
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSmtpTemplateRequest {
    /// Absolute url of the attachment (**no local file**). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps'
    #[serde(
        rename = "attachmentUrl",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_url: Option<String>,
    /// Body of the message (HTML version). The field must have more than 10 characters. **REQUIRED if htmlUrl is empty**
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// Url which contents the body of the email message. REQUIRED if htmlContent is empty
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none", default)]
    pub html_url: Option<String>,
    /// Status of template. isActive = true means template is active and isActive = false means template is inactive
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none", default)]
    pub is_active: Option<bool>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<crate::types::Email>,
    /// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    #[serde(rename = "sender")]
    pub sender: CreateSmtpTemplateRequestSender,
    /// Subject of the template
    #[serde(rename = "subject")]
    pub subject: String,
    /// Tag of the template
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// Name of the template
    #[serde(rename = "templateName")]
    pub template_name: String,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none", default)]
    pub to_field: Option<String>,
}

/// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmtpTemplateRequestSender {
    /// Email of the sender
    #[serde(rename = "email", skip_serializing_if = "Option::is_none", default)]
    pub email: Option<crate::types::Email>,
    /// Select the sender for the template on the basis of sender id. _In order to select a sender with specific pool of IP’s, dedicated ip users shall pass id (instead of email)_.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Name of the sender
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmtpTemplateRequest {
    /// Absolute url of the attachment (**no local file**). Extensions allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(
        rename = "attachmentUrl",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub attachment_url: Option<String>,
    /// **Required if htmlUrl is empty**. If the template is designed using Drag & Drop editor via HTML content, then the design page will not have Drag & Drop editor access for that template. Body of the message (HTML must have more than 10 characters)
    #[serde(
        rename = "htmlContent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub html_content: Option<String>,
    /// **Required if htmlContent is empty**. URL to the body of the email (HTML)
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none", default)]
    pub html_url: Option<String>,
    /// Status of the template. isActive = false means template is inactive, isActive = true means template is active
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none", default)]
    pub is_active: Option<bool>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none", default)]
    pub reply_to: Option<crate::types::Email>,
    /// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<UpdateSmtpTemplateRequestSender>,
    /// Subject of the email
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    /// Tag of the template
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// Name of the template
    #[serde(
        rename = "templateName",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub template_name: Option<String>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none", default)]
    pub to_field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAsyncTransactionalSmsResponse {
    #[serde(rename = "messageId")]
    pub message_id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTransacSmsResponse {
    #[serde(rename = "messageId")]
    pub message_id: i64,
    #[serde(rename = "reference")]
    pub reference: String,
    /// Remaining SMS credits of the user
    #[serde(
        rename = "remainingCredits",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remaining_credits: Option<f32>,
    /// Count of SMS's to send multiple text messages
    #[serde(rename = "smsCount", skip_serializing_if = "Option::is_none", default)]
    pub sms_count: Option<i64>,
    /// SMS credits used per text message
    #[serde(
        rename = "usedCredits",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub used_credits: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacAggregatedSmsReportResponse {
    /// Number of accepted SMS for the timeframe
    #[serde(rename = "accepted", skip_serializing_if = "Option::is_none", default)]
    pub accepted: Option<i64>,
    /// Number of blocked contact for the timeframe
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none", default)]
    pub blocked: Option<i64>,
    /// Number of delivered SMS for the timeframe
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none", default)]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the timeframe
    #[serde(
        rename = "hardBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hard_bounces: Option<i64>,
    /// Time frame of the report
    #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
    pub range: Option<String>,
    /// Number of rejected SMS for the timeframe
    #[serde(rename = "rejected", skip_serializing_if = "Option::is_none", default)]
    pub rejected: Option<i64>,
    /// Number of answered SMS for the timeframe
    #[serde(rename = "replied", skip_serializing_if = "Option::is_none", default)]
    pub replied: Option<i64>,
    /// Number of requests for the timeframe
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none", default)]
    pub requests: Option<i64>,
    /// Number of skipped SMS for the timeframe
    #[serde(rename = "skipped", skip_serializing_if = "Option::is_none", default)]
    pub skipped: Option<i64>,
    /// Number of softbounces for the timeframe
    #[serde(
        rename = "softBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub soft_bounces: Option<i64>,
    /// Number of unsubscription for the timeframe
    #[serde(
        rename = "unsubscribed",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsEventsEvent {
    #[serde(rename = "bounces")]
    Bounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "unsubscription")]
    Unsubscription,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "skipped")]
    Skipped,
}
impl Default for GetSmsEventsEvent {
    fn default() -> Self {
        Self::Bounces
    }
}
impl core::fmt::Display for GetSmsEventsEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Sent => write!(f, "sent"),
            Self::Accepted => write!(f, "accepted"),
            Self::Unsubscription => write!(f, "unsubscription"),
            Self::Replies => write!(f, "replies"),
            Self::Blocked => write!(f, "blocked"),
            Self::Rejected => write!(f, "rejected"),
            Self::Skipped => write!(f, "skipped"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsEventsSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSmsEventsSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSmsEventsSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Event which occurred
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSmsEventsResponseEventsItemEvent {
    #[serde(rename = "bounces")]
    Bounces,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "unsubscription")]
    Unsubscription,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "skipped")]
    Skipped,
}
impl Default for GetSmsEventsResponseEventsItemEvent {
    fn default() -> Self {
        Self::Bounces
    }
}
impl core::fmt::Display for GetSmsEventsResponseEventsItemEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Sent => write!(f, "sent"),
            Self::Accepted => write!(f, "accepted"),
            Self::Unsubscription => write!(f, "unsubscription"),
            Self::Replies => write!(f, "replies"),
            Self::Blocked => write!(f, "blocked"),
            Self::Rejected => write!(f, "rejected"),
            Self::Skipped => write!(f, "skipped"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsEventsResponseEventsItem {
    /// UTC date-time on which the event has been generated
    #[serde(rename = "date", skip_serializing_if = "Option::is_none", default)]
    pub date: Option<String>,
    /// Event which occurred
    #[serde(rename = "event", skip_serializing_if = "Option::is_none", default)]
    pub event: Option<GetSmsEventsResponseEventsItemEvent>,
    /// Message ID which generated the event
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none", default)]
    pub message_id: Option<String>,
    /// Phone number which has generated the event
    #[serde(
        rename = "phoneNumber",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub phone_number: Option<String>,
    /// Reason of bounce (only available if the event is hardbounce or softbounce)
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none", default)]
    pub reason: Option<String>,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none", default)]
    pub reply: Option<String>,
    /// Tag of the SMS which generated the event
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsEventsResponse {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none", default)]
    pub events: Option<Vec<GetSmsEventsResponseEventsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetTransacSmsReportSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetTransacSmsReportSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetTransacSmsReportSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacSmsReportResponseReportsItem {
    /// Number of accepted SMS for the date
    #[serde(rename = "accepted", skip_serializing_if = "Option::is_none", default)]
    pub accepted: Option<i64>,
    /// Number of blocked contact for the date
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none", default)]
    pub blocked: Option<i64>,
    /// Date for which statistics are retrieved
    #[serde(rename = "date", skip_serializing_if = "Option::is_none", default)]
    pub date: Option<time::Date>,
    /// Number of delivered SMS for the date
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none", default)]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the date
    #[serde(
        rename = "hardBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hard_bounces: Option<i64>,
    /// Number of rejected SMS for the date
    #[serde(rename = "rejected", skip_serializing_if = "Option::is_none", default)]
    pub rejected: Option<i64>,
    /// Number of answered SMS for the date
    #[serde(rename = "replied", skip_serializing_if = "Option::is_none", default)]
    pub replied: Option<i64>,
    /// Number of requests for the date
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none", default)]
    pub requests: Option<i64>,
    /// Number of skipped SMS for the date
    #[serde(rename = "skipped", skip_serializing_if = "Option::is_none", default)]
    pub skipped: Option<i64>,
    /// Number of softbounces for the date
    #[serde(
        rename = "softBounces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub soft_bounces: Option<i64>,
    /// Number of unsubscription for the date
    #[serde(
        rename = "unsubscribed",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unsubscribed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransacSmsReportResponse {
    #[serde(rename = "reports", skip_serializing_if = "Option::is_none", default)]
    pub reports: Option<Vec<GetTransacSmsReportResponseReportsItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetSMSTemplatesSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl Default for GetSMSTemplatesSort {
    fn default() -> Self {
        Self::Asc
    }
}
impl core::fmt::Display for GetSMSTemplatesSort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Manage US compliance
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSMSTemplatesResponseTemplatesItemCompliance {
    /// US compliance is enabled.
    #[serde(
        rename = "compliance_toggle",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub compliance_toggle: Option<bool>,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by the U.S. carriers.
    #[serde(
        rename = "organisation_prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation_prefix: Option<String>,
    /// Instructions to unsubscribe from future communications. Must include STOP keyword.
    #[serde(
        rename = "stop_keyword",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub stop_keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSMSTemplatesResponseTemplatesItem {
    /// ID of the template
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// Name of the template
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// URL shortening is enabled
    #[serde(
        rename = "short_url_enabled",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub short_url_enabled: Option<bool>,
    /// message content for the sms
    #[serde(rename = "message", skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
    /// Manage US compliance
    #[serde(
        rename = "compliance",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub compliance: Option<GetSMSTemplatesResponseTemplatesItemCompliance>,
    /// Enable Unicode encryption.
    #[serde(
        rename = "encryption",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub encryption: Option<bool>,
    /// Support unicode characters in the sms.
    #[serde(
        rename = "unicode_sms",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub unicode_sms: Option<bool>,
    /// file name of the media.
    #[serde(
        rename = "media_file",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub media_file: Option<String>,
    /// file size in bytes.
    #[serde(
        rename = "media_file_size",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub media_file_size: Option<String>,
    /// URL of the image.
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none", default)]
    pub media_url: Option<String>,
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSMSTemplatesResponse {
    /// Count of transactional sms templates
    #[serde(rename = "count", skip_serializing_if = "Option::is_none", default)]
    pub count: Option<i64>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none", default)]
    pub templates: Option<Vec<GetSMSTemplatesResponseTemplatesItem>>,
}
