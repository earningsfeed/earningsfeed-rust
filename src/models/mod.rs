//! Data models for the EarningsFeed API.
//!
//! This module contains all the data types used for API requests and responses.

mod common;
mod company;
mod filing;
mod insider;
mod institutional;
mod params;

pub use common::PaginatedResponse;
pub use company::{Address, Company, CompanySearchResult, SicCode, Ticker};
pub use filing::{EntityClass, Filing, FilingCompany, FilingDetail, FilingDocument, FilingRole};
pub use insider::{AcquiredDisposed, DirectIndirect, InsiderTransaction};
pub use institutional::{InstitutionalHolding, InvestmentDiscretion, PutCall, SharesType};
pub use params::{
    FilingStatus, ListFilingsParams, ListInsiderParams, ListInstitutionalParams,
    PutCallFilter, SearchCompaniesParams, TransactionDirection,
};
