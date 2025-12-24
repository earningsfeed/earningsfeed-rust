//! Error types for the EarningsFeed client.
//!
//! This module provides a comprehensive error hierarchy matching the
//! Node.js and Python SDK error types.

use std::time::Duration;
use thiserror::Error;

/// Error types for the EarningsFeed client.
///
/// These errors mirror the error types in the Node.js and Python SDKs
/// for consistency across all official client libraries.
#[derive(Error, Debug)]
pub enum Error {
    /// Authentication failed - API key is missing or invalid.
    ///
    /// This error is returned when the API responds with HTTP 401.
    #[error("authentication failed: invalid or missing API key")]
    Authentication,

    /// Rate limit exceeded.
    ///
    /// This error is returned when the API responds with HTTP 429.
    /// The `reset_at` field contains the Unix timestamp when the rate limit resets,
    /// extracted from the `X-RateLimit-Reset` header.
    #[error("rate limit exceeded (resets at: {reset_at:?})")]
    RateLimit {
        /// Unix timestamp when rate limit resets.
        reset_at: Option<u64>,
    },

    /// Requested resource was not found.
    ///
    /// This error is returned when the API responds with HTTP 404.
    #[error("resource not found: {path}")]
    NotFound {
        /// Path that was not found.
        path: String,
    },

    /// Request validation failed.
    ///
    /// This error is returned when the API responds with HTTP 400.
    #[error("validation error: {message}")]
    Validation {
        /// Validation error message.
        message: String,
    },

    /// General API error.
    ///
    /// This error is returned for other HTTP error status codes (4xx/5xx).
    #[error("API error ({status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
        /// Error code from the API (e.g., "INTERNAL_ERROR").
        code: Option<String>,
    },

    /// Request timed out.
    #[error("request timeout after {0:?}")]
    Timeout(Duration),

    /// HTTP transport error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration.
    #[error("configuration error: {0}")]
    Config(String),
}

/// A specialized `Result` type for EarningsFeed operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_error_display() {
        let err = Error::Authentication;
        assert_eq!(
            err.to_string(),
            "authentication failed: invalid or missing API key"
        );
    }

    #[test]
    fn test_rate_limit_error_with_reset_at() {
        let err = Error::RateLimit {
            reset_at: Some(1703520000),
        };
        assert_eq!(
            err.to_string(),
            "rate limit exceeded (resets at: Some(1703520000))"
        );
    }

    #[test]
    fn test_rate_limit_error_without_reset_at() {
        let err = Error::RateLimit { reset_at: None };
        assert_eq!(err.to_string(), "rate limit exceeded (resets at: None)");
    }

    #[test]
    fn test_not_found_error_display() {
        let err = Error::NotFound {
            path: "/api/v1/filings/invalid".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "resource not found: /api/v1/filings/invalid"
        );
    }

    #[test]
    fn test_validation_error_display() {
        let err = Error::Validation {
            message: "limit must be between 1 and 100".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "validation error: limit must be between 1 and 100"
        );
    }

    #[test]
    fn test_api_error_display() {
        let err = Error::Api {
            status: 500,
            message: "Internal server error".to_string(),
            code: Some("INTERNAL_ERROR".to_string()),
        };
        assert_eq!(err.to_string(), "API error (500): Internal server error");
    }

    #[test]
    fn test_api_error_without_code() {
        let err = Error::Api {
            status: 503,
            message: "Service unavailable".to_string(),
            code: None,
        };
        assert_eq!(err.to_string(), "API error (503): Service unavailable");
    }

    #[test]
    fn test_timeout_error_display() {
        let err = Error::Timeout(Duration::from_secs(30));
        assert_eq!(err.to_string(), "request timeout after 30s");
    }

    #[test]
    fn test_config_error_display() {
        let err = Error::Config("invalid API key format".to_string());
        assert_eq!(err.to_string(), "configuration error: invalid API key format");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }
}
