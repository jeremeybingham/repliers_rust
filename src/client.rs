//! Repliers API client implementation
//!
//! This module contains the main client struct and methods for interacting
//! with the Repliers API.

use crate::error::RepliersError;
use reqwest::Client;

/// The main client for interacting with the Repliers API
///
/// # Examples
///
/// ```no_run
/// use repliers_beta::RepliersClient;
///
/// let client = RepliersClient::new("your_api_key".to_string());
/// ```
pub struct RepliersClient {
    /// HTTP client for making requests
    client: Client,
    /// API key for authentication
    api_key: String,
    /// Base URL for the Repliers API
    base_url: String,
}

impl RepliersClient {
    /// Creates a new Repliers API client with the given API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Repliers API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use repliers_beta::RepliersClient;
    ///
    /// let client = RepliersClient::new("your_api_key".to_string());
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.repliers.io".to_string(),
        }
    }

    /// Creates a new client by reading the API key from the environment
    ///
    /// Looks for the `REPLIERS_API_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable is not set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use repliers_beta::RepliersClient;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> Result<Self, RepliersError> {
        dotenvy::dotenv().ok(); // Load .env file if present

        let api_key =
            std::env::var("REPLIERS_API_KEY").map_err(|_| RepliersError::InvalidApiKey)?;

        Ok(Self::new(api_key))
    }

    /// Returns a reference to the base URL
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns a reference to the HTTP client
    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    /// Returns a reference to the API key
    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Validates that the API key is not empty
    ///
    /// # Errors
    ///
    /// Returns `RepliersError::InvalidApiKey` if the API key is empty
    pub fn validate_api_key(&self) -> Result<(), RepliersError> {
        if self.api_key.is_empty() {
            return Err(RepliersError::InvalidApiKey);
        }
        Ok(())
    }

    /// Helper method to check HTTP response status and handle errors
    ///
    /// # Errors
    ///
    /// Returns `RepliersError::ApiError` if the response status is not successful
    pub(crate) async fn check_response(
        response: reqwest::Response,
    ) -> Result<reqwest::Response, RepliersError> {
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(RepliersError::ApiError(format!(
                "Status {}: {}",
                status, error_text
            )));
        }
        Ok(response)
    }

    /// Helper method to create a GET request with standard headers
    ///
    /// Sets up the request with API key authentication and Content-Type header.
    pub(crate) fn get_request(&self, url: &str) -> reqwest::RequestBuilder {
        self.client()
            .get(url)
            .header("REPLIERS-API-KEY", self.api_key())
            .header("Content-Type", "application/json")
    }

    /// Helper method to create a POST request with standard headers
    ///
    /// Sets up the request with API key authentication and Content-Type header.
    pub(crate) fn post_request(&self, url: &str) -> reqwest::RequestBuilder {
        self.client()
            .post(url)
            .header("REPLIERS-API-KEY", self.api_key())
            .header("Content-Type", "application/json")
    }

    // Note: Endpoint methods are implemented in separate modules under src/endpoints/
    // - search_listings (endpoints/search.rs)
    // - ai_search_listings (endpoints/nlp.rs)
    // - get_listing (endpoints/listing.rs)
    // - get_similar_listings (endpoints/similar.rs)
    // - get_address_history (endpoints/history.rs)
    // - get_deleted_listings (endpoints/deleted.rs)
}
