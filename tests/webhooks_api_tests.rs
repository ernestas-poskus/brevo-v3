use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use brevo_v3::types::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_webhook_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "id": 777
    });

    Mock::given(method("POST"))
        .and(path("/webhooks"))
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

    let body = CreateWebhookRequest {
        url: "https://example.com/webhook".to_string(),
        description: Some("Transactional Delivery Webhook".to_string()),
        events: Some(vec![CreateWebhookRequestEventsItem::Delivered]),
        type_: Some(CreateWebhookRequestType::Transactional),
        channel: None,
        domain: None,
        batched: None,
        auth: None,
        headers: None,
    };

    let res = client.webhooks().create_webhook(&body).await.unwrap();
    assert_eq!(res.get("id").and_then(|v| v.as_i64()), Some(777));
}

#[tokio::test]
async fn test_delete_webhook_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/webhooks/777"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let res = client.webhooks().delete_webhook(WebhookId::new(777)).await;
    assert!(res.is_ok());
}
