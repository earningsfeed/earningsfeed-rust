//! Institutional holdings resource.
//!
//! This module provides methods for listing and iterating
//! over 13F institutional holdings data.

use async_stream::try_stream;
use futures::Stream;

use crate::client::EarningsFeed;
use crate::error::Result;
use crate::models::{InstitutionalHolding, ListInstitutionalParams, PaginatedResponse};

/// Resource for accessing institutional holdings.
///
/// Obtain an instance via [`EarningsFeed::institutional()`].
pub struct InstitutionalResource<'a> {
    client: &'a EarningsFeed,
}

impl<'a> InstitutionalResource<'a> {
    /// Create a new institutional resource.
    pub(crate) fn new(client: &'a EarningsFeed) -> Self {
        Self { client }
    }

    /// List institutional holdings with optional filters.
    ///
    /// Returns a paginated response. Use [`iter`](Self::iter) for automatic pagination.
    pub async fn list(
        &self,
        params: &ListInstitutionalParams,
    ) -> Result<PaginatedResponse<InstitutionalHolding>> {
        self.client
            .get("/api/v1/institutional/holdings", Some(params))
            .await
    }

    /// Iterate over all institutional holdings matching the given parameters.
    ///
    /// Returns an async stream that automatically handles pagination.
    pub fn iter(
        &self,
        params: ListInstitutionalParams,
    ) -> impl Stream<Item = Result<InstitutionalHolding>> + '_ {
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
    async fn test_list_institutional_holdings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/institutional/holdings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "cusip": "037833100",
                        "issuerName": "APPLE INC",
                        "classTitle": "COM",
                        "value": "5000000",
                        "shares": "25000",
                        "sharesType": "SH",
                        "investmentDiscretion": "SOLE",
                        "managerCik": 102909,
                        "managerName": "BERKSHIRE HATHAWAY INC",
                        "reportPeriodDate": "2024-09-30",
                        "filedAt": "2024-11-14T16:30:00Z",
                        "accessionNumber": "0000950123-24-012345"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInstitutionalParams::default();
        let response = client.institutional().list(&params).await.unwrap();

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].issuer_name, "APPLE INC");
        assert_eq!(response.items[0].manager_name, "BERKSHIRE HATHAWAY INC");
        assert!(!response.has_more);
    }

    #[tokio::test]
    async fn test_list_institutional_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/institutional/holdings"))
            .and(query_param("ticker", "AAPL"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInstitutionalParams::builder().ticker("AAPL").build();
        let response = client.institutional().list(&params).await.unwrap();

        assert!(response.items.is_empty());
    }

    #[tokio::test]
    async fn test_iter_institutional_holdings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/institutional/holdings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "cusip": "037833100",
                        "issuerName": "APPLE INC",
                        "classTitle": "COM",
                        "value": "5000000",
                        "shares": "25000",
                        "sharesType": "SH",
                        "investmentDiscretion": "SOLE",
                        "managerCik": 102909,
                        "managerName": "TEST MANAGER",
                        "reportPeriodDate": "2024-09-30",
                        "filedAt": "2024-11-14T16:30:00Z",
                        "accessionNumber": "0000950123-24-012345"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInstitutionalParams::default();
        let institutional_resource = client.institutional();
        let mut stream = pin!(institutional_resource.iter(params));

        let mut count = 0;
        while let Some(result) = stream.next().await {
            let holding = result.unwrap();
            assert_eq!(holding.issuer_name, "APPLE INC");
            count += 1;
        }

        assert_eq!(count, 1);
    }
}
