# Repliers API Client for Rust

A simple asynchronous Rust client library for the [Repliers Real Estate API](https://docs.repliers.io/).

## Important Notice

**This is a demonstration repository and proof-of-concept implementation** showing how to interact with the Repliers Real Estate API using Rust. This project is intended for:

- Educational purposes
- Demonstrating Rust API client patterns
- Providing code examples for developers learning to integrate with the Repliers API
- Showcasing async/await patterns in Rust

**This is NOT a production-ready library** and should be used as a reference implementation only.

### Zero to Rust in 5 minutes

See below for more detailed documentation, but for the simplest way to get started with from a blank slate, follow these steps:

```bash
# 1. Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone this repo and enter it
git clone https://github.com/  && cd rust-api

# 3. Copy environment template to .env
cp .env.example .env

# 4. Add your API key to .env in a code editor or via cli
echo "REPLIERS_API_KEY=your_key_here" > .env

# 5. Copy config template to config.toml (the default values will work with a demo API key)
cp config.toml.example config.toml

# 6. Run the comprehensive demo example (the app will build automatically)
cargo run --example comprehensive_demo
```

## Limitations

**Important**: Endpoints Not Working in Examples:

- **AI Listings Search (POST /nlp)** - Requires production API key
- **Get Address History (GET /listings/history)** - Returns 404 despite well-formed requests?

The following endpoints work with test API keys:
- Listings Search (POST /listings) ✓
- Get Single Listing (GET /listings/{mlsNumber}) ✓
- Get Similar Listings (GET /listings/{mlsNumber}/similar) ✓
- Get Deleted Listings (GET /listings/deleted) ✓

## Project Status

6 API endpoints implemented with working examples (4 fully testable with demo keys, 2 not working)

## Overview

This demonstration library provides a Rust interface to the Repliers API, which offers access to MLS (Multiple Listing Service) data including property listings, AI-powered search, historical data, and more.

### Key Features (Demonstration Purposes)

- **Async/await** support using Tokio
- **Listing endpoint coverage** for all 6 major Repliers API Listing endpoints
- **Error handling** with custom error types using `thiserror`
- **Environment-based configuration** for API keys
- **Comprehensive examples** for each endpoint

### Purpose and Scope

This repository serves as a **proof-of-concept** and **educational resource** for developers who want to:
- Understand the Repliers API structure and endpoints
- See practical examples of error handling and type safety in Rust
- Use as a starting point for their own Repliers API integration

## API Endpoints

The library implements the following Repliers API endpoints:

1. **Listings Search (POST)** - Search properties with filters (city, price, bedrooms, etc.) ✓ Works with demo keys
2. **AI Listings Search (POST)** - Natural language property search using AI ⚠️ Requires production API key
3. **Get Single Listing (GET)** - Retrieve detailed info for a specific property by MLS number ✓ Works with demo keys
4. **Get Similar Listings (GET)** - Find properties similar to a given listing ✓ Works with demo keys
5. **Get Address History (GET)** - Retrieve complete MLS history for an address ⚠️ Returns 404 consistently
6. **Get Deleted Listings (GET)** - Retrieve listings removed from MLS ✓ Works with demo keys


## Getting Started

Install Rust via Rustup if necessary [via the official instructions for your system.](https://rust-lang.org/tools/install/)

On most linux flavors where `curl` is available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Setup

```bash
# 1. Copy environment template
cp .env.example .env

# 2. Add your API key to .env
echo "REPLIERS_API_KEY=your_key_here" > .env

# 3. Copy config template
cp config.toml.example config.toml

# 4. Edit config.toml with your test data (optional but recommended)
# Update MLS numbers, addresses, cities, etc. to match data accessible via your API key
```

### Configuration File

All examples use a configuration file (`config.toml`) to store test data and parameters. This approach:

- **Separates test data from code** - No hardcoded MLS numbers or addresses in examples
- **Makes examples reusable** - Update config once, all examples use your data
- **Prevents accidental commits** - `config.toml` is gitignored (only `config.toml.example` is tracked)

The configuration file includes:
- **Search parameters** - Cities, price ranges, property types for search examples
- **MLS numbers** - Valid listing IDs for your accessible MLS boards
- **Addresses** - Real addresses for history lookups
- **Discovery settings** - Cities and property types for database analysis
- **Date ranges** - For deleted listings queries

**Example config.toml structure:**
```toml
[search.example1]
city = "Toronto"
status = ["Active"]
min_price = 500000.0
max_price = 1000000.0
bedrooms = 3
property_type = ["Condo"]

[listing]
mls_number = "RTC2788401"

[address_history]
street_number = "2612"
street_name = "N Mt. Juliet Road"
city = "Mt. Juliet"
state = "TN"
```

See `config.toml.example` for the complete configuration template with all available options.

### Running Examples

4 of the "Listings" API endpoints have working examples, and two examples are currently not functional. 

```bash
# 1. Basic listings search with filters ✓ Works with demo API key
cargo run --example search_listings

# 2. AI-powered natural language search ⚠️ REQUIRES PRODUCTION API KEY
cargo run --example ai_search

# 3. Get detailed info for a single listing ✓ Works with demo API key
cargo run --example get_listing

# 4. Find properties similar to a given listing ✓ Works with demo API key
cargo run --example similar_listings

# 5. Get complete MLS history for an address ⚠️ ALWAYS RETURNS 404??
cargo run --example address_history

# 6. Retrieve deleted/removed listings ✓ Works with demo API key
cargo run --example deleted_listings

# Database introspection example ✓ Works with demo API key
cargo run --example discovery

# Export listings to JSON file ✓ Works with demo API key
cargo run --example export_listings

# COMPREHENSIVE DEMO - All 4 working endpoints in one! ✓ Works with demo API key
# This demo showcases all working endpoints with rich output and JSON export
cargo run --example comprehensive_demo
```

**Note**: Examples marked with ⚠️ not fully implemented.

### Comprehensive Demo

The `comprehensive_demo` example is a special showcase that demonstrates all 4 working endpoints in a single execution:

1. **Search Listings** - Find properties matching criteria
2. **Get Single Listing** - Fetch detailed information for a specific property
3. **Find Similar Listings** - Locate properties similar to a reference listing
4. **Get Deleted Listings** - Retrieve recently deleted MLS listings

**Features:**
- Rich, formatted console output with section headers and summaries
- Complete JSON report export with all results and metadata
- Execution time tracking and statistics
- Perfect for API demonstrations and testing

**Configuration:**
The demo uses the `[demo]` section in `config.toml`. All parameters are customizable:
```toml
[demo]
city = "Toronto"
status = ["Active"]
min_price = 500000.0
max_price = 1000000.0
bedrooms = 2
property_type = ["Condo"]
results_per_page = 10
mls_number = "RTC2788401"
similar_radius = 5.0
similar_price_range = 50000.0
deleted_min_date = "2025-01-01"
deleted_max_date = "2025-10-31"
deleted_results_per_page = 20
output_file = "api_demo_report.json"
```

**Output:**
- Console: Rich formatted output with statistics and sample data
- File: Complete JSON report saved to configured output file (default: `api_demo_report.json`)

### Running Tests

```bash
cargo test
```

## Dependencies

The project will use:

- `reqwest` (0.12) - HTTP client with async support
- `serde` (1.0) - Serialization/deserialization
- `serde_json` (1.0) - JSON support
- `tokio` (1.0) - Async runtime
- `thiserror` (1.0) - Error handling
- `dotenvy` - Environment variable loading
- `mockito` (dev) - HTTP mocking for tests

## API Reference

See [Repliers API Documentation](https://docs.repliers.io/) for detailed API information.

Key documentation links:
- [Authentication Guide](https://help.repliers.com/en/article/repliers-api-authentication-guide-1pmm1p2/)
- [Search & Filtering Guide](https://help.repliers.com/en/article/searching-filtering-and-pagination-guide-1q1n7x0)
- [API Reference](https://docs.repliers.io/reference/get-a-listing)
