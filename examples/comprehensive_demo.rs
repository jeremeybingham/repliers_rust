//! Example: Comprehensive API Demo
//!
//! This example demonstrates all 4 working endpoints of the Repliers API in a single run:
//! 1. Search Listings (POST /listings)
//! 2. Get Single Listing (GET /listings/{mlsNumber})
//! 3. Get Similar Listings (GET /listings/{mlsNumber}/similar)
//! 4. Get Deleted Listings (GET /listings/deleted)
//!
//! The demo provides rich console output and exports a comprehensive JSON report.
//!
//! Usage:
//!   cargo run --example comprehensive_demo
//!
//! Expected output:
//!   - Formatted console output with section headers and summaries
//!   - JSON report exported to configured output file
//!
//! Configuration:
//!   This example reads parameters from config.toml under the [demo] section
//!   Copy config.toml.example to config.toml and adjust values as needed

use repliers_beta::{
    config::Config, DeletedListingsQuery, ListingSearchRequest, RepliersClient,
    SimilarListingsRequest,
};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

/// Report structure for JSON export
#[derive(Serialize)]
struct DemoReport {
    metadata: ReportMetadata,
    search_results: SearchResults,
    listing_details: ListingDetails,
    similar_listings: SimilarResults,
    deleted_listings: DeletedResults,
    summary: ReportSummary,
}

#[derive(Serialize)]
struct ReportMetadata {
    timestamp: String,
    demo_version: String,
    endpoints_tested: u32,
    total_duration_ms: u128,
}

#[derive(Serialize)]
struct SearchResults {
    query: SearchQuery,
    total_count: u32,
    num_pages: u32,
    page_size: u32,
    listings: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct SearchQuery {
    city: String,
    status: Vec<String>,
    min_price: f64,
    max_price: f64,
    bedrooms: u32,
    property_type: Vec<String>,
}

#[derive(Serialize)]
struct ListingDetails {
    mls_number: String,
    data: serde_json::Value,
}

#[derive(Serialize)]
struct SimilarResults {
    reference_mls: String,
    search_radius_km: f64,
    price_range: f64,
    count: u32,
    listings: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct DeletedResults {
    date_range: DateRange,
    total_count: u32,
    num_pages: u32,
    listings: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct DateRange {
    min_date: String,
    max_date: String,
}

#[derive(Serialize)]
struct ReportSummary {
    total_active_listings_found: u32,
    listing_details_fetched: bool,
    similar_listings_count: u32,
    deleted_listings_count: u32,
    endpoints_successful: u32,
}

fn print_header(title: &str) {
    let border = "=".repeat(80);
    println!("\n{}", border);
    println!("  {}", title);
    println!("{}\n", border);
}

fn print_section(title: &str) {
    println!("\n{}", "-".repeat(80));
    println!("  {}", title);
    println!("{}\n", "-".repeat(80));
}

fn print_stat(label: &str, value: &str) {
    println!("  {} {}", label, value);
}

fn format_price(price: f64) -> String {
    let price_str = format!("{:.0}", price);
    let mut result = String::from("$");
    let chars: Vec<char> = price_str.chars().collect();
    let len = chars.len();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

fn format_number(num: f64) -> String {
    let num_str = format!("{:.0}", num);
    let mut result = String::new();
    let chars: Vec<char> = num_str.chars().collect();
    let len = chars.len();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

fn extract_listing_info(listing: &serde_json::Value) -> (String, String, String) {
    let mls = listing["mlsNumber"]
        .as_str()
        .unwrap_or("N/A")
        .to_string();
    let address = listing["address"]["streetName"]
        .as_str()
        .or(listing["address"]["full"].as_str())
        .unwrap_or("Address not available");
    let price = listing["listPrice"]
        .as_f64()
        .map(format_price)
        .unwrap_or_else(|| "N/A".to_string());
    (mls, address.to_string(), price)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // Load configuration
    let config = Config::load_or_exit();
    let cfg = &config.demo;

    // Initialize API client
    let client = RepliersClient::from_env()?;

    print_header("REPLIERS API COMPREHENSIVE DEMO");
    println!("This demo showcases all 4 working endpoints of the Repliers Real Estate API:");
    println!("  1. Search Listings");
    println!("  2. Get Single Listing Details");
    println!("  3. Find Similar Listings");
    println!("  4. Get Deleted Listings");
    println!("\nStarting demo execution...");

    // Track success metrics
    let mut endpoints_successful = 0;

    // ========================================================================
    // STEP 1: Search for Listings
    // ========================================================================
    print_section("STEP 1: SEARCHING FOR LISTINGS");

    let search_request = ListingSearchRequest {
        city: Some(cfg.city.clone()),
        status: Some(cfg.status.clone()),
        min_price: Some(cfg.min_price),
        max_price: Some(cfg.max_price),
        bedrooms: Some(cfg.bedrooms),
        property_type: Some(cfg.property_type.clone()),
        page: Some(1),
        results_per_page: Some(cfg.results_per_page),
        ..Default::default()
    };

    println!("Search Parameters:");
    print_stat("  City:", &cfg.city);
    print_stat(
        "  Price Range:",
        &format!(
            "{} - {}",
            format_price(cfg.min_price),
            format_price(cfg.max_price)
        ),
    );
    print_stat("  Bedrooms:", &cfg.bedrooms.to_string());
    print_stat("  Property Types:", &cfg.property_type.join(", "));
    print_stat("  Status:", &cfg.status.join(", "));
    println!("\nExecuting search...");

    let search_response = client.search_listings(search_request).await?;
    endpoints_successful += 1;

    println!("\n✓ Search completed successfully!");
    print_stat("  Total listings found:", &search_response.count.to_string());
    print_stat("  Total pages:", &search_response.num_pages.to_string());
    print_stat(
        "  Results on this page:",
        &search_response.listings.len().to_string(),
    );

    if !search_response.listings.is_empty() {
        println!("\nTop {} listings:", search_response.listings.len().min(3));
        for (i, listing) in search_response.listings.iter().take(3).enumerate() {
            let (mls, address, price) = extract_listing_info(listing);
            println!("  {}. MLS #{}: {} - {}", i + 1, mls, address, price);
        }
    }

    // ========================================================================
    // STEP 2: Get Single Listing Details
    // ========================================================================
    print_section("STEP 2: GETTING LISTING DETAILS");

    println!("Fetching detailed information for MLS #{}...", cfg.mls_number);

    let listing_details = client.get_listing(&cfg.mls_number, None).await?;
    endpoints_successful += 1;

    println!("\n✓ Listing details retrieved successfully!");

    // Extract key information
    if let Some(address) = listing_details["address"].as_object() {
        if let Some(street) = address.get("streetName").and_then(|v| v.as_str()) {
            print_stat("  Address:", street);
        }
        if let Some(city) = address.get("city").and_then(|v| v.as_str()) {
            print_stat("  City:", city);
        }
    }
    if let Some(price) = listing_details["listPrice"].as_f64() {
        print_stat("  List Price:", &format_price(price));
    }
    if let Some(beds) = listing_details["bedrooms"].as_u64() {
        print_stat("  Bedrooms:", &beds.to_string());
    }
    if let Some(baths) = listing_details["bathrooms"].as_u64() {
        print_stat("  Bathrooms:", &baths.to_string());
    }
    if let Some(sqft) = listing_details["squareFeet"].as_f64() {
        print_stat("  Square Feet:", &format_number(sqft));
    }
    if let Some(prop_type) = listing_details["propertyType"].as_str() {
        print_stat("  Property Type:", prop_type);
    }

    // ========================================================================
    // STEP 3: Find Similar Listings
    // ========================================================================
    print_section("STEP 3: FINDING SIMILAR LISTINGS");

    let similar_request = SimilarListingsRequest {
        mls_number: cfg.mls_number.clone(),
        radius: Some(cfg.similar_radius),
        list_price_range: Some(cfg.similar_price_range),
        ..Default::default()
    };

    println!("Search Parameters:");
    print_stat("  Reference MLS:", &cfg.mls_number);
    print_stat("  Search Radius:", &format!("{} km", cfg.similar_radius));
    print_stat(
        "  Price Range:",
        &format!("±{}", format_price(cfg.similar_price_range)),
    );
    println!("\nExecuting similarity search...");

    let similar_response = client.get_similar_listings(similar_request).await?;
    endpoints_successful += 1;

    println!("\n✓ Similar listings search completed!");
    print_stat(
        "  Similar listings found:",
        &similar_response.count.to_string(),
    );

    if !similar_response.similar.is_empty() {
        println!(
            "\nTop {} similar listings:",
            similar_response.similar.len().min(3)
        );
        for (i, listing) in similar_response.similar.iter().take(3).enumerate() {
            let (mls, address, price) = extract_listing_info(listing);
            println!("  {}. MLS #{}: {} - {}", i + 1, mls, address, price);
        }
    }

    // ========================================================================
    // STEP 4: Get Deleted Listings
    // ========================================================================
    print_section("STEP 4: RETRIEVING DELETED LISTINGS");

    let deleted_query = DeletedListingsQuery {
        min_updated_on: Some(cfg.deleted_min_date.clone()),
        max_updated_on: Some(cfg.deleted_max_date.clone()),
        page: Some(1),
        results_per_page: Some(cfg.deleted_results_per_page),
        ..Default::default()
    };

    println!("Query Parameters:");
    print_stat("  Date Range:", &format!("{} to {}", cfg.deleted_min_date, cfg.deleted_max_date));
    print_stat(
        "  Results per Page:",
        &cfg.deleted_results_per_page.to_string(),
    );
    println!("\nExecuting query...");

    let deleted_response = client.get_deleted_listings(deleted_query).await?;
    endpoints_successful += 1;

    println!("\n✓ Deleted listings retrieved successfully!");
    print_stat("  Total deleted listings:", &deleted_response.count.to_string());
    print_stat("  Total pages:", &deleted_response.num_pages.to_string());
    print_stat(
        "  Results on this page:",
        &deleted_response.listings.len().to_string(),
    );

    if !deleted_response.listings.is_empty() {
        println!(
            "\nSample deleted listings (showing up to 3):"
        );
        for (i, listing) in deleted_response.listings.iter().take(3).enumerate() {
            let mut address_parts = Vec::new();
            if let Some(num) = &listing.address.street_number {
                address_parts.push(num.clone());
            }
            if let Some(name) = &listing.address.street_name {
                address_parts.push(name.clone());
            }
            let address = if address_parts.is_empty() {
                "Address not available".to_string()
            } else {
                address_parts.join(" ")
            };
            println!(
                "  {}. MLS #{}: {} (Updated: {})",
                i + 1,
                listing.mls_number,
                address,
                listing.timestamps.listing_updated
            );
        }
    }

    // ========================================================================
    // Export Results to JSON
    // ========================================================================
    print_section("EXPORTING RESULTS");

    let duration = start_time.elapsed().as_millis();

    // Convert deleted listings to JSON for export
    let deleted_listings_json: Vec<serde_json::Value> = deleted_response
        .listings
        .iter()
        .map(|l| serde_json::to_value(l).unwrap())
        .collect();

    let report = DemoReport {
        metadata: ReportMetadata {
            timestamp: chrono::Utc::now().to_rfc3339(),
            demo_version: "1.0".to_string(),
            endpoints_tested: 4,
            total_duration_ms: duration,
        },
        search_results: SearchResults {
            query: SearchQuery {
                city: cfg.city.clone(),
                status: cfg.status.clone(),
                min_price: cfg.min_price,
                max_price: cfg.max_price,
                bedrooms: cfg.bedrooms,
                property_type: cfg.property_type.clone(),
            },
            total_count: search_response.count,
            num_pages: search_response.num_pages,
            page_size: search_response.page_size,
            listings: search_response.listings,
        },
        listing_details: ListingDetails {
            mls_number: cfg.mls_number.clone(),
            data: listing_details,
        },
        similar_listings: SimilarResults {
            reference_mls: cfg.mls_number.clone(),
            search_radius_km: cfg.similar_radius,
            price_range: cfg.similar_price_range,
            count: similar_response.count,
            listings: similar_response.similar,
        },
        deleted_listings: DeletedResults {
            date_range: DateRange {
                min_date: cfg.deleted_min_date.clone(),
                max_date: cfg.deleted_max_date.clone(),
            },
            total_count: deleted_response.count,
            num_pages: deleted_response.num_pages,
            listings: deleted_listings_json,
        },
        summary: ReportSummary {
            total_active_listings_found: search_response.count,
            listing_details_fetched: true,
            similar_listings_count: similar_response.count,
            deleted_listings_count: deleted_response.count,
            endpoints_successful,
        },
    };

    let json_output = serde_json::to_string_pretty(&report)?;
    let mut file = File::create(&cfg.output_file)?;
    file.write_all(json_output.as_bytes())?;

    println!("✓ Report exported successfully!");
    print_stat("  Output file:", &cfg.output_file);
    print_stat("  File size:", &format!("{} bytes", json_output.len()));

    // ========================================================================
    // Final Summary
    // ========================================================================
    print_header("DEMO SUMMARY");

    println!("Execution Results:");
    print_stat(
        "  ✓ Endpoints tested:",
        &format!("{}/4", endpoints_successful),
    );
    print_stat("  ✓ Active listings found:", &search_response.count.to_string());
    print_stat("  ✓ Similar listings found:", &similar_response.count.to_string());
    print_stat("  ✓ Deleted listings found:", &deleted_response.count.to_string());
    print_stat("  ✓ Total execution time:", &format!("{} ms", duration));

    println!("\nReport Location:");
    print_stat("  ", &format!("📄 {}", cfg.output_file));

    println!("\n{}", "=".repeat(80));
    println!("Demo completed successfully! All endpoints working as expected.");
    println!("{}\n", "=".repeat(80));

    Ok(())
}
