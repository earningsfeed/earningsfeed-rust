//! Common types used across all API responses.

use serde::{Deserialize, Serialize};

/// Paginated API response wrapper.
///
/// All list endpoints return items wrapped in this structure with
/// cursor-based pagination support.
///
/// # Example
///
/// ```rust,ignore
/// use earningsfeed::{EarningsFeed, PaginatedResponse, Filing};
///
/// let response: PaginatedResponse<Filing> = client.filings().list(&params).await?;
/// for filing in response.items {
///     println!("{}", filing.title);
/// }
///
/// if response.has_more {
///     // Use response.next_cursor for next page
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    /// Items in this page.
    pub items: Vec<T>,
    /// Cursor for fetching the next page.
    pub next_cursor: Option<String>,
    /// Whether more results exist beyond this page.
    pub has_more: bool,
}

impl<T> Default for PaginatedResponse<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_paginated_response() {
        let json = json!({
            "items": ["a", "b", "c"],
            "nextCursor": "abc123",
            "hasMore": true
        });

        let response: PaginatedResponse<String> = serde_json::from_value(json).unwrap();
        assert_eq!(response.items, vec!["a", "b", "c"]);
        assert_eq!(response.next_cursor, Some("abc123".to_string()));
        assert!(response.has_more);
    }

    #[test]
    fn test_deserialize_empty_response() {
        let json = json!({
            "items": [],
            "nextCursor": null,
            "hasMore": false
        });

        let response: PaginatedResponse<String> = serde_json::from_value(json).unwrap();
        assert!(response.items.is_empty());
        assert!(response.next_cursor.is_none());
        assert!(!response.has_more);
    }

    #[test]
    fn test_serialize_paginated_response() {
        let response = PaginatedResponse {
            items: vec![1, 2, 3],
            next_cursor: Some("cursor".to_string()),
            has_more: true,
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["items"], json!([1, 2, 3]));
        assert_eq!(json["nextCursor"], "cursor");
        assert_eq!(json["hasMore"], true);
    }

    #[test]
    fn test_default() {
        let response: PaginatedResponse<i32> = PaginatedResponse::default();
        assert!(response.items.is_empty());
        assert!(response.next_cursor.is_none());
        assert!(!response.has_more);
    }

    #[test]
    fn test_paginated_response_is_clone() {
        let response = PaginatedResponse {
            items: vec!["test".to_string()],
            next_cursor: Some("cursor".to_string()),
            has_more: true,
        };
        let cloned = response.clone();
        assert_eq!(cloned.items, response.items);
    }

    #[test]
    fn test_paginated_response_is_debug() {
        let response: PaginatedResponse<i32> = PaginatedResponse::default();
        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("PaginatedResponse"));
    }
}
