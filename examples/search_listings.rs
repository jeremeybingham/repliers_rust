//! Example: Search for listings
//!
//! Demonstrates how to search for property listings with filters.
//!
//! Usage:
//!   cargo run --example search_listings
//!
//! Expected output:
//!   Searching for listings in [city]...
//!   Found X listings across Y pages
//!   Showing page 1 (N results per page)
//!
//! Configuration:
//!   This example reads parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{config::Config, ListingSearchRequest, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    // Load API key from environment (.env file or REPLIERS_API_KEY env var)
    let client = RepliersClient::from_env()?;

    println!("=== Example 1: Using struct directly ===");

    // Build search request using struct initialization with values from config
    let cfg = &config.search.example1;
    let request = ListingSearchRequest {
        city: Some(cfg.city.clone()),
        status: Some(cfg.status.clone()),
        min_price: Some(cfg.min_price),
        max_price: Some(cfg.max_price),
        bedrooms: Some(cfg.bedrooms),
        property_type: Some(cfg.property_type.clone()),
        page: Some(cfg.page),
        results_per_page: Some(cfg.results_per_page),
    };

    println!("Searching for listings in {}...", cfg.city);

    match client.search_listings(request).await {
        Ok(results) => {
            println!(
                "Found {} listings across {} pages",
                results.count, results.num_pages
            );
            println!(
                "Showing page {} ({} results per page)",
                results.page, results.page_size
            );

            // Display first few listings (if any)
            if !results.listings.is_empty() {
                println!("\nFirst listing preview:");
                println!("{}", serde_json::to_string_pretty(&results.listings[0])?);
            }
        }
        Err(e) => {
            eprintln!("Error searching listings: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example 2: Using builder pattern ===");

    // Build search request using the builder pattern with values from config
    let cfg = &config.search.example2;
    let request = ListingSearchRequest::builder()
        .city(&cfg.city)
        .add_status(&cfg.status[0])
        .price_range(cfg.min_price, cfg.max_price)
        .bedrooms(cfg.bedrooms)
        .add_property_type(&cfg.property_type[0])
        .page(cfg.page)
        .results_per_page(cfg.results_per_page)
        .build();

    println!("Searching for listings in {}...", cfg.city);

    match client.search_listings(request).await {
        Ok(results) => {
            println!(
                "Found {} listings across {} pages",
                results.count, results.num_pages
            );
            println!(
                "Showing page {} ({} results per page)",
                results.page, results.page_size
            );

            // Display first few listings (if any)
            if !results.listings.is_empty() {
                println!("\nFirst listing preview:");
                println!("{}", serde_json::to_string_pretty(&results.listings[0])?);
            }
        }
        Err(e) => {
            eprintln!("Error searching listings: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
