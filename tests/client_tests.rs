use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use brevo_v3::types::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_email_invariant() {
    assert!(Email::new("valid@example.com").is_ok());
    assert!(Email::new("invalid-email").is_err());
    assert!(Email::new("").is_err());
    assert!(Email::new("   ").is_err());
}

#[test]
fn test_id_newtypes() {
    let cid = ContactId::new(42);
    assert_eq!(cid.get(), 42);
    assert_eq!(cid.to_string(), "42");

    let mid = MessageId::new("msg_123");
    assert_eq!(mid.as_str(), "msg_123");
}

#[tokio::test]
async fn test_get_account_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "organization_id": "org_123",
        "email": "alex.morgan@example.com",
        "firstName": "Alex",
        "lastName": "Morgan",
        "companyName": "Acme Marketing",
        "enterprise": false,
        "user_id": 12345,
        "plan": [
            {
                "credits": 250,
                "creditsType": "sendLimit",
                "type": "free"
            }
        ],
        "relay": {
            "enabled": true,
            "data": {
                "userName": "alex.morgan@example.com",
                "relay": "smtp-relay.brevo.com",
                "port": 587
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/account"))
        .and(header("api-key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-api-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let account = client.account().get_account().await.unwrap();
    assert_eq!(account.first_name, "Alex");
    assert_eq!(account.company_name, "Acme Marketing");
    assert_eq!(account.organization_id, "org_123");
}

#[tokio::test]
async fn test_send_transac_email_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "messageId": "<2026090812345@smtp-relay.brevo.com>"
    });

    Mock::given(method("POST"))
        .and(path("/smtp/email"))
        .and(header("api-key", "test-api-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-api-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let body = SendTransacEmailRequest {
        subject: Some("Hello from Rust SDK".to_string()),
        html_content: Some("<p>Welcome to Brevo!</p>".to_string()),
        sender: None,
        to: None,
        bcc: None,
        cc: None,
        text_content: None,
        reply_to: None,
        attachment: None,
        headers: None,
        template_id: None,
        params: None,
        message_versions: None,
        tags: None,
        scheduled_at: None,
        batch_id: None,
    };

    let result = client
        .transactional_emails()
        .send_transac_email(&body)
        .await
        .unwrap();
    assert_eq!(
        result.message_id,
        Some("<2026090812345@smtp-relay.brevo.com>".to_string())
    );
}

#[tokio::test]
async fn test_rate_limit_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/account"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "15")
                .set_body_string("Rate limit exceeded"),
        )
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-api-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let err = client.account().get_account().await.unwrap_err();
    match err {
        brevo_v3::error::Error::RateLimited {
            retry_after_seconds,
        } => {
            assert_eq!(retry_after_seconds, Some(15));
        }
        other => panic!("Expected RateLimited error, got {other:?}"),
    }
}
