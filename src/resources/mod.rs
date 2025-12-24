//! API resources for the EarningsFeed client.
//!
//! Each resource provides methods for accessing a specific API endpoint.

mod companies;
mod filings;
mod insider;
mod institutional;

pub use companies::CompaniesResource;
pub use filings::FilingsResource;
pub use insider::InsiderResource;
pub use institutional::InstitutionalResource;
