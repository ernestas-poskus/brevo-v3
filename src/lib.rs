//! # Brevo v3 Rust SDK
//!
//! An idiomatic, asynchronous, strongly typed Rust client for the Brevo (Sendinblue) v3 API.
//!
//! ## Features
//! - Built on `tokio` (1.x) and `reqwest` (0.12+ with hyper & rustls).
//! - Robust error handling with `thiserror`.
//! - RFC3339 / ISO8601 date-time support with `time` crate.
//! - Strongly typed IDs and email domain invariants.
//! - Complete coverage of 207 endpoints across 28 resource tags.

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod types;

// Re-exports
pub use client::{BrevoClient, BrevoClientBuilder, BrevoConfig, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use types::*;

mod client_accessors;
