# EarningsFeed Rust SDK

[![Crates.io](https://img.shields.io/crates/v/earningsfeed)](https://crates.io/crates/earningsfeed)
[![docs.rs](https://img.shields.io/docsrs/earningsfeed)](https://docs.rs/earningsfeed)

Official Rust client for the [EarningsFeed API](https://earningsfeed.com/api) — SEC filings, insider transactions, and institutional holdings.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
earningsfeed = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use earningsfeed::EarningsFeed;

#[tokio::main]
async fn main() -> Result<(), earningsfeed::Error> {
    let client = EarningsFeed::new("your_api_key")?;

    // Get recent 10-K and 10-Q filings
    let params = earningsfeed::ListFilingsParams::builder()
        .forms(vec!["10-K", "10-Q"])
        .limit(10)
        .build();

    let filings = client.filings().list(&params).await?;
    for filing in filings.items {
        println!("{}: {} - {}", filing.form_type, filing.company_name, filing.title);
    }

    // Get a company profile
    let apple = client.companies().get(320193).await?;
    println!("{} ({})", apple.name, apple.primary_ticker.unwrap_or_default());

    Ok(())
}
```

## Features

- **Async/await** — Built on tokio and reqwest
- **Streaming pagination** — Auto-pagination with `Stream`
- **Type-safe** — Full Rust type definitions
- **Builder patterns** — Ergonomic parameter construction

## Usage

### SEC Filings

```rust
use earningsfeed::{EarningsFeed, ListFilingsParams};
use futures::StreamExt;
use std::pin::pin;

let client = EarningsFeed::new("your_api_key")?;

// List filings with filters
let params = ListFilingsParams::builder()
    .ticker("AAPL")
    .forms(vec!["10-K", "10-Q", "8-K"])
    .limit(25)
    .build();

let filings = client.filings().list(&params).await?;

// Iterate through all filings (auto-pagination)
let params = ListFilingsParams::builder()
    .ticker("AAPL")
    .forms(vec!["8-K"])
    .build();

let filings_resource = client.filings();
let mut stream = pin!(filings_resource.iter(params));

while let Some(result) = stream.next().await {
    let filing = result?;
    println!("{}", filing.title);
}

// Get filing details with documents
let detail = client.filings().get("0000320193-24-000123").await?;
for doc in detail.documents {
    println!("{}: {}", doc.doc_type, doc.filename);
}
```

### Insider Transactions

```rust
use earningsfeed::{EarningsFeed, ListInsiderParams, TransactionDirection};
use futures::StreamExt;
use std::pin::pin;

let client = EarningsFeed::new("your_api_key")?;

// Recent insider purchases
let params = ListInsiderParams::builder()
    .ticker("AAPL")
    .direction(TransactionDirection::Buy)
    .codes(vec!["P"]) // Open market purchases
    .limit(50)
    .build();

let purchases = client.insider().list(&params).await?;

for txn in purchases.items {
    println!(
        "{}: {} shares @ ${:.2}",
        txn.person_name,
        txn.shares.unwrap_or_default(),
        txn.price_per_share.unwrap_or_default()
    );
}

// Large sales across all companies
let params = ListInsiderParams::builder()
    .direction(TransactionDirection::Sell)
    .min_value(1_000_000)
    .build();

let insider_resource = client.insider();
let mut stream = pin!(insider_resource.iter(params));

while let Some(result) = stream.next().await {
    let txn = result?;
    println!("{}: ${}", txn.company_name.unwrap_or_default(), txn.transaction_value.unwrap_or_default());
}
```

### Institutional Holdings (13F)

```rust
use earningsfeed::{EarningsFeed, ListInstitutionalParams};
use futures::StreamExt;
use std::pin::pin;

let client = EarningsFeed::new("your_api_key")?;

// Who owns Apple?
let params = ListInstitutionalParams::builder()
    .ticker("AAPL")
    .min_value(1_000_000_000) // $1B+ positions
    .build();

let holdings = client.institutional().list(&params).await?;

for h in holdings.items {
    println!("{}: {} shares (${:.0})", h.manager_name, h.shares, h.value);
}

// Track a specific fund (Berkshire Hathaway)
let params = ListInstitutionalParams::builder()
    .manager_cik(1067983)
    .build();

let institutional_resource = client.institutional();
let mut stream = pin!(institutional_resource.iter(params));

while let Some(result) = stream.next().await {
    let h = result?;
    println!("{}: {} shares", h.issuer_name, h.shares);
}
```

### Companies

```rust
use earningsfeed::{EarningsFeed, SearchCompaniesParams};

let client = EarningsFeed::new("your_api_key")?;

// Get company profile
let company = client.companies().get(320193).await?;
println!("{}", company.name);
println!("Ticker: {}", company.primary_ticker.unwrap_or_default());
if let Some(sic) = company.sic_codes.first() {
    println!("Industry: {}", sic.description.as_deref().unwrap_or("Unknown"));
}

// Search companies
let params = SearchCompaniesParams::builder()
    .q("software")
    .state("CA")
    .limit(10)
    .build();

let results = client.companies().search(&params).await?;
for company in results.items {
    println!("{} ({})", company.name, company.ticker.unwrap_or_default());
}
```

## Error Handling

```rust
use earningsfeed::{EarningsFeed, Error};

let client = EarningsFeed::new("your_api_key")?;

match client.filings().get("invalid-accession").await {
    Ok(filing) => println!("Found: {}", filing.title),
    Err(Error::NotFound { path }) => println!("Filing not found: {}", path),
    Err(Error::RateLimit { reset_at }) => {
        println!("Rate limited. Resets at: {:?}", reset_at);
    }
    Err(Error::Authentication) => println!("Invalid API key"),
    Err(e) => println!("Error: {}", e),
}
```

## Configuration

```rust
use earningsfeed::{EarningsFeed, ClientConfig};
use std::time::Duration;

// Custom configuration
let config = ClientConfig::builder()
    .api_key("your_api_key")
    .timeout(Duration::from_secs(60))
    .base_url("https://earningsfeed.com") // Optional, this is the default
    .build()?;

let client = EarningsFeed::with_config(config)?;
```

## API Reference

Full API documentation: [earningsfeed.com/docs/api](https://earningsfeed.com/docs/api)

## License

MIT
