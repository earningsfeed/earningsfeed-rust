//! Filing-related types.
//!
//! This module contains types for SEC filings including 10-K, 10-Q, 8-K,
//! and other form types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Company details attached to a filing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingCompany {
    /// SEC Central Index Key.
    pub cik: u64,
    /// Company name.
    pub name: String,
    /// State/country code.
    pub state_of_incorporation: Option<String>,
    /// Full state/country name.
    pub state_of_incorporation_description: Option<String>,
    /// Fiscal year end (MMDD format).
    pub fiscal_year_end: Option<String>,
}

/// Entity type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityClass {
    /// Company entity.
    Company,
    /// Person entity.
    Person,
}

/// SEC filing from the filings feed.
///
/// Represents a filing in the list endpoint response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filing {
    /// SEC accession number (e.g., "0000950170-24-000001").
    pub accession_number: String,
    /// Accession number without dashes.
    pub accession_no_dashes: Option<String>,
    /// Filer CIK.
    pub cik: u64,
    /// Company name.
    pub company_name: Option<String>,
    /// SEC form type (10-K, 8-K, etc.).
    pub form_type: String,
    /// Filing submission time.
    pub filed_at: DateTime<Utc>,
    /// SEC acceptance time.
    pub accept_ts: Option<DateTime<Utc>>,
    /// Whether filing is provisional.
    pub provisional: bool,
    /// Feed day (YYYY-MM-DD).
    pub feed_day: Option<String>,
    /// Primary document size in bytes.
    pub size_bytes: u64,
    /// SEC EDGAR URL.
    pub url: String,
    /// Filing title.
    pub title: String,
    /// Filing status.
    pub status: String,
    /// Last updated timestamp.
    pub updated_at: DateTime<Utc>,
    /// Primary stock ticker.
    pub primary_ticker: Option<String>,
    /// Primary exchange.
    pub primary_exchange: Option<String>,
    /// Company details.
    pub company: Option<FilingCompany>,
    /// Sort timestamp.
    pub sorted_at: DateTime<Utc>,
    /// Company logo URL.
    pub logo_url: Option<String>,
    /// Entity class.
    pub entity_class: Option<EntityClass>,
}

/// Document within a filing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingDocument {
    /// Document sequence number.
    pub seq: u32,
    /// Filename on SEC EDGAR.
    pub filename: String,
    /// Document type.
    pub doc_type: String,
    /// Document description.
    pub description: Option<String>,
    /// Whether this is the primary document.
    pub is_primary: bool,
}

/// Entity role in a filing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingRole {
    /// Entity CIK.
    pub cik: u64,
    /// Role type (filer, issuer, reporting-owner, etc.).
    pub role: String,
}

/// Detailed filing information.
///
/// Returned from the single filing endpoint with full details
/// including documents and roles.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingDetail {
    /// SEC accession number.
    pub accession_number: String,
    /// Accession number without dashes.
    pub accession_no_dashes: Option<String>,
    /// Filer CIK.
    pub cik: u64,
    /// SEC form type.
    pub form_type: String,
    /// Filing submission time.
    pub filed_at: DateTime<Utc>,
    /// SEC acceptance time.
    pub accept_ts: Option<DateTime<Utc>>,
    /// Whether filing is provisional.
    pub provisional: bool,
    /// Feed day (YYYY-MM-DD).
    pub feed_day: Option<String>,
    /// Filing title.
    pub title: String,
    /// SEC EDGAR URL.
    pub url: String,
    /// Primary document size in bytes.
    pub size_bytes: u64,
    /// SEC relative directory.
    pub sec_relative_dir: Option<String>,
    /// Company name.
    pub company_name: Option<String>,
    /// Primary stock ticker.
    pub primary_ticker: Option<String>,
    /// Company details.
    pub company: Option<FilingCompany>,
    /// Filing documents.
    pub documents: Vec<FilingDocument>,
    /// Entity roles.
    pub roles: Vec<FilingRole>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_filing_company() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "stateOfIncorporation": "CA",
            "stateOfIncorporationDescription": "California",
            "fiscalYearEnd": "0930"
        });

        let company: FilingCompany = serde_json::from_value(json).unwrap();
        assert_eq!(company.cik, 320193);
        assert_eq!(company.name, "Apple Inc.");
        assert_eq!(company.state_of_incorporation, Some("CA".to_string()));
        assert_eq!(company.fiscal_year_end, Some("0930".to_string()));
    }

    #[test]
    fn test_deserialize_entity_class() {
        let json = json!("company");
        let entity_class: EntityClass = serde_json::from_value(json).unwrap();
        assert_eq!(entity_class, EntityClass::Company);

        let json = json!("person");
        let entity_class: EntityClass = serde_json::from_value(json).unwrap();
        assert_eq!(entity_class, EntityClass::Person);
    }

    #[test]
    fn test_deserialize_filing() {
        let json = json!({
            "accessionNumber": "0000950170-24-000001",
            "accessionNoDashes": "0000950170240000001",
            "cik": 320193,
            "companyName": "Apple Inc.",
            "formType": "10-K",
            "filedAt": "2024-01-15T16:30:00Z",
            "acceptTs": "2024-01-15T16:25:00Z",
            "provisional": false,
            "feedDay": "2024-01-15",
            "sizeBytes": 12345678,
            "url": "https://www.sec.gov/Archives/edgar/data/320193/000095017024000001/0000950170-24-000001-index.htm",
            "title": "Form 10-K",
            "status": "final",
            "updatedAt": "2024-01-15T17:00:00Z",
            "primaryTicker": "AAPL",
            "primaryExchange": "NASDAQ",
            "sortedAt": "2024-01-15T16:30:00Z",
            "entityClass": "company"
        });

        let filing: Filing = serde_json::from_value(json).unwrap();
        assert_eq!(filing.accession_number, "0000950170-24-000001");
        assert_eq!(filing.cik, 320193);
        assert_eq!(filing.form_type, "10-K");
        assert!(!filing.provisional);
        assert_eq!(filing.primary_ticker, Some("AAPL".to_string()));
        assert_eq!(filing.entity_class, Some(EntityClass::Company));
    }

    #[test]
    fn test_deserialize_filing_minimal() {
        let json = json!({
            "accessionNumber": "0000950170-24-000001",
            "cik": 320193,
            "formType": "8-K",
            "filedAt": "2024-01-15T16:30:00Z",
            "provisional": true,
            "sizeBytes": 1000,
            "url": "https://www.sec.gov/...",
            "title": "Form 8-K",
            "status": "provisional",
            "updatedAt": "2024-01-15T17:00:00Z",
            "sortedAt": "2024-01-15T16:30:00Z"
        });

        let filing: Filing = serde_json::from_value(json).unwrap();
        assert!(filing.provisional);
        assert!(filing.company_name.is_none());
        assert!(filing.primary_ticker.is_none());
        assert!(filing.entity_class.is_none());
    }

    #[test]
    fn test_deserialize_filing_document() {
        let json = json!({
            "seq": 1,
            "filename": "aapl-20231230.htm",
            "docType": "10-K",
            "description": "10-K Annual Report",
            "isPrimary": true
        });

        let doc: FilingDocument = serde_json::from_value(json).unwrap();
        assert_eq!(doc.seq, 1);
        assert_eq!(doc.filename, "aapl-20231230.htm");
        assert!(doc.is_primary);
    }

    #[test]
    fn test_deserialize_filing_role() {
        let json = json!({
            "cik": 320193,
            "role": "filer"
        });

        let role: FilingRole = serde_json::from_value(json).unwrap();
        assert_eq!(role.cik, 320193);
        assert_eq!(role.role, "filer");
    }

    #[test]
    fn test_deserialize_filing_detail() {
        let json = json!({
            "accessionNumber": "0000950170-24-000001",
            "cik": 320193,
            "formType": "10-K",
            "filedAt": "2024-01-15T16:30:00Z",
            "provisional": false,
            "title": "Form 10-K",
            "url": "https://www.sec.gov/...",
            "sizeBytes": 12345678,
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
        });

        let detail: FilingDetail = serde_json::from_value(json).unwrap();
        assert_eq!(detail.accession_number, "0000950170-24-000001");
        assert_eq!(detail.documents.len(), 1);
        assert_eq!(detail.roles.len(), 1);
        assert!(detail.documents[0].is_primary);
    }

    #[test]
    fn test_filing_is_clone() {
        let json = json!({
            "accessionNumber": "0000950170-24-000001",
            "cik": 320193,
            "formType": "10-K",
            "filedAt": "2024-01-15T16:30:00Z",
            "provisional": false,
            "sizeBytes": 1000,
            "url": "https://www.sec.gov/...",
            "title": "Form 10-K",
            "status": "final",
            "updatedAt": "2024-01-15T17:00:00Z",
            "sortedAt": "2024-01-15T16:30:00Z"
        });

        let filing: Filing = serde_json::from_value(json).unwrap();
        let cloned = filing.clone();
        assert_eq!(cloned.accession_number, filing.accession_number);
    }

    #[test]
    fn test_serialize_filing() {
        let json = json!({
            "accessionNumber": "0000950170-24-000001",
            "cik": 320193,
            "formType": "10-K",
            "filedAt": "2024-01-15T16:30:00Z",
            "provisional": false,
            "sizeBytes": 1000,
            "url": "https://www.sec.gov/...",
            "title": "Form 10-K",
            "status": "final",
            "updatedAt": "2024-01-15T17:00:00Z",
            "sortedAt": "2024-01-15T16:30:00Z"
        });

        let filing: Filing = serde_json::from_value(json.clone()).unwrap();
        let serialized = serde_json::to_value(&filing).unwrap();
        assert_eq!(serialized["accessionNumber"], "0000950170-24-000001");
        assert_eq!(serialized["formType"], "10-K");
    }
}
