//! Error types for the Repliers API client

use thiserror::Error;

/// Error types that can occur when using the Repliers API client
#[derive(Error, Debug)]
pub enum RepliersError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API returned error: {0}")]
    ApiError(String),

    /// Invalid or missing API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Failed to parse response
    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

// Note: Additional error types can be added as needed for more granular error handling:
// - RateLimitExceeded: For HTTP 429 rate limiting errors
// - ResourceNotFound: For HTTP 404 errors when a listing is not found
// - ValidationError: For invalid request parameters
// - AuthenticationError: For HTTP 401/403 authentication/authorization failures
//
// The current error types provide sufficient coverage for the proof-of-concept implementation.
