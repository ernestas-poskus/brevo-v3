//! Core HTTP client, configuration, and request execution logic.

use crate::error::{Error, Result};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// Default Brevo API base endpoint.
pub const DEFAULT_BASE_URL: &str = "https://api.brevo.com/v3";

/// Configuration options for building a [`BrevoClient`].
#[derive(Debug, Clone)]
pub struct BrevoConfig {
    /// Base URL of the Brevo API.
    pub base_url: Url,
    /// Brevo `api-key` header value.
    pub api_key: Option<String>,
    /// Brevo `partner-key` header value.
    pub partner_key: Option<String>,
    /// HTTP request timeout.
    pub timeout: Duration,
    /// Custom User-Agent header.
    pub user_agent: String,
}

impl Default for BrevoConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("valid default base URL"),
            api_key: None,
            partner_key: None,
            timeout: Duration::from_secs(30),
            user_agent: format!("brevo-rust-sdk/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

/// Builder for [`BrevoClient`].
#[derive(Debug, Default)]
pub struct BrevoClientBuilder {
    config: BrevoConfig,
}

impl BrevoClientBuilder {
    /// Sets the primary API key for Brevo.
    #[must_use]
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.config.api_key = Some(key.into());
        self
    }

    /// Sets the partner key if authenticating as an authorized partner.
    #[must_use]
    pub fn partner_key(mut self, key: impl Into<String>) -> Self {
        self.config.partner_key = Some(key.into());
        self
    }

    /// Sets a custom base URL (e.g. for mock tests or enterprise routing).
    pub fn base_url(mut self, url: impl AsRef<str>) -> Result<Self> {
        self.config.base_url = Url::parse(url.as_ref())?;
        Ok(self)
    }

    /// Sets the HTTP request timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Sets a custom user agent string.
    #[must_use]
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.config.user_agent = ua.into();
        self
    }

    /// Builds and returns the configured [`BrevoClient`].
    pub fn build(self) -> Result<BrevoClient> {
        if self.config.api_key.is_none() && self.config.partner_key.is_none() {
            return Err(Error::Config(
                "Either `api_key` or `partner_key` must be specified for BrevoClient".to_string(),
            ));
        }

        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let ua_val = HeaderValue::from_str(&self.config.user_agent)
            .map_err(|e| Error::Config(format!("Invalid User-Agent header value: {e}")))?;
        default_headers.insert(USER_AGENT, ua_val);

        if let Some(ref key) = self.config.api_key {
            let mut key_val = HeaderValue::from_str(key)
                .map_err(|e| Error::Config(format!("Invalid api-key header: {e}")))?;
            key_val.set_sensitive(true);
            default_headers.insert("api-key", key_val);
        }

        if let Some(ref key) = self.config.partner_key {
            let mut key_val = HeaderValue::from_str(key)
                .map_err(|e| Error::Config(format!("Invalid partner-key header: {e}")))?;
            key_val.set_sensitive(true);
            default_headers.insert("partner-key", key_val);
        }

        let http = reqwest::Client::builder()
            .default_headers(default_headers)
            .timeout(self.config.timeout)
            .build()?;

        Ok(BrevoClient {
            inner: Arc::new(BrevoClientInner {
                http,
                base_url: self.config.base_url,
            }),
        })
    }
}

#[derive(Debug)]
struct BrevoClientInner {
    http: reqwest::Client,
    base_url: Url,
}

/// Asynchronous client for interacting with the Brevo v3 API.
#[derive(Debug, Clone)]
pub struct BrevoClient {
    inner: Arc<BrevoClientInner>,
}

impl BrevoClient {
    /// Creates a new `BrevoClient` initialized with the given API key.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Initializes a [`BrevoClientBuilder`] for customization.
    #[must_use]
    pub fn builder() -> BrevoClientBuilder {
        BrevoClientBuilder::default()
    }

    /// Returns a reference to the configured base URL.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// Returns a reference to the underlying reqwest client.
    #[must_use]
    pub fn http(&self) -> &reqwest::Client {
        &self.inner.http
    }

    /// Internal request dispatcher that executes HTTP requests and parses Brevo error formats.
    pub async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::Request,
    ) -> Result<T> {
        let response = self.inner.http.execute(request).await?;
        let status = response.status();

        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                // Return default/empty for 204
                let empty_json =
                    serde_json::from_str("{}").or_else(|_| serde_json::from_str("null"))?;
                return Ok(empty_json);
            }
            let body = response.bytes().await?;
            if body.is_empty() {
                let empty_json =
                    serde_json::from_str("{}").or_else(|_| serde_json::from_str("null"))?;
                return Ok(empty_json);
            }
            let parsed = serde_json::from_slice(&body)?;
            return Ok(parsed);
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            return Err(Error::RateLimited {
                retry_after_seconds: retry_after,
            });
        }

        let raw_text = response.text().await.unwrap_or_default();
        let (code, message) = if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw_text)
        {
            let code = val
                .get("code")
                .and_then(|c| c.as_str())
                .map(ToString::to_string);
            let msg = val
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or(&raw_text)
                .to_string();
            (code, msg)
        } else {
            (None, raw_text.clone())
        };

        Err(Error::Api {
            status,
            code,
            message,
            raw_body: Some(raw_text),
        })
    }
}
