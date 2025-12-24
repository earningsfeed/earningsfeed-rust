//! Test error handling against the live API.
//!
//! Run with: EARNINGSFEED_API_KEY=your_key cargo run --example error_test

use earningsfeed::{EarningsFeed, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test 1: Invalid API key (401)
    println!("=== Testing Error Handling ===\n");

    println!("Test 1: Invalid API key...");
    let bad_client = EarningsFeed::new("invalid_key_12345")?;
    match bad_client.companies().get(320193).await {
        Err(Error::Authentication) => println!("✓ Got Authentication error as expected"),
        Err(e) => println!("✗ Got unexpected error: {:?}", e),
        Ok(_) => println!("✗ Should have failed but succeeded"),
    }

    // Test 2: Not found (404)
    let api_key = std::env::var("EARNINGSFEED_API_KEY")
        .expect("EARNINGSFEED_API_KEY environment variable required");
    let client = EarningsFeed::new(&api_key)?;

    println!("\nTest 2: Non-existent filing (404)...");
    match client.filings().get("0000000000-00-000000").await {
        Err(Error::NotFound { path }) => println!("✓ Got NotFound error for path: {}", path),
        Err(e) => println!("✗ Got unexpected error: {:?}", e),
        Ok(_) => println!("✗ Should have failed but succeeded"),
    }

    // Test 3: Non-existent company (404)
    println!("\nTest 3: Non-existent company (404)...");
    match client.companies().get(999999999).await {
        Err(Error::NotFound { path }) => println!("✓ Got NotFound error for path: {}", path),
        Err(e) => println!("✗ Got unexpected error: {:?}", e),
        Ok(_) => println!("✗ Should have failed but succeeded"),
    }

    // Test 4: Validation error (400)
    println!("\nTest 4: Invalid limit (validation error)...");
    let params = earningsfeed::ListFilingsParams::builder()
        .limit(9999) // Over the limit
        .build();

    match client.filings().list(&params).await {
        Err(Error::Validation { message }) => println!("✓ Got Validation error: {}", message),
        Err(e) => println!("? Got error (may be valid): {:?}", e),
        Ok(_) => println!("? Request succeeded (limit may be valid)"),
    }

    println!("\n=== Error handling tests complete! ===");
    Ok(())
}
