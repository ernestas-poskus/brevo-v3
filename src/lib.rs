//! # `brevo-v3`
//!
//! An asynchronous, type-safe, and idiomatic Rust client library for the **[Brevo (Sendinblue) v3 REST API](https://developers.brevo.com/docs/api-clients)**.
//!
//! ## Overview
//!
//! - **Full API Coverage**: All 291 endpoints covering Transactional Emails, SMS, WhatsApp, Contacts, Campaigns, Deals, Companies, Tasks, Webhooks, Inbound Parsing, Ecommerce, and Account Management.
//! - **Domain Invariants**: Strong unforgeable newtypes (`Email`, `PhoneNumber`, `ContactId`, `CampaignId`, `TemplateId`, `DealId`, `CompanyId`, etc.) following the *"Parse, Don't Validate"* philosophy.
//! - **Async Runtime**: Built on `tokio` (1.x) and `reqwest` (0.12+ with hyper & rustls).
//! - **Error Handling**: Structured API errors with `thiserror` and automatic 429 rate limit backoff parsing.
//! - **Date/Time**: Powered by the `time` crate (0.3) for RFC3339 / ISO8601 formatting.
//!
//! ## Quick Start
//!
//! ```no_run
//! use brevo_v3::client::BrevoClient;
//! use brevo_v3::models::{SendTransacEmailRequest, SendTransacEmailRequestSender, SendTransacEmailRequestToItem};
//! use brevo_v3::types::Email;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BrevoClient::new("xkeysib-your-api-key")?;
//!
//!     let recipient = Email::new("alice@example.com")?;
//!     let sender = Email::new("notifications@mycompany.com")?;
//!
//!     let request = SendTransacEmailRequest {
//!         subject: Some("Order Confirmation".to_string()),
//!         html_content: Some("<h1>Order Received!</h1>".to_string()),
//!         sender: Some(SendTransacEmailRequestSender {
//!             email: Some(sender),
//!             name: Some("Store Notifications".to_string()),
//!             id: None,
//!         }),
//!         to: Some(vec![SendTransacEmailRequestToItem {
//!             email: recipient,
//!             name: Some("Alice".to_string()),
//!         }]),
//!         ..Default::default()
//!     };
//!
//!     let response = client.transactional_emails().send_transac_email(&request).await?;
//!     println!("Sent message ID: {:?}", response.message_id);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod types;

mod client_accessors;

// Re-exports
pub use client::{BrevoClient, BrevoClientBuilder, BrevoConfig, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use types::*;
