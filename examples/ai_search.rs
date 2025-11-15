//! Example: AI-powered natural language search
//!
//! Demonstrates how to use natural language prompts for property search.
//!
//! Usage:
//!   cargo run --example ai_search
//!
//! Configuration:
//!   This example reads the AI search prompt from config.toml
//!   Copy config.toml.example to config.toml and adjust values as needed
//!
//! Note: This endpoint requires a production API key

use repliers_beta::{config::Config, RepliersClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from config.toml
    let config = Config::load_or_exit();

    let client = RepliersClient::from_env()?;

    let prompt = &config.ai_search.prompt;

    println!("Converting natural language prompt to API parameters...");
    println!("Prompt: {}\n", prompt);

    let result = client.ai_search_listings(prompt, None).await?;

    println!("✓ AI successfully converted prompt to API parameters!\n");
    println!("Converted to API URL: {}\n", result.url);
    println!("Extracted parameters:");
    for (key, value) in &result.params {
        println!("  {}: {:?}", key, value);
    }

    Ok(())
}
