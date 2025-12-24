//! EarningsFeed API client.
//!
//! This module provides the main [`EarningsFeed`] client for interacting
//! with the EarningsFeed API.

use std::sync::Arc;

use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::{ClientConfig, DEFAULT_BASE_URL, DEFAULT_TIMEOUT};
use crate::error::{Error, Result};
use crate::resources::{CompaniesResource, FilingsResource, InsiderResource, InstitutionalResource};

/// Version of this SDK (used in User-Agent header).
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client for the EarningsFeed API.
///
/// The client is the main entry point for interacting with the EarningsFeed API.
/// It handles authentication, request construction, and error handling.
///
/// # Example
///
/// ```rust,ignore
/// use earningsfeed::EarningsFeed;
///
/// #[tokio::main]
/// async fn main() -> Result<(), earningsfeed::Error> {
///     let client = EarningsFeed::new("your_api_key")?;
///
///     // Access resources via the client
///     let filings = client.filings().list(&Default::default()).await?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct EarningsFeed {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: Client,
    base_url: String,
}

impl EarningsFeed {
    /// Create a new client with the given API key.
    ///
    /// Uses default configuration (base URL: `https://earningsfeed.com`, timeout: 30s).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your EarningsFeed API key
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use earningsfeed::EarningsFeed;
    ///
    /// let client = EarningsFeed::new("your_api_key")?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty or if the HTTP client cannot be created.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::builder().api_key(api_key).build()?;
        Self::with_config(config)
    }

    /// Create a new client with custom configuration.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use earningsfeed::{EarningsFeed, ClientConfig};
    /// use std::time::Duration;
    ///
    /// let config = ClientConfig::builder()
    ///     .api_key("your_api_key")
    ///     .timeout(Duration::from_secs(60))
    ///     .build()?;
    ///
    /// let client = EarningsFeed::with_config(config)?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        // Authorization header
        let auth_value = format!("Bearer {}", config.api_key);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value)
                .map_err(|_| Error::Config("invalid API key format".into()))?,
        );

        // User-Agent header
        let user_agent = format!("earningsfeed-rust/{}", VERSION);
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&user_agent)
                .map_err(|_| Error::Config("invalid user agent".into()))?,
        );

        // Accept header
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let timeout = config.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let http = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()?;

        let base_url = config
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        Ok(Self {
            inner: Arc::new(ClientInner { http, base_url }),
        })
    }

    /// Create a configuration builder.
    ///
    /// Convenience method for creating a new [`ClientConfig`] builder.
    #[must_use]
    pub fn builder() -> crate::config::ClientConfigBuilder {
        ClientConfig::builder()
    }

    /// Get the base URL for API requests.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Access the filings resource.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let response = client.filings().list(&params).await?;
    /// let detail = client.filings().get("0000950170-24-000001").await?;
    /// ```
    #[must_use]
    pub fn filings(&self) -> FilingsResource<'_> {
        FilingsResource::new(self)
    }

    /// Access the insider transactions resource.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let response = client.insider().list(&params).await?;
    /// ```
    #[must_use]
    pub fn insider(&self) -> InsiderResource<'_> {
        InsiderResource::new(self)
    }

    /// Access the institutional holdings resource.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let response = client.institutional().list(&params).await?;
    /// ```
    #[must_use]
    pub fn institutional(&self) -> InstitutionalResource<'_> {
        InstitutionalResource::new(self)
    }

    /// Access the companies resource.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let company = client.companies().get(320193).await?;
    /// let results = client.companies().search(&params).await?;
    /// ```
    #[must_use]
    pub fn companies(&self) -> CompaniesResource<'_> {
        CompaniesResource::new(self)
    }

    /// Make a GET request to the API.
    ///
    /// This is an internal method used by resource implementations.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The response type (must implement `DeserializeOwned`)
    /// * `P` - The query parameters type (must implement `Serialize`)
    ///
    /// # Arguments
    ///
    /// * `path` - The API path (e.g., "/api/v1/filings")
    /// * `params` - Optional query parameters
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub(crate) async fn get<T, P>(&self, path: &str, params: Option<&P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.inner.base_url, path);

        let mut request = self.inner.http.get(&url);
        if let Some(p) = params {
            request = request.query(p);
        }

        let response = request.send().await?;
        let status = response.status();

        match status.as_u16() {
            200..=299 => {
                let body = response.json().await?;
                Ok(body)
            }
            401 => Err(Error::Authentication),
            404 => Err(Error::NotFound { path: path.into() }),
            429 => {
                let reset_at = response
                    .headers()
                    .get("X-RateLimit-Reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse().ok());
                Err(Error::RateLimit { reset_at })
            }
            400 => {
                let body: serde_json::Value = response.json().await.unwrap_or_default();
                let message = body["error"]
                    .as_str()
                    .unwrap_or("Invalid request")
                    .to_string();
                Err(Error::Validation { message })
            }
            _ => {
                let body: serde_json::Value = response.json().await.unwrap_or_default();
                Err(Error::Api {
                    status: status.as_u16(),
                    message: body["error"]
                        .as_str()
                        .unwrap_or("Unknown error")
                        .to_string(),
                    code: body["code"].as_str().map(String::from),
                })
            }
        }
    }
}

impl std::fmt::Debug for EarningsFeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EarningsFeed")
            .field("base_url", &self.inner.base_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_api_key() {
        let client = EarningsFeed::new("test_api_key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_with_empty_api_key_fails() {
        let result = EarningsFeed::new("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_with_config() {
        let config = ClientConfig::builder()
            .api_key("test_key")
            .base_url("https://custom.example.com")
            .build()
            .unwrap();

        let client = EarningsFeed::with_config(config).unwrap();
        assert_eq!(client.base_url(), "https://custom.example.com");
    }

    #[test]
    fn test_default_base_url() {
        let client = EarningsFeed::new("test_key").unwrap();
        assert_eq!(client.base_url(), "https://earningsfeed.com");
    }

    #[test]
    fn test_client_is_clone() {
        let client = EarningsFeed::new("test_key").unwrap();
        let cloned = client.clone();
        assert_eq!(cloned.base_url(), client.base_url());
    }

    #[test]
    fn test_client_is_debug() {
        let client = EarningsFeed::new("test_key").unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("EarningsFeed"));
        assert!(debug_str.contains("base_url"));
    }

    #[test]
    fn test_builder_convenience_method() {
        let config = EarningsFeed::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test_key");
    }

    #[test]
    fn test_client_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EarningsFeed>();
    }
}
