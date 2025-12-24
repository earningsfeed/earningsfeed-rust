//! Institutional holdings types.
//!
//! This module contains types for 13F institutional holdings data.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Shares type indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharesType {
    /// Shares (stock).
    SH,
    /// Principal amount (bonds/notes).
    PRN,
}

/// Put/Call indicator for options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PutCall {
    /// Put option.
    Put,
    /// Call option.
    Call,
}

/// Investment discretion type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvestmentDiscretion {
    /// Sole discretion.
    Sole,
    /// Defined discretion.
    Dfnd,
    /// Other discretion.
    Other,
}

/// Institutional holding from 13F filing.
///
/// Represents a single holding position from an institutional manager's
/// 13F-HR filing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalHolding {
    /// 9-character CUSIP identifier.
    pub cusip: String,
    /// Issuer name.
    pub issuer_name: String,
    /// Share class title.
    pub class_title: String,
    /// Company CIK.
    pub company_cik: Option<u64>,
    /// Stock ticker.
    pub ticker: Option<String>,
    /// Market value in USD.
    pub value: Decimal,
    /// Number of shares.
    pub shares: Decimal,
    /// Shares type: SH (shares) or PRN (principal amount).
    pub shares_type: SharesType,
    /// Put or Call option indicator.
    pub put_call: Option<PutCall>,
    /// Investment discretion type.
    pub investment_discretion: InvestmentDiscretion,
    /// Other manager identifier.
    pub other_manager: Option<String>,
    /// Sole voting authority shares.
    pub voting_sole: Option<Decimal>,
    /// Shared voting authority shares.
    pub voting_shared: Option<Decimal>,
    /// No voting authority shares.
    pub voting_none: Option<Decimal>,
    /// Manager CIK.
    pub manager_cik: u64,
    /// Manager name.
    pub manager_name: String,
    /// Quarter end date (YYYY-MM-DD).
    pub report_period_date: NaiveDate,
    /// Filing submission time.
    pub filed_at: DateTime<Utc>,
    /// SEC accession number.
    pub accession_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_shares_type() {
        let json = json!("SH");
        let st: SharesType = serde_json::from_value(json).unwrap();
        assert_eq!(st, SharesType::SH);

        let json = json!("PRN");
        let st: SharesType = serde_json::from_value(json).unwrap();
        assert_eq!(st, SharesType::PRN);
    }

    #[test]
    fn test_deserialize_put_call() {
        let json = json!("Put");
        let pc: PutCall = serde_json::from_value(json).unwrap();
        assert_eq!(pc, PutCall::Put);

        let json = json!("Call");
        let pc: PutCall = serde_json::from_value(json).unwrap();
        assert_eq!(pc, PutCall::Call);
    }

    #[test]
    fn test_deserialize_investment_discretion() {
        let json = json!("SOLE");
        let id: InvestmentDiscretion = serde_json::from_value(json).unwrap();
        assert_eq!(id, InvestmentDiscretion::Sole);

        let json = json!("DFND");
        let id: InvestmentDiscretion = serde_json::from_value(json).unwrap();
        assert_eq!(id, InvestmentDiscretion::Dfnd);

        let json = json!("OTHER");
        let id: InvestmentDiscretion = serde_json::from_value(json).unwrap();
        assert_eq!(id, InvestmentDiscretion::Other);
    }

    #[test]
    fn test_deserialize_institutional_holding() {
        let json = json!({
            "cusip": "037833100",
            "issuerName": "APPLE INC",
            "classTitle": "COM",
            "companyCik": 320193,
            "ticker": "AAPL",
            "value": "5000000",
            "shares": "25000",
            "sharesType": "SH",
            "investmentDiscretion": "SOLE",
            "votingSole": "25000",
            "votingShared": "0",
            "votingNone": "0",
            "managerCik": 102909,
            "managerName": "BERKSHIRE HATHAWAY INC",
            "reportPeriodDate": "2024-09-30",
            "filedAt": "2024-11-14T16:30:00Z",
            "accessionNumber": "0000950123-24-012345"
        });

        let holding: InstitutionalHolding = serde_json::from_value(json).unwrap();
        assert_eq!(holding.cusip, "037833100");
        assert_eq!(holding.issuer_name, "APPLE INC");
        assert_eq!(holding.class_title, "COM");
        assert_eq!(holding.company_cik, Some(320193));
        assert_eq!(holding.ticker, Some("AAPL".to_string()));
        assert_eq!(holding.value, Decimal::from(5000000));
        assert_eq!(holding.shares, Decimal::from(25000));
        assert_eq!(holding.shares_type, SharesType::SH);
        assert!(holding.put_call.is_none());
        assert_eq!(holding.investment_discretion, InvestmentDiscretion::Sole);
        assert_eq!(holding.manager_cik, 102909);
    }

    #[test]
    fn test_deserialize_option_holding() {
        let json = json!({
            "cusip": "037833100",
            "issuerName": "APPLE INC",
            "classTitle": "CALL",
            "value": "1000000",
            "shares": "5000",
            "sharesType": "SH",
            "putCall": "Call",
            "investmentDiscretion": "DFND",
            "managerCik": 102909,
            "managerName": "HEDGE FUND LLC",
            "reportPeriodDate": "2024-09-30",
            "filedAt": "2024-11-14T16:30:00Z",
            "accessionNumber": "0000950123-24-012346"
        });

        let holding: InstitutionalHolding = serde_json::from_value(json).unwrap();
        assert_eq!(holding.put_call, Some(PutCall::Call));
        assert_eq!(holding.investment_discretion, InvestmentDiscretion::Dfnd);
        assert!(holding.company_cik.is_none());
        assert!(holding.ticker.is_none());
    }

    #[test]
    fn test_deserialize_principal_holding() {
        let json = json!({
            "cusip": "912828XY0",
            "issuerName": "UNITED STATES TREASURY",
            "classTitle": "NOTE",
            "value": "10000000",
            "shares": "10000000",
            "sharesType": "PRN",
            "investmentDiscretion": "SOLE",
            "managerCik": 102909,
            "managerName": "BOND FUND LLC",
            "reportPeriodDate": "2024-09-30",
            "filedAt": "2024-11-14T16:30:00Z",
            "accessionNumber": "0000950123-24-012347"
        });

        let holding: InstitutionalHolding = serde_json::from_value(json).unwrap();
        assert_eq!(holding.shares_type, SharesType::PRN);
        assert_eq!(holding.shares, Decimal::from(10000000));
    }

    #[test]
    fn test_institutional_holding_is_clone() {
        let json = json!({
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
        });

        let holding: InstitutionalHolding = serde_json::from_value(json).unwrap();
        let cloned = holding.clone();
        assert_eq!(cloned.cusip, holding.cusip);
        assert_eq!(cloned.manager_cik, holding.manager_cik);
    }

    #[test]
    fn test_serialize_institutional_holding() {
        let json = json!({
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
        });

        let holding: InstitutionalHolding = serde_json::from_value(json).unwrap();
        let serialized = serde_json::to_value(&holding).unwrap();
        assert_eq!(serialized["cusip"], "037833100");
        assert_eq!(serialized["sharesType"], "SH");
        assert_eq!(serialized["investmentDiscretion"], "SOLE");
    }
}
