//! Example: Discovery - Database Introspection
//!
//! Demonstrates how to use the search API to discover and analyze the property database.
//! This example performs multiple queries to generate a comprehensive JSON report with:
//! - Total properties available
//! - Top 5 cities by listing count
//! - Example listings from different categories
//! - Property type distribution
//! - Status distribution
//! - Price range statistics
//!
//! Usage:
//!   cargo run --example discovery
//!
//! Optional: Save output to file
//!   cargo run --example discovery > discovery_report.json
//!
//! Configuration:
//!   This example reads analysis parameters from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{ListingSearchRequest, RepliersClient};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct DiscoveryReport {
    generated_at: String,
    total_properties: u32,
    top_cities: Vec<CityStats>,
    property_types: HashMap<String, u32>,
    status_distribution: HashMap<String, u32>,
    price_statistics: PriceStats,
    example_listings: Vec<serde_json::Value>,
    sample_queries: Vec<QueryExample>,
}

#[derive(Debug, Serialize)]
struct CityStats {
    city: String,
    count: u32,
    avg_price: Option<f64>,
}

#[derive(Debug, Serialize)]
struct PriceStats {
    total_listings_analyzed: u32,
    ranges: Vec<PriceRangeCount>,
}

#[derive(Debug, Serialize)]
struct PriceRangeCount {
    range: String,
    count: u32,
}

#[derive(Debug, Serialize)]
struct QueryExample {
    description: String,
    city: Option<String>,
    status: Option<Vec<String>>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    bedrooms: Option<u32>,
    property_type: Option<Vec<String>>,
    result_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let client = RepliersClient::from_env()?;

    eprintln!("🔍 Starting database discovery...\n");

    // 1. Get total properties count
    eprintln!("📊 Fetching total properties...");
    let total_request = ListingSearchRequest::builder()
        .results_per_page(1)
        .page(1)
        .build();

    let total_response = client.search_listings(total_request).await?;
    let total_properties = total_response.count;
    eprintln!("   Found {} total properties\n", total_properties);

    // 2. Discover top cities
    eprintln!("🏙️  Analyzing top cities...");
    // Load configuration from config.toml
    let config = repliers_beta::config::Config::load_or_exit();
    let test_cities = &config.discovery.test_cities;

    let mut city_stats = Vec::new();
    for city in test_cities {
        let request = ListingSearchRequest::builder()
            .city(city)
            .results_per_page(50)
            .page(1)
            .build();

        if let Ok(response) = client.search_listings(request).await {
            if response.count > 0 {
                // Calculate average price from first page
                let avg_price = if !response.listings.is_empty() {
                    let prices: Vec<f64> = response.listings.iter()
                        .filter_map(|l| l.get("listPrice")?.as_f64())
                        .collect();
                    if !prices.is_empty() {
                        Some(prices.iter().sum::<f64>() / prices.len() as f64)
                    } else {
                        None
                    }
                } else {
                    None
                };

                city_stats.push(CityStats {
                    city: city.to_string(),
                    count: response.count,
                    avg_price,
                });
                eprintln!("   ✓ {}: {} listings", city, response.count);
            }
        }
    }

    // Sort by count and take top 5
    city_stats.sort_by(|a, b| b.count.cmp(&a.count));
    let top_cities: Vec<CityStats> = city_stats.into_iter().take(5).collect();
    eprintln!("   Top city: {} with {} listings\n", top_cities[0].city, top_cities[0].count);

    // 3. Analyze property types
    eprintln!("🏠 Analyzing property types...");
    let property_types = &config.discovery.property_types;
    let mut property_type_distribution = HashMap::new();

    for prop_type in property_types {
        let request = ListingSearchRequest::builder()
            .add_property_type(prop_type.clone())
            .results_per_page(1)
            .build();

        if let Ok(response) = client.search_listings(request).await {
            property_type_distribution.insert(prop_type.to_string(), response.count);
            eprintln!("   {}: {} listings", prop_type, response.count);
        }
    }
    eprintln!();

    // 4. Analyze status distribution
    eprintln!("📈 Analyzing listing status...");
    let statuses = &config.discovery.statuses;
    let mut status_distribution = HashMap::new();

    for status in statuses {
        let request = ListingSearchRequest::builder()
            .add_status(status.clone())
            .results_per_page(1)
            .build();

        if let Ok(response) = client.search_listings(request).await {
            status_distribution.insert(status.to_string(), response.count);
            eprintln!("   {}: {} listings", status, response.count);
        }
    }
    eprintln!();

    // 5. Analyze price ranges
    eprintln!("💰 Analyzing price ranges...");
    let price_ranges = &config.discovery.price_ranges;

    let mut price_range_counts = Vec::new();
    for range in price_ranges {
        let mut builder = ListingSearchRequest::builder()
            .results_per_page(1);

        if let Some(min_price) = range.min_price {
            builder = builder.min_price(min_price);
        }
        if let Some(max_price) = range.max_price {
            builder = builder.max_price(max_price);
        }

        let request = builder.build();
        if let Ok(response) = client.search_listings(request).await {
            price_range_counts.push(PriceRangeCount {
                range: range.label.clone(),
                count: response.count,
            });
            eprintln!("   {}: {} listings", range.label, response.count);
        }
    }
    eprintln!();

    // 6. Collect example listings from different categories
    eprintln!("📋 Collecting example listings...");
    let mut example_listings = Vec::new();

    // Get a few diverse examples
    let example_queries = vec![
        ("Active luxury condo", ListingSearchRequest::builder()
            .add_status("Active")
            .add_property_type("Condo")
            .min_price(1000000.0)
            .results_per_page(2)
            .build()),
        ("Affordable townhouse", ListingSearchRequest::builder()
            .add_property_type("Townhouse")
            .max_price(500000.0)
            .results_per_page(2)
            .build()),
        ("Family home (3+ beds)", ListingSearchRequest::builder()
            .bedrooms(3)
            .add_property_type("Detached")
            .results_per_page(2)
            .build()),
    ];

    for (desc, request) in example_queries {
        if let Ok(response) = client.search_listings(request).await {
            for listing in response.listings.into_iter().take(2) {
                example_listings.push(listing);
            }
            eprintln!("   ✓ Collected {} examples", desc);
        }
    }

    // Limit to 10 examples total
    example_listings.truncate(10);
    eprintln!("   Total examples: {}\n", example_listings.len());

    // 7. Create sample queries for documentation
    eprintln!("📝 Generating sample queries...");
    let sample_queries = vec![
        QueryExample {
            description: "All active listings in Toronto".to_string(),
            city: Some("Toronto".to_string()),
            status: Some(vec!["Active".to_string()]),
            min_price: None,
            max_price: None,
            bedrooms: None,
            property_type: None,
            result_count: get_query_count(&client, ListingSearchRequest::builder()
                .city("Toronto")
                .add_status("Active")
                .build()).await,
        },
        QueryExample {
            description: "3-bedroom condos under $800k".to_string(),
            city: None,
            status: None,
            min_price: None,
            max_price: Some(800000.0),
            bedrooms: Some(3),
            property_type: Some(vec!["Condo".to_string()]),
            result_count: get_query_count(&client, ListingSearchRequest::builder()
                .bedrooms(3)
                .add_property_type("Condo")
                .max_price(800000.0)
                .build()).await,
        },
        QueryExample {
            description: "Luxury homes in Vancouver over $2M".to_string(),
            city: Some("Vancouver".to_string()),
            status: None,
            min_price: Some(2000000.0),
            max_price: None,
            bedrooms: None,
            property_type: Some(vec!["Detached".to_string()]),
            result_count: get_query_count(&client, ListingSearchRequest::builder()
                .city("Vancouver")
                .add_property_type("Detached")
                .min_price(2000000.0)
                .build()).await,
        },
        QueryExample {
            description: "Recently sold properties".to_string(),
            city: None,
            status: Some(vec!["Sold".to_string()]),
            min_price: None,
            max_price: None,
            bedrooms: None,
            property_type: None,
            result_count: get_query_count(&client, ListingSearchRequest::builder()
                .add_status("Sold")
                .build()).await,
        },
    ];
    eprintln!("   Generated {} sample queries\n", sample_queries.len());

    // 8. Generate final report
    eprintln!("✅ Discovery complete! Generating report...\n");

    let report = DiscoveryReport {
        generated_at: chrono::Utc::now().to_rfc3339(),
        total_properties,
        top_cities,
        property_types: property_type_distribution,
        status_distribution,
        price_statistics: PriceStats {
            total_listings_analyzed: total_properties,
            ranges: price_range_counts,
        },
        example_listings,
        sample_queries,
    };

    // Output JSON report to stdout
    println!("{}", serde_json::to_string_pretty(&report)?);

    eprintln!("\n✨ Report generated successfully!");
    eprintln!("💡 Tip: Redirect output to a file: cargo run --example discovery > report.json");

    Ok(())
}

/// Helper function to get count from a query
async fn get_query_count(client: &RepliersClient, request: ListingSearchRequest) -> u32 {
    client.search_listings(request)
        .await
        .map(|r| r.count)
        .unwrap_or(0)
}
