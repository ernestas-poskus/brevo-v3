use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_send_transactional_sms_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "reference": "ref_sms_123",
        "messageId": 456789,
        "smsCount": 1,
        "usedCredits": 1.0,
        "remainingCredits": 99.0
    });

    Mock::given(method("POST"))
        .and(path("/transactionalSMS/sms"))
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

    let body = json!({
        "sender": "Brevo",
        "recipient": "33689965433",
        "content": "Your verification code is 1234"
    });

    let res = client
        .transactional_sms()
        .send_transac_sms(&body)
        .await
        .unwrap();
    assert_eq!(res.reference, "ref_sms_123");
    assert_eq!(res.message_id, 456789);
}

#[tokio::test]
async fn test_send_smtp_email_full_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "messageId": "<smtp-test-msg-001@brevo.com>"
    });

    Mock::given(method("POST"))
        .and(path("/smtp/email"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let body = SendTransacEmailRequest {
        subject: Some("Invoice Attached".to_string()),
        html_content: Some("<p>Please find your invoice attached.</p>".to_string()),
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

    let res = client
        .transactional_emails()
        .send_transac_email(&body)
        .await
        .unwrap();
    assert_eq!(
        res.message_id,
        Some("<smtp-test-msg-001@brevo.com>".to_string())
    );
}
