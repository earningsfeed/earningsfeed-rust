//! Insider transaction types.
//!
//! This module contains types for Form 3/4/5 insider trading data.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Direction of transaction (acquired or disposed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquiredDisposed {
    /// Shares were acquired.
    A,
    /// Shares were disposed.
    D,
}

/// Ownership type (direct or indirect).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectIndirect {
    /// Direct ownership.
    D,
    /// Indirect ownership.
    I,
}

/// Insider transaction from Form 3/4/5.
///
/// Represents a single transaction from an insider trading filing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTransaction {
    /// SEC accession number.
    pub accession_number: String,
    /// Filing submission time.
    pub filed_at: DateTime<Utc>,
    /// Form type (3, 4, or 5).
    pub form_type: String,
    /// Insider's CIK.
    pub person_cik: u64,
    /// Insider's name.
    pub person_name: String,
    /// Company CIK.
    pub company_cik: u64,
    /// Company name.
    pub company_name: Option<String>,
    /// Stock ticker.
    pub ticker: Option<String>,
    /// Whether insider is a director.
    pub is_director: bool,
    /// Whether insider is an officer.
    pub is_officer: bool,
    /// Whether insider is a 10% owner.
    pub is_ten_percent_owner: bool,
    /// Whether insider has other relationship.
    pub is_other: bool,
    /// Officer title.
    pub officer_title: Option<String>,
    /// Security title.
    pub security_title: String,
    /// Whether this is a derivative transaction.
    pub is_derivative: bool,
    /// Transaction date (YYYY-MM-DD).
    pub transaction_date: NaiveDate,
    /// Transaction code (P, S, A, M, G, etc.).
    pub transaction_code: String,
    /// Whether equity swap was involved.
    pub equity_swap_involved: bool,
    /// Number of shares.
    pub shares: Option<Decimal>,
    /// Price per share.
    pub price_per_share: Option<Decimal>,
    /// Acquired (A) or Disposed (D).
    pub acquired_disposed: AcquiredDisposed,
    /// Shares owned after transaction.
    pub shares_after: Option<Decimal>,
    /// Direct (D) or Indirect (I) ownership.
    pub direct_indirect: DirectIndirect,
    /// Nature of indirect ownership.
    pub ownership_nature: Option<String>,
    /// Derivative conversion/exercise price.
    pub conversion_or_exercise_price: Option<Decimal>,
    /// Derivative exercise date.
    pub exercise_date: Option<NaiveDate>,
    /// Derivative expiration date.
    pub expiration_date: Option<NaiveDate>,
    /// Underlying security title.
    pub underlying_security_title: Option<String>,
    /// Underlying shares.
    pub underlying_shares: Option<Decimal>,
    /// Total transaction value.
    pub transaction_value: Option<Decimal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_acquired_disposed() {
        let json = json!("A");
        let ad: AcquiredDisposed = serde_json::from_value(json).unwrap();
        assert_eq!(ad, AcquiredDisposed::A);

        let json = json!("D");
        let ad: AcquiredDisposed = serde_json::from_value(json).unwrap();
        assert_eq!(ad, AcquiredDisposed::D);
    }

    #[test]
    fn test_deserialize_direct_indirect() {
        let json = json!("D");
        let di: DirectIndirect = serde_json::from_value(json).unwrap();
        assert_eq!(di, DirectIndirect::D);

        let json = json!("I");
        let di: DirectIndirect = serde_json::from_value(json).unwrap();
        assert_eq!(di, DirectIndirect::I);
    }

    #[test]
    fn test_deserialize_insider_transaction() {
        let json = json!({
            "accessionNumber": "0001127602-24-000001",
            "filedAt": "2024-01-15T18:30:00Z",
            "formType": "4",
            "personCik": 1234567,
            "personName": "Cook Timothy D",
            "companyCik": 320193,
            "companyName": "Apple Inc.",
            "ticker": "AAPL",
            "isDirector": true,
            "isOfficer": true,
            "isTenPercentOwner": false,
            "isOther": false,
            "officerTitle": "Chief Executive Officer",
            "securityTitle": "Common Stock",
            "isDerivative": false,
            "transactionDate": "2024-01-12",
            "transactionCode": "S",
            "equitySwapInvolved": false,
            "shares": "10000",
            "pricePerShare": "185.50",
            "acquiredDisposed": "D",
            "sharesAfter": "500000",
            "directIndirect": "D",
            "transactionValue": "1855000"
        });

        let txn: InsiderTransaction = serde_json::from_value(json).unwrap();
        assert_eq!(txn.accession_number, "0001127602-24-000001");
        assert_eq!(txn.person_name, "Cook Timothy D");
        assert_eq!(txn.company_cik, 320193);
        assert!(txn.is_director);
        assert!(txn.is_officer);
        assert!(!txn.is_derivative);
        assert_eq!(txn.transaction_code, "S");
        assert_eq!(txn.acquired_disposed, AcquiredDisposed::D);
        assert_eq!(txn.direct_indirect, DirectIndirect::D);
        assert_eq!(txn.shares, Some(Decimal::from(10000)));
    }

    #[test]
    fn test_deserialize_derivative_transaction() {
        let json = json!({
            "accessionNumber": "0001127602-24-000002",
            "filedAt": "2024-01-15T18:30:00Z",
            "formType": "4",
            "personCik": 1234567,
            "personName": "Cook Timothy D",
            "companyCik": 320193,
            "isDirector": true,
            "isOfficer": true,
            "isTenPercentOwner": false,
            "isOther": false,
            "securityTitle": "Stock Option (Right to Buy)",
            "isDerivative": true,
            "transactionDate": "2024-01-12",
            "transactionCode": "M",
            "equitySwapInvolved": false,
            "shares": "5000",
            "pricePerShare": "150.00",
            "acquiredDisposed": "A",
            "sharesAfter": "50000",
            "directIndirect": "D",
            "conversionOrExercisePrice": "150.00",
            "exerciseDate": "2024-01-12",
            "expirationDate": "2030-01-12",
            "underlyingSecurityTitle": "Common Stock",
            "underlyingShares": "5000"
        });

        let txn: InsiderTransaction = serde_json::from_value(json).unwrap();
        assert!(txn.is_derivative);
        assert_eq!(txn.transaction_code, "M");
        assert_eq!(txn.acquired_disposed, AcquiredDisposed::A);
        assert!(txn.conversion_or_exercise_price.is_some());
        assert!(txn.exercise_date.is_some());
        assert!(txn.expiration_date.is_some());
        assert_eq!(
            txn.underlying_security_title,
            Some("Common Stock".to_string())
        );
    }

    #[test]
    fn test_deserialize_minimal_transaction() {
        let json = json!({
            "accessionNumber": "0001127602-24-000003",
            "filedAt": "2024-01-15T18:30:00Z",
            "formType": "3",
            "personCik": 1234567,
            "personName": "New Director",
            "companyCik": 320193,
            "isDirector": true,
            "isOfficer": false,
            "isTenPercentOwner": false,
            "isOther": false,
            "securityTitle": "Common Stock",
            "isDerivative": false,
            "transactionDate": "2024-01-12",
            "transactionCode": "I",
            "equitySwapInvolved": false,
            "acquiredDisposed": "A",
            "directIndirect": "D"
        });

        let txn: InsiderTransaction = serde_json::from_value(json).unwrap();
        assert_eq!(txn.form_type, "3");
        assert!(txn.shares.is_none());
        assert!(txn.price_per_share.is_none());
        assert!(txn.company_name.is_none());
    }

    #[test]
    fn test_insider_transaction_is_clone() {
        let json = json!({
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
        });

        let txn: InsiderTransaction = serde_json::from_value(json).unwrap();
        let cloned = txn.clone();
        assert_eq!(cloned.accession_number, txn.accession_number);
        assert_eq!(cloned.person_name, txn.person_name);
    }

    #[test]
    fn test_serialize_insider_transaction() {
        let json = json!({
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
        });

        let txn: InsiderTransaction = serde_json::from_value(json).unwrap();
        let serialized = serde_json::to_value(&txn).unwrap();
        assert_eq!(serialized["accessionNumber"], "0001127602-24-000001");
        assert_eq!(serialized["formType"], "4");
        assert_eq!(serialized["acquiredDisposed"], "A");
    }
}
