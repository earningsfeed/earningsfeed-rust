//! Companies resource.
//!
//! This module provides methods for searching and retrieving
//! company profiles.

use async_stream::try_stream;
use futures::Stream;

use crate::client::EarningsFeed;
use crate::error::Result;
use crate::models::{Company, CompanySearchResult, PaginatedResponse, SearchCompaniesParams};

/// Resource for accessing company data.
///
/// Obtain an instance via [`EarningsFeed::companies()`].
pub struct CompaniesResource<'a> {
    client: &'a EarningsFeed,
}

impl<'a> CompaniesResource<'a> {
    /// Create a new companies resource.
    pub(crate) fn new(client: &'a EarningsFeed) -> Self {
        Self { client }
    }

    /// Get a company by CIK.
    ///
    /// Returns the full company profile.
    pub async fn get(&self, cik: u64) -> Result<Company> {
        let path = format!("/api/v1/companies/{}", cik);
        self.client.get::<Company, ()>(&path, None).await
    }

    /// Search for companies.
    ///
    /// Returns a paginated response. Use [`iter_search`](Self::iter_search) for automatic pagination.
    pub async fn search(
        &self,
        params: &SearchCompaniesParams,
    ) -> Result<PaginatedResponse<CompanySearchResult>> {
        self.client
            .get("/api/v1/companies/search", Some(params))
            .await
    }

    /// Iterate over all companies matching the search parameters.
    ///
    /// Returns an async stream that automatically handles pagination.
    pub fn iter_search(
        &self,
        params: SearchCompaniesParams,
    ) -> impl Stream<Item = Result<CompanySearchResult>> + '_ {
        try_stream! {
            let mut current_params = params;

            loop {
                let response = self.search(&current_params).await?;

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
    async fn test_get_company() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/companies/320193"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "cik": 320193,
                "name": "Apple Inc.",
                "tickers": [
                    {"symbol": "AAPL", "exchange": "NASDAQ", "isPrimary": true}
                ],
                "primaryTicker": "AAPL",
                "sicCodes": [],
                "addresses": [],
                "hasInsiderTransactions": true,
                "isInsider": false,
                "updatedAt": "2024-01-15T12:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let company = client.companies().get(320193).await.unwrap();

        assert_eq!(company.cik, 320193);
        assert_eq!(company.name, "Apple Inc.");
        assert_eq!(company.primary_ticker, Some("AAPL".to_string()));
    }

    #[tokio::test]
    async fn test_get_company_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/companies/999999999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Company not found"
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let result = client.companies().get(999999999).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::Error::NotFound { .. }
        ));
    }

    #[tokio::test]
    async fn test_search_companies() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/companies/search"))
            .and(query_param("q", "Apple"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "cik": 320193,
                        "name": "Apple Inc.",
                        "ticker": "AAPL",
                        "exchange": "NASDAQ"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = SearchCompaniesParams::builder().q("Apple").build();
        let response = client.companies().search(&params).await.unwrap();

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].name, "Apple Inc.");
        assert_eq!(response.items[0].ticker, Some("AAPL".to_string()));
    }

    #[tokio::test]
    async fn test_iter_search() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/companies/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "cik": 320193,
                        "name": "Apple Inc."
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = SearchCompaniesParams::default();
        let companies_resource = client.companies();
        let mut stream = pin!(companies_resource.iter_search(params));

        let mut count = 0;
        while let Some(result) = stream.next().await {
            let company = result.unwrap();
            assert_eq!(company.name, "Apple Inc.");
            count += 1;
        }

        assert_eq!(count, 1);
    }
}
