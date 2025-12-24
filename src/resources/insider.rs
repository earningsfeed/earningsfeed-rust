//! Insider transactions resource.
//!
//! This module provides methods for listing and iterating
//! over Form 3/4/5 insider trading data.

use async_stream::try_stream;
use futures::Stream;

use crate::client::EarningsFeed;
use crate::error::Result;
use crate::models::{InsiderTransaction, ListInsiderParams, PaginatedResponse};

/// Resource for accessing insider transactions.
///
/// Obtain an instance via [`EarningsFeed::insider()`].
pub struct InsiderResource<'a> {
    client: &'a EarningsFeed,
}

impl<'a> InsiderResource<'a> {
    /// Create a new insider resource.
    pub(crate) fn new(client: &'a EarningsFeed) -> Self {
        Self { client }
    }

    /// List insider transactions with optional filters.
    ///
    /// Returns a paginated response. Use [`iter`](Self::iter) for automatic pagination.
    pub async fn list(
        &self,
        params: &ListInsiderParams,
    ) -> Result<PaginatedResponse<InsiderTransaction>> {
        self.client
            .get("/api/v1/insider/transactions", Some(params))
            .await
    }

    /// Iterate over all insider transactions matching the given parameters.
    ///
    /// Returns an async stream that automatically handles pagination.
    pub fn iter(
        &self,
        params: ListInsiderParams,
    ) -> impl Stream<Item = Result<InsiderTransaction>> + '_ {
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
    async fn test_list_insider_transactions() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/insider/transactions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0001127602-24-000001",
                        "filedAt": "2024-01-15T18:30:00Z",
                        "formType": "4",
                        "personCik": 1234567,
                        "personName": "Cook Timothy D",
                        "companyCik": 320193,
                        "isDirector": true,
                        "isOfficer": true,
                        "isTenPercentOwner": false,
                        "isOther": false,
                        "securityTitle": "Common Stock",
                        "isDerivative": false,
                        "transactionDate": "2024-01-12",
                        "transactionCode": "S",
                        "equitySwapInvolved": false,
                        "acquiredDisposed": "D",
                        "directIndirect": "D"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInsiderParams::default();
        let response = client.insider().list(&params).await.unwrap();

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].person_name, "Cook Timothy D");
        assert!(!response.has_more);
    }

    #[tokio::test]
    async fn test_list_insider_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/insider/transactions"))
            .and(query_param("ticker", "AAPL"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInsiderParams::builder().ticker("AAPL").build();
        let response = client.insider().list(&params).await.unwrap();

        assert!(response.items.is_empty());
    }

    #[tokio::test]
    async fn test_iter_insider_transactions() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/insider/transactions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {
                        "accessionNumber": "0001127602-24-000001",
                        "filedAt": "2024-01-15T18:30:00Z",
                        "formType": "4",
                        "personCik": 1234567,
                        "personName": "Test Person",
                        "companyCik": 320193,
                        "isDirector": false,
                        "isOfficer": false,
                        "isTenPercentOwner": true,
                        "isOther": false,
                        "securityTitle": "Common Stock",
                        "isDerivative": false,
                        "transactionDate": "2024-01-12",
                        "transactionCode": "P",
                        "equitySwapInvolved": false,
                        "acquiredDisposed": "A",
                        "directIndirect": "D"
                    }
                ],
                "nextCursor": null,
                "hasMore": false
            })))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let params = ListInsiderParams::default();
        let insider_resource = client.insider();
        let mut stream = pin!(insider_resource.iter(params));

        let mut count = 0;
        while let Some(result) = stream.next().await {
            let txn = result.unwrap();
            assert_eq!(txn.person_name, "Test Person");
            count += 1;
        }

        assert_eq!(count, 1);
    }
}
