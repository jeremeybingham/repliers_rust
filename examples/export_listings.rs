//! Example: Export listings to JSON file
//!
//! Demonstrates how to fetch property listings and export them to a local JSON file
//! for offline analysis.
//!
//! Usage:
//!   cargo run --example export_listings
//!
//! Expected output:
//!   Fetching 20 active properties in Toronto...
//!   Successfully retrieved X listings
//!   Exported listings to: toronto_listings.json
//!
//! Configuration:
//!   This example reads parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{config::Config, ListingSearchRequest, RepliersClient};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    // Load API key from environment (.env file or REPLIERS_API_KEY env var)
    let client = RepliersClient::from_env()?;

    let cfg = &config.export;

    println!(
        "Fetching {} active properties in {}...",
        cfg.results_per_page, cfg.city
    );

    // Build search request for active listings
    let request = ListingSearchRequest {
        city: Some(cfg.city.clone()),
        status: Some(cfg.status.clone()),
        page: Some(1),
        results_per_page: Some(cfg.results_per_page),
        ..Default::default()
    };

    // Execute search
    match client.search_listings(request).await {
        Ok(results) => {
            println!("Successfully retrieved {} listings", results.listings.len());
            println!(
                "Total available: {} listings across {} pages",
                results.count, results.num_pages
            );

            // Convert to pretty JSON
            let json_output = serde_json::to_string_pretty(&results.listings)?;

            // Write to file
            let mut file = File::create(&cfg.output_file)?;
            file.write_all(json_output.as_bytes())?;

            println!("\nExported listings to: {}", cfg.output_file);
            println!("File size: {} bytes", json_output.len());

            // Show preview of first listing
            if !results.listings.is_empty() {
                println!("\nPreview of first listing:");
                println!("{}", serde_json::to_string_pretty(&results.listings[0])?);
            }
        }
        Err(e) => {
            eprintln!("Error fetching listings: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
