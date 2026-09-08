//! Example: Send a Transactional Email using Brevo v3 SDK.
//!
//! Run with:
//! ```bash
//! BREVO_API_KEY="your-api-key" cargo run --example send_transactional_email
//! ```

use brevo_v3::client::BrevoClient;
use brevo_v3::models::{
    SendTransacEmailRequest, SendTransacEmailRequestSender, SendTransacEmailRequestToItem,
};
use brevo_v3::types::Email;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("BREVO_API_KEY").expect("BREVO_API_KEY environment variable required");
    let client = BrevoClient::new(api_key)?;

    let recipient_email = Email::new("recipient@example.com")?;
    let sender_email = Email::new("sender@yourdomain.com")?;

    let email_request = SendTransacEmailRequest {
        subject: Some("Hello from Brevo Rust SDK".to_string()),
        html_content: Some("<h1>Welcome!</h1><p>Sent with type-safe Rust SDK.</p>".to_string()),
        sender: Some(SendTransacEmailRequestSender {
            email: Some(sender_email),
            name: Some("Rust Notifications".to_string()),
            id: None,
        }),
        to: Some(vec![SendTransacEmailRequestToItem {
            email: recipient_email,
            name: Some("Valued Customer".to_string()),
            contact_pixel_tracking_consent: None,
        }]),
        attachment: None,
        batch_id: None,
        bcc: None,
        cc: None,
        headers: None,
        message_versions: None,
        params: None,
        reply_to: None,
        scheduled_at: None,
        tags: None,
        template_id: None,
        text_content: None,
    };

    println!("Sending email...");
    let response = client
        .transactional_emails()
        .send_transac_email(&email_request)
        .await?;
    println!(
        "Email sent successfully! Message ID: {:?}",
        response.message_id
    );

    Ok(())
}
