//! Company-related types.
//!
//! This module contains types for company profiles and search results.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Stock ticker information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Ticker symbol.
    pub symbol: String,
    /// Exchange name.
    pub exchange: String,
    /// Whether this is the primary ticker.
    pub is_primary: bool,
}

/// Standard Industrial Classification code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SicCode {
    /// SIC code number.
    pub code: u32,
    /// SIC description.
    pub description: String,
}

/// Company address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Address type (mailing, business).
    #[serde(rename = "type")]
    pub address_type: String,
    /// Street line 1.
    pub street1: Option<String>,
    /// Street line 2.
    pub street2: Option<String>,
    /// City.
    pub city: Option<String>,
    /// State or country code.
    pub state_or_country: Option<String>,
    /// State or country name.
    pub state_or_country_description: Option<String>,
    /// ZIP/postal code.
    pub zip_code: Option<String>,
}

/// Company profile.
///
/// Full company information returned from the company detail endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    /// SEC Central Index Key.
    pub cik: u64,
    /// Company name.
    pub name: String,
    /// Entity type.
    pub entity_type: Option<String>,
    /// Category.
    pub category: Option<String>,
    /// Company description.
    pub description: Option<String>,
    /// Stock tickers.
    pub tickers: Vec<Ticker>,
    /// Primary ticker symbol.
    pub primary_ticker: Option<String>,
    /// SIC codes.
    pub sic_codes: Vec<SicCode>,
    /// Employer Identification Number.
    pub ein: Option<String>,
    /// Fiscal year end (MMDD format).
    pub fiscal_year_end: Option<String>,
    /// State of incorporation code.
    pub state_of_incorporation: Option<String>,
    /// State of incorporation name.
    pub state_of_incorporation_description: Option<String>,
    /// Phone number.
    pub phone: Option<String>,
    /// Company website.
    pub website: Option<String>,
    /// Investor relations website.
    pub investor_website: Option<String>,
    /// Company addresses.
    pub addresses: Vec<Address>,
    /// Company logo URL.
    pub logo_url: Option<String>,
    /// Whether company has insider transactions.
    pub has_insider_transactions: bool,
    /// Whether entity is an insider.
    pub is_insider: bool,
    /// Last updated timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Company search result.
///
/// Simplified company information returned from search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySearchResult {
    /// SEC Central Index Key.
    pub cik: u64,
    /// Company name.
    pub name: String,
    /// Primary ticker symbol.
    pub ticker: Option<String>,
    /// Primary exchange.
    pub exchange: Option<String>,
    /// Entity type.
    pub entity_type: Option<String>,
    /// Category.
    pub category: Option<String>,
    /// SIC code.
    pub sic_code: Option<u32>,
    /// SIC description.
    pub sic_description: Option<String>,
    /// Company logo URL.
    pub logo_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_ticker() {
        let json = json!({
            "symbol": "AAPL",
            "exchange": "NASDAQ",
            "isPrimary": true
        });

        let ticker: Ticker = serde_json::from_value(json).unwrap();
        assert_eq!(ticker.symbol, "AAPL");
        assert_eq!(ticker.exchange, "NASDAQ");
        assert!(ticker.is_primary);
    }

    #[test]
    fn test_deserialize_sic_code() {
        let json = json!({
            "code": 3571,
            "description": "Electronic Computers"
        });

        let sic: SicCode = serde_json::from_value(json).unwrap();
        assert_eq!(sic.code, 3571);
        assert_eq!(sic.description, "Electronic Computers");
    }

    #[test]
    fn test_deserialize_address() {
        let json = json!({
            "type": "business",
            "street1": "One Apple Park Way",
            "city": "Cupertino",
            "stateOrCountry": "CA",
            "stateOrCountryDescription": "California",
            "zipCode": "95014"
        });

        let address: Address = serde_json::from_value(json).unwrap();
        assert_eq!(address.address_type, "business");
        assert_eq!(address.street1, Some("One Apple Park Way".to_string()));
        assert_eq!(address.city, Some("Cupertino".to_string()));
        assert_eq!(address.state_or_country, Some("CA".to_string()));
        assert!(address.street2.is_none());
    }

    #[test]
    fn test_deserialize_company() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "entityType": "operating",
            "category": "Large accelerated filer",
            "description": "Apple designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.",
            "tickers": [
                {"symbol": "AAPL", "exchange": "NASDAQ", "isPrimary": true}
            ],
            "primaryTicker": "AAPL",
            "sicCodes": [
                {"code": 3571, "description": "Electronic Computers"}
            ],
            "ein": "94-2404110",
            "fiscalYearEnd": "0930",
            "stateOfIncorporation": "CA",
            "stateOfIncorporationDescription": "California",
            "phone": "408-996-1010",
            "website": "https://www.apple.com",
            "investorWebsite": "https://investor.apple.com",
            "addresses": [
                {
                    "type": "business",
                    "street1": "One Apple Park Way",
                    "city": "Cupertino",
                    "stateOrCountry": "CA",
                    "zipCode": "95014"
                }
            ],
            "logoUrl": "https://example.com/aapl.png",
            "hasInsiderTransactions": true,
            "isInsider": false,
            "updatedAt": "2024-01-15T12:00:00Z"
        });

        let company: Company = serde_json::from_value(json).unwrap();
        assert_eq!(company.cik, 320193);
        assert_eq!(company.name, "Apple Inc.");
        assert_eq!(company.primary_ticker, Some("AAPL".to_string()));
        assert_eq!(company.tickers.len(), 1);
        assert_eq!(company.sic_codes.len(), 1);
        assert_eq!(company.addresses.len(), 1);
        assert!(company.has_insider_transactions);
        assert!(!company.is_insider);
    }

    #[test]
    fn test_deserialize_company_minimal() {
        let json = json!({
            "cik": 1234567,
            "name": "Test Company LLC",
            "tickers": [],
            "sicCodes": [],
            "addresses": [],
            "hasInsiderTransactions": false,
            "isInsider": false,
            "updatedAt": "2024-01-15T12:00:00Z"
        });

        let company: Company = serde_json::from_value(json).unwrap();
        assert_eq!(company.cik, 1234567);
        assert!(company.tickers.is_empty());
        assert!(company.primary_ticker.is_none());
        assert!(company.entity_type.is_none());
    }

    #[test]
    fn test_deserialize_company_search_result() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "ticker": "AAPL",
            "exchange": "NASDAQ",
            "entityType": "operating",
            "category": "Large accelerated filer",
            "sicCode": 3571,
            "sicDescription": "Electronic Computers",
            "logoUrl": "https://example.com/aapl.png"
        });

        let result: CompanySearchResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.cik, 320193);
        assert_eq!(result.name, "Apple Inc.");
        assert_eq!(result.ticker, Some("AAPL".to_string()));
        assert_eq!(result.exchange, Some("NASDAQ".to_string()));
        assert_eq!(result.sic_code, Some(3571));
    }

    #[test]
    fn test_deserialize_search_result_minimal() {
        let json = json!({
            "cik": 1234567,
            "name": "Unknown Entity"
        });

        let result: CompanySearchResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.cik, 1234567);
        assert!(result.ticker.is_none());
        assert!(result.sic_code.is_none());
    }

    #[test]
    fn test_company_is_clone() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "tickers": [],
            "sicCodes": [],
            "addresses": [],
            "hasInsiderTransactions": true,
            "isInsider": false,
            "updatedAt": "2024-01-15T12:00:00Z"
        });

        let company: Company = serde_json::from_value(json).unwrap();
        let cloned = company.clone();
        assert_eq!(cloned.cik, company.cik);
        assert_eq!(cloned.name, company.name);
    }

    #[test]
    fn test_serialize_company() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "tickers": [],
            "sicCodes": [],
            "addresses": [],
            "hasInsiderTransactions": true,
            "isInsider": false,
            "updatedAt": "2024-01-15T12:00:00Z"
        });

        let company: Company = serde_json::from_value(json).unwrap();
        let serialized = serde_json::to_value(&company).unwrap();
        assert_eq!(serialized["cik"], 320193);
        assert_eq!(serialized["name"], "Apple Inc.");
        assert_eq!(serialized["hasInsiderTransactions"], true);
    }

    #[test]
    fn test_serialize_search_result() {
        let json = json!({
            "cik": 320193,
            "name": "Apple Inc.",
            "ticker": "AAPL"
        });

        let result: CompanySearchResult = serde_json::from_value(json).unwrap();
        let serialized = serde_json::to_value(&result).unwrap();
        assert_eq!(serialized["cik"], 320193);
        assert_eq!(serialized["ticker"], "AAPL");
    }
}
