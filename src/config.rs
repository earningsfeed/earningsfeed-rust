//! Client configuration for the EarningsFeed SDK.
//!
//! This module provides the [`ClientConfig`] struct and its builder
//! for configuring the HTTP client.

use std::time::Duration;

use crate::error::{Error, Result};

/// Default base URL for the EarningsFeed API.
pub const DEFAULT_BASE_URL: &str = "https://earningsfeed.com";

/// Default request timeout (30 seconds).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Configuration for the EarningsFeed client.
///
/// Use [`ClientConfig::builder()`] to create a new configuration.
///
/// # Example
///
/// ```rust
/// use earningsfeed::ClientConfig;
/// use std::time::Duration;
///
/// let config = ClientConfig::builder()
///     .api_key("your_api_key")
///     .timeout(Duration::from_secs(60))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authentication.
    pub api_key: String,
    /// Base URL for API requests.
    pub base_url: Option<String>,
    /// Request timeout.
    pub timeout: Option<Duration>,
}

impl ClientConfig {
    /// Create a new configuration builder.
    #[must_use]
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

/// Builder for [`ClientConfig`].
#[derive(Debug, Default)]
pub struct ClientConfigBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl ClientConfigBuilder {
    /// Set the API key.
    ///
    /// This is required for authentication.
    #[must_use]
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the base URL for API requests.
    ///
    /// Defaults to `https://earningsfeed.com` if not specified.
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout.
    ///
    /// Defaults to 30 seconds if not specified.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is not set.
    pub fn build(self) -> Result<ClientConfig> {
        let api_key = self
            .api_key
            .ok_or_else(|| Error::Config("API key is required".into()))?;

        if api_key.is_empty() {
            return Err(Error::Config("API key cannot be empty".into()));
        }

        Ok(ClientConfig {
            api_key,
            base_url: self.base_url,
            timeout: self.timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_api_key() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test_key");
        assert!(config.base_url.is_none());
        assert!(config.timeout.is_none());
    }

    #[test]
    fn test_builder_with_all_options() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .base_url("https://custom.example.com")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test_key");
        assert_eq!(
            config.base_url,
            Some("https://custom.example.com".to_string())
        );
        assert_eq!(config.timeout, Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_builder_without_api_key_fails() {
        let result = ClientConfig::builder().build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Config(_)));
        assert_eq!(err.to_string(), "configuration error: API key is required");
    }

    #[test]
    fn test_builder_with_empty_api_key_fails() {
        let result = ClientConfig::builder().api_key("").build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Config(_)));
        assert_eq!(
            err.to_string(),
            "configuration error: API key cannot be empty"
        );
    }

    #[test]
    fn test_builder_accepts_string() {
        let config = ClientConfig::builder()
            .api_key(String::from("test_key"))
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test_key");
    }

    #[test]
    fn test_builder_accepts_str() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test_key");
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_BASE_URL, "https://earningsfeed.com");
        assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));
    }

    #[test]
    fn test_config_is_clone() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let cloned = config.clone();
        assert_eq!(cloned.api_key, config.api_key);
    }

    #[test]
    fn test_config_is_debug() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ClientConfig"));
        assert!(debug_str.contains("test_key"));
    }
}
