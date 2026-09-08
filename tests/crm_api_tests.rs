use brevo_v3::client::BrevoClient;
use brevo_v3::models::*;
use brevo_v3::types::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_deal_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "id": "deal_12345"
    });

    Mock::given(method("POST"))
        .and(path("/crm/deals"))
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

    let body = PostCrmDealsRequest {
        name: "Enterprise Renewal 2026".to_string(),
        attributes: None,
        linked_companies_ids: None,
        linked_contacts_ids: None,
    };

    let res = client.deals().post_crm_deals(&body).await.unwrap();
    assert_eq!(res.id, "deal_12345");
}

#[tokio::test]
async fn test_get_deal_mock() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "id": "deal_12345",
        "attributes": {
            "deal_name": "Enterprise Renewal 2026",
            "amount": 50000
        }
    });

    Mock::given(method("GET"))
        .and(path("/crm/deals/deal_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let deal_id = DealId::new("deal_12345");
    let res = client.deals().get_crm_deals_id(&deal_id).await.unwrap();
    assert_eq!(res.get("id").and_then(|v| v.as_str()), Some("deal_12345"));
}
