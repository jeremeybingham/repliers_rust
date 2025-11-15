//! Natural Language Processing (AI search) models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for AI-powered natural language search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPSearchRequest {
    /// Natural language search prompt
    pub prompt: String,

    /// Optional board ID for multi-MLS accounts
    #[serde(skip_serializing_if = "Option::is_none", rename = "boardId")]
    pub board_id: Option<String>,
}

/// Response from AI search containing structured parameters
#[derive(Debug, Clone, Deserialize)]
pub struct NLPSearchResponse {
    /// Constructed API URL for the search
    pub url: String,

    /// Extracted parameters from the natural language prompt
    pub params: HashMap<String, serde_json::Value>,

    /// Original prompt that was processed
    pub prompt: String,
}
