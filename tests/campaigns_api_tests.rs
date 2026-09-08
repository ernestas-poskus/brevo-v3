use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use brevo_v3::types::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_email_campaign_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "id": 888
    });

    Mock::given(method("POST"))
        .and(path("/emailCampaigns"))
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

    let body = CreateEmailCampaignRequest {
        name: "Spring Sale Newsletter".to_string(),
        subject: Some("Special discounts inside".to_string()),
        sender: CreateEmailCampaignRequestSender {
            email: Some(Email::new("marketing@example.com").unwrap()),
            id: None,
            name: Some("Marketing".to_string()),
        },
        tag: None,
        html_content: Some("<h1>Spring Sale</h1>".to_string()),
        html_url: None,
        template_id: None,
        scheduled_at: None,
        subject_a: None,
        subject_b: None,
        split_rule: None,
        winner_criteria: None,
        winner_delay: None,
        email_expiration_date: None,
        initial_quota: None,
        inline_image_activation: None,
        ip_warmup_enable: None,
        preview_text: None,
        mirror_active: None,
        footer: None,
        header: None,
        utm_campaign: None,
        params: None,
        send_at_best_time: None,
        ab_testing: None,
        recipients: None,
        attachment_url: None,
        reply_to: None,
        to_field: None,
        increase_rate: None,
        unsubscription_page_id: None,
        update_form_id: None,
        utm_content: None,
        utm_term: None,
    };

    let res = client
        .email_campaigns()
        .create_email_campaign(&body)
        .await
        .unwrap();
    assert_eq!(res.get("id").and_then(|v| v.as_i64()), Some(888));
}

#[tokio::test]
async fn test_send_test_email_campaign_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/emailCampaigns/888/sendTest"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let body = json!({
        "emailTo": ["tester@example.com"]
    });

    let res = client
        .email_campaigns()
        .send_test_email(CampaignId::new(888), &body)
        .await;
    assert!(res.is_ok());
}
