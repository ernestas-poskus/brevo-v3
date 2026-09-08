//! Example: Create and fetch a Contact using Brevo v3 SDK.
//!
//! Run with:
//! ```bash
//! BREVO_API_KEY="your-api-key" cargo run --example manage_contacts
//! ```

use brevo_v3::client::BrevoClient;
use brevo_v3::models::CreateContactRequest;
use brevo_v3::types::Email;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("BREVO_API_KEY").expect("BREVO_API_KEY environment variable required");
    let client = BrevoClient::new(api_key)?;

    let target_email = Email::new("developer@example.com")?;

    let mut attrs = HashMap::new();
    attrs.insert(
        "FIRSTNAME".to_string(),
        serde_json::Value::String("Developer".to_string()),
    );
    attrs.insert(
        "LASTNAME".to_string(),
        serde_json::Value::String("Rustacean".to_string()),
    );

    let new_contact = CreateContactRequest {
        email: Some(target_email.clone()),
        attributes: Some(attrs),
        update_enabled: Some(true),
        email_blacklisted: None,
        ext_id: None,
        force_merge: None,
        get_id: None,
        list_ids: None,
        sms_blacklisted: None,
        smtp_blacklist_sender: None,
    };

    println!("Creating contact: {}", target_email);
    let create_res = client.contacts().create_contact(&new_contact).await?;
    println!("Contact created or updated with ID: {:?}", create_res.id);

    println!("Fetching contact info...");
    let details = client
        .contacts()
        .get_contact_info(&target_email, None, None, None)
        .await?;
    println!("Contact details: {:?}", details);

    Ok(())
}
