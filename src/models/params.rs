//! Request parameter types.
//!
//! This module contains builder-style parameter types for API requests.

use serde::Serialize;

/// Filing status filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FilingStatus {
    /// All filings.
    All,
    /// Provisional filings only.
    Provisional,
    /// Final filings only.
    Final,
}

/// Transaction direction filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionDirection {
    /// Buy transactions.
    Buy,
    /// Sell transactions.
    Sell,
}

/// Put/call filter for institutional holdings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PutCallFilter {
    /// Put options only.
    Put,
    /// Call options only.
    Call,
    /// Equity (non-option) positions only.
    Equity,
}

/// Parameters for listing filings.
///
/// Use the builder pattern to construct parameters:
///
/// ```rust
/// use earningsfeed::ListFilingsParams;
///
/// let params = ListFilingsParams::builder()
///     .ticker("AAPL")
///     .forms(vec!["10-K", "10-Q"])
///     .limit(10)
///     .build();
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFilingsParams {
    /// Filter by form types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forms: Option<String>,
    /// Filter by ticker symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Filter by CIK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<u64>,
    /// Filter by filing status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FilingStatus>,
    /// Start date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Results per page (1-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListFilingsParams {
    /// Create a new builder for filing parameters.
    #[must_use]
    pub fn builder() -> ListFilingsParamsBuilder {
        ListFilingsParamsBuilder::default()
    }
}

/// Builder for [`ListFilingsParams`].
#[derive(Debug, Default)]
pub struct ListFilingsParamsBuilder {
    params: ListFilingsParams,
}

impl ListFilingsParamsBuilder {
    /// Filter by form types.
    #[must_use]
    pub fn forms<I, S>(mut self, forms: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let forms_str: Vec<String> = forms.into_iter().map(|s| s.as_ref().to_string()).collect();
        self.params.forms = Some(forms_str.join(","));
        self
    }

    /// Filter by ticker symbol.
    #[must_use]
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.params.ticker = Some(ticker.into());
        self
    }

    /// Filter by CIK.
    #[must_use]
    pub fn cik(mut self, cik: u64) -> Self {
        self.params.cik = Some(cik);
        self
    }

    /// Filter by filing status.
    #[must_use]
    pub fn status(mut self, status: FilingStatus) -> Self {
        self.params.status = Some(status);
        self
    }

    /// Filter by start date (YYYY-MM-DD).
    #[must_use]
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.params.start_date = Some(date.into());
        self
    }

    /// Filter by end date (YYYY-MM-DD).
    #[must_use]
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.params.end_date = Some(date.into());
        self
    }

    /// Search query.
    #[must_use]
    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.params.q = Some(query.into());
        self
    }

    /// Results per page (1-100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Pagination cursor.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.params.cursor = Some(cursor.into());
        self
    }

    /// Build the parameters.
    #[must_use]
    pub fn build(self) -> ListFilingsParams {
        self.params
    }
}

/// Parameters for listing insider transactions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInsiderParams {
    /// Filter by ticker symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Filter by company CIK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<u64>,
    /// Filter by person CIK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_cik: Option<u64>,
    /// Filter by direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TransactionDirection>,
    /// Filter by transaction codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<String>,
    /// Filter derivatives only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivative: Option<bool>,
    /// Minimum transaction value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<u64>,
    /// Start date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Results per page (1-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListInsiderParams {
    /// Create a new builder for insider parameters.
    #[must_use]
    pub fn builder() -> ListInsiderParamsBuilder {
        ListInsiderParamsBuilder::default()
    }
}

/// Builder for [`ListInsiderParams`].
#[derive(Debug, Default)]
pub struct ListInsiderParamsBuilder {
    params: ListInsiderParams,
}

impl ListInsiderParamsBuilder {
    /// Filter by ticker symbol.
    #[must_use]
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.params.ticker = Some(ticker.into());
        self
    }

    /// Filter by company CIK.
    #[must_use]
    pub fn cik(mut self, cik: u64) -> Self {
        self.params.cik = Some(cik);
        self
    }

    /// Filter by person CIK.
    #[must_use]
    pub fn person_cik(mut self, cik: u64) -> Self {
        self.params.person_cik = Some(cik);
        self
    }

    /// Filter by direction (buy/sell).
    #[must_use]
    pub fn direction(mut self, direction: TransactionDirection) -> Self {
        self.params.direction = Some(direction);
        self
    }

    /// Filter by transaction codes.
    #[must_use]
    pub fn codes<I, S>(mut self, codes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let codes_str: Vec<String> = codes.into_iter().map(|s| s.as_ref().to_string()).collect();
        self.params.codes = Some(codes_str.join(","));
        self
    }

    /// Filter derivatives only.
    #[must_use]
    pub fn derivative(mut self, derivative: bool) -> Self {
        self.params.derivative = Some(derivative);
        self
    }

    /// Minimum transaction value.
    #[must_use]
    pub fn min_value(mut self, value: u64) -> Self {
        self.params.min_value = Some(value);
        self
    }

    /// Filter by start date (YYYY-MM-DD).
    #[must_use]
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.params.start_date = Some(date.into());
        self
    }

    /// Filter by end date (YYYY-MM-DD).
    #[must_use]
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.params.end_date = Some(date.into());
        self
    }

    /// Results per page (1-100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Pagination cursor.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.params.cursor = Some(cursor.into());
        self
    }

    /// Build the parameters.
    #[must_use]
    pub fn build(self) -> ListInsiderParams {
        self.params
    }
}

/// Parameters for listing institutional holdings.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInstitutionalParams {
    /// Filter by company CIK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<u64>,
    /// Filter by ticker symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Filter by CUSIP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Filter by manager CIK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_cik: Option<u64>,
    /// Filter by minimum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<u64>,
    /// Filter by put/call/equity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_call: Option<PutCallFilter>,
    /// Filter by report period (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_period: Option<String>,
    /// Results per page (1-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListInstitutionalParams {
    /// Create a new builder for institutional parameters.
    #[must_use]
    pub fn builder() -> ListInstitutionalParamsBuilder {
        ListInstitutionalParamsBuilder::default()
    }
}

/// Builder for [`ListInstitutionalParams`].
#[derive(Debug, Default)]
pub struct ListInstitutionalParamsBuilder {
    params: ListInstitutionalParams,
}

impl ListInstitutionalParamsBuilder {
    /// Filter by company CIK.
    #[must_use]
    pub fn cik(mut self, cik: u64) -> Self {
        self.params.cik = Some(cik);
        self
    }

    /// Filter by ticker symbol.
    #[must_use]
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.params.ticker = Some(ticker.into());
        self
    }

    /// Filter by CUSIP.
    #[must_use]
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.params.cusip = Some(cusip.into());
        self
    }

    /// Filter by manager CIK.
    #[must_use]
    pub fn manager_cik(mut self, cik: u64) -> Self {
        self.params.manager_cik = Some(cik);
        self
    }

    /// Filter by minimum value.
    #[must_use]
    pub fn min_value(mut self, value: u64) -> Self {
        self.params.min_value = Some(value);
        self
    }

    /// Filter by put/call/equity.
    #[must_use]
    pub fn put_call(mut self, put_call: PutCallFilter) -> Self {
        self.params.put_call = Some(put_call);
        self
    }

    /// Filter by report period (YYYY-MM-DD).
    #[must_use]
    pub fn report_period(mut self, date: impl Into<String>) -> Self {
        self.params.report_period = Some(date.into());
        self
    }

    /// Results per page (1-100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Pagination cursor.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.params.cursor = Some(cursor.into());
        self
    }

    /// Build the parameters.
    #[must_use]
    pub fn build(self) -> ListInstitutionalParams {
        self.params
    }
}

/// Parameters for searching companies.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCompaniesParams {
    /// Search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Filter by ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Filter by SIC code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sic_code: Option<u32>,
    /// Filter by state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Results per page (1-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SearchCompaniesParams {
    /// Create a new builder for company search parameters.
    #[must_use]
    pub fn builder() -> SearchCompaniesParamsBuilder {
        SearchCompaniesParamsBuilder::default()
    }
}

/// Builder for [`SearchCompaniesParams`].
#[derive(Debug, Default)]
pub struct SearchCompaniesParamsBuilder {
    params: SearchCompaniesParams,
}

impl SearchCompaniesParamsBuilder {
    /// Search query.
    #[must_use]
    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.params.q = Some(query.into());
        self
    }

    /// Filter by ticker.
    #[must_use]
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.params.ticker = Some(ticker.into());
        self
    }

    /// Filter by SIC code.
    #[must_use]
    pub fn sic_code(mut self, code: u32) -> Self {
        self.params.sic_code = Some(code);
        self
    }

    /// Filter by state.
    #[must_use]
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.params.state = Some(state.into());
        self
    }

    /// Results per page (1-100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Pagination cursor.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.params.cursor = Some(cursor.into());
        self
    }

    /// Build the parameters.
    #[must_use]
    pub fn build(self) -> SearchCompaniesParams {
        self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_filings_params_builder() {
        let params = ListFilingsParams::builder()
            .ticker("AAPL")
            .forms(vec!["10-K", "10-Q"])
            .limit(10)
            .build();

        assert_eq!(params.ticker, Some("AAPL".to_string()));
        assert_eq!(params.forms, Some("10-K,10-Q".to_string()));
        assert_eq!(params.limit, Some(10));
    }

    #[test]
    fn test_list_filings_params_default() {
        let params = ListFilingsParams::default();
        assert!(params.ticker.is_none());
        assert!(params.forms.is_none());
        assert!(params.limit.is_none());
    }

    #[test]
    fn test_list_filings_params_serialize() {
        let params = ListFilingsParams::builder()
            .ticker("AAPL")
            .status(FilingStatus::Final)
            .limit(20)
            .build();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["ticker"], "AAPL");
        assert_eq!(json["status"], "final");
        assert_eq!(json["limit"], 20);
        // None fields should not be serialized
        assert!(json.get("cik").is_none());
    }

    #[test]
    fn test_list_filings_params_with_dates() {
        let params = ListFilingsParams::builder()
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .build();

        assert_eq!(params.start_date, Some("2024-01-01".to_string()));
        assert_eq!(params.end_date, Some("2024-12-31".to_string()));
    }

    #[test]
    fn test_list_insider_params_builder() {
        let params = ListInsiderParams::builder()
            .ticker("AAPL")
            .direction(TransactionDirection::Buy)
            .min_value(100000)
            .build();

        assert_eq!(params.ticker, Some("AAPL".to_string()));
        assert_eq!(params.direction, Some(TransactionDirection::Buy));
        assert_eq!(params.min_value, Some(100000));
    }

    #[test]
    fn test_list_insider_params_with_codes() {
        let params = ListInsiderParams::builder()
            .codes(vec!["P", "S", "M"])
            .build();

        assert_eq!(params.codes, Some("P,S,M".to_string()));
    }

    #[test]
    fn test_list_insider_params_serialize() {
        let params = ListInsiderParams::builder()
            .direction(TransactionDirection::Sell)
            .derivative(true)
            .build();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["direction"], "sell");
        assert_eq!(json["derivative"], true);
    }

    #[test]
    fn test_list_institutional_params_builder() {
        let params = ListInstitutionalParams::builder()
            .ticker("AAPL")
            .manager_cik(102909)
            .put_call(PutCallFilter::Equity)
            .build();

        assert_eq!(params.ticker, Some("AAPL".to_string()));
        assert_eq!(params.manager_cik, Some(102909));
        assert_eq!(params.put_call, Some(PutCallFilter::Equity));
    }

    #[test]
    fn test_list_institutional_params_serialize() {
        let params = ListInstitutionalParams::builder()
            .cusip("037833100")
            .report_period("2024-09-30")
            .build();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["cusip"], "037833100");
        assert_eq!(json["reportPeriod"], "2024-09-30");
    }

    #[test]
    fn test_search_companies_params_builder() {
        let params = SearchCompaniesParams::builder()
            .q("Apple")
            .state("CA")
            .limit(25)
            .build();

        assert_eq!(params.q, Some("Apple".to_string()));
        assert_eq!(params.state, Some("CA".to_string()));
        assert_eq!(params.limit, Some(25));
    }

    #[test]
    fn test_search_companies_params_serialize() {
        let params = SearchCompaniesParams::builder()
            .sic_code(3571)
            .build();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["sicCode"], 3571);
    }

    #[test]
    fn test_filing_status_serialize() {
        assert_eq!(
            serde_json::to_value(FilingStatus::All).unwrap(),
            serde_json::json!("all")
        );
        assert_eq!(
            serde_json::to_value(FilingStatus::Provisional).unwrap(),
            serde_json::json!("provisional")
        );
        assert_eq!(
            serde_json::to_value(FilingStatus::Final).unwrap(),
            serde_json::json!("final")
        );
    }

    #[test]
    fn test_transaction_direction_serialize() {
        assert_eq!(
            serde_json::to_value(TransactionDirection::Buy).unwrap(),
            serde_json::json!("buy")
        );
        assert_eq!(
            serde_json::to_value(TransactionDirection::Sell).unwrap(),
            serde_json::json!("sell")
        );
    }

    #[test]
    fn test_put_call_filter_serialize() {
        assert_eq!(
            serde_json::to_value(PutCallFilter::Put).unwrap(),
            serde_json::json!("put")
        );
        assert_eq!(
            serde_json::to_value(PutCallFilter::Call).unwrap(),
            serde_json::json!("call")
        );
        assert_eq!(
            serde_json::to_value(PutCallFilter::Equity).unwrap(),
            serde_json::json!("equity")
        );
    }
}
