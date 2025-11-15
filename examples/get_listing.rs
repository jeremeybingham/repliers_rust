//! Example: Get a single listing by MLS number
//!
//! Demonstrates retrieving detailed information for a specific property.
//!
//! Usage:
//!   cargo run --example get_listing
//!
//! Configuration:
//!   This example reads the MLS number from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{config::Config, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    let client = RepliersClient::from_env()?;

    let mls_number = &config.listing.mls_number;
    let board_id = config.listing.board_id.as_deref();

    println!("Fetching listing {}...", mls_number);
    println!("Note: Update the MLS number in config.toml with a valid one from your MLS board\n");

    match client.get_listing(mls_number, board_id).await {
        Ok(listing) => {
            println!("Listing details retrieved successfully!");
            println!("{:#}", listing);
        }
        Err(e) => {
            eprintln!("Error fetching listing: {}", e);
            eprintln!("\nMake sure you:");
            eprintln!("1. Have set REPLIERS_API_KEY in your .env file");
            eprintln!("2. Updated config.toml with a valid MLS number from your accessible MLS board");
            eprintln!(
                "3. If you have multi-MLS access, set the boardId in config.toml"
            );
            return Err(e.into());
        }
    }

    Ok(())
}
