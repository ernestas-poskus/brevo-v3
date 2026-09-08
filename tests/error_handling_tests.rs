use brevo_v3::client::BrevoClient;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_error_400_bad_request() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "code": "invalid_parameter",
        "message": "Invalid email address format"
    });

    Mock::given(method("GET"))
        .and(path("/account"))
        .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let err = client.account().get_account().await.unwrap_err();
    match err {
        brevo_v3::error::Error::Api {
            status,
            code,
            message,
            ..
        } => {
            assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(code, Some("invalid_parameter".to_string()));
            assert_eq!(message, "Invalid email address format");
        }
        other => panic!("Expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_error_401_unauthorized() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "code": "unauthorized",
        "message": "Key not found"
    });

    Mock::given(method("GET"))
        .and(path("/account"))
        .respond_with(ResponseTemplate::new(401).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("bad-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let err = client.account().get_account().await.unwrap_err();
    match err {
        brevo_v3::error::Error::Api { status, code, .. } => {
            assert_eq!(status, reqwest::StatusCode::UNAUTHORIZED);
            assert_eq!(code, Some("unauthorized".to_string()));
        }
        other => panic!("Expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_error_500_fallback() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/account"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Gateway Error"))
        .mount(&mock_server)
        .await;

    let client = BrevoClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .unwrap()
        .build()
        .unwrap();

    let err = client.account().get_account().await.unwrap_err();
    match err {
        brevo_v3::error::Error::Api {
            status, message, ..
        } => {
            assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(message, "Internal Gateway Error");
        }
        other => panic!("Expected Api error, got {other:?}"),
    }
}
