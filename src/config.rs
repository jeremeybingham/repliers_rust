//! Configuration module for examples
//!
//! This module provides configuration loading from a TOML file for all examples.
//! It allows externalizing test data and parameters instead of hardcoding them in examples.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Main configuration structure
#[derive(Debug, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
    pub ai_search: AiSearchConfig,
    pub listing: ListingConfig,
    pub similar: SimilarConfig,
    pub address_history: AddressHistoryConfig,
    pub deleted: DeletedConfig,
    pub discovery: DiscoveryConfig,
    pub export: ExportConfig,
    pub demo: DemoConfig,
}

/// Search listings configuration
#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    pub example1: SearchExample,
    pub example2: SearchExample,
}

/// Individual search example
#[derive(Debug, Deserialize)]
pub struct SearchExample {
    pub city: String,
    pub status: Vec<String>,
    pub min_price: f64,
    pub max_price: f64,
    pub bedrooms: u32,
    pub property_type: Vec<String>,
    pub page: u32,
    pub results_per_page: u32,
}

/// AI search configuration
#[derive(Debug, Deserialize)]
pub struct AiSearchConfig {
    pub prompt: String,
}

/// Single listing configuration
#[derive(Debug, Deserialize)]
pub struct ListingConfig {
    pub mls_number: String,
    #[serde(default)]
    pub board_id: Option<String>,
}

/// Similar listings configuration
#[derive(Debug, Deserialize)]
pub struct SimilarConfig {
    pub mls_number: String,
    pub radius: f64,
    pub list_price_range: f64,
}

/// Address history configuration
#[derive(Debug, Deserialize)]
pub struct AddressHistoryConfig {
    pub street_number: String,
    pub street_name: String,
    pub city: String,
    pub state: String,
    #[serde(default)]
    pub zip: Option<String>,
    #[serde(default)]
    pub board_id: Option<String>,
}

/// Deleted listings configuration
#[derive(Debug, Deserialize)]
pub struct DeletedConfig {
    pub min_updated_on: String,
    pub max_updated_on: String,
    pub page: u32,
    pub results_per_page: u32,
}

/// Discovery example configuration
#[derive(Debug, Deserialize)]
pub struct DiscoveryConfig {
    pub test_cities: Vec<String>,
    pub property_types: Vec<String>,
    pub statuses: Vec<String>,
    pub price_ranges: Vec<PriceRange>,
}

/// Price range for discovery
#[derive(Debug, Deserialize)]
pub struct PriceRange {
    pub label: String,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

/// Export listings configuration
#[derive(Debug, Deserialize)]
pub struct ExportConfig {
    pub city: String,
    pub status: Vec<String>,
    pub results_per_page: u32,
    pub output_file: String,
}

/// Comprehensive demo configuration
#[derive(Debug, Deserialize)]
pub struct DemoConfig {
    pub city: String,
    pub status: Vec<String>,
    pub min_price: f64,
    pub max_price: f64,
    pub bedrooms: u32,
    pub property_type: Vec<String>,
    pub results_per_page: u32,
    pub mls_number: String,
    pub similar_radius: f64,
    pub similar_price_range: f64,
    pub deleted_min_date: String,
    pub deleted_max_date: String,
    pub deleted_results_per_page: u32,
    pub output_file: String,
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the config.toml file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from default location (./config.toml)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_file("config.toml")
    }

    /// Load configuration from default location or return a helpful error message
    ///
    /// This is a convenience method that provides a user-friendly error message
    /// if the config file is not found.
    pub fn load_or_exit() -> Self {
        match Self::load() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error loading configuration: {}", e);
                eprintln!("\nMake sure you have:");
                eprintln!("1. Created a config.toml file in the project root");
                eprintln!("2. Used config.toml.example as a template:");
                eprintln!("   cp config.toml.example config.toml");
                eprintln!("3. Updated the values in config.toml with your test data");
                std::process::exit(1);
            }
        }
    }
}
