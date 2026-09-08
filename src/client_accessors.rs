use crate::api;
use crate::client::BrevoClient;

impl BrevoClient {
    /// Access the [`api::AccountApi`] endpoints.
    #[must_use]
    pub fn account(&self) -> api::AccountApi {
        api::AccountApi::new(self.clone())
    }

    /// Access the [`api::MasterAccountApi`] endpoints.
    #[must_use]
    pub fn master_account(&self) -> api::MasterAccountApi {
        api::MasterAccountApi::new(self.clone())
    }

    /// Access the [`api::UserApi`] endpoints.
    #[must_use]
    pub fn user(&self) -> api::UserApi {
        api::UserApi::new(self.clone())
    }

    /// Access the [`api::ProcessApi`] endpoints.
    #[must_use]
    pub fn process(&self) -> api::ProcessApi {
        api::ProcessApi::new(self.clone())
    }

    /// Access the [`api::SendersApi`] endpoints.
    #[must_use]
    pub fn senders(&self) -> api::SendersApi {
        api::SendersApi::new(self.clone())
    }

    /// Access the [`api::DomainsApi`] endpoints.
    #[must_use]
    pub fn domains(&self) -> api::DomainsApi {
        api::DomainsApi::new(self.clone())
    }

    /// Access the [`api::WebhooksApi`] endpoints.
    #[must_use]
    pub fn webhooks(&self) -> api::WebhooksApi {
        api::WebhooksApi::new(self.clone())
    }

    /// Access the [`api::ExternalFeedsApi`] endpoints.
    #[must_use]
    pub fn external_feeds(&self) -> api::ExternalFeedsApi {
        api::ExternalFeedsApi::new(self.clone())
    }

    /// Access the [`api::CustomObjectsApi`] endpoints.
    #[must_use]
    pub fn custom_objects(&self) -> api::CustomObjectsApi {
        api::CustomObjectsApi::new(self.clone())
    }

    /// Access the [`api::ContactsApi`] endpoints.
    #[must_use]
    pub fn contacts(&self) -> api::ContactsApi {
        api::ContactsApi::new(self.clone())
    }

    /// Access the [`api::ConsentGroupsApi`] endpoints.
    #[must_use]
    pub fn consent_groups(&self) -> api::ConsentGroupsApi {
        api::ConsentGroupsApi::new(self.clone())
    }

    /// Access the [`api::ConversationsApi`] endpoints.
    #[must_use]
    pub fn conversations(&self) -> api::ConversationsApi {
        api::ConversationsApi::new(self.clone())
    }

    /// Access the [`api::EcommerceApi`] endpoints.
    #[must_use]
    pub fn ecommerce(&self) -> api::EcommerceApi {
        api::EcommerceApi::new(self.clone())
    }

    /// Access the [`api::CouponsApi`] endpoints.
    #[must_use]
    pub fn coupons(&self) -> api::CouponsApi {
        api::CouponsApi::new(self.clone())
    }

    /// Access the [`api::PaymentsApi`] endpoints.
    #[must_use]
    pub fn payments(&self) -> api::PaymentsApi {
        api::PaymentsApi::new(self.clone())
    }

    /// Access the [`api::EventApi`] endpoints.
    #[must_use]
    pub fn event(&self) -> api::EventApi {
        api::EventApi::new(self.clone())
    }

    /// Access the [`api::InboundParsingApi`] endpoints.
    #[must_use]
    pub fn inbound_parsing(&self) -> api::InboundParsingApi {
        api::InboundParsingApi::new(self.clone())
    }

    /// Access the [`api::BalanceApi`] endpoints.
    #[must_use]
    pub fn balance(&self) -> api::BalanceApi {
        api::BalanceApi::new(self.clone())
    }

    /// Access the [`api::ProgramApi`] endpoints.
    #[must_use]
    pub fn program(&self) -> api::ProgramApi {
        api::ProgramApi::new(self.clone())
    }

    /// Access the [`api::RewardApi`] endpoints.
    #[must_use]
    pub fn reward(&self) -> api::RewardApi {
        api::RewardApi::new(self.clone())
    }

    /// Access the [`api::TierApi`] endpoints.
    #[must_use]
    pub fn tier(&self) -> api::TierApi {
        api::TierApi::new(self.clone())
    }

    /// Access the [`api::WalletApi`] endpoints.
    #[must_use]
    pub fn wallet(&self) -> api::WalletApi {
        api::WalletApi::new(self.clone())
    }

    /// Access the [`api::EmailCampaignsApi`] endpoints.
    #[must_use]
    pub fn email_campaigns(&self) -> api::EmailCampaignsApi {
        api::EmailCampaignsApi::new(self.clone())
    }

    /// Access the [`api::SMSCampaignsApi`] endpoints.
    #[must_use]
    pub fn sms_campaigns(&self) -> api::SMSCampaignsApi {
        api::SMSCampaignsApi::new(self.clone())
    }

    /// Access the [`api::WhatsAppCampaignsApi`] endpoints.
    #[must_use]
    pub fn whats_app_campaigns(&self) -> api::WhatsAppCampaignsApi {
        api::WhatsAppCampaignsApi::new(self.clone())
    }

    /// Access the [`api::CompaniesApi`] endpoints.
    #[must_use]
    pub fn companies(&self) -> api::CompaniesApi {
        api::CompaniesApi::new(self.clone())
    }

    /// Access the [`api::DealsApi`] endpoints.
    #[must_use]
    pub fn deals(&self) -> api::DealsApi {
        api::DealsApi::new(self.clone())
    }

    /// Access the [`api::FilesApi`] endpoints.
    #[must_use]
    pub fn files(&self) -> api::FilesApi {
        api::FilesApi::new(self.clone())
    }

    /// Access the [`api::NotesApi`] endpoints.
    #[must_use]
    pub fn notes(&self) -> api::NotesApi {
        api::NotesApi::new(self.clone())
    }

    /// Access the [`api::TasksApi`] endpoints.
    #[must_use]
    pub fn tasks(&self) -> api::TasksApi {
        api::TasksApi::new(self.clone())
    }

    /// Access the [`api::TransactionalWhatsAppApi`] endpoints.
    #[must_use]
    pub fn transactional_whats_app(&self) -> api::TransactionalWhatsAppApi {
        api::TransactionalWhatsAppApi::new(self.clone())
    }

    /// Access the [`api::TransactionalEmailsApi`] endpoints.
    #[must_use]
    pub fn transactional_emails(&self) -> api::TransactionalEmailsApi {
        api::TransactionalEmailsApi::new(self.clone())
    }

    /// Access the [`api::TransactionalSMSApi`] endpoints.
    #[must_use]
    pub fn transactional_sms(&self) -> api::TransactionalSMSApi {
        api::TransactionalSMSApi::new(self.clone())
    }

    /// Access the [`api::SMSTemplatesApi`] endpoints.
    #[must_use]
    pub fn sms_templates(&self) -> api::SMSTemplatesApi {
        api::SMSTemplatesApi::new(self.clone())
    }
}
