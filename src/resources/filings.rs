//! Filings resource for accessing SEC filings.
//!
//! This module provides methods for listing, retrieving, and iterating
//! over SEC filings.

use async_stream::try_stream;
use futures::Stream;

use crate::client::EarningsFeed;
use crate::error::Result;
use crate::models::{Filing, FilingDetail, ListFilingsParams, PaginatedResponse};

/// Resource for accessing SEC filings.
///
/// Obtain an instance via [`EarningsFeed::filings()`].
///
/// # Example
///
/// ```rust,ignore
/// use earningsfeed::{EarningsFeed, ListFilingsParams};
///
/// let client = EarningsFeed::new("your_api_key")?;
///
/// // List filings
/// let params = ListFilingsParams::builder()
///     .ticker("AAPL")
///     .limit(10)
///     .build();
/// let response = client.filings().list(&params).await?;
///
/// // Get a specific filing
/// let filing = client.filings().get("0000950170-24-000001").await?;
/// ```
pub struct FilingsResource<'a> {
    client: &'a EarningsFeed,
}

impl<'a> FilingsResource<'a> {
    /// Create a new filings resource.
    pub(crate) fn new(client: &'a EarningsFeed) -> Self {
        Self { client }
    }

    /// List filings with optional filters.
    ///
    /// Returns a paginated response. Use [`iter`](Self::iter) for automatic pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let params = ListFilingsParams::builder()
    ///     .ticker("AAPL")
    ///     .forms(vec!["10-K", "10-Q"])
    ///     .limit(25)
    ///     .build();
    ///
    /// let response = client.filings().list(&params).await?;
    /// for filing in response.items {
    ///     println!("{}: {}", filing.form_type, filing.title);
    /// }
    /// ```
    pub async fn list(&self, params: &ListFilingsParams) -> Result<PaginatedResponse<Filing>> {
        self.client.get("/api/v1/filings", Some(params)).await
    }

    /// Get a specific filing by accession number.
    ///
    /// Returns detailed filing information including documents and roles.
    ///
    /// # Arguments
    ///
    /// * `accession_number` - The SEC accession number (e.g., "0000950170-24-000001")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let filing = client.filings().get("0000950170-24-000001").await?;
    /// println!("Title: {}", filing.title);
    /// println!("Documents: {:?}", filing.documents.len());
    /// ```
    pub async fn get(&self, accession_number: &str) -> Result<FilingDetail> {
        let path = format!("/api/v1/filings/{}", accession_number);
        self.client.get::<FilingDetail, ()>(&path, None).await
    }

    /// Iterate over all filings matching the given parameters.
    ///
    /// Returns an async stream that automatically handles pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering (cursor will be managed automatically)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use futures::StreamExt;
    ///
    /// let params = ListFilingsParams::builder()
    ///     .ticker("AAPL")
    ///     .build();
    ///
    /// let mut stream = client.filings().iter(params);
    /// while let Some(result) = stream.next().await {
    ///     let filing = result?;
    ///     println!("{}: {}", filing.form_type, filing.title);
    /// }
    /// ```
    pub fn iter(&self, params: ListFilingsParams) -> impl Stream<Item = Result<Filing>> + '_ {
        try_stream! {
            let mut current_params = params;

            loop {
                let response = self.list(&current_params).await?;

                for item in response.items {
                    yield item;
                }

                if !response.has_more {
                    break;
                }

                match response.next_cursor {
                    Some(cursor) => {
                        current_params.cursor = Some(cursor);
                    }
                    None => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FilingStatus;
    use futures::StreamExt;
    use std::pin::pin;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_client(mock_server: &MockServer) -> EarningsFeed {
        let config = EarningsFeed::builder()
            .api_key("test_key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();
        EarningsFeed::with_config(config).unwrap()
    }

    #[tokio::test]
    async fn test_list_filings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0000950170-24-000001",
                        "cik": 320193,
                        "formType": "10-K",
                        "filedAt": "2024-01-15T16:30:00Z",
                        "provisional": false,
                        "sizeBytes": 12345,
                        "url": "https://www.sec.gov/...",
                        "title": "Form 10-K",
                        "status": "final",
                        "updatedAt": "2024-01-15T17:00:00Z",
                        "sortedAt": "2024-01-15T16:30:00Z"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::default();
        let response = client.filings().list(&params).await.unwrap();

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].accession_number, "0000950170-24-000001");
        assert_eq!(response.items[0].form_type, "10-K");
        assert!(!response.has_more);
    }

    #[tokio::test]
    async fn test_list_filings_with_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .and(query_param("ticker", "AAPL"))
            .and(query_param("limit", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::builder()
            .ticker("AAPL")
            .limit(10)
            .build();
        let response = client.filings().list(&params).await.unwrap();

        assert!(response.items.is_empty());
    }

    #[tokio::test]
    async fn test_list_filings_with_forms() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .and(query_param("forms", "10-K,10-Q"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::builder()
            .forms(vec!["10-K", "10-Q"])
            .build();
        let response = client.filings().list(&params).await.unwrap();

        assert!(response.items.is_empty());
    }

    #[tokio::test]
    async fn test_list_filings_with_status() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .and(query_param("status", "final"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::builder()
            .status(FilingStatus::Final)
            .build();
        let response = client.filings().list(&params).await.unwrap();

        assert!(response.items.is_empty());
    }

    #[tokio::test]
    async fn test_get_filing() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings/0000950170-24-000001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "accessionNumber": "0000950170-24-000001",
                "cik": 320193,
                "formType": "10-K",
                "filedAt": "2024-01-15T16:30:00Z",
                "provisional": false,
                "title": "Form 10-K",
                "url": "https://www.sec.gov/...",
                "sizeBytes": 12345,
                "documents": [
                    {
                        "seq": 1,
                        "filename": "aapl-20231230.htm",
                        "docType": "10-K",
                        "isPrimary": true
                    }
                ],
                "roles": [
                    {
                        "cik": 320193,
                        "role": "filer"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let filing = client.filings().get("0000950170-24-000001").await.unwrap();

        assert_eq!(filing.accession_number, "0000950170-24-000001");
        assert_eq!(filing.form_type, "10-K");
        assert_eq!(filing.documents.len(), 1);
        assert_eq!(filing.roles.len(), 1);
    }

    #[tokio::test]
    async fn test_get_filing_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings/invalid"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Filing not found"
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let result = client.filings().get("invalid").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::error::Error::NotFound { .. }));
    }

    #[tokio::test]
    async fn test_iter_filings_single_page() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0000950170-24-000001",
                        "cik": 320193,
                        "formType": "10-K",
                        "filedAt": "2024-01-15T16:30:00Z",
                        "provisional": false,
                        "sizeBytes": 12345,
                        "url": "https://www.sec.gov/...",
                        "title": "Form 10-K",
                        "status": "final",
                        "updatedAt": "2024-01-15T17:00:00Z",
                        "sortedAt": "2024-01-15T16:30:00Z"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::default();
        let filings_resource = client.filings();
        let mut stream = pin!(filings_resource.iter(params));

        let mut count = 0;
        while let Some(result) = stream.next().await {
            let filing = result.unwrap();
            assert_eq!(filing.accession_number, "0000950170-24-000001");
            count += 1;
        }

        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_iter_filings_multiple_pages() {
        let mock_server = MockServer::start().await;

        // First page
        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0000950170-24-000001",
                        "cik": 320193,
                        "formType": "10-K",
                        "filedAt": "2024-01-15T16:30:00Z",
                        "provisional": false,
                        "sizeBytes": 12345,
                        "url": "https://www.sec.gov/...",
                        "title": "Form 10-K Page 1",
                        "status": "final",
                        "updatedAt": "2024-01-15T17:00:00Z",
                        "sortedAt": "2024-01-15T16:30:00Z"
                    }
                ],
                "nextCursor": "cursor_page_2",
                "hasMore": true
            })))
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;

        // Second page
        Mock::given(method("GET"))
            .and(path("/api/v1/filings"))
            .and(query_param("cursor", "cursor_page_2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0000950170-24-000002",
                        "cik": 320193,
                        "formType": "10-Q",
                        "filedAt": "2024-01-14T16:30:00Z",
                        "provisional": false,
                        "sizeBytes": 12345,
                        "url": "https://www.sec.gov/...",
                        "title": "Form 10-Q Page 2",
                        "status": "final",
                        "updatedAt": "2024-01-14T17:00:00Z",
                        "sortedAt": "2024-01-14T16:30:00Z"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListFilingsParams::default();
        let filings_resource = client.filings();
        let stream = pin!(filings_resource.iter(params));

        let filings: Vec<_> = stream.collect::<Vec<_>>().await;

        assert_eq!(filings.len(), 2);
        assert!(filings[0].is_ok());
        assert!(filings[1].is_ok());
        assert_eq!(filings[0].as_ref().unwrap().form_type, "10-K");
        assert_eq!(filings[1].as_ref().unwrap().form_type, "10-Q");
    }
}
