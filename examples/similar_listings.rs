//! Example: Find similar listings
//!
//! Demonstrates finding properties similar to a given listing.
//!
//! Usage:
//!   cargo run --example similar_listings
//!
//! Configuration:
//!   This example reads parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{config::Config, models::search::SimilarListingsRequest, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    let client = RepliersClient::from_env()?;

    // Build the similar listings request from config
    let cfg = &config.similar;
    let request = SimilarListingsRequest {
        mls_number: cfg.mls_number.clone(),
        radius: Some(cfg.radius),
        list_price_range: Some(cfg.list_price_range),
        ..Default::default()
    };

    println!("Finding listings similar to {}...", request.mls_number);
    println!("  - Radius: {:?} km", request.radius);
    println!("  - Price range: ±${:?}\n", request.list_price_range);

    match client.get_similar_listings(request).await {
        Ok(similar) => {
            println!("Similar listings retrieved successfully!");
            println!("  - Total similar listings: {}", similar.count);
            println!("  - Page {} of {}", similar.page, similar.num_pages);
            println!("  - Page size: {}\n", similar.page_size);

            for (i, listing) in similar.similar.iter().enumerate() {
                println!("Similar listing #{}", i + 1);
                println!("{:#}", listing);
            }
        }
        Err(e) => {
            eprintln!("Error fetching similar listings: {}", e);
            eprintln!("\nMake sure you:");
            eprintln!("1. Have set REPLIERS_API_KEY in your .env file");
            eprintln!("2. Updated config.toml with a valid MLS number from your accessible MLS board");
            eprintln!("3. The listing exists and is accessible via your API key");
            eprintln!("\nNote: You can adjust the radius and list_price_range in config.toml");
            return Err(e.into());
        }
    }

    Ok(())
}
