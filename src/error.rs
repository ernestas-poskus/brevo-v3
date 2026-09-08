//! Error definitions for the Brevo v3 SDK.

/// Primary error type returned by the Brevo SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error originating from the underlying HTTP transport (reqwest).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// Error during JSON payload serialization or deserialization.
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Structured error response returned by the Brevo v3 API.
    #[error("Brevo API error ({status}): {message} (code: {code:?})")]
    Api {
        /// HTTP status code.
        status: reqwest::StatusCode,
        /// Machine-readable error code if provided by Brevo.
        code: Option<String>,
        /// Human-readable error message.
        message: String,
        /// Raw JSON response body if available.
        raw_body: Option<String>,
    },

    /// The client was rate limited by Brevo (HTTP 429).
    #[error("Rate limited (HTTP 429): retry after {retry_after_seconds:?}s")]
    RateLimited {
        /// Suggested retry-after delay in seconds if indicated in response headers.
        retry_after_seconds: Option<u64>,
    },

    /// Invalid client configuration or missing credentials.
    #[error("Client configuration error: {0}")]
    Config(String),

    /// Domain invariant violation or validation error.
    #[error("Domain invariant violation: {0}")]
    Invariant(String),

    /// URL parsing error.
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),
}

/// A specialized `Result` type for Brevo API operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;
