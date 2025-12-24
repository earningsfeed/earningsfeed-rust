//! Integration test against the live EarningsFeed API.
//!
//! Run with: EARNINGSFEED_API_KEY=your_key cargo run --example integration_test

use earningsfeed::{EarningsFeed, ListFilingsParams, ListInsiderParams, ListInstitutionalParams, SearchCompaniesParams};
use futures::StreamExt;
use std::pin::pin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("EARNINGSFEED_API_KEY")
        .expect("EARNINGSFEED_API_KEY environment variable required");

    println!("Creating client...");
    let client = EarningsFeed::new(&api_key)?;
    println!("✓ Client created\n");

    // Test 1: List filings
    println!("=== Testing Filings ===");
    let params = ListFilingsParams::builder()
        .forms(vec!["10-K", "10-Q"])
        .limit(3)
        .build();

    let filings = client.filings().list(&params).await?;
    println!("✓ Listed {} filings (has_more: {})", filings.items.len(), filings.has_more);
    for f in &filings.items {
        println!("  - {} | {} | {}", f.form_type, f.company_name.as_deref().unwrap_or("N/A"), f.filed_at);
    }

    // Test 2: Get filing detail
    if let Some(first) = filings.items.first() {
        println!("\nGetting filing detail for {}...", first.accession_number);
        let detail = client.filings().get(&first.accession_number).await?;
        println!("✓ Got detail: {} documents", detail.documents.len());
        for doc in detail.documents.iter().take(3) {
            println!("  - {} ({})", doc.filename, doc.doc_type);
        }
    }

    // Test 3: Filings iterator (pagination)
    println!("\nTesting filings iterator (first 5 items)...");
    let iter_params = ListFilingsParams::builder()
        .ticker("AAPL")
        .limit(2) // Small pages to test pagination
        .build();

    let filings_resource = client.filings();
    let mut stream = pin!(filings_resource.iter(iter_params));
    let mut count = 0;
    while let Some(result) = stream.next().await {
        let filing = result?;
        println!("  - {} | {}", filing.form_type, filing.title);
        count += 1;
        if count >= 5 { break; }
    }
    println!("✓ Iterated {} filings", count);

    // Test 4: Insider transactions
    println!("\n=== Testing Insider Transactions ===");
    let params = ListInsiderParams::builder()
        .ticker("AAPL")
        .limit(3)
        .build();

    let insider = client.insider().list(&params).await?;
    println!("✓ Listed {} insider transactions", insider.items.len());
    for txn in &insider.items {
        println!("  - {} | {} | {:?} shares",
            txn.person_name,
            txn.transaction_code,
            txn.shares);
    }

    // Test 5: Institutional holdings
    println!("\n=== Testing Institutional Holdings ===");
    let params = ListInstitutionalParams::builder()
        .ticker("AAPL")
        .limit(3)
        .build();

    let holdings = client.institutional().list(&params).await?;
    println!("✓ Listed {} institutional holdings", holdings.items.len());
    for h in &holdings.items {
        println!("  - {} | {} shares | ${}", h.manager_name, h.shares, h.value);
    }

    // Test 6: Company lookup
    println!("\n=== Testing Companies ===");
    let company = client.companies().get(320193).await?; // Apple's CIK
    println!("✓ Got company: {} (CIK: {})", company.name, company.cik);
    println!("  Ticker: {}", company.primary_ticker.as_deref().unwrap_or("N/A"));

    // Test 7: Company search
    println!("\nSearching for 'microsoft'...");
    let params = SearchCompaniesParams::builder()
        .q("microsoft")
        .limit(3)
        .build();

    let results = client.companies().search(&params).await?;
    println!("✓ Found {} results", results.items.len());
    for c in &results.items {
        println!("  - {} ({})", c.name, c.ticker.as_deref().unwrap_or("N/A"));
    }

    println!("\n=== All tests passed! ===");
    Ok(())
}
