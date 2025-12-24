//! # EarningsFeed
//!
//! Official Rust client for the [EarningsFeed API](https://earningsfeed.com).
//!
//! This library provides access to SEC filings, insider transactions,
//! and institutional holdings data.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use earningsfeed::{EarningsFeed, ListFilingsParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), earningsfeed::Error> {
//!     let client = EarningsFeed::new("your_api_key")?;
//!
//!     let params = ListFilingsParams::builder()
//!         .ticker("AAPL")
//!         .forms(vec!["10-K", "10-Q"])
//!         .limit(10)
//!         .build();
//!
//!     let response = client.filings().list(&params).await?;
//!
//!     for filing in response.items {
//!         println!("{}: {}", filing.form_type, filing.title);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **SEC Filings**: Access 10-K, 10-Q, 8-K, and other SEC filings
//! - **Insider Transactions**: Track Form 4 insider trading data
//! - **Institutional Holdings**: 13F institutional holdings data
//! - **Company Search**: Search and lookup company profiles
//! - **Async/Await**: Built on tokio and reqwest
//! - **Pagination**: Automatic pagination with async streams

mod client;
mod config;
mod error;
mod models;
mod resources;

pub use client::EarningsFeed;
pub use config::{ClientConfig, ClientConfigBuilder, DEFAULT_BASE_URL, DEFAULT_TIMEOUT};
pub use error::{Error, Result};
pub use models::{
    // Common
    PaginatedResponse,
    // Filing types
    EntityClass, Filing, FilingCompany, FilingDetail, FilingDocument, FilingRole,
    // Insider types
    AcquiredDisposed, DirectIndirect, InsiderTransaction,
    // Institutional types
    InstitutionalHolding, InvestmentDiscretion, PutCall, SharesType,
    // Company types
    Address, Company, CompanySearchResult, SicCode, Ticker,
    // Parameter types
    FilingStatus, ListFilingsParams, ListInsiderParams, ListInstitutionalParams,
    PutCallFilter, SearchCompaniesParams, TransactionDirection,
};
