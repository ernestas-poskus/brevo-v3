# `brevo-v3`

[![Crates.io](https://img.shields.io/crates/v/brevo-v3.svg)](https://crates.io/crates/brevo-v3)
[![Documentation](https://docs.rs/brevo-v3/badge.svg)](https://docs.rs/brevo-v3)
[![License](https://img.shields.io/crates/l/brevo-v3.svg)](https://github.com/ernestas-poskus/brevo-v3#license)
[![CI](https://github.com/ernestas-poskus/brevo-v3/actions/workflows/ci.yml/badge.svg)](https://github.com/ernestas-poskus/brevo-v3/actions)

An asynchronous, type-safe, and idiomatic Rust client library for the **[Brevo (Sendinblue) v3 REST API](https://developers.brevo.com/docs/api-clients)**.

---

## Features

- **Full API Coverage**: All 291 endpoints and operations covering Transactional Emails, SMS, WhatsApp, Contacts, Campaigns, Deals, Companies, Tasks, Webhooks, Inbound Parsing, Ecommerce, and Account Management.
- **Type-Safe Domain Invariants**: Built on the *"Parse, Don't Validate"* philosophy. Domain primitives such as `Email`, `PhoneNumber`, `ContactId`, `CampaignId`, `TemplateId`, `DealId`, and `CompanyId` are strongly typed unforgeable newtypes.
- **Async & Fast**: Built on `tokio` (1.x) and `reqwest` (0.12+) with `rustls-tls` and HTTP/2 connection pooling.
- **Robust Error Handling**: Powered by `thiserror` with typed API error bodies, status codes, and automatic 429 rate limit backoff parsing (`retry-after`).
- **Date/Time Integration**: First-class `time` crate (0.3) support for RFC3339 / ISO8601 formatting and parsing.
- **Strict Linting**: Clean builds under Clippy pedantic and rustdoc warnings (`-D warnings`).

---

## Installation

Add `brevo-v3` and `tokio` to your `Cargo.toml`:

```toml
[dependencies]
brevo-v3 = "0.1"
tokio = { version = "1.40", features = ["full"] }
```

---

## Quick Start

### 1. Send a Transactional Email

```rust
use brevo_v3::client::BrevoClient;
use brevo_v3::models::{SendTransacEmailRequest, SendTransacEmailRequestSender, SendTransacEmailRequestToItem};
use brevo_v3::types::Email;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let client = BrevoClient::new("xkeysib-your-api-key")?;

    let recipient = Email::new("alice@example.com")?;
    let sender = Email::new("notifications@mycompany.com")?;

    let request = SendTransacEmailRequest {
        subject: Some("Your Order Confirmation".to_string()),
        html_content: Some("<h1>Thank you for your order!</h1><p>Your items are on the way.</p>".to_string()),
        sender: Some(SendTransacEmailRequestSender {
            email: Some(sender),
            name: Some("My Company".to_string()),
            id: None,
        }),
        to: Some(vec![SendTransacEmailRequestToItem {
            email: recipient,
            name: Some("Alice".to_string()),
        }]),
        ..Default::default()
    };

    let response = client.transactional_emails().send_transac_email(&request).await?;
    println!("Email sent! Message ID: {:?}", response.message_id);

    Ok(())
}
```

### 2. Manage Contacts

```rust
use brevo_v3::client::BrevoClient;
use brevo_v3::models::CreateContactRequest;
use brevo_v3::types::{Email, ListId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrevoClient::new("xkeysib-your-api-key")?;

    let email = Email::new("bob@example.com")?;
    let request = CreateContactRequest {
        email: Some(email.clone()),
        list_ids: Some(vec![10, 20]),
        update_enabled: Some(true),
        ..Default::default()
    };

    let contact = client.contacts().create_contact(&request).await?;
    println!("Contact created: {:?}", contact.id);

    // Fetch contact details
    let info = client.contacts().get_contact_info(&email, None, None, None).await?;
    println!("Contact info: {:?}", info);

    Ok(())
}
```

### 3. Handle Rate Limits & API Errors

```rust
use brevo_v3::client::BrevoClient;
use brevo_v3::error::Error;

#[tokio::main]
async fn main() {
    let client = BrevoClient::new("xkeysib-your-api-key").unwrap();

    match client.account().get_account().await {
        Ok(account) => println!("Logged in as: {}", account.email),
        Err(Error::RateLimited { retry_after_seconds }) => {
            eprintln!("Rate limited! Retry after: {retry_after_seconds:?} seconds");
        }
        Err(Error::Api { status, code, message, .. }) => {
            eprintln!("Brevo API Error ({status}): {message} (code: {code:?})");
        }
        Err(e) => eprintln!("Unexpected error: {e}"),
    }
}
```

---

## API Resource Modules

All Brevo endpoints are accessible through typed namespaces on [`BrevoClient`]:

| Namespace | Accessor | Description |
|:---|:---|:---|
| **Account** | `client.account()` | Account profile, relays, and plan credits |
| **Contacts** | `client.contacts()` | Contact creation, attributes, lists, folders, segments |
| **Transactional Emails** | `client.transactional_emails()` | SMTP transactional messages, templates, logs |
| **Transactional SMS** | `client.transactional_sms()` | Single and batch transactional SMS |
| **WhatsApp** | `client.transactional_whats_app()` | Transactional WhatsApp messaging & templates |
| **Email Campaigns** | `client.email_campaigns()` | Marketing email campaign creation, scheduling, statistics |
| **SMS Campaigns** | `client.sms_campaigns()` | Marketing SMS campaign creation & delivery |
| **Deals (CRM)** | `client.deals()` | CRM pipeline deals management |
| **Companies (CRM)** | `client.companies()` | CRM company records |
| **Tasks & Notes** | `client.tasks()`, `client.notes()` | CRM task and note management |
| **Webhooks** | `client.webhooks()` | Webhook subscriptions and history |
| **Inbound Parsing** | `client.inbound_parsing()` | Inbound email events and webhooks |
| **Master Account** | `client.master_account()` | Corporate sub-accounts, SSO tokens, and permissions |

---

## Examples

Runnable examples can be found in the [`examples/`](https://github.com/ernestas-poskus/brevo-v3/tree/main/examples) directory:

```bash
BREVO_API_KEY="your-api-key" cargo run --example send_transactional_email
BREVO_API_KEY="your-api-key" cargo run --example manage_contacts
```

---

## Testing & Quality

To run tests using `cargo-nextest`:

```bash
cargo nextest run --all-features
```

To verify documentation without warnings:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

---

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
