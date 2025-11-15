//! Example: Get address history
//!
//! Demonstrates retrieving the complete MLS history for a specific address.
//!
//! Usage:
//!   cargo run --example address_history
//!
//! Configuration:
//!   This example reads address parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed
//!
//! Note: This endpoint requires a production API key

use repliers_beta::{config::Config, AddressHistoryQuery, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    let client = RepliersClient::from_env()?;

    // Create structured address query from config
    let cfg = &config.address_history;
    let query = AddressHistoryQuery {
        street_number: cfg.street_number.clone(),
        street_name: cfg.street_name.clone(),
        city: Some(cfg.city.clone()),
        state: Some(cfg.state.clone()),
        zip: cfg.zip.clone(),
        board_id: cfg.board_id.clone(),
    };

    println!("Fetching history for: {} {} {}, {}",
        query.street_number,
        query.street_name,
        query.city.as_ref().unwrap_or(&"".to_string()),
        query.state.as_ref().unwrap_or(&"".to_string())
    );

    let history = client.get_address_history(query).await?;

    println!("\nAddress history for: {}", history.address);
    println!("Found {} historical entries\n", history.history.len());

    for entry in &history.history {
        println!("MLS #{}: {} - ${:?}",
            entry.mls_number,
            entry.status,
            entry.list_price
        );
        if let Some(list_date) = &entry.list_date {
            println!("  Listed: {}", list_date);
        }
        if let Some(sold_date) = &entry.sold_date {
            println!("  Sold: {} for ${:?}", sold_date, entry.sold_price);
        }
        println!();
    }

    Ok(())
}
