//! Address history and deleted listing models

use serde::{Deserialize, Serialize};

/// Query parameters for address history
///
/// **Important**: The API requires `street_number` and `street_name`, plus at least one of
/// `city` or `zip`. Requests missing these required fields will return an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressHistoryQuery {
    /// Street number (e.g., "2031") - **Required**
    #[serde(rename = "streetNumber")]
    pub street_number: String,

    /// Street name (e.g., "N. Mt. Juliet Road") - **Required**
    #[serde(rename = "streetName")]
    pub street_name: String,

    /// City name - **At least one of city or zip is required**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// ZIP code - **At least one of city or zip is required**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// State (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Board ID for multi-MLS accounts
    #[serde(skip_serializing_if = "Option::is_none", rename = "boardId")]
    pub board_id: Option<String>,
}

/// Response containing address history
#[derive(Debug, Clone, Deserialize)]
pub struct AddressHistoryResponse {
    /// Historical listing entries for the address
    pub history: Vec<HistoryEntry>,

    /// The address that was searched
    pub address: String,
}

/// A single entry in the address history
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryEntry {
    /// MLS number for this listing
    pub mls_number: String,

    /// Listing price
    pub list_price: Option<f64>,

    /// Sold price (if sold)
    pub sold_price: Option<f64>,

    /// Status of the listing
    pub status: String,

    /// Date when listed
    pub list_date: Option<String>,

    /// Date when sold
    pub sold_date: Option<String>,

    /// Property type
    pub property_type: Option<String>,

    /// Number of bedrooms
    pub bedrooms: Option<u32>,

    // Note: Additional fields available from the API that could be added:
    // - bathrooms: Number of bathrooms
    // - square_footage: Property size
    // - days_on_market: How long the listing was active
    // - listing_agent: Agent information
    // - remarks: Property description/notes
    // - price_changes: History of price adjustments
    //
    // Add fields as needed based on specific use case requirements.
}

/// Query parameters for deleted listings
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedListingsQuery {
    /// Get deletions from a specific date (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedOn")]
    pub updated_on: Option<String>,

    /// Minimum date for deletions (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none", rename = "minUpdatedOn")]
    pub min_updated_on: Option<String>,

    /// Maximum date for deletions (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxUpdatedOn")]
    pub max_updated_on: Option<String>,

    /// Board ID for multi-MLS accounts
    #[serde(skip_serializing_if = "Option::is_none", rename = "boardId")]
    pub board_id: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Results per page
    #[serde(skip_serializing_if = "Option::is_none", rename = "resultsPerPage")]
    pub results_per_page: Option<u32>,
}

/// Response containing deleted listings
#[derive(Debug, Clone, Deserialize)]
pub struct DeletedListingsResponse {
    /// Deleted listings
    pub listings: Vec<DeletedListing>,

    /// Current page
    pub page: u32,

    /// Total pages
    #[serde(rename = "numPages")]
    pub num_pages: u32,

    /// Page size (number of results per page)
    #[serde(rename = "pageSize")]
    pub page_size: u32,

    /// Total count
    pub count: u32,
}

/// Address structure for deleted listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedListingAddress {
    /// Area/county
    pub area: Option<String>,

    /// City
    pub city: Option<String>,

    /// Country
    pub country: Option<String>,

    /// District
    pub district: Option<String>,

    /// Major intersection
    #[serde(rename = "majorIntersection")]
    pub major_intersection: Option<String>,

    /// Neighborhood
    pub neighborhood: Option<String>,

    /// Street direction (N, S, E, W, etc.)
    #[serde(rename = "streetDirection")]
    pub street_direction: Option<String>,

    /// Street name
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,

    /// Street number
    #[serde(rename = "streetNumber")]
    pub street_number: Option<String>,

    /// Street suffix (Road, Drive, etc.)
    #[serde(rename = "streetSuffix")]
    pub street_suffix: Option<String>,

    /// Unit number
    #[serde(rename = "unitNumber")]
    pub unit_number: Option<String>,

    /// ZIP code
    pub zip: Option<String>,

    /// State
    pub state: Option<String>,

    /// Community code
    #[serde(rename = "communityCode")]
    pub community_code: Option<String>,
}

/// Timestamp information for deleted listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedListingTimestamps {
    /// When the listing was last updated before deletion
    #[serde(rename = "listingUpdated")]
    pub listing_updated: String,
}

/// A deleted listing entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedListing {
    /// Board ID
    #[serde(rename = "boardId")]
    pub board_id: u32,

    /// MLS number
    #[serde(rename = "mlsNumber")]
    pub mls_number: String,

    /// Resource type
    pub resource: String,

    /// Property address (structured object)
    pub address: DeletedListingAddress,

    /// Timestamp information
    pub timestamps: DeletedListingTimestamps,
}
