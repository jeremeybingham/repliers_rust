//! Example: Get deleted listings
//!
//! Demonstrates retrieving listings that have been deleted from the MLS.
//!
//! Usage:
//!   cargo run --example deleted_listings
//!
//! Configuration:
//!   This example reads parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{config::Config, DeletedListingsQuery, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    let client = RepliersClient::from_env()?;

    let cfg = &config.deleted;
    let query = DeletedListingsQuery {
        min_updated_on: Some(cfg.min_updated_on.clone()),
        max_updated_on: Some(cfg.max_updated_on.clone()),
        page: Some(cfg.page),
        results_per_page: Some(cfg.results_per_page),
        ..Default::default()
    };

    println!("Fetching deleted listings...");

    let deleted = client.get_deleted_listings(query).await?;

    println!("Found {} deleted listings across {} pages\n", deleted.count, deleted.num_pages);

    for listing in &deleted.listings {
        // Format the address
        let mut address_parts = Vec::new();
        if let Some(num) = &listing.address.street_number {
            address_parts.push(num.clone());
        }
        if let Some(name) = &listing.address.street_name {
            address_parts.push(name.clone());
        }
        if let Some(suffix) = &listing.address.street_suffix {
            address_parts.push(suffix.clone());
        }
        let street = address_parts.join(" ");

        let mut full_address = street.clone();
        if let Some(city) = &listing.address.city {
            if !full_address.is_empty() {
                full_address.push_str(", ");
            }
            full_address.push_str(city);
        }
        if let Some(state) = &listing.address.state {
            full_address.push_str(", ");
            full_address.push_str(state);
        }
        if let Some(zip) = &listing.address.zip {
            full_address.push(' ');
            full_address.push_str(zip);
        }

        println!("MLS #{}: {}", listing.mls_number, full_address);
        println!("  Board ID: {}", listing.board_id);
        println!("  Resource: {}", listing.resource);
        println!("  Last updated: {}", listing.timestamps.listing_updated);

        if let Some(neighborhood) = &listing.address.neighborhood {
            println!("  Neighborhood: {}", neighborhood);
        }
        if let Some(area) = &listing.address.area {
            println!("  Area: {}", area);
        }
        println!();
    }

    Ok(())
}
