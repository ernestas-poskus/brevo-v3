use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use brevo_v3::types::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_contact_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "id": 123456
    });

    Mock::given(method("POST"))
        .and(path("/contacts"))
        .and(header("api-key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let email = Email::new("newuser@example.com").unwrap();
    let body = CreateContactRequest {
        email: Some(email),
        email_blacklisted: Some(false),
        sms_blacklisted: Some(false),
        list_ids: Some(vec![1, 2]),
        update_enabled: Some(true),
        smtp_blacklist_sender: None,
        attributes: None,
        ext_id: None,
        force_merge: None,
        get_id: None,
    };

    let res = client.contacts().create_contact(&body).await.unwrap();
    assert_eq!(res.id, Some(123456));
}

#[tokio::test]
async fn test_get_contact_info_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "email": "user@example.com",
        "id": 999,
        "emailBlacklisted": false,
        "smsBlacklisted": false,
        "createdAt": "2026-09-08T10:00:00.000Z",
        "modifiedAt": "2026-09-08T10:30:00.000Z",
        "listIds": [10, 20],
        "statistics": {
            "delivered": []
        }
    });

    Mock::given(method("GET"))
        .and(path("/contacts/user@example.com"))
        .and(header("api-key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let email = Email::new("user@example.com").unwrap();
    let contact = client
        .contacts()
        .get_contact_info(&email, None, None, None)
        .await
        .unwrap();

    assert!(contact.statistics.delivered.is_some());
}
