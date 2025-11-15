//! Repliers API Client for Rust
//!
//! A demonstration/proof-of-concept type-safe, asynchronous Rust client library for the
//! [Repliers Real Estate API](https://docs.repliers.io/).
//!
//! # ⚠️ Important Notice
//!
//! **This is a demonstration repository and proof-of-concept implementation** for educational
//! purposes only. This project is intended to:
//!
//! - Demonstrate Rust API client patterns
//! - Provide code examples for developers learning to integrate with the Repliers API
//! - Showcase async/await patterns in Rust
//!
//! **This is NOT a production-ready library** and should be used as a reference implementation only.
//!
//! # API Key Limitations
//!
//! Some endpoints require a production API key and will not work with test/demo API keys:
//!
//! - **AI Listings Search (POST /nlp)** - ⚠️ Requires production API key
//! - **Get Address History (GET /listings/history)** - ⚠️ Requires production API key
//!
//! The following endpoints work with test API keys:
//! - Listings Search (POST /listings) ✓
//! - Get Single Listing (GET /listings/{mlsNumber}) ✓
//! - Get Similar Listings (GET /listings/{mlsNumber}/similar) ✓
//! - Get Deleted Listings (GET /listings/deleted) ✓
//!
//! # Features
//!
//! - Type-safe API with strongly-typed requests and responses
//! - Async/await support using Tokio
//! - Complete endpoint coverage for all 6 major Repliers API endpoints
//! - Custom error types with detailed error information
//! - Environment-based configuration for API keys
//!
//! # Examples
//!
//! ```no_run
//! use repliers_beta::RepliersClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load API key from environment
//!     let client = RepliersClient::from_env()?;
//!
//!     // Note: This example requires a production API key
//!     // For demo keys, use search_listings(), get_listing(), etc.
//!
//!     Ok(())
//! }
//! ```

// Re-export main types
pub use client::RepliersClient;
pub use error::RepliersError;

// Module declarations
pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod models;

// Re-export commonly used types
pub use models::*;
